use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use uuid::Uuid;

use crate::{
    app_state::AppState,
    dto::{CertificateResponse, CreateCertificateRequest},
    errors::AppError,
};

pub async fn create_certificate(
    State(state): State<AppState>,
    Json(req): Json<CreateCertificateRequest>,
) -> Result<(StatusCode, Json<CertificateResponse>), AppError> {
    let cert = state.certificate_service.create_certificate(&req).await?;
    Ok((StatusCode::CREATED, Json(cert)))
}

pub async fn get_certificate(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<CertificateResponse>, AppError> {
    let cert = state.certificate_service.get_certificate(id).await?;
    Ok(Json(cert))
}
