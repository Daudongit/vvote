use std::env;
use std::pin::Pin;
use std::future::Future;

use jelly::chrono::Utc;
use jelly::error::error::Error;
use actix_easy_multipart::tempfile::Tempfile;


pub trait LocalFileUpload{
    type Future: Future;

    fn persist_file(self, name: String)->Self::Future;
}

impl LocalFileUpload for Tempfile {
    type Future = Pin<Box<dyn Future<Output = Result<Option<String>, Error>>>>;

    fn persist_file(self, name: String)->Self::Future{
        Box::pin(async move {
            let mut image = self;
            let upload_path = env::var("UPLOAD_PATH")?;
            let mut image_fullname = None;
            let filename = image.file_name.take().unwrap_or_else(||".jpg".into());
            if image.size > 0 && !filename.is_empty() {
                let timestamp = Utc::now().timestamp().to_string();
                let timestamp: String = timestamp.chars().rev().take(5).collect(); //asci chars
                let new_name = name + &timestamp + &filename;
                image.file.persist(upload_path +"/"+ &new_name)
                .map_err(|err|{
                    Error::Generic(format!("Error while saving file: {:?}", err))
                })?;
                image_fullname = Some(new_name);
            }
            Ok(image_fullname)
        })
    }
}
