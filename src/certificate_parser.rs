use chrono::{DateTime, TimeZone, Utc};
use x509_parser::prelude::*;

use crate::errors::AppError;
use crate::errors::certificate::CertificateError;

pub struct ParsedCertificate {
    pub subject: String,
    pub issuer: String,
    pub expiration: DateTime<Utc>,
    pub san_entries: Vec<String>,
}

// TODO: parse partern
pub fn parse_pem(pem: &str) -> Result<ParsedCertificate, AppError> {
    let (_, pem_obj) = parse_x509_pem(pem.as_bytes()).map_err(|_| {
        AppError::Certificate(CertificateError::InvalidPem(
            "Failed to parse PEM".to_string(),
        ))
    })?;
    let (_, cert) = parse_x509_certificate(&pem_obj.contents)
        .map_err(|_| AppError::Internal("Failed to parse X509 certificate".to_string()))?;

    let subject = cert.subject().to_string();
    let issuer = cert.issuer().to_string();

    let expiration = Utc
        .timestamp_opt(cert.validity().not_after.timestamp(), 0)
        .single()
        .ok_or_else(|| AppError::Internal("Invalid expiration date".to_string()))?;

    let san_entries = cert
        .extensions()
        .iter()
        .find_map(|ext| {
            if let ParsedExtension::SubjectAlternativeName(san) = ext.parsed_extension() {
                Some(
                    san.general_names
                        .iter()
                        .map(|name| match name {
                            GeneralName::DNSName(dns) => dns.to_string(),
                            GeneralName::IPAddress(ip) => ip
                                .iter()
                                .map(|b| b.to_string())
                                .collect::<Vec<_>>()
                                .join("."),
                            GeneralName::RFC822Name(email) => email.to_string(),
                            GeneralName::URI(uri) => uri.to_string(),
                            _ => name.to_string(),
                        })
                        .collect(),
                )
            } else {
                None
            }
        })
        .unwrap_or_default();

    Ok(ParsedCertificate {
        subject,
        issuer,
        expiration,
        san_entries,
    })
}
