use jelly::Result;
use jelly::prelude::*;
use jelly::actix_web::{web::{Query, Path}, HttpRequest};

use crate::models::result::Results;
use crate::admin::forms::RequestQParam;

/// Display a listing of the result.
pub async fn index(request: HttpRequest, param: Query<RequestQParam>) -> Result<HttpResponse> {
    let current_page = param.page.unwrap_or(1);
    let db = request.db_pool()?;
    let results = 
        Results::get_paginated_record(current_page, db).await?;
    request.render(200, "admin/votes.html", {
        let mut context = Context::new();
        context.insert("results", &results);
        context
    })
}

/// Display the specified election slot (result) from storage.
pub async fn show(request: HttpRequest, path: Path<(i32,)>, param: Query<RequestQParam>) 
    -> Result<HttpResponse> {
        let (id,) = path.into_inner();
        let db = request.db_pool()?;
        let current_page = param.page.unwrap_or(1);
    let election = Results::get_election_by_id(id, db).await?;
    let (slots, slot_ids) = 
        Results::get_paginated_slot_by_election_id(id, current_page, db).await?;
    let nominees = 
        Results::get_slot_nominees_by_election_id(slot_ids, id, db).await?;
    request.render(200, "admin/results.html", {
        let mut context = Context::new();
        context.insert("election", &election);
        context.insert("nominees", &nominees);
        context.insert("slots", &slots);
        context
    })
}
