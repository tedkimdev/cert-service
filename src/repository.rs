use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::{CertificateResponse, CreateCertificateRequest};

#[async_trait]
pub trait CertificatesRepository {
    async fn insert(
        &self,
        req: &CreateCertificateRequest,
    ) -> Result<CertificateResponse, sqlx::Error>;

    async fn find_by_id(&self, id: Uuid) -> Result<CertificateResponse, sqlx::Error>;
}

pub struct PostgresCertificateRepository {
    pool: PgPool,
}

impl PostgresCertificateRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl CertificatesRepository for PostgresCertificateRepository {
    async fn insert(
        &self,
        req: &CreateCertificateRequest,
    ) -> Result<CertificateResponse, sqlx::Error> {
        let cert_id = Uuid::now_v7();

        let mut tx = self.pool.begin().await?;

        sqlx::query!(
            r#"
                INSERT INTO certificates (id, subject, issuer, expiration)
                VALUES ($1, $2, $3, $4)
            "#,
            cert_id,
            req.subject,
            req.issuer,
            req.expiration,
        )
        .execute(&mut *tx)
        .await?;

        for san in &req.san_entries {
            let san_id = Uuid::now_v7();
            sqlx::query!(
                r#"
                    INSERT INTO san_entries (id, certificate_id, value)
                    VALUES ($1, $2, $3)
                "#,
                san_id,
                cert_id,
                san,
            )
            .execute(&mut *tx)
            .await?;
        }

        tx.commit().await?;

        self.find_by_id(cert_id).await
    }

    async fn find_by_id(&self, id: Uuid) -> Result<CertificateResponse, sqlx::Error> {
        let cert = sqlx::query!(
            r#"
                SELECT id, subject, issuer, expiration, created_at
                FROM certificates
                WHERE id = $1
            "#,
            id,
        )
        .fetch_one(&self.pool)
        .await?;

        let san_entries = sqlx::query!(
            r#"
                SELECT value FROM san_entries
                WHERE certificate_id = $1
            "#,
            id,
        )
        .fetch_all(&self.pool)
        .await?
        .into_iter()
        .map(|row| row.value)
        .collect();

        Ok(CertificateResponse {
            id: cert.id,
            subject: cert.subject,
            issuer: cert.issuer,
            expiration: cert.expiration,
            san_entries,
            created_at: cert.created_at,
        })
    }
}
