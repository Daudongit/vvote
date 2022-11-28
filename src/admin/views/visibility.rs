use jelly::Result;
use jelly::prelude::*;
use crate::models::election::Election;
use jelly::guards::csrf::extractor::Csrf;
use crate::admin::forms::ElectionVisibilityForm;
use jelly::actix_web::{web::{Form, Path}, HttpRequest};

type VisibilityForm = Csrf<Form<ElectionVisibilityForm>>;

/// Patch the specified election in storage. (only (voter) can_see_result field on db)
pub async fn update(request: HttpRequest, path: Path<(i32,)>, form: VisibilityForm)->Result<HttpResponse>
{
    let (id,) = path.into_inner();
    let form = &*form;
    let db = request.db_pool()?;
    match Election::set_visibility(id, form, db).await {
        Ok(can_see_result)=>{
            let lock_unlock = if can_see_result {"unlock"} else {"lock"};
            let msg = format!("Voters' result page successfully {lock_unlock}.");
            request.flash("success", msg.as_str())?;
        }
        Err(e)=>{
            request.flash("error", "An error occure while updating election.")?; dbg!(e);
        }
    }
    request.redirect("/admin/elections/")
}