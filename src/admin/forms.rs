use jelly::forms::Validator::{Max, Min};
use jelly::guards::csrf::extractor::{CsrfGuarded, CsrfToken};
use actix_easy_multipart::{MultipartForm, tempfile::Tempfile, text::Text};
use jelly::serde::{Deserialize, Serialize, Serializer, ser::SerializeStruct};
use jelly::forms::{TextField, EmailField, Validation, NumberField, DateTimeField};

#[derive(Deserialize)]
pub struct RequestQParam{
    pub page: Option<u16>,
}

#[derive(Debug, Deserialize)] 
pub struct DeleteForm {
    pub csrf: CsrfToken,
    pub removed_image: Option<String>
}

impl CsrfGuarded for DeleteForm {
    fn csrf_token(&self) -> &CsrfToken {
        &self.csrf
    }
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct ElectionForm {
    pub csrf: CsrfToken,
    pub title: TextField,
    pub start_date: DateTimeField,
    pub end_date: DateTimeField,
    pub slots: Vec<u64>
}

impl From<Vec<(String, String)>> for ElectionForm {
    fn from(form_vec: Vec<(String, String)>)->Self{
        let mut form = Self::default();
        for (key, value) in form_vec {
            match key.as_str() {
                "csrf" => form.csrf = value.into(),
                "title" => form.title = value.into(),
                "start_date" => form.start_date = value.into(),
                "end_date" => form.end_date = value.into(),
                "slots" => form.slots.push(value.parse().unwrap_or_default()),
                _ => ()
            }
        }
        form
    }
}

impl CsrfGuarded for ElectionForm {
    fn csrf_token(&self) -> &CsrfToken {
        &self.csrf
    }
}

impl Validation for ElectionForm {
    fn is_valid(&mut self) -> bool {
        let mut is_valid = 
            self.title.is_valid_with_validators("title", vec![Min(3), Max(100)]) &&
            self.start_date.is_valid_field("start_date") &&
            self.end_date.is_valid_field("end_date");
        if self.start_date.date > self.end_date.date {
            self.end_date.errors.push("end_date must be gerater than start_date.".into());
            is_valid = false;
        }
        for slot in &self.slots{
            let slot = *slot;
            if slot < 1 {
                self.title.errors.push(format!("`slots`: {slot} is not valid slot"));
                is_valid = false;
            }
        }
        is_valid
    }
}

#[derive(Debug, Deserialize)]
pub struct ToggleElectionForm {
    pub csrf: CsrfToken,
    pub status: u8
}

impl CsrfGuarded for ToggleElectionForm {
    fn csrf_token(&self) -> &CsrfToken {
        &self.csrf
    }
}

#[derive(Debug, Deserialize)]
pub struct ElectionVisibilityForm {
    pub csrf: CsrfToken,
    pub can_see_result: bool
}

impl CsrfGuarded for ElectionVisibilityForm {
    fn csrf_token(&self) -> &CsrfToken {
        &self.csrf
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PositonForm {
    pub csrf: CsrfToken,
    pub name: TextField
}

impl CsrfGuarded for PositonForm {
    fn csrf_token(&self) -> &CsrfToken {
        &self.csrf
    }
}

impl Validation for PositonForm {
    fn is_valid(&mut self) -> bool {
        self.name.is_valid_with_validators("name", vec![Min(3), Max(100)])
    }
}

#[derive(Debug, MultipartForm)]
pub struct MultipartNomineeForm {
    pub csrf: Text<CsrfToken>,
    pub first_name: Text<TextField>,
    pub last_name: Text<TextField>,
    pub email: Text<EmailField>,
    pub description: Text<TextField>,
    pub removed_image: Option<Text<String>>,
    // #[serde(skip_serializing)]
    // #[serde(skip_deserializing)]
    pub image: Option<Tempfile>
}

impl CsrfGuarded for MultipartNomineeForm {
    fn csrf_token(&self) -> &CsrfToken {
        &self.csrf
    }
} 

impl Validation for MultipartNomineeForm {
    fn is_valid(&mut self) -> bool {
        self.first_name.is_valid_with_validators(
            "first_name", vec![Min(3), Max(100)]
        ) &&
        self.last_name.is_valid_with_validators(
            "last_name", vec![Min(3), Max(100)]
        ) &&
        self.description.is_valid_with_validators(
            "description", vec![Min(3), Max(250)]
        ) &&
        self.email.is_valid_field("email")
    }
}

impl Serialize for MultipartNomineeForm {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer
    {
        let mut mform = 
            serializer.serialize_struct("MultipartNomineeForm", 4)?;
        mform.serialize_field("first_name", &*self.first_name)?;
        mform.serialize_field("last_name", &*self.last_name)?;
        mform.serialize_field("description", &*self.description)?;
        mform.serialize_field("email", &*self.email)?;
        mform.end()
    }
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct SlotForm {
    pub csrf: CsrfToken,
    pub position_id: NumberField<u32>,
    pub nominees: Vec<u64>
}

impl From<Vec<(String, String)>> for SlotForm{
    fn from(form_vec: Vec<(String, String)>) -> Self {
        let mut form = Self::default();
        for (key, value) in form_vec {
            match key.as_str() {
                "csrf" => form.csrf = value.into(),
                "position_id" => form.position_id = value.parse().unwrap_or_default(),
                "nominees" => form.nominees.push(value.parse().unwrap_or_default()),
                _ => ()
            }
        }
        form
    }
}

impl CsrfGuarded for SlotForm {
    fn csrf_token(&self) -> &CsrfToken {
        &self.csrf
    }
}

impl Validation for SlotForm {
    fn is_valid(&mut self) -> bool {
        let mut is_valid = 
            self.position_id.is_valid_with_validators(
                "position_id", vec![Min(1)]
            );
        for nominee in &self.nominees{
            let nominee = *nominee;
            if nominee < 1 {
                self.position_id.errors.push(
                    format!("`nominees`: {nominee} is not valid nominee")
                );
                is_valid = false;
            }
        }
        is_valid
    }
}
