use uuid::Uuid;

#[derive(Debug)]
pub enum CertificateError {
    NotFound(Uuid),
    AlreadyExpired,
    InvalidSan(String),
    InvalidPem(String),
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    #[test]
    fn test_certificate_not_found_error() {
        let id = Uuid::now_v7();
        let err = CertificateError::NotFound(id);
        assert!(matches!(err, CertificateError::NotFound(_)));
    }

    #[test]
    fn test_certificate_invalid_pem_error() {
        let err = CertificateError::InvalidPem("bad pem".to_string());
        assert!(matches!(err, CertificateError::InvalidPem(_)));
    }

    #[test]
    fn test_certificate_already_expired_error() {
        let err = CertificateError::AlreadyExpired;
        assert!(matches!(err, CertificateError::AlreadyExpired));
    }

    #[test]
    fn test_certificate_invalid_san_error() {
        let err = CertificateError::InvalidSan("bad san".to_string());
        assert!(matches!(err, CertificateError::InvalidSan(_)));
    }
}
