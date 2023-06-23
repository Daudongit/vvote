use std::collections::HashMap;

use jelly::serde::Serialize;
use futures::TryStreamExt as _;
use jelly::error::error::Error;
use jelly::forms::DateTimeField;
use jelly::sqlx::postgres::{PgRow, PgPool};
use jelly::sqlx::{self, QueryBuilder, Postgres, Row as _};
use jelly::chrono::{NaiveDateTime, Utc, DateTime, TimeZone};

use crate::models::PaginatedEntity;
use crate::admin::forms::{ElectionForm, ToggleElectionForm, ElectionVisibilityForm};

type VisibilityForm = ElectionVisibilityForm;
type ResultMap = Result<HashMap<i32, Vec<SlotId>>, Error>;
type ResultElection = Result<HashMap<i32, Vec<SlotId>>, Error>;
type ResultDate = Result<(NaiveDateTime, NaiveDateTime), Error>;
type ResultElectionIds = Result<(Vec<i32>,Vec<Election>), Error>;
type ResultSlotNominees = Result<HashMap<i32, Vec<Nominee>>, Error>;
type ResultSlotPosition = Result<(Vec<i32>, Vec<SlotPosition>), Error>;
type ResultElectionPaginated = Result<PaginatedEntity<Election>, Error>;
type ResultElectionDateStatus = Result<(i32, ElectionStatus, (NaiveDateTime, NaiveDateTime, String)), Error>;


#[derive(Debug, Serialize)]
pub struct PaginatedElection {
    current_page: u16,
    data: (Vec<Election>, HashMap<i32, Vec<SlotId>>),
    per_page: u8,
    total: u64
}

#[derive(Debug, Serialize)]
pub struct Nominee{
    id: i32,
    first_name: String,
    last_name: String,
    description: String,
    image: String
}

#[derive(Debug, Serialize)]
pub struct Slot {
    pub id: i32,
    pub position_name: String
}

#[derive(Debug, Serialize)]
pub struct SlotId(pub i32);

#[derive(PartialEq)]
struct SlotIds{slot_id:i32}

#[derive(Debug, Serialize)]
pub struct SlotPosition{
    slot_id: i32,
    position_id: i32,
    position_name: Option<String>
}

#[derive(Debug, Serialize)]
pub struct ElectionStatus {
    is_locked: bool,
    is_open: bool,
    has_started: bool,
    is_end: bool,
    is_running: bool,
    is_comingsoon: bool
}

#[derive(Debug, Serialize)]
pub struct Election {
    pub id: i32,
    pub title: String,
    pub status: i32,
    pub statuses: ElectionStatus,
    pub start: NaiveDateTime,
    pub end: NaiveDateTime,
    pub can_see_result: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub results_count: i64,
    pub end_timeago: String,
    pub slots_count: i64
}

struct ElectionRow(i32, Election);

impl ElectionRow{
    fn compute_status<Tz>(status:i32, start_date:&DateTime<Tz>, end_date:&DateTime<Tz>)
        -> ElectionStatus where Tz:TimeZone<Offset=Utc> {
        let now = &Utc::now();
        let is_locked = status == 0;
        let is_open = !is_locked;
        let has_started = start_date <= now;
        let is_end = now > end_date;
        let is_running = is_open && has_started;
        let is_comingsoon =  is_open && (now < start_date);
        ElectionStatus{
            is_comingsoon, is_end, is_locked, is_open, is_running, has_started
        }
    }

    fn naive_date_to_utc<Tz>(start_date:NaiveDateTime, end_date:NaiveDateTime)
        ->(DateTime<Tz>, DateTime<Tz>) where Tz:TimeZone<Offset=Utc> {
        let start_date = 
            DateTime::<Tz>::from_utc(start_date, Utc);
        let end_date = 
            DateTime::<Tz>::from_utc(end_date, Utc);
        (start_date, end_date)
    }

