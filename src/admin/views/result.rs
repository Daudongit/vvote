use jelly::Result;
use jelly::prelude::*;
use crate::models::{result::Results, RequestQParam};
use jelly::actix_web::{web::{Query, Path}, HttpRequest};

/// Display a listing of the result.
pub async fn index(request: HttpRequest, param: Query<RequestQParam>) -> Result<HttpResponse> {
    let current_page = param.page.unwrap_or(1);
    let db = request.db_pool()?;
    let results = Results::get_paginated_record(current_page, db).await?;
    request.render(200, "admin/votes.html", {
        let mut context = Context::new();
        context.insert("results", &results);
        context
    })
}

/// Display the specified election slot (result) from storage.
pub async fn show(request: HttpRequest, path: Path<(i32,)>) -> Result<HttpResponse> {
    let (id,) = path.into_inner();
    let db = request.db_pool()?;
    let election = Results::get_election_by_id(id, db).await?;
    let slots = Results::get_paginated_slot_record(id, db).await?;
    request.render(200, "admin/results.html", {
        let mut context = Context::new();
        context.insert("election", &election);
        context.insert("slots", &slots);
        context
    })
}

