use axum::{http::StatusCode, response::IntoResponse, Json};
use thiserror::Error;

use crate::ServiceResponse;

/// App Errors Defined
#[derive(Debug, Error)]
pub enum AppError {
    #[error("sql error: {0}")]
    SqlError(#[from] sqlx::Error),

    #[error("io error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Not found record: {0}")]
    NotFoundRecord(String),

    #[error("Already exists url record: {0}")]
    AlreadyExists(String),

    #[error(transparent)]
    AnyhowError(#[from] anyhow::Error),

    #[error("Service inner error")]
    UnknownError,
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let status = match &self {
            AppError::SqlError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::IoError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::AnyhowError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::UnknownError => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::NotFoundRecord(_) => StatusCode::NOT_FOUND,
            AppError::AlreadyExists(_) => StatusCode::CONFLICT,
        };

        (status, Json(ServiceResponse::error(self.to_string()))).into_response()
    }
}