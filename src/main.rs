use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};

use cert_service_practice::{app::Application, config::AppConfig};

#[tokio::main]
async fn main() {
    // env
    dotenvy::dotenv().ok();

    let config = AppConfig::load();

    // logging
    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let app = Application::build(&config)
        .await
        .expect("Failed to build applciation");
    app.run_https(&config).await.expect("Failed to run application");
}
