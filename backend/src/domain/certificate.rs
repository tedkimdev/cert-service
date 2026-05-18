use chrono::{DateTime, Utc};
use uuid::Uuid;

// Core domain entity
pub struct Certificate {
    pub id: Uuid,
    pub subject: String,
    pub issuer: String,
    pub expiration: DateTime<Utc>,
    pub san_entries: Vec<String>,
    pub created_at: DateTime<Utc>,
}
