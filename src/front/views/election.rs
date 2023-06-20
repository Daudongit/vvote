use std::env;
use std::collections::HashMap;

use jelly::Result;
use jelly::prelude::*;
use jelly::guards::AuthSessionName;
use jelly::guards::csrf::extractor::Csrf;
use jelly::actix_web::web::{Query, Form};

use crate::models::result::Results;
use crate::models::election::Election;
use crate::extractors::dynamic_form::FormGuard;
use crate::front::forms::{
    RequestQParam, ElectionForm, ElectionFormVec, ElectionRoomForm as ElectRoomForm
};

type ElectionRoomForm = Csrf<Form<ElectRoomForm>>;
type DynamicForm = Csrf<FormGuard<Form<HashMap<String, String>>, ElectionForm>>;

/// Display a listing of the elections.
pub async fn index(request: HttpRequest, param: Query<RequestQParam>)->Result<HttpResponse>{
    let current_page = param.page.unwrap_or(1);
    let db = request.db_pool()?;
    let elections = 
        Election::paginate(current_page, db).await?;
    request.render(200, "front/election.html", {
        let mut context = Context::new();
        context.insert("elections", &elections);
        context
    })
}

/// Display election ground for voting (pulling)
pub async fn show(request: HttpRequest, form: ElectionRoomForm) -> Result<HttpResponse> {
    let mut form = form.into_inner().into_inner();
    if form.is_valid() {
        let db = request.db_pool()?;
        let signature = form.get_election_signature();
        let (can_vote, _) = 
            super::can_vote_with_signature(&request, signature).await?;
        if can_vote {
            let election_id = form.election_id.value;
            let (slots_ids, positions) = 
                Election::get_slot_ids_positions(election_id as i32, db).await?;
            let nominees = 
                Election::get_slot_nominees(slots_ids, db).await?;
            return request.render(200, "front/vote.html", {
                let mut context = Context::new();
                context.insert("election_id", &election_id);
                context.insert("positions", &positions);
                context.insert("nominees", &nominees);
                context
            })
        }
    }else{
        request.flash_form(form)?;
    }   
    request.redirect("/elections/")
}

/// Submit vote
pub async fn store(request: HttpRequest, form: DynamicForm) -> Result<HttpResponse> {
    let session_name = env::var("VOTER_SESSION_NAME")?;
    let auth_config_name = AuthSessionName(session_name);
    let mut form = form.into_inner().into_inner();
    if form.is_valid() {
        let db = request.db_pool()?;
        let signature = form.get_election_signature();
        let (can_vote, voter_ip) = 
            super::can_vote_with_signature(&request, signature).await?;
        if can_vote {
            let voter_id = request.user(&auth_config_name)?.id;
            let forms:ElectionFormVec = (form, (voter_id, voter_ip.as_ref())).into();
            Results::create(forms, db).await?;
            request.flash("success", "Voting successfully completed.")?;
        }
    }else{
        request.flash_form(form)?;
    }   
    request.redirect("/elections/")
}

// Helper
// async fn verify_csrf_token(req: &HttpRequest, form_csrf: &str)->Result<bool>{
//     let token = CsrfCookie::extract(&req).await
//     .map_err(|err|Error::CsrfToken(CsrfExtractorError::Inner(err)))?;
//     Ok(token.validate(dbg!(form_csrf)))
// }