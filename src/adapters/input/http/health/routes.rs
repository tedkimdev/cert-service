use axum::{Router, routing::get};

use crate::{
    adapters::input::http::health::handler::{liveness, readiness},
    app_state::AppState,
};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/health/live", get(liveness))
        .route("/health/ready", get(readiness))
}
