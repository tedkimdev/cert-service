use axum::{Router, routing::get};

use crate::{
    app_state::AppState,
    handlers::health::{liveness, readiness},
};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/health/live", get(liveness))
        .route("/health/ready", get(readiness))
}
