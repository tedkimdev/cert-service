use axum::{
    Router,
    routing::{get, post},
};

use crate::app_state::AppState;
use crate::handlers::certificates::{create_certificate, get_certificate, list_certificates};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/certificates", post(create_certificate))
        .route("/certificates", get(list_certificates))
        .route("/certificates/{id}", get(get_certificate))
}
