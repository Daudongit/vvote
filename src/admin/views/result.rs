use jelly::prelude::*;
use jelly::Result;

pub async fn index(request: HttpRequest) -> Result<HttpResponse> {

    request.render(200, "admin/votes.html", {
        let context = Context::new();
        context
    })
}

pub async fn show(request: HttpRequest) -> Result<HttpResponse> {

    request.render(200, "admin/results.html", {
        let context = Context::new();
        context
    })
}

