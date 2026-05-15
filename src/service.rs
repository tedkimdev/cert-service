use async_trait::async_trait;
use std::sync::Arc;
use uuid::Uuid;

use crate::{
    certificate_parser::parse_pem,
    dto::{CertificateListResponse, CertificateResponse, CreateCertificateRequest},
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
    ) -> Result<CertificateResponse, AppError>;

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
}

impl CertificateServiceImpl {
    pub fn new(repo: Arc<dyn CertificatesRepository + Send + Sync>) -> Self {
        Self { repo }
    }
}

#[async_trait]
impl CertificateService for CertificateServiceImpl {
    async fn create_certificate(
        &self,
        req: &CreateCertificateRequest,
    ) -> Result<CertificateResponse, AppError> {
        let param = match req {
            CreateCertificateRequest::Manual {
                subject,
                issuer,
                expiration,
                san_entries,
            } => InsertCertificateParam {
                subject: subject.clone(),
                issuer: issuer.clone(),
                expiration: *expiration,
                san_entries: san_entries.clone(),
            },
            CreateCertificateRequest::Pem { pem } => {
                let parsed = parse_pem(pem)?;
                InsertCertificateParam {
                    subject: parsed.subject,
                    issuer: parsed.issuer,
                    expiration: parsed.expiration,
                    san_entries: parsed.san_entries,
                }
            }
        };
        let cert = self.repo.insert(&param).await?;
        Ok(cert.into())
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
        })
    }
}
