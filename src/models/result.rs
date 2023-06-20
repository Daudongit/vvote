use std::collections::HashMap;

use excel::*;
use jelly::serde::Serialize;
use futures::TryStreamExt as _;
use jelly::error::error::Error;
use jelly::sqlx::{self, QueryBuilder, Postgres, postgres::PgPool, FromRow, Row as _};

use crate::models::{PaginatedEntity, nominee::Nominee};
use crate::front::forms::{ElectionFormVec, ElectionItem};

type Signature<'a> = (&'a str, (u64, String));
type ResultNomineeRow = Result<Vec<NomineeRow>, Error>;
type ResultPosition = Result<Vec<PositionResult>, Error>;
type ResultMap = Result<HashMap<i32, Vec<NomineeItem>>, Error>;
type ResultEntityPaginate = Result<PaginatedEntity<PositionResult>, Error>;
type ResultSlotPaginate = Result<(PaginatedEntity<Slot>, Vec<i32>), Error>;

#[derive(Debug, FromRow)]
pub struct NomineeRow {
    result_count: i32,
    #[sqlx(flatten)]
    nominee: Nominee
}

#[derive(Debug, FromRow, Serialize)]
pub struct PositionResult {
    voter: String,
    election_title: String,
    position_name: String,
    nominee_name: String
}

#[derive(Debug, Serialize)]
pub struct Slot {
    pub id: i32,
    pub position_id: i32,
    pub position_name: String
}

#[derive(Debug, Serialize)]
pub struct NomineeItem{
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub results_count: i64
}

#[derive(Debug, Serialize)]
pub struct Election {
    pub id: i32,
    pub title: String
}

pub struct Results;

impl Results{
    pub async fn vote_count(pool: &PgPool)->Result<i64, Error>{
        let sub_sql = "
            select voter_id, election_id, position_id, string_agg(id::text, ',') as ids 
            from results group by voter_id, election_id, position_id
        ";
        let sql = format!(
            "select count(voter_id) as vote_count from ({}) as results_count", &sub_sql
        );
        Ok(sqlx::query(&sql).fetch_one(pool).await?.get("vote_count"))
    }

