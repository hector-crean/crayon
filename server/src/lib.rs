// #![feature(impl_trait_in_assoc_type)]

pub mod database;
pub mod file_storage;
pub mod handlers;
pub mod request_client;

use axum::{
    response::IntoResponse,
    routing::{delete, get, post, put, patch},
    Router,
};
use database::{surrealdb::SurrealDatabase, Database};
use file_storage::{
    s3::{S3Bucket, S3Error},
    FileStorage,
};
use surrealdb::{engine::remote::ws::Client, opt::auth::Root};
use http::{HeaderName, Method, StatusCode};
use log::{error, info};

use axum::Json;
use serde::Serialize;
use surrealdb::{engine::remote::ws::Ws, Surreal};
use std::{future::Future, net::SocketAddr};
use tower_http::cors::{Any, CorsLayer};

#[derive(thiserror::Error, Debug)]
pub enum CrayonServerError {
    #[error("Failed to bind to address: {0}")]
    BindError(#[from] std::io::Error),
    #[error(transparent)]
    AxumError(#[from] axum::Error),
    #[error(transparent)]
    S3Error(#[from] S3Error),
    #[error(transparent)]
    MultipartError(#[from] axum::extract::multipart::MultipartError),
    #[error(transparent)]
    SurrealdbError(#[from] surrealdb::Error),
    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),
    #[error(transparent)]
    UrlError(#[from] url::ParseError),
}

#[derive(Serialize)]
pub struct ErrorResponse {
    status: u16,
    error_type: &'static str,
    message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    details: Option<String>,
}

impl IntoResponse for CrayonServerError {
    fn into_response(self) -> axum::response::Response {
        let (status_code, error_type, details) = match &self {
            CrayonServerError::BindError(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "BIND_ERROR",
                Some(e.to_string()),
            ),
            CrayonServerError::AxumError(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "SERVER_ERROR",
                Some(e.to_string()),
            ),
            CrayonServerError::S3Error(e) => (
                StatusCode::BAD_GATEWAY,
                "STORAGE_ERROR",
                Some(e.to_string()),
            ),
            CrayonServerError::MultipartError(e) => (
                StatusCode::BAD_REQUEST,
                "INVALID_REQUEST",
                Some(e.to_string()),
            ),
            CrayonServerError::SurrealdbError(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "DATABASE_ERROR",
                Some(e.to_string()),
            ),
            CrayonServerError::ReqwestError(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "REQUEST_ERROR",
                Some(e.to_string()),
            ),
            CrayonServerError::UrlError(e) => {
                (StatusCode::BAD_REQUEST, "INVALID_URL", Some(e.to_string()))
            }
        };

        let error_response = ErrorResponse {
            status: status_code.as_u16(),
            error_type,
            message: self.to_string(),
            details,
        };

        (status_code, Json(error_response)).into_response()
    }
}

pub trait AppState: Clone + Send + Sync + 'static {
    /// Database for the application
    type D: Database;
    /// File storage for the application
    type F: FileStorage;
    /// Configuration for the application
    type C;
    /// Request handler for the application
    // type R: for<'a> HttpClient<'a>;

    fn new(config: Self::C) -> impl Future<Output = Self> + Send;

    fn database(&self) -> &Self::D;

    fn file_storage(&self) -> &Self::F;

    fn request_client(&self) -> &reqwest::Client;

    fn run(&self, port: u16) -> impl Future<Output = Result<(), CrayonServerError>> + Send {
        async move {
            info!("Server running on port: {}", port);

            let cors = CorsLayer::new()
                // allow `GET` and `POST` when accessing the resource
                .allow_methods([Method::GET, Method::POST])
                // allow the Content-Type header
                .allow_headers([HeaderName::from_static("content-type")])
                // allow requests from any origin
                .allow_origin(Any);

            // Build our application with routes
            let app = Router::new()
                // users resource
                // Create a new user
                // POST /users
                // Request Body: JSON representing the user to create
                // Response: JSON of the newly created user with ID
                .route("/user", post(handlers::user::create_user::<Self>))
                // List all users
                // GET /users
                // Optional Query Params: ?format=html to return HTML representation of all users
                // Response: JSON array of users (or HTML if requested)
                .route("/user", get(handlers::user::list_users::<Self>))
                // Get a single user by ID
                // GET /users/:id
                // Path Param: id (string or UUID)
                // Response: JSON of the requested user
                .route("/user/:id", get(handlers::user::get_user::<Self>))
                // Get multiple users by ID
                // GET /users/bulk
                // Path Param: ids (array of strings or UUIDs)
                // Response: JSON array of users
                .route("/user/bulk", get(handlers::user::get_users::<Self>))
                // Upsert a user
                // POST /users/upsert
                // Request Body: JSON representing the user to upsert
                // Response: JSON of the upserted user with ID
                .route("/user/upsert", post(handlers::user::upsert_user::<Self>))
                // Update a user fully
                // PUT /users/:id
                // Path Param: id (string or UUID)
                // Request Body: JSON representing the entire updated user resource
                // Response: JSON of the updated user
                // .route("/users/:id", put(handlers::user::update_user::<Self>))
                // Partially update a user
                // PATCH /users/:id
                // Path Param: id (string or UUID)
                // Request Body: JSON with partial fields to update
                // Response: JSON of the updated user
                // .route("/users/:id", patch(handlers::user::patch_user::<Self>))
                // Delete a user by ID
                // DELETE /users/:id
                // Path Param: id (string or UUID)
                // Response: Status 204 on success
                // .route("/users/:id", delete(handlers::user::delete_user::<Self>))
               

                // .route("/users/search", get(handlers::user::search_users::<Self>))
             
                // Bulk create users
                // POST /users/bulk
                // Request Body: JSON array of users to create
                // Response: JSON array of created users with IDs
                // .route(
                //     "/users/bulk",
                //     post(handlers::user::bulk_create_users::<Self>),
                // )

             
              
                // Add state and layers as needed
                .with_state(self.clone())
                .layer(cors);

            let addr: SocketAddr = SocketAddr::from(([0, 0, 0, 0], port));
            // let addr: SocketAddr = SocketAddr::from(([127, 0, 0, 1], port));

            let listener = tokio::net::TcpListener::bind(addr).await?;

            info!("Server running on http://{}", addr);
            match axum::serve(listener, app).await {
                Ok(_) => info!("Server shut down gracefully"),
                Err(e) => error!("Server error: {}", e),
            }

            Ok(())
        }
    }
}

#[derive(Clone)]
pub struct Application {
    db: SurrealDatabase,
    fs: S3Bucket,
    request_client: reqwest::Client,
}

#[derive(Clone)]
pub struct ApplicationConfig {
    pub aws_key: String,
    pub aws_key_secret: String,
    pub s3_region: String,
    pub aws_bucket: String,
    pub surreal_url: String,
    // pub surreal_username: String,  
    // pub surreal_password: String,
}

impl AppState for Application {
    type D = SurrealDatabase;
    type F = S3Bucket;
    type C = ApplicationConfig;
    // type R = ReqwestClient;

    async fn new(config: Self::C) -> Self {
        let aws_config = aws_sdk_s3::config::Builder::new()
            .region(aws_sdk_s3::config::Region::new(config.s3_region.clone()))
            .credentials_provider(aws_sdk_s3::config::Credentials::new(
                config.aws_key,
                config.aws_key_secret,
                None,
                None,
                "loaded-from-custom-env",
            ))
            .build();

        let fs = S3Bucket::new(aws_config, &config.s3_region, &config.aws_bucket);

        info!("Connecting to SurrealDB at {}", config.surreal_url);
        let client = match Surreal::new::<Ws>(config.surreal_url.clone()).await {
            Ok(client) => client,
            Err(e) => {
                error!("Failed to connect to SurrealDB at {}: {}", config.surreal_url, e);
                panic!("Database connection failed. Please check your SurrealDB URL and ensure the server is running.");
            }
        };

        match client
            .signin(Root {
                username: "root",
                password: "root",
            })
            .await
        {
            Ok(_) => info!("Successfully authenticated with SurrealDB"),
            Err(e) => {
                error!("Failed to authenticate with SurrealDB: {}", e);
                panic!("Database authentication failed. Please check your credentials.");
            }
        }

        match client.use_ns("crayon").use_db("v1").await {
            Ok(_) => info!("Successfully connected to namespace and database"),
            Err(e) => {
                error!("Failed to use namespace and database: {}", e);
                panic!("Failed to select namespace and database.");
            }
        }

        // if let Err(e) = setup_schema(&client).await {
        //     error!("Failed to setup schema: {}", e);
        //     panic!("Schema setup failed.");
        // }

        let db = SurrealDatabase::new(client);

        let request_client = reqwest::Client::new();

        Self {
            db,
            fs,
            request_client,
        }
    }

    fn database(&self) -> &Self::D {
        &self.db
    }
    fn file_storage(&self) -> &Self::F {
        &self.fs
    }
    fn request_client(&self) -> &reqwest::Client {
        &self.request_client
    }
}

async fn setup_schema(client: &Surreal<Client>) -> Result<(), surrealdb::Error> {
    // Define your schema
    let schema_setup = r#"
    -- Define a user table
    DEFINE TABLE user SCHEMAFULL;
    
    -- Define fields for the user table with types and constraints
    DEFINE FIELD email ON TABLE user TYPE string;
    DEFINE FIELD first_name ON TABLE user TYPE option<string>;
    DEFINE FIELD last_name ON TABLE user TYPE option<string>;
    DEFINE FIELD image_url ON TABLE user TYPE string;
    DEFINE FIELD created_at ON TABLE user TYPE datetime DEFAULT time::now();
    DEFINE FIELD updated_at ON TABLE user TYPE datetime DEFAULT time::now();
    
    -- Define an index for faster lookups
    DEFINE INDEX userEmailIndex ON TABLE user COLUMNS email UNIQUE;
    
"#;
    
    let _ = client.query(schema_setup).await?;
    Ok(())
}