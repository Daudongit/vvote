use jelly::Result;
use jelly::prelude::*;

pub async fn index(request: HttpRequest) -> Result<HttpResponse> {
    request.flash("result_check", "true")?;
    request.redirect("/elections/")
}

pub async fn show(request: HttpRequest) -> Result<HttpResponse> {
    request.redirect("/")
}