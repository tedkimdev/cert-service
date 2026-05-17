use async_trait::async_trait;
use std::sync::Arc;
use uuid::Uuid;

use crate::{
    ca::CaService,
    certificate_parser::parse_pem,
    dto::{
        CertificateListResponse, CertificateResponse, CreateCertificateRequest,
        IssueCertificateResponse,
    },
    errors::AppError,
    models::InsertCertificateParam,
    repository::CertificatesRepository,
};

// Service trait
#[async_trait]
pub trait CertificateService {
    async fn create_certificate(
        &self,
        req: &CreateCertificateRequest,
    ) -> Result<IssueCertificateResponse, AppError>;

    async fn get_certificate(&self, id: Uuid) -> Result<CertificateResponse, AppError>;
    async fn list_certificates(
        &self,
        cursor: Option<Uuid>,
        limit: Option<i64>,
    ) -> Result<CertificateListResponse, AppError>;
}

// 구현체
pub struct CertificateServiceImpl {
    repo: Arc<dyn CertificatesRepository + Send + Sync>,
    ca_service: Arc<dyn CaService + Send + Sync>,
}

impl CertificateServiceImpl {
    pub fn new(
        repo: Arc<dyn CertificatesRepository + Send + Sync>,
        ca_service: Arc<dyn CaService + Send + Sync>,
    ) -> Self {
        Self { repo, ca_service }
    }
}

#[async_trait]
impl CertificateService for CertificateServiceImpl {
    async fn create_certificate(
        &self,
        req: &CreateCertificateRequest,
    ) -> Result<IssueCertificateResponse, AppError> {
        let param = match req {
            CreateCertificateRequest::Manual {
                subject,
                issuer,
                expiration,
                san_entries,
            } => {
                let issued = self
                    .ca_service
                    .issue_certificate(subject, san_entries, *expiration)
                    .await?;

                (
                    InsertCertificateParam {
                        subject: subject.clone(),
                        issuer: issuer.clone(),
                        expiration: *expiration,
                        san_entries: san_entries.clone(),
                    },
                    issued.pem,
                )
            }
            CreateCertificateRequest::Pem { pem } => {
                let parsed = parse_pem(pem)?;
                let issued = self
                    .ca_service
                    .issue_certificate(&parsed.subject, &parsed.san_entries, parsed.expiration)
                    .await?;

                (
                    InsertCertificateParam {
                        subject: parsed.subject,
                        issuer: parsed.issuer,
                        expiration: parsed.expiration,
                        san_entries: parsed.san_entries,
                    },
                    issued.pem,
                )
            }
        };
        let (insert_param, pem) = param;
        let cert = self.repo.insert(&insert_param).await?;
        Ok(IssueCertificateResponse {
            id: cert.id,
            subject: cert.subject,
            issuer: cert.issuer,
            expiration: cert.expiration,
            san_entries: cert.san_entries,
            created_at: cert.created_at,
            pem,
        })
    }

    async fn get_certificate(&self, id: Uuid) -> Result<CertificateResponse, AppError> {
        let cert = self.repo.find_by_id(id).await?;
        Ok(cert.into())
    }

    async fn list_certificates(
        &self,
        cursor: Option<Uuid>,
        limit: Option<i64>,
    ) -> Result<CertificateListResponse, AppError> {
        let limit = limit.unwrap_or(10).min(100);

        let (mut certs, total) = self.repo.find_all(cursor, limit).await?;
        let expiring_soon_count = self.repo.count_expiring_soon().await?;

        let has_more = certs.len() as i64 > limit;
        if has_more {
            certs.pop();
        }

        let next_cursor = if has_more {
            certs.last().map(|c| c.id)
        } else {
            None
        };

        Ok(CertificateListResponse {
            data: certs.into_iter().map(|c| c.into()).collect(),
            next_cursor,
            has_more,
            total,
            expiring_soon_count,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use chrono::Utc;
    use std::sync::Arc;
    use uuid::Uuid;

    use crate::models::{Certificate, InsertCertificateParam};

    struct MockCertificateRepository {
        should_find: bool,
    }

    #[async_trait]
    impl CertificatesRepository for MockCertificateRepository {
        async fn insert(&self, req: &InsertCertificateParam) -> Result<Certificate, sqlx::Error> {
            Ok(Certificate {
                id: Uuid::now_v7(),
                subject: req.subject.clone(),
                issuer: req.issuer.clone(),
                expiration: req.expiration,
                san_entries: req.san_entries.clone(),
                created_at: Utc::now(),
            })
        }

        async fn find_by_id(&self, id: Uuid) -> Result<Certificate, sqlx::Error> {
            if self.should_find {
                Ok(Certificate {
                    id,
                    subject: "example.com".to_string(),
                    issuer: "MyRootCA".to_string(),
                    expiration: Utc::now() + chrono::Duration::days(365),
                    san_entries: vec!["www.example.com".to_string()],
                    created_at: Utc::now(),
                })
            } else {
                Err(sqlx::Error::RowNotFound)
            }
        }

        async fn find_all(
            &self,
            _cursor: Option<Uuid>,
            _limit: i64,
        ) -> Result<(Vec<Certificate>, i64), sqlx::Error> {
            Ok((vec![], 0))
        }

        async fn count_expiring_soon(&self) -> Result<i64, sqlx::Error> {
            Ok(0)
        }
    }

    fn make_service() -> CertificateServiceImpl {
        CertificateServiceImpl::new(Arc::new(MockCertificateRepository { should_find: false }))
    }

    #[tokio::test]
    async fn test_create_certificate_manual() {
        let service = make_service();
        let req = CreateCertificateRequest::Manual {
            subject: "example.com".to_string(),
            issuer: "MyRootCA".to_string(),
            expiration: Utc::now() + chrono::Duration::days(365),
            san_entries: vec!["www.example.com".to_string()],
        };

        let result = service.create_certificate(&req).await;
        assert!(result.is_ok());

        let cert = result.unwrap();
        assert_eq!(cert.subject, "example.com");
        assert_eq!(cert.issuer, "MyRootCA");
    }

    #[tokio::test]
    async fn test_get_certificate_success() {
        let service =
            CertificateServiceImpl::new(Arc::new(MockCertificateRepository { should_find: true }));
        let result = service.get_certificate(Uuid::now_v7()).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_certificate_not_found() {
        let service =
            CertificateServiceImpl::new(Arc::new(MockCertificateRepository { should_find: false }));
        let result = service.get_certificate(Uuid::now_v7()).await;
        assert!(result.is_err());
    }
}
