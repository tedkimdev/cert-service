use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::json;

use crate::errors::certificate::CertificateError;

pub mod certificate;

#[derive(Debug)]
pub enum AppError {
    Certificate(CertificateError),
    Internal(String),
    Database(sqlx::Error),
}

impl From<sqlx::Error> for AppError {
    fn from(e: sqlx::Error) -> Self {
        match e {
            _ => AppError::Database(e),
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::Certificate(e) => match e {
                CertificateError::NotFound(id) => (
                    StatusCode::NOT_FOUND,
                    format!("Certificate not foudn: {}", id),
                ),
                CertificateError::AlreadyExpired => (
                    StatusCode::BAD_REQUEST,
                    "Certificate is already expired".to_string(),
                ),
                CertificateError::InvalidSan(san) => (
                    StatusCode::BAD_REQUEST,
                    format!("Invalid SAN entry: {}", san),
                ),
                CertificateError::InvalidPem(msg) => {
                    (StatusCode::BAD_REQUEST, format!("Invalid PEM: {}", msg))
                }
            },
            AppError::Database(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Database error: {}", e),
            ),
            AppError::Internal(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
        };

        (status, Json(json!({ "error": message }))).into_response()
    }
}
