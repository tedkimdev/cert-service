use std::sync::Arc;

use sqlx::PgPool;

use crate::application::certificate::ports::input::certificate_service::CertificateService;

#[derive(Clone)]
pub struct AppState {
    pub certificate_service: Arc<dyn CertificateService + Send + Sync>,
    pub pool: PgPool,
}
