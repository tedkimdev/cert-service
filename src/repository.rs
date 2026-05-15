use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::{Certificate, InsertCertificateParam};

#[async_trait]
pub trait CertificatesRepository {
    async fn insert(&self, req: &InsertCertificateParam) -> Result<Certificate, sqlx::Error>;

    async fn find_by_id(&self, id: Uuid) -> Result<Certificate, sqlx::Error>;
    async fn find_all(
        &self,
        cursor: Option<Uuid>,
        limit: i64,
    ) -> Result<(Vec<Certificate>, i64), sqlx::Error>;
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
    async fn insert(&self, req: &InsertCertificateParam) -> Result<(Certificate), sqlx::Error> {
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

    async fn find_by_id(&self, id: Uuid) -> Result<Certificate, sqlx::Error> {
        let cert = sqlx::query!(
            r#"
                SELECT 
                    c.id,
                    c.subject,
                    c.issuer,
                    c.expiration,
                    c.created_at,
                    ARRAY_AGG(s.value) FILTER (WHERE s.value IS NOT NULL) as san_entries
                FROM certificates c
                LEFT JOIN san_entries s ON s.certificate_id = c.id
                WHERE c.id = $1
                GROUP BY c.id
            "#,
            id,
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(Certificate {
            id: cert.id,
            subject: cert.subject,
            issuer: cert.issuer,
            expiration: cert.expiration,
            san_entries: cert.san_entries.unwrap_or_default(),
            created_at: cert.created_at,
        })
    }

    async fn find_all(
        &self,
        cursor: Option<Uuid>,
        limit: i64,
    ) -> Result<(Vec<Certificate>, i64), sqlx::Error> {
        let total: i64 = sqlx::query_scalar!(r#"SELECT COUNT(*) FROM certificates"#)
            .fetch_one(&self.pool)
            .await?
            .unwrap_or(0);

        let certs = sqlx::query!(
            r#"
                SELECT 
                    c.id,
                    c.subject,
                    c.issuer,
                    c.expiration,
                    c.created_at,
                    ARRAY_AGG(s.value) FILTER (WHERE s.value IS NOT NULL) as san_entries
                FROM certificates c
                LEFT JOIN san_entries s ON s.certificate_id = c.id
                WHERE ($1::uuid IS NULL OR c.id > $1)
                GROUP BY c.id
                ORDER BY c.id ASC
                LIMIT $2
            "#,
            cursor as Option<Uuid>,
            limit + 1,
        )
        .fetch_all(&self.pool)
        .await?;

        let mut result = Vec::new();
        for cert in certs {
            result.push(Certificate {
                id: cert.id,
                subject: cert.subject,
                issuer: cert.issuer,
                expiration: cert.expiration,
                san_entries: cert.san_entries.unwrap_or_default(),
                created_at: cert.created_at,
            });
        }

        Ok((result, total))
    }
}