    fn format_date_with_status(row: &PgRow)->ResultElectionDateStatus{
        let id = row.try_get("id")?;
        let status = row.try_get("status")?;
        let end:NaiveDateTime = row.try_get("end")?;
        let start:NaiveDateTime = row.try_get("start")?;
        let (start_date, end_date) = 
            Self::naive_date_to_utc::<Utc>(start, end);
        let statuses = Self::compute_status(status, &start_date, &end_date);
        let now = Utc::now();
        let from = if now > end_date {end_date}else{now};
        let to = if now > end_date {now}else{end_date};
        let mut end_timeago = timeago::Formatter::new()
            .convert_chrono(from, to);
        if end_date > now {
            end_timeago = end_timeago.replace("ago", "from now");
        }
        Ok((id, statuses, (start, end, end_timeago)))
    }
}

impl TryFrom<PgRow> for ElectionRow{
    type Error = Error;

    fn try_from(row: PgRow) -> Result<Self, Self::Error> {
        let (id, statuses, dates) = 
            Self::format_date_with_status(&row)?;
        let (start, end,  end_timeago) = dates;
        let election =  Election{
            id, end, start, statuses, end_timeago,
            status: row.try_get("status")?,
            title: row.try_get("title")?,
            can_see_result: row.try_get("can_see_result")?,
            created_at: row.try_get("created_at")?,
            updated_at: row.try_get("updated_at")?,
            results_count: row.try_get("results_count")?,
            slots_count: row.try_get("slots_count").unwrap_or(0)
        };
        Ok(Self(id, election))
    }
}

impl Election{
    pub async fn count(pool: &PgPool)->Result<i64, Error>{
        Ok(sqlx::query!("select count(*) as election_count from elections")
        .fetch_one(pool).await?.election_count.unwrap_or(0))
    }

    pub async fn paginate(page: u16, pool: &PgPool)->ResultElectionPaginated{
        let total_election_count = 
            super::get_aggregate_count("elections", pool).await?;
        let (offset, items_per_page) = 
            super::calculate_pagination_offset(page)?;
        let sql:String = format!("select elections.*, (
            select count(*) from results 
            where elections.id = results.election_id
        ) as results_count, (
            select count(election_slot.slot_id) from election_slot 
            where elections.id = election_slot.election_id
        ) as slots_count from elections order by updated_at, id desc 
        limit {} offset {}", items_per_page, offset); 
        let (_, elections) = 
            Self::get_processed_record(sql, pool).await?;
        Ok((page, total_election_count as u64, elections).into())
    }

    pub async fn with_slot_paginate(page: u16, pool: &PgPool)
        ->Result<PaginatedElection, Error>{
        let items_per_page = super::items_per_page();
        let total_election_count = 
            super::get_aggregate_count("elections", pool).await?;
        let (election_ids, elections) = 
            Self::get_paginated_record(page, pool).await?;
        let election_slots = 
            Self::get_election_slots(election_ids, pool).await?;
        Ok(
            PaginatedElection{
                current_page:page,
                data:(elections, election_slots),
                per_page:items_per_page,
                total:total_election_count as u64,
            }
        )
    }

    pub async fn get_paginated_record(page: u16, pool: &PgPool)->ResultElectionIds{
        let (offset, items_per_page) = 
            super::calculate_pagination_offset(page)?;
        let sql:String = format!("select elections.*, (
            select count(*) from results where elections.id = results.election_id
        ) as results_count from elections order by created_at, id asc 
        limit {} offset {}", items_per_page, offset);
        let (election_ids, elections) = 
            Self::get_processed_record(sql, pool).await?;
        Ok((election_ids, elections))
    }

    pub async fn get_election_slots(election_ids: Vec<i32>, pool: &PgPool)->ResultElection{
        let sql: String = "
            select slots.id, election_slot.election_id as pivot_election_id from slots 
            inner join election_slot on slots.id = election_slot.slot_id
            where election_slot.election_id = any($1)
        ".into();
        let slots = 
            Self::process_slot_record(sql, election_ids, pool).await?;
        Ok(slots)
    }

