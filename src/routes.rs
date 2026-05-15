use axum::{
    routing::{get, post},
    Router,
};

use crate::{
    app_state::AppState, handlers::{create_certificate, get_certificate}
};

pub fn certificate_routes() -> Router<AppState> {
    Router::new()
        .route("/certificates", post(create_certificate))
        .route("/certificates/{id}", get(get_certificate))
}