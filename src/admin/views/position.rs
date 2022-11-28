use jelly::Result;
use jelly::prelude::*;
use jelly::guards::csrf::extractor::Csrf;
use crate::models::{position::Position, RequestQParam};
use jelly::actix_web::{web::{Query, Form, Path}, HttpRequest};
use crate::admin::forms::{PositonForm as PositonEntityForm, DeleteForm as DeleteEntityForm};

type DeleteForm = Csrf<Form<DeleteEntityForm>>;
type PositonForm = Csrf<Form<PositonEntityForm>>;

/// Display a listing of the position(candidate).
pub async fn index(request: HttpRequest, param: Query<RequestQParam>) -> Result<HttpResponse> {
    let current_page = param.page.unwrap_or(1);
    let db = request.db_pool()?;
    let positions = 
        Position::paginate(current_page, db).await?;
    request.render(200, "admin/position/index.html", {
        let mut context = Context::new();
        context.insert("positions", &positions);
        context
    })
}

/// Store a newly created position in storage.
pub async fn store(request: HttpRequest, form: PositonForm) -> Result<HttpResponse> {
    let mut form = form.into_inner();
    if form.is_valid() {
        let db = request.db_pool()?;
        Position::create(&form, db).await?;
        request.flash("success", "Position successfully created.")?;
    }else{
        request.flash_form(form.into_inner())?;
    }
    request.redirect("/admin/positions/")
}

/// Update the specified position in storage.
pub async fn update(request: HttpRequest, path: Path<(i32,)>, form: PositonForm)->Result<HttpResponse>{
    let mut form = form.into_inner();
    let (id,) = path.into_inner();
    if form.is_valid() {
        let db = request.db_pool()?;
        if let Err(err) = Position::update(&form, id, db).await {
            request.flash("error", "Unable to update position.")?; dbg!(err);
        }else{
            request.flash("success", "Position successfully updated.")?;
        }
    }else{
        request.flash_form(form.into_inner())?;
    }
    request.redirect("/admin/positions/")
}

/// Remove the specified position from storage.
pub async fn destroy(request: HttpRequest, path: Path<(i32,)>, _: DeleteForm)->Result<HttpResponse>{
    let (id,) = path.into_inner();
    let db = request.db_pool()?;
    match Position::delete(id, db).await {
        Ok(is_deleted)=>{
            if is_deleted {
                request.flash("success", "Position successfully deleted.")?;
            }else{
                request.flash("error", "Position is still in use.")?;
            }
        }
        Err(e)=>{
            request.flash("error", "An error occure while deleting position.")?; dbg!(e);
        }
    }
    request.redirect("/admin/positions/")
}
