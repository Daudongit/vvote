use std::fs::File;
use std::path::Path;
use std::io::Read as _;
use std::collections::BTreeMap;

use reqwest::Client;
use jelly::chrono::Utc;
use sha1::{Digest, Sha1};
use reqwest::multipart::{Form, Part};
use jelly::error::error::Error as JellyError;

use crate::error::Error;
use crate::helpers::cloudinary::JoinValue as _;
use crate::helpers::cloudinary::result::UploadResult;
use crate::helpers::cloudinary::validation::Validate as _;

pub struct Cloudinary{
    api_key: String,
    cloud_name: String,
    api_secret: String
}

impl Cloudinary{
    pub fn new(api_key: String, cloud_name: String, api_secret: String)->Self{
        Self{api_key, cloud_name, api_secret}
    }

    fn read_uploaded_file(path: &str) -> Result<(Vec<u8>, String), JellyError> {
        let mut file = File::open(path)?;
    
        let size_estimate = file
            .metadata()
            .map(|md| md.len())
            .unwrap_or(1024)
            .try_into()
            .map_err(|err|{
                dbg!("=========== file size error ==========", err);
                JellyError::Generic("File too big".into())
            })?;
    
        let mut contents = Vec::with_capacity(size_estimate);
        file.read_to_end(&mut contents)?;

        let filename = Path::new(path)
            .file_name().unwrap().to_string_lossy().into_owned();
        Ok((contents, filename))
    }

    pub async fn upload_image(&self, path: String, options: BTreeMap<String, String>)->Result<UploadResult, Error>{
        let client = Client::new();
        let (file_chunks, filename) = Self::read_uploaded_file(&path)?;
        let file = Part::bytes(file_chunks)
            .file_name(filename).mime_str("image/*")?;
        let multipart = self.build_form_data(options).part("file", file);
        let response = client
            .post(format!(
                "https://api.cloudinary.com/v1_1/{}/image/upload", self.cloud_name
            ))
            .multipart(multipart)
            .send()
            .await?;
        let text = response.text().await?;
        let json = serde_json::from_str(&text)?;
        Ok(json)
    }

    fn build_form_data(&self, options: BTreeMap<String, String>) -> Form {
        let mut map = options.get_valid_options();
        let resource_type = map.remove("resource_type");
        let timestamp = Utc::now().timestamp_millis().to_string();

        let mut form = Form::new()
            .text("api_key", self.api_key.clone())
            .text("timestamp", timestamp.clone());

        if let Some(value) = resource_type {
            form = form.text("resource_type", value);
        }

        let str = map.iter().map(|(k, v)| format!("{k}={v}")).join("&");
        let mut hasher = Sha1::new();
        if !str.is_empty() {
            hasher.update(str);
            hasher.update("&");
        }
        hasher.update(format!("timestamp={}{}", timestamp, self.api_secret));
        let signature = hasher.finalize();

        form = form.text("signature", format!("{:x}", signature));
        for (k, v) in map.iter() {
            form = form.text(k.clone(), v.clone());
        }
        form
    }
}
