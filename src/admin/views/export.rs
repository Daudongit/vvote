use jelly::prelude::*;
use jelly::Result;

pub async fn index(request: HttpRequest) -> Result<HttpResponse> {
    // return download
    request.render(200, "sample.html", {
        let context = Context::new();
        context
    })
}

pub async fn show(request: HttpRequest) -> Result<HttpResponse> {
    // return download
    request.render(200, "sample.html", {
        let context = Context::new();
        context
    })
}

