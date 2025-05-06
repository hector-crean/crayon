use std::{future::Future, path::Path};

use axum::response::IntoResponse;

use crate::CrayonServerError;

pub mod s3;



pub trait FileStorage {
    type Error: std::error::Error + IntoResponse  + Into<CrayonServerError>;
    
    fn upload_file<P: AsRef<Path> + Send>(&self, file_path: P, key: &str) -> impl Future<Output = Result<String, Self::Error>> + Send;
    // fn upload_directory<P: AsRef<Path> + Send>(&self, file_path: P, key: &str) -> impl Future<Output = Result<String, Self::Error>> + Send;
    fn delete_file(&self, key: &str) -> impl Future<Output = Result<bool, Self::Error>> + Send;
}

