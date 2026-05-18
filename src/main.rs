use axum::Router;
use axum_server::tls_rustls::RustlsConfig;
use sqlx::postgres::PgPoolOptions;
use std::{net::SocketAddr, sync::Arc};
use tower_http::{
    request_id::{MakeRequestUuid, PropagateRequestIdLayer, SetRequestIdLayer},
    trace::TraceLayer,
};
use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};

use crate::{
    adapters::{
        input::http::{certificates, health},
        output::{
            certificate_repository::PostgresCertificateRepository, dummy_ca_service::DummyCaService,
        },
    },
    application::certificate::service::CertificateServiceImpl,
};

mod adapters;
mod app_state;
mod application;
mod domain;
mod errors;

#[tokio::main]
async fn main() {
    // TODO: Move to config.rs
    // - database max_connections
    // - server host/port
    // - TLS cert/key paths
    // - RUST_LOG filter

    // env
    dotenvy::dotenv().ok();

    // logging
    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // db
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to database");

    // migration
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    tracing::info!("Database connected and migrations applied");

    let repo = Arc::new(PostgresCertificateRepository::new(pool.clone()));
    let ca_service = Arc::new(DummyCaService);
    let service = Arc::new(CertificateServiceImpl::new(repo, ca_service));

    let app_state = app_state::AppState {
        certificate_service: service,
        pool,
    };

    // router
    let app = Router::new()
        .merge(health::routes::routes())
        .merge(certificates::routes::routes())
        .layer(PropagateRequestIdLayer::x_request_id())
        .layer(
            TraceLayer::new_for_http().make_span_with(|request: &axum::http::Request<_>| {
                let request_id = request
                    .headers()
                    .get("x-request-id")
                    .and_then(|v| v.to_str().ok())
                    .unwrap_or("unknown");

                tracing::info_span!(
                    "request",
                    method = %request.method(),
                    uri = %request.uri(),
                    request_id = %request_id,
                )
            }),
        )
        .layer(SetRequestIdLayer::x_request_id(MakeRequestUuid))
        .with_state(app_state);

    // server
    let config = RustlsConfig::from_pem_file("certs/cert.pem", "certs/key.pem")
        .await
        .expect("Failed to load TLS config");

    let addr: SocketAddr = "0.0.0.0:3000".parse().expect("Failed to parse address");
    tracing::info!("Server running on port 3000 with TLS");

    axum_server::bind_rustls(addr, config)
        .serve(app.into_make_service())
        .await
        .expect("Server failed");
}
