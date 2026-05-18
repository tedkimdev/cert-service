use async_trait::async_trait;
use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::domain::certificate::Certificate;

// TODO: Consider defining a dedicated RepositoryError enum instead of using sqlx::Error directly.
// This would make the port truly infrastructure-agnostic:
//
// pub enum RepositoryError {
//     NotFound(Uuid),
//     Conflict,
//     Internal(String),
// }
//
// And convert to AppError in the application layer:
// impl From<RepositoryError> for AppError { ... }

// Input for creating a new certificate
pub struct InsertCertificateParam {
    pub subject: String,
    pub issuer: String,
    pub expiration: DateTime<Utc>,
    pub san_entries: Vec<String>,
}

#[async_trait]
pub trait CertificatesRepository {
    async fn insert(&self, req: &InsertCertificateParam) -> Result<Certificate, sqlx::Error>;
    async fn find_by_id(&self, id: Uuid) -> Result<Certificate, sqlx::Error>;
    async fn find_all(
        &self,
        cursor: Option<Uuid>,
        limit: i64,
    ) -> Result<(Vec<Certificate>, i64), sqlx::Error>;
    async fn count_expiring_soon(&self) -> Result<i64, sqlx::Error>;
}
