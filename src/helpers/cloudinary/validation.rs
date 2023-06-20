use std::collections::BTreeMap;

const POSSIBLE_OPTIONS: [&str; 40] = [ 
    "return_delete_token",
    "use_filename", "filename_override", "resource_type",
    "discard_original_filename", "overwrite", "colors", "faces", 
    "folder", "upload_preset", "type", "access_mode", "public_id", 
    "quality_analysis", "accessibility_analysis", "cinemagraph_analysis",
    "detection", "ocr", "eager", "eager_async", "eager_notification_url", 
    "transformation", "format", "custom_coordinates", "face_coordinates", 
    "image_metadata", "phash", "responsive_breakpoints", "categorization",
    "background_removal", "raw_convert", "allowed_formats", "async", "backup", 
    "callback", "eval", "headers", "invalidate", "notification_url", "proxy", 
];

pub trait Validate {
    fn get_valid_options(self)->Self;
}

impl Validate for BTreeMap<String, String> {
    fn get_valid_options(self)->Self {
        let mut this = self;
        let keys: Vec<String> = this.keys().cloned().collect();
        for key in keys {
            if !POSSIBLE_OPTIONS.contains(&key.as_ref()){
                this.remove(&key);
            }
        }
        this
    }
}