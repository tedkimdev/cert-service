use async_trait::async_trait;
use chrono::{DateTime, Utc};

use crate::errors::AppError;

pub struct IssuedCertificate {
    pub pem: String,
    pub subject: String,
    pub issuer: String,
    pub expiration: DateTime<Utc>,
    pub san_entries: Vec<String>,
}

#[async_trait]
pub trait CaService: Send + Sync {
    async fn issue_certificate(
        &self,
        subject: &str,
        san_entries: &[String],
        expiration: DateTime<Utc>,
    ) -> Result<IssuedCertificate, AppError>;
}
