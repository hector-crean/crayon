use std::path::Path;
use aws_sdk_s3::{
    client::Client, error::SdkError, operation::put_object::PutObjectError, primitives::ByteStream,
};
use axum::response::IntoResponse;
use http::StatusCode;
use tokio::io::AsyncReadExt;

use super::FileStorage;



#[derive(Clone)]
pub struct S3Bucket {
    client: Client,
    region: String,
    bucket_name: String,
}

#[derive(thiserror::Error, Debug)]
pub enum S3Error {
    #[error("File not found")]
    FileNotFound(#[from] std::io::Error),

    #[error("File size conversion error")]
    FileSizeConversion(#[from] std::num::TryFromIntError),

    #[error(transparent)]
    S3Error(#[from] aws_sdk_s3::Error),

    #[error(transparent)]
    PutObjectError(#[from] SdkError<PutObjectError>),

    #[error("Environment variable error")]
    EnvVarError(#[from] std::env::VarError),
}

impl IntoResponse for S3Error {
    fn into_response(self) -> axum::response::Response {
        let body = "we have an issue with s3 :(";

        // it's often easiest to implement `IntoResponse` by calling other implementations
        (StatusCode::INTERNAL_SERVER_ERROR, body).into_response()
    }
}







impl S3Bucket {
    pub fn new(config: aws_sdk_s3::Config, region: &str, bucket_name: &str) -> Self {
        Self {
            client: aws_sdk_s3::Client::from_conf(config),
            region: region.to_string(),
            bucket_name: bucket_name.to_string(),
        }
    }

    pub fn url(&self, key: &str) -> Result<String, S3Error> {
        Ok(format!(
            "https://{}.s3.{}.amazonaws.com/{key}",
            self.bucket_name, self.region,
        ))
    }

   
}

impl FileStorage for S3Bucket {
    type Error = S3Error;
    async fn delete_file(&self, key: &str) -> Result<bool, Self::Error> {
        Ok(self
            .client
            .delete_object()
            .bucket(&self.bucket_name)
            .key(key)
            .send()
            .await
            .is_ok())
    }
    async fn upload_file<P: AsRef<Path> + Send>(&self, file_path: P, key: &str) -> Result<String, Self::Error> {
        let mut file = tokio::fs::File::open(file_path).await?;

        let size_estimate: usize = file
            .metadata()
            .await
            .map(|md| md.len())
            .unwrap_or(1024)
            .try_into()?;

        let mut contents = Vec::with_capacity(size_estimate);

        file.read_to_end(&mut contents).await?;

        let _put_object_output = self
            .client
            .put_object()
            .bucket(&self.bucket_name)
            .key(key)
            .body(ByteStream::from(contents))
            .send()
            .await?;

        let url = self.url(key)?;

        Ok(url)
    }
}



#[cfg(test)]
pub mod tests {
    use aws_sdk_s3::config::{Credentials, Region};
    use rand::{distributions::Alphanumeric, Rng};

    use super::*;

    // for `call`
    // for `oneshot` and `ready`

    async fn bucket_singleton() -> Result<S3Bucket, S3Error> {
        use dotenv::dotenv;

        dotenv().ok();

        let aws_key = std::env::var("AWS_ACCESS_KEY_ID")?;
        let aws_key_secret = std::env::var("AWS_SECRET_ACCESS_KEY")?;
        let s3_region = std::env::var("AWS_REGION")?;
        let aws_bucket = std::env::var("S3_BUCKET_NAME")?;

        let aws_config = aws_sdk_s3::config::Builder::new()
            .region(Region::new(s3_region.clone()))
            .credentials_provider(Credentials::new(
                aws_key,
                aws_key_secret,
                None,
                None,
                "loaded-from-custom-env",
            ))
            .build();

        let bucket = S3Bucket::new(aws_config, &s3_region, &aws_bucket);

        Ok(bucket)
    }

    #[tokio::test]
    async fn upload_gltf() -> Result<(), S3Error> {
        let key: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(7)
            .map(char::from)
            .collect();

        let bucket = bucket_singleton().await?;

        let url = bucket
            .upload_file(
                "/Users/hectorcrean/projects/bibe_server/assets/glb/Eye_AMD_Atrophy.glb",
                format!("{}.glb", &key).as_str(),
            )
            .await?;

        println!("{}", url);

        assert_eq!(1, 1);

        Ok(())
    }
}