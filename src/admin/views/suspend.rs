use jelly::prelude::*;
use jelly::Result;

pub async fn update(request: HttpRequest) -> Result<HttpResponse> {

    request.render(200, "admin/election/index.html", {
        let context = Context::new();
        context
    })
}
