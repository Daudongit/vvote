pub mod result;
pub mod election;

use jelly::Result;
use jelly::prelude::*;

use crate::models::result::Results;

async fn can_vote_with_signature(request: &HttpRequest, signature: (u64, String))
    ->Result<(bool, String)>{
    let ip_error = Error::Generic("Valid ip address is needed to vote".into());
    let voter_ip = request.peer_addr().ok_or(ip_error)?.ip().to_string();
    let db = request.db_pool()?;
    let signature = (voter_ip.as_str(), signature);
    let count = Results::check_signature_count(signature, db).await?;
    let mut can_vote = true;
    if count > 0 {
        can_vote = false;
        request.flash("error", "You can only vote once.")?
    }
    Ok((can_vote, voter_ip))
}
