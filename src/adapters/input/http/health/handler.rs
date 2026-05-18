use axum::{Json, extract::State};
use serde::Serialize;

use crate::{app_state::AppState, errors::AppError};

#[derive(Serialize)]
pub struct LivenessResponse {
    pub status: &'static str,
    pub version: &'static str,
}

#[derive(Serialize)]
pub struct ReadinessResponse {
    pub status: &'static str,
    pub db: &'static str,
}

pub async fn liveness() -> Json<LivenessResponse> {
    Json(LivenessResponse {
        status: "ok",
        version: env!("CARGO_PKG_VERSION"),
    })
}

pub async fn readiness(State(state): State<AppState>) -> Result<Json<ReadinessResponse>, AppError> {
    sqlx::query("SELECT 1")
        .execute(&state.pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    Ok(Json(ReadinessResponse {
        status: "ok",
        db: "ok",
    }))
}
