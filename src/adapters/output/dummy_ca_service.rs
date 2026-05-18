use async_trait::async_trait;
use chrono::{DateTime, Utc};
use rcgen::{CertificateParams, DistinguishedName, DnType, KeyPair, SanType, string::Ia5String};

use crate::{
    application::certificate::ports::output::ca_service::{CaService, IssuedCertificate},
    errors::AppError,
};

pub struct DummyCaService;

#[async_trait]
impl CaService for DummyCaService {
    async fn issue_certificate(
        &self,
        subject: &str,
        san_entries: &[String],
        expiration: DateTime<Utc>,
    ) -> Result<IssuedCertificate, AppError> {
        // Generate a new key pair (public + private key)
        // In production, the private key would be stored in KMS/HSM
        let key_pair = KeyPair::generate()
            .map_err(|e| AppError::Internal(format!("Failed to generate key pair: {}", e)))?;

        let mut params = CertificateParams::default();

        // Set the Subject CN (Common Name)
        let mut distinguished_name = DistinguishedName::new();
        distinguished_name.push(DnType::CommonName, subject);
        params.distinguished_name = distinguished_name;

        // Set SAN (Subject Alternative Name) entries
        // Only DNS names are supported here — IP addresses and emails could be added
        params.subject_alt_names = san_entries
            .iter()
            .filter_map(|san| Ia5String::try_from(san.as_str()).ok().map(SanType::DnsName))
            .collect();

        // Generate a self-signed certificate
        // In production, this would be replaced with a CSR sent to External Vault PKI
        let cert = params
            .self_signed(&key_pair)
            .map_err(|e| AppError::Internal(format!("Failed to generate certificate: {}", e)))?;

        // Serialize the certificate to PEM format
        let pem = cert.pem();

        Ok(IssuedCertificate {
            pem,
            subject: subject.to_string(),
            // Self-signed certificate — no real CA involved.
            // In production, issuer would be the CA's distinguished name (e.g., "CN=MyRootCA, O=MyOrg")
            issuer: "Dummy CA".to_string(),
            expiration,
            san_entries: san_entries.to_vec(),
        })
    }
}
