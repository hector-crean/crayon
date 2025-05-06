use color_eyre::eyre;
use server::{AppState, Application, ApplicationConfig};

#[tokio::main]
async fn main() -> eyre::Result<()> {
    color_eyre::install()?;
    dotenv::dotenv().ok();
    env_logger::init();

    let aws_key = std::env::var("AWS_ACCESS_KEY_ID").expect("Failed to get AWS key.");
    let aws_key_secret =
        std::env::var("AWS_SECRET_ACCESS_KEY").expect("Failed to get AWS secret key.");
    let s3_region = std::env::var("AWS_REGION").unwrap_or("eu-west-2".to_string());
    let aws_bucket = std::env::var("S3_BUCKET_NAME").expect("Failed to get AWS Bucket key");

    let config = ApplicationConfig {
        aws_key,
        aws_key_secret,
        s3_region,
        aws_bucket,
        surreal_url: "127.0.0.1:8000".to_string(),
    };

    let app = Application::new(config).await;

    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse::<u16>()?;

    app.run(port).await?;

    Ok(())
}