    pub async fn get_paginated_record(page: u16, pool: &PgPool)-> ResultEntityPaginate {
        let (offset, items_per_page) = super::calculate_pagination_offset(page)?;
        let total_result_count = super::get_aggregate_count("results", pool).await?;
        let mut results_stream = sqlx::query!(
            r#"select (select case when ip is null then email else ip end
                from voters where voters.id=results.voter_id
            ) as "voter!",
            (select title from elections where elections.id=results.election_id) as "election_title!", 
            (select name from positions where positions.id=results.position_id) as "position_name!", 
            (select concat(first_name, ' ', last_name) as n_name 
                from nominees where nominees.id=results.nominee_id) as nominee_name 
            from results order by created_at, id asc limit $1 offset $2
            "#, items_per_page as i64, offset as i64
        ).fetch(pool);
        let mut results = Vec::new();
        while let Some(result_row) = results_stream.try_next().await? {
            let nominee_name = 
                result_row.nominee_name.unwrap_or_else(||"Unknow Candidate".into());
            results.push(PositionResult{
                voter: result_row.voter, election_title: result_row.election_title,
                position_name: result_row.position_name, nominee_name: nominee_name
            })
        }
        Ok((page, total_result_count as u64, results).into())
    }

    pub async fn get_election_by_id(id: i32, pool: &PgPool)->Result<Election, Error>{
        let election = sqlx::query!(
            "select id, title from elections where id=$1", id
        ).fetch_one(pool).await?;
        let title = election.title.ok_or_else(
            ||Error::Generic("title can not be none".into())
        )?;
        Ok(Election{title, id: election.id})
    }

    pub async fn get_paginated_slot_by_election_id(id: i32, page: u16, pool: &PgPool)
        ->ResultSlotPaginate{
        let mut slots = Vec::new();
        let mut slot_ids = Vec::new();
        let (offset, items_per_page) = super::calculate_pagination_offset(page)?;
        let total_result_count = Self::get_slot_aggregate_count(id, pool).await?;
        let mut slots_stream = sqlx::query!(r#"
            select slots.id, slots.position_id, (
                select name from positions where positions.id=slots.position_id
            ) as "position_name!" from slots
            inner join election_slot on slots.id = election_slot.slot_id
            where election_slot.election_id=$1 
            order by slots.updated_at desc limit $2 offset $3
        "#, id, items_per_page as i64, offset as i64).fetch(pool);
        while let Some(slot_row) = slots_stream.try_next().await? {
            let id = slot_row.id;
            let position_id = slot_row.position_id;
            slots.push(Slot{id, position_id, position_name: slot_row.position_name});
            slot_ids.push(id);
        }
        let paginated_slots = (page, total_result_count as u64, slots).into();
        Ok((paginated_slots, slot_ids))
    }

    pub async fn get_slot_nominees_by_election_id(slot_ids:Vec<i32>, id:i32, pool:&PgPool)->ResultMap{
        let mut nominees: HashMap<i32, Vec<_>> = HashMap::new();
        let mut nominee_rows = sqlx::query!(
            r#"select nominees.id, nominees.first_name, nominees.last_name, 
            nominees.email as "email!", nominee_slot.slot_id as pivot_slot_id, 
            (select count(*) from (select results.voter_id from results 
                where exists (select slots.position_id from slots 
                    where slots.position_id=results.position_id
                ) and nominees.id=results.nominee_id and results.election_id=$1 
            ) as nominee_votes) as "results_count!" from nominees
            inner join nominee_slot on nominees.id = nominee_slot.nominee_id
            where nominee_slot.slot_id = any($2)"#, id, &slot_ids[..]
        ).fetch(pool);
        while let Some(row) = nominee_rows.try_next().await? {
            let slot_id = row.pivot_slot_id;
            let nominee = NomineeItem{
                id: row.id, email: row.email, last_name: row.last_name,
                first_name: row.first_name, results_count: row.results_count
            };
            if let Some(slot_nominees) = nominees.get_mut(&slot_id) {
                slot_nominees.push(nominee);
            }else{
                nominees.insert(slot_id, vec![nominee]);
            }
        }
        Ok(nominees)
    }

    pub async fn group_by_voter_election_position(param:(i32, i32), pool: &PgPool)->ResultPosition{
        let (election_id, position_id) = param;
        let position_result:Vec<PositionResult> = sqlx::query_as(
            "select (select ip from voters where voters.id=results.voter_id) as voter,
            (select title from elections where elections.id=results.election_id) as election_title, 
            (select name from positions where positions.id=results.position_id) as position_name, 
            (select concat(first_name, ' ', last_name) as n_name 
                from nominees where nominees.id=results.nominee_id) as nominee_name  
            from results 
            group by voter_id, election_id, position_id, nominee_id
            where election_id=$1 and position_id=$2"
        ).bind(election_id).bind(position_id).fetch_all(pool).await?;
        Ok(position_result)
    }

    pub async fn nominees_with_result_count(param:(i32, i32), pool: &PgPool)->ResultNomineeRow{
        let (slot_id, election_id) = param;
        let nominee_rows:Vec<NomineeRow> = sqlx::query_as(
        "select nominees.*, (select count(*) from (
                select voter_id, election_id, position_id 
                from results where nominees.id=results.nominee_id
                group by voter_id, election_id, position_id having results.election_id=$1
            ) as nominee_votes) as result_count 
            from nominees
            inner join nominee_slot on nominees.id = nominee_slot.nominee_id
            where nominee_slot.slot_id = $2"
        ).bind(election_id).bind(slot_id).fetch_all(pool).await?;
        Ok(nominee_rows)
    }

    pub async fn position_election_title(param: (i32, i32), pool: &PgPool)->Result<String, Error>{
        let (slot_id, election_id) = param;
        let position_name = sqlx::query!(
            "select slots.id, slots.position_id, (
                select positions.name from positions where positions.id=slots.position_id
            ) as position_name 
            from slots where slots.id=$1", slot_id
        ).fetch_one(pool).await?.position_name;
        let election_title = sqlx::query!(
            "select title from elections where id=$1", election_id
        ).fetch_one(pool).await?.title;
        let position_name = position_name.unwrap_or_default()
            .to_lowercase().replace(' ', "_");
        let election_title = election_title.unwrap_or_default()
            .to_lowercase().replace(' ', "_");
        Ok(format!("{election_title}_{position_name}"))
    }

    pub async fn get_slot_aggregate_count(id: i32, pool: &PgPool)->Result<i64, Error>{
        let row = sqlx::query!(
            "select count(slots.id) from slots
            inner join election_slot on slots.id = election_slot.slot_id
            where election_slot.election_id=$1", id
        )
        .fetch_one(pool).await?;
        let aggregate = row.count.unwrap_or(0);
        Ok(aggregate)
    }

    pub async fn position_result_name(param: (i32, i32), pool: &PgPool)->Result<String, Error>{
        let (election_id, position_id) = param;
        let position_name = sqlx::query!(
            "select name from positions where id=$1", position_id
        ).fetch_one(pool).await?.name;
        let election_title = sqlx::query!(
            "select title from elections where id=$1", election_id
        ).fetch_one(pool).await?.title;
        let position_name = position_name
            .to_lowercase().replace(' ', "_");
        let election_title = election_title.unwrap_or_default()
            .to_lowercase().replace(' ', "_");
        Ok(format!("{election_title}_{position_name}"))
    }

    pub async fn check_signature_count(signature: Signature<'_>, pool: &PgPool)->Result<i64, Error>{
        let (voter_ip, (election_id, fingerprint_token)) = signature;
        let row = sqlx::query!(
            "select count(id) from results where voter_ip=$1 and election_id=$2",
            voter_ip, election_id as i32
        ).fetch_one(pool).await?;
        let mut aggregate = row.count.unwrap_or(0);
        if aggregate == 0{
            let row = sqlx::query!(
                "select count(id) from results where voter_code=$1 and election_id=$2",
                fingerprint_token, election_id as i32
            ).fetch_one(pool).await?;
            aggregate = row.count.unwrap_or(0);
        }
        Ok(aggregate)
    }

    pub async fn create<'a>(forms: ElectionFormVec<'a>, pool: &PgPool) -> Result<(), Error> {
        let forms = forms.into_inner();
        let mut tx = pool.begin().await?;
         sqlx::query!(
            "insert into ipvalidations (ip, election_id, created_at, updated_at) 
            values ($1, $2, now(), now())",
            forms[0].voter_ip, forms[0].election_id as i32
        ).execute(&mut tx).await?;
        let mut query_builder = Self::result_qb(forms);
        query_builder.build().execute(&mut tx).await?;
        tx.commit().await?;
        Ok(())
    }

    pub fn result_qb<'a>(forms: Vec<ElectionItem<'a>>)->QueryBuilder<'a, Postgres>{
        let mut query_builder = QueryBuilder::new(
            "insert into results (
                voter_id, position_id, nominee_id, election_id, voter_ip, 
                voter_code, created_at, updated_at
            ) "
        );
        query_builder.push_values(
            forms, |mut b, form| {
            b.push_bind(form.voter_id)
                .push_bind(form.position_id as i32)
                .push_bind(form.nominee_id as i32)
                .push_bind(form.election_id as i32)
                .push_bind(form.voter_ip)
                .push_bind(form.fingerprint_token.to_string())
                .push(" now() ")   
                .push(" now() ");
        });
        query_builder
    }

    pub fn nominees_result_bytes(nominee_rows: Vec<NomineeRow>, sheet_name: String)->Result<Vec<u8>, Error>{
        let mut wb = Workbook::create_in_memory();
        let sheet_name = format!("{sheet_name}_nominee_result");
        let mut sheet = wb.create_sheet(sheet_name.as_str());
        // set columns size if needed,
        wb.write_sheet(&mut sheet, |sheet_writer| {
            let sw = sheet_writer;
            sw.append_row(row!["id", "Candidate", "Description", "Vote Count"])?;
            for nominee_row in nominee_rows {
                let id = nominee_row.nominee.id.to_string();
                let first_name = nominee_row.nominee.first_name;
                let last_name = nominee_row.nominee.last_name;
                let candidate_name = format!("{first_name} {last_name}");
                let description = nominee_row.nominee.description;
                let results_count = nominee_row.result_count.to_string();
                sw.append_row(row![id, candidate_name, description, results_count])?;
            }
            Ok(())
        })?;

        let excel_bytes = wb.close()?;
        let excel_bytes = excel_bytes.ok_or_else(
            ||Error::Generic("unable to generate excel bytes".into())
        )?;
        Ok(excel_bytes)
    }

    pub fn position_result_bytes(position_rows: Vec<PositionResult>, sheet_name: String)->Result<Vec<u8>, Error>{
        let mut wb = Workbook::create_in_memory();
        let sheet_name = format!("{sheet_name}_position_result");
        let mut sheet = wb.create_sheet(sheet_name.as_str());
        // set columns size if needed,
        wb.write_sheet(&mut sheet, |sheet_writer| {
            let sw = sheet_writer;
            sw.append_row(row!["Cooperator", "Office", "Aspirant", "Election"])?;
            for position_row in position_rows {
                let cooperator = position_row.voter;
                let office = position_row.position_name;
                let aspirant = position_row.nominee_name ;
                let election = position_row.election_title;
                sw.append_row(row![cooperator, office, aspirant, election])?;
            }
            Ok(())
        })?;

        let excel_bytes = wb.close()?;
        let excel_bytes = excel_bytes.ok_or_else(
            ||Error::Generic("unable to generate excel bytes".into())
        )?;
        Ok(excel_bytes)
    }
}

