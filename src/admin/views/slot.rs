use jelly::Result;
use jelly::prelude::*;
use jelly::guards::csrf::extractor::Csrf;
use jelly::actix_web::{web::{Query, Form, Path}, HttpRequest};

use crate::models::slot::Slot;
use crate::admin::forms::RequestQParam;
use crate::extractors::dynamic_form::FormGuard;
use crate::admin::forms::{SlotForm as SlotEntityForm, DeleteForm as DeleteEntityForm};

type DeleteForm = Csrf<Form<DeleteEntityForm>>;
type SlotForm = Csrf<FormGuard<Form<Vec<(String, String)>>, SlotEntityForm>>;

/// Display a listing of the slots.
pub async fn index(request: HttpRequest, param: Query<RequestQParam>) -> Result<HttpResponse> {
    let current_page = param.page.unwrap_or(1);
    let db = request.db_pool()?;
    let slots = Slot::with_nominee_paginate(current_page, db).await?;
    let positions = Slot::positions(db).await?;
    let nominees = Slot::nominees(db).await?;
    request.render(200, "admin/slot/index.html", {
        let mut context = Context::new();
        context.insert("slots", &slots);
        context.insert("positions", &positions);
        context.insert("nominees", &nominees);
        context
    })
}

/// Store a newly created slot in storage.
pub async fn store(request: HttpRequest, form: SlotForm) -> Result<HttpResponse> {
    let mut form = form.into_inner().into_inner();
    if form.is_valid() {
        let db = request.db_pool()?;
        Slot::create(&form, db).await?;
        request.flash("success", "Slot successfully created.")?;
    }else{
        request.flash_form(form)?;
    }
    request.redirect("/admin/slots/")
}

/// Update the specified slot in storage.
pub async fn update(request: HttpRequest, path: Path<(i32,)>, form: SlotForm)->Result<HttpResponse>{
    let (id,) = path.into_inner();
    let mut form = form.into_inner().into_inner();
    if form.is_valid() {
        let db = request.db_pool()?;
        if let Err(err) = Slot::update(&form, id, db).await {
            request.flash("error", "Unable to update slot.")?; dbg!(err);
        }else{
            request.flash("success", "Slot successfully updated.")?;
        }
    }else{
        request.flash_form(form)?;
    }
    request.redirect("/admin/slots/")
}

/// Remove the specified slot from storage.
pub async fn destroy(request: HttpRequest, path: Path<(i32,)>, _: DeleteForm)->Result<HttpResponse>{
    let (id,) = path.into_inner();
    let db = request.db_pool()?;
    match Slot::delete(id, db).await {
        Ok(is_deleted)=>{
            if is_deleted {
                request.flash("success", "Slot successfully deleted.")?;
            }else{
                request.flash("error", "Slot is still in use.")?;
            }
        },
        Err(e)=>{
            request.flash("error", "An error occure while deleting slot.")?; 
            dbg!("====== Unable to delete slot ================", e);
        }
    }
    request.redirect("/admin/slots/")
}
