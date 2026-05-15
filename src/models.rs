use chrono::{DateTime, Utc};
use uuid::Uuid;

// TODO: Move to domain/certificate.rs when refactoring to Clean Architecture

// service → repository
pub struct InsertCertificateParam {
    pub subject: String,
    pub issuer: String,
    pub expiration: DateTime<Utc>,
    pub san_entries: Vec<String>,
}

pub struct Certificate {
    pub id: Uuid,
    pub subject: String,
    pub issuer: String,
    pub expiration: DateTime<Utc>,
    pub san_entries: Vec<String>,
    pub created_at: DateTime<Utc>,
}