    pub async fn get_slot_ids_positions(election_id: i32, pool: &PgPool) -> ResultSlotPosition{
        let mut positions = vec![];
        let mut slot_ids = vec![];
        let mut slot_positions = sqlx::query!(
            "select slots.id, slots.position_id, (
                select name from positions where positions.id=slots.position_id
            ) as position_name from slots
            inner join election_slot on slots.id = election_slot.slot_id
            where election_slot.election_id = $1", election_id
        ).fetch(pool);
        while let Some(row) = slot_positions.try_next().await? {
            let slot_id = row.id;
            slot_ids.push(slot_id);
            positions.push(
                SlotPosition{
                    slot_id, position_id: row.position_id, 
                    position_name: row.position_name
                }
            );
        }
        Ok((slot_ids, positions))
    }

    pub async fn get_slot_nominees(slot_ids: Vec<i32>, pool: &PgPool) ->ResultSlotNominees{
        let mut slot_nominees_qs = sqlx::query!(
            "select nominees.id, nominees.first_name, nominees.last_name, 
            nominees.description, nominees.image, nominee_slot.slot_id as pivot_slot_id
            from nominees inner join nominee_slot on nominees.id = nominee_slot.nominee_id
            where nominee_slot.slot_id = any($1)
            ", &slot_ids[..]
        ).fetch(pool);
        let mut slot_nominees:HashMap<i32, Vec<Nominee>> = HashMap::new();
        while let Some(row) = slot_nominees_qs.try_next().await? {
            let slot_id = row.pivot_slot_id;
            let nominee = Nominee{
                id: row.id,
                first_name: row.first_name,
                last_name: row.last_name,
                image: row.image.unwrap_or_default(),
                description: row.description.unwrap_or_default()
            };
            if let Some(nominees) = slot_nominees.get_mut(&slot_id) {
                nominees.push(nominee);
            } else {
                slot_nominees.insert(slot_id, vec![nominee]);
            }
        }
        Ok(slot_nominees)
    }

    pub async fn get_processed_record(sql: String, pool: &PgPool)->Result<(Vec<i32>,Vec<Election>), Error>{
        let mut election_ids: Vec<i32> = vec![];
        let mut elections: Vec<Election> = vec![];
        let mut rows = 
            sqlx::query(sql.as_str()).fetch(pool);
        while let Some(row) = rows.try_next().await? {
            let ElectionRow(id, election) = row.try_into()?;
            election_ids.push(id);
            elections.push(election);
        }
        Ok((election_ids, elections))
    }

    pub async fn process_slot_record(sql: String, ids: Vec<i32>, pool: &PgPool)->ResultMap{
        let mut slots: HashMap<i32, Vec<_>> = HashMap::new();
        let mut slot_rows = 
            sqlx::query(sql.as_str())
        .bind(&ids[..]).fetch(pool);
        while let Some(row) = slot_rows.try_next().await? {
            let election_id = row.try_get("pivot_election_id")?;
           let id = row.try_get("id")?;
           if let Some(election_slots) = slots.get_mut(&election_id){
                election_slots.push(SlotId(id));
           }else{
               slots.insert(election_id, vec![SlotId(id)]);
           }
        }
        Ok(slots)
    }

    pub async fn all_slot(pool: &PgPool)->Result<Vec<Slot>, Error>{
        let mut slots = vec![];
        let slot_rows = 
            sqlx::query!("
                select id, (
                    select name from positions where positions.id=slots.position_id
                ) as position_name from slots
            ").fetch_all(pool).await?;
            for row in slot_rows {
                let position_name =  
                    row.position_name.ok_or_else(||Error::Generic("Invalid name".into()))?;
                slots.push(Slot{ id: row.id, position_name })
            }
        Ok(slots)
    }

    pub async fn create(form: &mut ElectionForm, pool: &PgPool) -> Result<i32, Error> {
        let (start_date, end_date) = 
            Self::format_dates(&mut form.start_date, &mut form.end_date)?;
        let mut tx = pool.begin().await?;
        let election_id = sqlx::query!(
            "insert into elections (title, start, \"end\", created_at, updated_at) 
            values ($1, $2, $3, now(), now()) returning id",
            form.title.value, start_date, end_date
        ).fetch_one(&mut tx).await?.id;
        let mut query_builder = 
            Self::election_slot_qb(form.slots.clone(), election_id);
        query_builder.build().execute(&mut tx).await?;
        tx.commit().await?;
        Ok(election_id)
    }

    pub async fn update(form: &mut ElectionForm, id: i32, pool: &PgPool) -> Result<(), Error> {
        let (start_date, end_date) = 
            Self::format_dates(&mut form.start_date, &mut form.end_date)?;
        let mut tx = pool.begin().await?;
        sqlx::query!(
            "update elections set title=$1, start=$2, \"end\"=$3, updated_at=now() where id=$4", 
            form.title.value, start_date, end_date, id
        ).execute(&mut tx).await?;
        sqlx::query!("delete from election_slot where election_id=$1", id)
        .execute(&mut tx).await?;
        if !form.slots.is_empty() {
            let mut query_builder = 
                Self::election_slot_qb(form.slots.clone(), id);
            query_builder.build().execute(&mut tx).await?;
        }
        tx.commit().await?;
        Ok(())
    }

    pub async fn delete(id: i32, pool: &PgPool) -> Result<(), Error> {
        let mut tx = pool.begin().await?;
        sqlx::query!("delete from results where election_id=$1", id)
        .execute(&mut tx).await?;
        sqlx::query!("delete from election_slot where election_id=$1", id)
        .execute(&mut tx).await?;
        sqlx::query!("delete from ipvalidations where election_id=$1", id)
        .execute(&mut tx).await?;
        sqlx::query!("delete from elections where id=$1", id)
        .execute(&mut tx).await?;
        tx.commit().await?;
        Ok(())
    }

    pub async fn lock_unlock(id: i32, form: &ToggleElectionForm, pool: &PgPool)->Result<i32, Error>{
        let status = if form.status == 1 {0}else{1};
        sqlx::query!("update elections set status=$1, updated_at=now() where id=$2", status, id)
        .execute(pool).await?;
        Ok(status)
    }

    pub async fn set_visibility(id: i32, form: &VisibilityForm, pool: &PgPool)->Result<bool, Error>{
        let can_see_toggle = !form.can_see_result;
        sqlx::query!(
            "update elections set can_see_result=$1, updated_at=now() where id=$2",
            can_see_toggle, id
        )
        .execute(pool).await?;
        Ok(can_see_toggle)
    }

    fn election_slot_qb<'a>(slots: Vec<u64>, id: i32)->QueryBuilder<'a, Postgres>{
        let mut query_builder = QueryBuilder::new(
            "insert into election_slot (slot_id, election_id, created_at, updated_at) "
        );
        query_builder.push_values(
            slots, |mut b, slot| {
            b.push_bind(slot as i64)
                .push_bind(id)
                .push(" now() ")    
                .push(" now() ");
        });
        query_builder
    }

    fn format_dates(start_date: &mut DateTimeField, end_date: &mut DateTimeField) ->ResultDate {
        let start_date = start_date.date.take()
            .ok_or_else(|| Error::Generic("Invalid start date".into()))?;
            // .and_hms_opt(0, 0, 0).ok_or_else(|| Error::Generic("Invalid start time".into()))?;
        let end_date = end_date.date.take()
            .ok_or_else(|| Error::Generic("Invalid end date".into()))?;
        Ok((start_date, end_date))
    }
}
