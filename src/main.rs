use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};

use cert_service_practice::app::Application;

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

    let app = Application::build(&database_url, "0.0.0.0:3000")
        .await
        .expect("Failed to build applciation");
    app.run_https().await.expect("Failed to run application");
}
