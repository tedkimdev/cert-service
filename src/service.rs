use async_trait::async_trait;
use std::sync::Arc;
use uuid::Uuid;

use crate::{
    errors::AppError,
    models::{CertificateResponse, CreateCertificateRequest},
    repository::CertificatesRepository,
};

// Service trait
#[async_trait]
pub trait CertificateService {
    async fn create_certificate(
        &self,
        req: &CreateCertificateRequest,
    ) -> Result<CertificateResponse, AppError>;

    async fn get_certificate(
        &self,
        id: Uuid,
    ) -> Result<CertificateResponse, AppError>;
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
        let cert = self.repo.insert(req).await?;
        Ok(cert)
    }

    async fn get_certificate(
        &self,
        id: Uuid,
    ) -> Result<CertificateResponse, AppError> {
        let cert = self.repo.find_by_id(id).await?;
        Ok(cert)
    }
}