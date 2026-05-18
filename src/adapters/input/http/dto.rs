use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::certificate::Certificate;

// TODO: Consider renaming to request.rs/response.rs or api.rs
// TODO: Move to infrastructure/http/dto.rs when refactoring to Clean Architecture

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum CreateCertificateRequest {
    Manual {
        subject: String,
        issuer: String,
        expiration: DateTime<Utc>,
        san_entries: Vec<String>,
    },
    Pem {
        pem: String,
    },
}

#[derive(Debug, Serialize)]
pub struct CertificateResponse {
    pub id: Uuid,
    pub subject: String,
    pub issuer: String,
    pub expiration: DateTime<Utc>,
    pub san_entries: Vec<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct IssueCertificateResponse {
    pub id: Uuid,
    pub subject: String,
    pub issuer: String,
    pub expiration: DateTime<Utc>,
    pub san_entries: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub pem: String,
}

#[derive(Debug, Serialize)]
pub struct CertificateListResponse {
    pub data: Vec<CertificateResponse>,
    pub next_cursor: Option<Uuid>,
    pub has_more: bool,
    pub total: i64,
    pub expiring_soon_count: i64,
}

#[derive(Debug, Deserialize)]
pub struct ListCertificatesQuery {
    pub cursor: Option<Uuid>,
    pub limit: Option<i64>,
}

impl From<Certificate> for CertificateResponse {
    fn from(cert: Certificate) -> Self {
        CertificateResponse {
            id: cert.id,
            subject: cert.subject,
            issuer: cert.issuer,
            expiration: cert.expiration,
            san_entries: cert.san_entries,
            created_at: cert.created_at,
        }
    }
}
