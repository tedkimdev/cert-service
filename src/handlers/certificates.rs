use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
};
use uuid::Uuid;

use crate::{
    app_state::AppState,
    dto::{
        CertificateListResponse, CertificateResponse, CreateCertificateRequest,
        IssueCertificateResponse, ListCertificatesQuery,
    },
    errors::AppError,
};

pub async fn create_certificate(
    State(state): State<AppState>,
    Json(req): Json<CreateCertificateRequest>,
) -> Result<(StatusCode, Json<IssueCertificateResponse>), AppError> {
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

pub async fn list_certificates(
    State(state): State<AppState>,
    Query(params): Query<ListCertificatesQuery>,
) -> Result<Json<CertificateListResponse>, AppError> {
    let result = state
        .certificate_service
        .list_certificates(params.cursor, params.limit)
        .await?;
    Ok(Json(result))
}
