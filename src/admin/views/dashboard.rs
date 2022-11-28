use crate::models::{
    election::Election, result::Results, slot::Slot, position::Position, nominee::Nominee
};
use jelly::prelude::*;
use jelly::Result;


/// Returns an overview of everything in the system.
pub async fn index(request: HttpRequest) -> Result<HttpResponse> {
    let db = request.db_pool()?;
    let election_count = Election::count(db).await?;
    let vote_count = Results::vote_count(db).await?;
    let slot_count = Slot::count(db).await?;
    let post_count = Position::count(db).await?;
    let nominee_count = Nominee::count(db).await?;
    request.render(200, "admin/dashboard.html", {
        let mut context = Context::new();
        context.insert("election_count", &election_count);
        context.insert("vote_count", &vote_count);
        context.insert("slot_count", &slot_count);
        context.insert("post_count", &post_count);
        context.insert("nominee_count", &nominee_count);
        context
    })
}
