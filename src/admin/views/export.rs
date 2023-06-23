use jelly::Result;
use jelly::prelude::*;
use jelly::actix_web::web::Path;

use crate::models::result::Results;

pub async fn index(request: HttpRequest, path: Path<(i32, i32)>) -> Result<HttpResponse> {
    let (election, position) = path.into_inner();
    let db = request.db_pool()?;
    let sheet_name = 
        Results::position_result_name((election, position), db).await?;
    let position_result = 
        Results::group_by_voter_election_position((election, position), db).await?;
    let position_result_bytes = 
        Results::position_result_bytes(position_result, sheet_name);
    Ok(HttpResponse::Ok()
    .content_type("application/vnd.openxmlformats-officedocument.spreadsheetml.sheet")
    .body(position_result_bytes?))
}

pub async fn show(request: HttpRequest, path: Path<(i32, i32)>) -> Result<HttpResponse> {
    let (election_id, slot_id) = path.into_inner();
    let db = request.db_pool()?;
    let sheet_name = 
        Results::position_election_title((slot_id, election_id), db).await?;
    let nominees_result_count = 
        Results::nominees_with_result_count((slot_id, election_id), db).await?;
    let nominee_result_bytes = 
        Results::nominees_result_bytes(nominees_result_count, sheet_name);
    Ok(HttpResponse::Ok()
    .content_type("application/vnd.openxmlformats-officedocument.spreadsheetml.sheet")
    .body(nominee_result_bytes?))
}

