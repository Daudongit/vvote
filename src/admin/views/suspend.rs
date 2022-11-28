use jelly::actix_web::{web::{Form, Path}, HttpRequest};
use crate::admin::forms::ToggleElectionForm;
use jelly::guards::csrf::extractor::Csrf;
use crate::models::election::Election;
use jelly::prelude::*;
use jelly::Result;

type ToggleForm = Csrf<Form<ToggleElectionForm>>;

/// Patch the specified election in storage -> lock_unlock election. (only status field on db)
pub async fn update(request: HttpRequest, path: Path<(i32,)>, form: ToggleForm)->Result<HttpResponse>{
    let (id,) = path.into_inner();
    let form = form.into_inner();
    let db = request.db_pool()?;
    match Election::lock_unlock(id, &form, db).await {
        Ok(status)=>{
            let lock_unlock = if status == 1 {"unlock"} else {"lock"};
            let msg = format!("Election successfully {lock_unlock}.");
            request.flash("success", msg.as_str())?;
        }
        Err(e)=>{
            request.flash("error", "An error occured while updating election.")?; dbg!(e);
        }
    }
    request.redirect("/admin/elections/")
}