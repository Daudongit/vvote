use jelly::Result;
use jelly::prelude::*;
use actix_easy_multipart::MultipartForm;
use jelly::guards::csrf::extractor::Csrf;
use jelly::actix_web::{web::{Query, Form, Path}, HttpRequest};

use crate::models::nominee::Nominee;
use crate::extractors::multipart_form::MultipartGuard;
use crate::admin::forms::{
    MultipartNomineeForm, DeleteForm as DeleteEntityForm, RequestQParam
};

type DeleteForm = Csrf<Form<DeleteEntityForm>>;
type NomineeForm = Csrf<MultipartGuard<MultipartForm<MultipartNomineeForm>>>;

/// Display a listing of the nominee.
pub async fn index(request: HttpRequest, param: Query<RequestQParam>)->Result<HttpResponse>{
    let current_page = param.page.unwrap_or(1);
    let db = request.db_pool()?;
    let nominees = 
        Nominee::get_paginated_record(current_page, db).await?;
    request.render(200, "admin/nominee/index.html", {
        let mut context = Context::new();
        context.insert("nominees", &nominees);
        context
    })
}

/// Store a newly created nominee in storage.
pub async fn store(request: HttpRequest, form: NomineeForm)->Result<HttpResponse>{
    let mut form = 
        form.into_inner().into_inner().into_inner();
    let image_file = form.image.take();
    if form.is_valid() {
        let db = request.db_pool()?;
        let image_name = match Nominee::save_upload_image(image_file).await {
            Err(err) => {
                request.flash("error", "Unable to save uploaded file.")?;
                dbg!("============= unable to upload file ===============", err); None
            }
            Ok(file_name) => file_name
        };
        Nominee::create((&form, &image_name), db).await?;
        request.flash("success", "Nominee successfully created.")?;
    }else{
        request.flash_form(form)?;
    }
    request.redirect("/admin/nominees/")
}

/// Update the specified nominee in storage.
pub async fn update(request: HttpRequest, path: Path<(i32,)>, form: NomineeForm)
    ->Result<HttpResponse>{
    let mut form = 
        form.into_inner().into_inner().into_inner();
    let (id,) = path.into_inner();
    let mut error_msg = None;
    if form.is_valid() {
        let db = request.db_pool()?;
        let image_file = form.image.take();
        let image_name = match Nominee::save_upload_image(image_file).await {
            Err(err) => {
                dbg!("============= unable to upload file =================", err);
                error_msg = Some("Unable to save uploaded file."); None
            }
            Ok(file_name) => file_name
        };
        let previous_image = form.removed_image.take();
        match Nominee::update((&form, &image_name), id, db).await {
            Err(_) => {
                Nominee::unlink_prev_image(image_name).await?;
                error_msg = Some("An error occure while creating nominee.");
            }
            _=>{
                if image_name.is_some() {
                    let previous_image = 
                        previous_image.map(|img|img.into_inner());
                    Nominee::unlink_prev_image(previous_image).await?;
                }
                request.flash("success", "Nominee successfully updated.")?;
            }
        }
    }else{
        request.flash_form(form)?;
    }
    if let Some(err) = error_msg {
        request.flash("error", err)?;
    }
    request.redirect("/admin/nominees/")
}

/// Remove the specified nominee from storage.
pub async fn destroy(request: HttpRequest, path: Path<(i32,)>, form: DeleteForm)->Result<HttpResponse>{
    let (id,) = path.into_inner();
    let form = form.into_inner().into_inner();
    let db = request.db_pool()?;
    match Nominee::delete(id, db).await {
        Ok(is_deleted)=>{
            if is_deleted {
                Nominee::unlink_prev_image(form.removed_image).await?;
                request.flash("success", "Nominee successfully deleted.")?;
            }else{
                request.flash("error", "Nominee is still in use.")?;
            }
        }
        Err(e)=>{
            dbg!("============ unable to complete detele ==========", e);
            request.flash("error", "An error occure while deleting nominee.")?;
        }
    }
    request.redirect("/admin/nominees/")
}
