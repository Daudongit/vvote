use jelly::Result;
use jelly::prelude::*;
use crate::models::election::{Election, RequestQParam};
use jelly::actix_web::{web::Query, HttpRequest};


pub async fn index(request: HttpRequest, param: Query<RequestQParam>) -> Result<HttpResponse> {
        //let user = request.user()?;
    let current_page = param.page.unwrap_or(1);
    let db = request.db_pool()?;
    let elections = Election::get_with_slot_paginated(current_page, db).await?;
    // eprintln!("{:?}",elections);
    request.render(200, "admin/election/index.html", {
        let mut context = Context::new();
        context.insert("elections", &elections);
        context
    })
}

pub async fn store(request: HttpRequest) -> Result<HttpResponse> {
    
    request.render(200, "admin/election/index.html", {
        let context = Context::new();
        context
    })
}

pub async fn update(request: HttpRequest) -> Result<HttpResponse> {
    
    request.render(200, "admin/election/index.html", {
        let context = Context::new();
        context
    })
}

pub async fn destroy(request: HttpRequest) -> Result<HttpResponse> {
    
    request.render(200, "admin/election/index.html", {
        let context = Context::new();
        context
    })
}
