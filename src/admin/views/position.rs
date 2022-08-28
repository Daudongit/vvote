use jelly::prelude::*;
use jelly::Result;

pub async fn index(request: HttpRequest) -> Result<HttpResponse> {
    
    request.render(200, "admin/position/index.html", {
        let context = Context::new();
        context
    })
}

pub async fn store(request: HttpRequest) -> Result<HttpResponse> {
    
    request.render(200, "admin/position/index.html", {
        let context = Context::new();
        context
    })
}

pub async fn update(request: HttpRequest) -> Result<HttpResponse> {
    
    request.render(200, "admin/position/index.html", {
        let context = Context::new();
        context
    })
}

pub async fn destroy(request: HttpRequest) -> Result<HttpResponse> {
    
    request.render(200, "admin/position/index.html", {
        let context = Context::new();
        context
    })
}
