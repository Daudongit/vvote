use std::rc::Rc;
use std::collections::HashMap;

use jelly::serde::{Deserialize, Serialize};
use jelly::guards::csrf::extractor::{CsrfGuarded, CsrfToken};
use jelly::forms::{NumberField, TextField, Validation, Validator::Min};

type VoterId = i32;
type NomineeId = u64;
type PositionId = u64;
type VoterIp<'a> = &'a str;

#[derive(Deserialize)]
pub struct RequestQParam{
    pub page: Option<u16>
}


#[derive(Debug, Deserialize, Serialize)]
pub struct ElectionRoomForm {
    pub csrf: CsrfToken,
    pub election_id: NumberField<u64>,
    pub fingerprint_token: TextField
}

impl ElectionRoomForm{
    pub fn get_election_signature(&self)->(u64, String){
        (self.election_id.value, self.fingerprint_token.to_string())
    }
}

impl CsrfGuarded for ElectionRoomForm {
    fn csrf_token(&self) -> &CsrfToken {
        &self.csrf
    }
}

impl Validation for ElectionRoomForm {
    fn is_valid(&mut self) -> bool {
        self.election_id.is_valid_with_validators(
            "election_id", vec![Min(1)]
        ) &&
        self.fingerprint_token.is_valid_with_validators(
            "fingerprint_token", vec![Min(20)]
        )
    }
}


#[derive(Debug, Deserialize, Serialize, Default)]
pub struct ElectionForm {
    pub csrf: CsrfToken,
    pub election_id: NumberField<u64>,
    pub fingerprint_token: TextField,
    pub position_nominee: Vec<(PositionId, NomineeId)>
}

impl ElectionForm{
    pub fn get_election_signature(&self)->(u64, String){
        (self.election_id.value, self.fingerprint_token.to_string())
    }
}

impl CsrfGuarded for ElectionForm {
    fn csrf_token(&self) -> &CsrfToken {
        &self.csrf
    }
}

impl From<HashMap<String, String>> for ElectionForm {
    fn from(form_map: HashMap<String, String>)->Self{
        let mut form = Self::default();
        for (key, value) in form_map {
            match key.as_str() {
                "csrf" => form.csrf = value.into(),
                "election_id" => {
                    form.election_id = value.parse().unwrap_or_default()
                }
                "fingerprint_token" => {
                    form.fingerprint_token = value.parse().unwrap_or_default()
                }
                _ => {
                    let position_id = key.parse().unwrap_or_default();
                    let nominee_id = value.parse().unwrap_or_default();
                    form.position_nominee.push((position_id, nominee_id));
                }
            }
        }
        form
    }
}

impl Validation for ElectionForm {
    fn is_valid(&mut self) -> bool {
        let mut is_valid = 
            self.election_id.is_valid_with_validators(
                "election_id", vec![Min(1)]
            ) &&
            self.fingerprint_token.is_valid_with_validators(
                "fingerprint_token", vec![Min(20)]
            );
        for pos_noms in &self.position_nominee{
            let (position, nominee) = *pos_noms;
            if position < 1 {
                self.election_id.errors.push(
                    format!("`position`: {position} is not valid position")
                );
                is_valid = false;
            }
            if nominee < 1 {
                self.election_id.errors.push(
                    format!("`nominee`: {nominee} is not valid nominee")
                );
                is_valid = false;
            }
        }
        is_valid
    }
}


#[derive(Debug, Default)]
pub struct ElectionItem<'a> {
    pub voter_id: i32,
    pub position_id: u64,
    pub nominee_id: u64,
    pub election_id: u64,
    pub voter_ip: VoterIp<'a>,
    pub fingerprint_token: Rc<String>
}


pub struct ElectionFormVec<'a>(Vec<ElectionItem<'a>>);

impl<'a>  ElectionFormVec<'a>{
    pub fn into_inner(self)-> Vec<ElectionItem<'a>>{
        self.0
    }

    pub fn get_first(&mut self)->&ElectionItem<'a>{
        if self.0.is_empty(){
            self.0.push(ElectionItem::default());
        }
        self.0.first().unwrap()
    }

    pub fn get_election_signature(&mut self)->(u64, String){
        let first_item = self.get_first();
        (first_item.election_id, first_item.fingerprint_token.to_string())
    }
}

impl<'a>  From<(ElectionForm, (VoterId, VoterIp<'a>))> for ElectionFormVec<'a> {
    fn from(voter_info: (ElectionForm, (VoterId, VoterIp<'a>))) -> Self {
        let (election_form, voter) = voter_info;
        let (voter_id, voter_ip) = voter;
        let election_id = election_form.election_id.value;
        let fingerprint_token = Rc::new(election_form.fingerprint_token.value);
        let position_nominee = election_form.position_nominee;
        let election_items:Vec<ElectionItem> = 
            position_nominee.into_iter().map(
                |(position_id, nominee_id)|{
                    ElectionItem{
                        fingerprint_token: fingerprint_token.clone(),
                        voter_id, position_id, nominee_id, 
                        election_id, voter_ip
                    }
                }
            ).collect();
        Self(election_items)
    }
}
