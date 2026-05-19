use async_trait::async_trait;
use uuid::Uuid;

use crate::adapters::input::http::certificates::dto::{
    CertificateListResponse, CertificateResponse, CreateCertificateRequest,
    IssueCertificateResponse,
};
use crate::errors::AppError;

#[async_trait]
pub trait CertificateService: Send + Sync {
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
