use axum::{Router, routing::get};
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() {
    // logging
    tracing_subscriber::fmt::init();

    // env
    dotenvy::dotenv().ok();

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

    // router
    let app = Router::new().route("/health", get(health_check));

    // server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Failed to bind to port 3000");

    tracing::info!("Server running on port 3000");

    axum::serve(listener, app).await.expect("Server failed");
}

async fn health_check() -> &'static str {
    "OK"
}
