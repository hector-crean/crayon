use axum::{response::IntoResponse, Json};
use http::StatusCode;
use surrealdb::{error::Db,  Error};

use crate::{database::DatabaseError, ErrorResponse};


#[derive(thiserror::Error, Debug)]
pub enum SurrealError {
    #[error(transparent)]
    Surreal(#[from] surrealdb::Error),
}


impl IntoResponse for SurrealError {
    fn into_response(self) -> axum::response::Response {
        let (status_code, error_type, details) = match &self {
            Self::Surreal(e) => match e {
                Error::Db(e) => match e {
                    Db::NoRecordFound => (StatusCode::NOT_FOUND, "NOT_FOUND", None),
                    Db::IdNotFound { rid } => (StatusCode::NOT_FOUND, "NOT_FOUND", Some(rid.to_string())),
                    Db::InvalidQuery(e) => (StatusCode::BAD_REQUEST, "INVALID_QUERY", Some(e.to_string())),
                    _ => (StatusCode::INTERNAL_SERVER_ERROR, "DATABASE_ERROR", Some(e.to_string())),
                },
                _ => (StatusCode::INTERNAL_SERVER_ERROR, "UNEXPECTED_ERROR", Some(e.to_string())),
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

impl DatabaseError for SurrealError {}