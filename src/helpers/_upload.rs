use std::fs::File;
use std::pin::Pin;
use std::io::Read as _;
use std::future::Future;

// use sha2::{Sha256, Digest};
use sha2::Sha256;
use hmac::{Hmac, Mac};

// use base64::{encode_config};
// use sha1::{Sha1, Digest};

use serde_json::Value;
use jelly::chrono::Utc;
use serde::{Deserialize, Serialize};
use jelly::error::error::Error as JellyError;
use actix_easy_multipart::tempfile::Tempfile;
use reqwest::{Client as ReqwestClient, multipart};
// use tokio::{fs, io::AsyncReadExt as _};

use crate::error::Error;

pub trait UploadFile{
    type Future: Future;

    fn persist_file(self, name: String)->Self::Future;
}

impl UploadFile for Tempfile {
    type Future = Pin<Box<dyn Future<Output = Result<Option<String>, Error>>>>;

    fn persist_file(self, name: String)->Self::Future{
        let mut mut_self = self;
        Box::pin(async move {
            let path = mut_self.file.path().to_str()
            .ok_or_else(||Error::Generic("Can not get file path".into()))?;
            let file_ext = mut_self.file_name.take().unwrap_or_else(||".jpg".into());
            let file_info = FileInfo::new(path.into(), (name, file_ext));
            let credential = 
                CloudinaryCredential::new(&file_info.timestamp, &file_info.name);
            let client = Client::new(file_info, credential);
            client.upload_to_cloudinary().await
            .map(|resp|Some(resp.public_id))
        })
    }
}

// #[derive(Debug, Serialize, Deserialize)]
struct CloudinaryCredential {
    url: String,
    api_key: String,
    api_secret: String,
    cloud_name: String,
    signature: String
}

impl CloudinaryCredential {
    pub fn new(timestamp: &str, name: &str)->Self{
        let cloud_name = "dolnl2kjz".into();
        let api_secret = String::from("iwH_YjV6mQdz7i-KlGLDvQmXNMc");
        Self {
            url: format!(
                "https://api.cloudinary.com/v1_1/{}/image/upload", cloud_name
            ),
            api_key: "333741988645388".into(), cloud_name,
            signature: Self::generate_signature((timestamp, name), &api_secret),
            api_secret
        } 
    }

    pub fn generate_signature(info: (&str, &str), api_secret: &str)->String{
        let (timestamp, public_id) = info; 
        // let signature_string = 
        //     format!("public_id={public_id}&timestamp={timestamp}{api_secret}");
        // dbg!("========== signature_string ============");
        // dbg!(&signature_string);
        // let mac = 
        //     Hmac::<Sha256>::new_from_slice(signature_string.as_bytes()).unwrap();

        let signature_string = 
            format!("public_id={public_id}&timestamp={timestamp}");            
        let mut mac = 
            Hmac::<Sha256>::new_from_slice(api_secret.as_bytes()).unwrap();
        mac.update(signature_string.as_bytes());
        let result = mac.finalize().into_bytes();
        hex::encode(result)
    }

    // pub fn generate_signature(timestamp: &str, api_secret: &str)->String{
    //     // let mut mac = Hmac::<Sha1>::from(api_secret.as_bytes());
    //     let mut mac = Hmac::<Sha1>::new_varkey(api_secret.as_bytes()).unwrap();
    //     mac.input(format!("timestamp={}", timestamp).as_bytes());
    //     mac.result().code().to_hex()
    // }
}

#[derive(Debug, Serialize, Deserialize)]
struct CloudinaryResponse {
    public_id: String,
    secure_url: String
}

#[derive(Debug, Serialize, Deserialize)]
struct FileInfo {
    path: String,
    name: String,
    timestamp: String
}

impl FileInfo {
    pub fn new(path: String, name: (String, String))->Self{
        let timestamp = Utc::now().timestamp().to_string();
        let (name_part, _extension) = name;
        let generate_name_part: String = timestamp.chars().rev().take(5).collect(); //only asci chars
        let name = name_part + &generate_name_part;
        Self {path, name, timestamp}
    }
}

struct Client{
    file_info: FileInfo,
    credential: CloudinaryCredential
}

impl Client {
    pub fn new(file_info: FileInfo, credential: CloudinaryCredential)->Self{
        Self {file_info, credential}
    }

    async fn upload_to_cloudinary(self)->Result<CloudinaryResponse, Error>{
        let url =format!(
            "https://api.cloudinary.com/v1_1/{}/image/upload", &self.credential.cloud_name
        );
        let client = ReqwestClient::new();
        let file_chunks = self.read_uploaded_file()?;
        let form = self.create_form(file_chunks);

        let response: Value = 
            client.post(url).multipart(form).send().await?.json().await?;
        if let Some(err) = response.get("error"){
            let mut err_msg = "An error occur while uploading".into();
            if let Some(message) = err.get("message"){
                err_msg = message.to_string();
            }
            return Err(Error::Generic(err_msg));
        }
        
        dbg!("============ response ===============", &response);
        let result: CloudinaryResponse = 
            serde_json::from_value(response["public_id"].clone())?;
        dbg!("======= result ==========");
        dbg!(&result);
        Ok(result)
    }

    fn read_uploaded_file(&self) -> Result<Vec<u8>, JellyError> {
        let mut file = File::open(&self.file_info.path)?;
    
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
        Ok(contents)
    }

    fn create_form(self, chunks: Vec<u8>)->multipart::Form{
        let form = multipart::Form::new()
        .text("api_key", self.credential.api_key)
        .text("timestamp", self.file_info.timestamp)
        .text("signature", self.credential.signature)
        .text("public_id", self.file_info.name);
        let file = multipart::Part::bytes(chunks);
        form.part("file", file)
        // form.text("eager", "w_400,h_300,c_pad|w_260,h_200,c_crop");
    }
}
