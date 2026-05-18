use std::net::SocketAddr;
use std::sync::Arc;

use axum::Router;
use axum_server::tls_rustls::RustlsConfig;
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;
use tower_http::request_id::{MakeRequestUuid, PropagateRequestIdLayer, SetRequestIdLayer};
use tower_http::trace::TraceLayer;

use crate::adapters::input::http::certificates::routes as certificate_routes;
use crate::adapters::input::http::health::routes as health_routes;
use crate::adapters::output::certificate_repository::PostgresCertificateRepository;
use crate::adapters::output::dummy_ca_service::DummyCaService;
use crate::app_state::AppState;
use crate::application::certificate::service::CertificateServiceImpl;

pub struct Application {
    pub address: String,
    pub router: Router,
    pub pool: PgPool,
}

impl Application {
    pub async fn build(database_url: &str, address: &str) -> Result<Self, anyhow::Error> {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(database_url)
            .await?;

        sqlx::migrate!("./migrations").run(&pool).await?;

        let repo = Arc::new(PostgresCertificateRepository::new(pool.clone()));
        let ca_service = Arc::new(DummyCaService);
        let service = Arc::new(CertificateServiceImpl::new(repo, ca_service));

        let app_state = AppState {
            certificate_service: service,
            pool: pool.clone(),
        };

        let router = Router::new()
            .merge(certificate_routes::routes())
            .merge(health_routes::routes())
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

        Ok(Self {
            address: address.to_string(),
            router,
            pool,
        })
    }

    pub async fn run_https(self) -> Result<(), anyhow::Error> {
        let config = RustlsConfig::from_pem_file("certs/cert.pem", "certs/key.pem").await?;
        let addr: SocketAddr = self.address.parse()?;

        tracing::info!("Server running on {} with TLS", self.address);

        axum_server::bind_rustls(addr, config)
            .serve(self.router.into_make_service())
            .await?;

        Ok(())
    }

    pub async fn run_http(self) -> Result<(), anyhow::Error> {
        let listener = tokio::net::TcpListener::bind(&self.address).await?;

        tracing::info!("Server running on {}", self.address);

        axum::serve(listener, self.router)
            .with_graceful_shutdown(shutdown_signal())
            .await?;

        Ok(())
    }

    // for test
    pub async fn run_http_random_port(self) -> String {
        let listener = tokio::net::TcpListener::bind("0.0.0.0:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        let address = format!("http://127.0.0.1:{}", addr.port());

        tokio::spawn(async move {
            axum::serve(listener, self.router).await.unwrap();
        });

        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

        address
    }
}

async fn shutdown_signal() {
    use tokio::signal;

    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("Failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("Failed to install SIGTERM handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {
            tracing::info!("Received Ctrl+C, shutting down...");
        },
        _ = terminate => {
            tracing::info!("Received SIGTERM, shutting down...");
        },
    }
}
