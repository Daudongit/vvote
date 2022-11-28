use jelly::Result;
use jelly::prelude::*;
use jelly::guards::csrf::extractor::Csrf;
use crate::models::election::{Election, RequestQParam};
use jelly::actix_web::{web::{Query, Form, Path}, HttpRequest};
use crate::admin::forms::{ElectionForm as ElectionEntityForm, DeleteForm as DeleteEntityForm};

type DeleteForm = Csrf<Form<DeleteEntityForm>>;
type ElectionForm = Csrf<Form<ElectionEntityForm>>;


/// Display a listing of the elections.
pub async fn index(request: HttpRequest, param: Query<RequestQParam>) -> Result<HttpResponse> {
        //let user = request.user()?;
    let current_page = param.page.unwrap_or(1);
    let db = request.db_pool()?;
    let elections = Election::with_slot_paginate(current_page, db).await?;
    request.render(200, "admin/election/index.html", {
        let mut context = Context::new();
        context.insert("elections", &elections);
        context
    })
}

/// Store a newly created election in storage.
pub async fn store(request: HttpRequest, form: ElectionForm) -> Result<HttpResponse> {
    let mut form = form.into_inner();
    if form.is_valid() {
        let db = request.db_pool()?;
        Election::create(&mut form, db).await?;
        request.flash("success", "Election successfully created.")?;
    }else{
        request.flash_form(form.into_inner())?;
    }
    request.redirect("/admin/elections/")
}

/// Update the specified election in storage.
pub async fn update(request: HttpRequest, path: Path<(i32,)>, form: ElectionForm)->Result<HttpResponse>{
    let (id,) = path.into_inner();
    let mut form = form.into_inner();
    if form.is_valid() {
        let db = request.db_pool()?;
        if let Err(err) = Election::update(&mut form, id, db).await {
            request.flash("error", "Unable to update election.")?; dbg!(err);
        }else{
            request.flash("success", "Election successfully updated.")?;
        }
    }else{
        request.flash_form(form.into_inner())?;
    }
    request.redirect("/admin/elections/")
}

/// Remove the specified election from storage.
pub async fn destroy(request: HttpRequest, path: Path<(i32,)>, _: DeleteForm)->Result<HttpResponse>{
    let (id,) = path.into_inner();
    let db = request.db_pool()?;
    match Election::delete(id, db).await {
        Ok(_)=>{
            request.flash("success", "Election successfully deleted.")?;
        }
        Err(e)=>{
            request.flash("error", "An error occure while deleting election.")?; dbg!(e);
        }
    }
    request.redirect("/admin/elections/")
}
