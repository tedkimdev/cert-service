use uuid::Uuid;

#[derive(Debug)]
pub enum CertificateError {
    NotFound(Uuid),
    AlreadyExpired,
    InvalidSan(String),
    InvalidPem(String),
}
