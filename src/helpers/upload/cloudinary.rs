use std::env;
use std::pin::Pin;
use std::future::Future;
use std::collections::BTreeMap;

use jelly::chrono::Utc;
use jelly::error::error::Error;
use actix_easy_multipart::tempfile::Tempfile;
use crate::helpers::cloudinary::{result::UploadResult, upload::Cloudinary};

pub trait CloudinaryFileUpload{
    type Future: Future;

    fn persist_file(self, name: String)->Self::Future;
}

impl CloudinaryFileUpload for Tempfile {
    type Future = Pin<Box<dyn Future<Output = Result<Option<String>, Error>>>>;

    fn persist_file(self, name: String)->Self::Future{
        Box::pin(async move {
            let path = self.file.path().to_str()
            .ok_or_else(||Error::Generic("Can not get file path".into()))?;

            let cloud_name = env::var("CLOUDINARY_CLOUD_NAME")?;
            let api_secret = env::var("CLOUDINARY_API_SECRET")?;
            let api_key = env::var("CLOUDINARY_API_KEY")?;

            let timestamp = Utc::now().timestamp().to_string();
            let generate_name_part: String = timestamp.chars().rev().take(5).collect(); //only asci chars
            let public_id = name + &generate_name_part;

            let mut options = BTreeMap::new();
            options.insert("public_id".to_owned(), public_id);
            let cloudinary = Cloudinary::new(api_key, cloud_name, api_secret);
            let result = 
                cloudinary.upload_image(path.into(), options).await
                .map_err(Into::<Error>::into)?;
            result.process_upload()
        })
    }
}

trait ProcessUpload{
    fn process_upload(self)->Result<Option<String>, Error>;
}

impl ProcessUpload for UploadResult{
    fn process_upload(self)->Result<Option<String>, Error>{
        match self {
            Self::Succes(resp) =>{
                Ok(Some(resp.secure_url))
            }
            Self::Error(err)=>{
                dbg!("=== Unable to process uploaded result ===", err); Ok(None)
            }
        }
    }
}
