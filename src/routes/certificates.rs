use axum::{
    Router,
    routing::{get, post},
};

use crate::app_state::AppState;
use crate::handlers::certificates::{create_certificate, get_certificate};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/certificates", post(create_certificate))
        .route("/certificates/{id}", get(get_certificate))
}
