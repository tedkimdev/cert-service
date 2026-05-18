use axum::{
    Router,
    routing::{get, post},
};

use crate::{
    adapters::input::http::certificates::handler::{
        create_certificate, get_certificate, list_certificates,
    },
    app_state::AppState,
};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/certificates", post(create_certificate))
        .route("/certificates", get(list_certificates))
        .route("/certificates/{id}", get(get_certificate))
}
