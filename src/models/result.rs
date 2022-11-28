use excel::*;
use jelly::error::error::Error;
use jelly::chrono::NaiveDateTime;
use jelly::serde::{Deserialize, Serialize};
use jelly::sqlx::{self, postgres::{PgPool, PgRow}, FromRow, Row as _};
use crate::models::{PaginatedEntity, election::{Election, Slot, Pivot}, nominee::Nominee};

type ResultNomineeRow = Result<Vec<NomineeRow>, Error>;
type ResultPosition = Result<Vec<PositionResult>, Error>;
type ResultEntityPaginate = Result<PaginatedEntity<Results>, Error>;

#[derive(FromRow)]
pub struct NomineeRow {
    result_count: i32,
    #[sqlx(flatten)]
    nominee: Nominee
}

#[derive(FromRow)]
pub struct PositionResult {
    voter: String,
    election_title: String,
    position_name: String,
    nominee_name: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Results{
    id: i32,
    voter_id: i32, 
    nominee_id: i32, 
    election_id: i32, 
    position_id: i32,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime 
}

impl FromRow<'_, PgRow> for Results {
    fn from_row(row: &PgRow) -> sqlx::Result<Self> {
        Ok(Self {
            id: row.try_get("id")?,
            voter_id: row.try_get("voter_id")?,
            nominee_id: row.try_get("nominee_id")?,
            election_id: row.try_get("election_id")?,
            position_id: row.try_get("position_id")?,
            created_at: row.try_get("created_at")?,
            updated_at: row.try_get("updated_at")?
        })
    }
}

impl Results{
    pub async fn vote_count(pool: &PgPool)->Result<i64, Error>{
        let sub_sql = "
            select voter_id, election_id, position_id, string_agg(id::text, ',') as ids 
            from results 
            group by voter_id, election_id, position_id
        ";
        let sql = format!(
            "select count(voter_id) as vote_count from ({}) as results_count", &sub_sql
        );
        Ok(sqlx::query(&sql).fetch_one(pool).await?.get("vote_count"))
    }

    pub async fn get_paginated_record(page: u16, pool: &PgPool)-> ResultEntityPaginate {
        let (offset, items_per_page) = super::calculate_pagination_offset(page)?;
        let total_result_count = super::get_aggregate_count("results", pool).await?;
        let sql = format!(
            "select * from results order by updated_at desc limit {} offset {}", 
            items_per_page, offset
        );
        let results: Vec<Results> = 
            sqlx::query_as(sql.as_str()).fetch_all(pool).await?;
        Ok((page, total_result_count as u64, results).into())
    }

    pub async fn get_election_by_id(id: i32, pool: &PgPool)->Result<Election, Error>{
        let election = sqlx::query!("select * from elections where id=$1", id)
            .fetch_one(pool).await?;
        let title = election.title.ok_or_else(
            ||Error::Generic("title can not be none".into())
        )?;
        let end = election.end.ok_or_else(
            ||Error::Generic("end_date can not be none".into())
        )?;
        let created_at = election.created_at.ok_or_else(
            ||Error::Generic("created_at can not be none".into())
        )?;
        let updated_at = election.updated_at.ok_or_else(
            ||Error::Generic("updated_at can not be none".into())
        )?;
        Ok(Election{
            end,
            title,
            created_at,
            updated_at,
            id: election.id,
            results_count: 0,
            start: election.start,
            status: election.status,
            can_see_result: election.can_see_result
        })
    }

    pub async fn get_paginated_slot_record(id: i32, pool: &PgPool)->Result<Slot, Error>{
        let slot = sqlx::query!("
            select slots.*, election_slot.election_id as pivot_election_id,
             election_slot.slot_id as pivot_slot_id 
            from slots
            inner join election_slot on slots.id = election_slot.slot_id
            where election_slot.election_id=$1
        ", id).fetch_one(pool).await?;
        let created_at = slot.created_at.ok_or_else(
            ||Error::Generic("created_at can not be none".into())
        )?;
        let updated_at = slot.updated_at.ok_or_else(
            ||Error::Generic("updated_at can not be none".into())
        )?;
        Ok(Slot{
            id: slot.id,
            created_at,
            updated_at,
            position_id: slot.position_id,
            pivot: Pivot{
                election_id: id, 
                slot_id: slot.pivot_slot_id
            }
        })
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
            having election_id=$1 and position_id=$2"
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
            inner join nominee_slot on nominees.id = nominee_slot.slot_id
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

    pub async fn position_result_name(param: (i32, i32), pool: &PgPool)->Result<String, Error>{
        let (election_id, position_id) = param;
        let position_name = sqlx::query!(
            "select name from positions where id=$1", position_id
        ).fetch_one(pool).await?.name;
        let election_title = sqlx::query!(
            "select title from elections where id=$1", election_id
        ).fetch_one(pool).await?.title;
        let position_name = position_name //.unwrap_or_default()
            .to_lowercase().replace(' ', "_");
        let election_title = election_title.unwrap_or_default()
            .to_lowercase().replace(' ', "_");
        Ok(format!("{election_title}_{position_name}"))
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

