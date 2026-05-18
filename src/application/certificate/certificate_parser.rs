use chrono::{DateTime, TimeZone, Utc};
use x509_parser::prelude::*;

use crate::errors::AppError;

pub struct ParsedCertificate {
    pub subject: String,
    pub issuer: String,
    pub expiration: DateTime<Utc>,
    pub san_entries: Vec<String>,
}

// TODO: Support other SAN entry types in addition to DNS names:
// - IP addresses (GeneralName::IPAddress)
// - Email addresses (GeneralName::RFC822Name)
// - URIs (GeneralName::URI)
// - Directory names (GeneralName::DirectoryName)
// - Registered IDs (GeneralName::RegisteredID)
// TODO: Consider returning a dedicated ParseError instead of AppError
pub fn parse_pem(pem: &str) -> Result<ParsedCertificate, AppError> {
    let (_, pem_obj) = parse_x509_pem(pem.as_bytes()).map_err(|_| {
        AppError::InvalidPem("Failed to parse PEM".to_string())
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

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_PEM: &str = r#"-----BEGIN CERTIFICATE-----
MIIDkDCCAnigAwIBAgIUJYc1EPML/XQQf6WSAU4gAaBYBWwwDQYJKoZIhvcNAQEL
BQAwOjEZMBcGA1UEAwwQdGVzdC5leGFtcGxlLmNvbTEQMA4GA1UECgwHVGVzdE9y
ZzELMAkGA1UEBhMCVVMwHhcNMjYwNTE2MTQxMDM3WhcNMjcwNTE2MTQxMDM3WjA6
MRkwFwYDVQQDDBB0ZXN0LmV4YW1wbGUuY29tMRAwDgYDVQQKDAdUZXN0T3JnMQsw
CQYDVQQGEwJVUzCCASIwDQYJKoZIhvcNAQEBBQADggEPADCCAQoCggEBANCvCaxx
LWmPa1yH7tuu4t4bT1QDIDMNaz9FlwRc01bWzCwN0bOHoZoHgJq+hdvvwOT/Sjlu
yo+b673etkuh8faPcV+QU5P9jev3WK0NKzOOJzRjUIMAZTeGPiNVt6fsVdw6a4R6
xkuOXNffAhF6CU52UMtKUBWQlrA7dSVHSk1QCkebqXlOw/6JIsFCLHI3lagdakRa
yAZGufQQ+L/BOiucKcBdWKbkI47/bB4O7gWkYyU2lbSlUWBz1QUgi5jpuAoMwbi+
qPCysH5S0BVLN608Hb0l7fUfLl9SQB24dUHTukm1Bgz2dngBd7UTwQBOVz8/0Bcj
DPZZIMn4meMHA+ECAwEAAaOBjTCBijAdBgNVHQ4EFgQUcUYLiBHu+BQYVcwez45e
yXdHkpIwHwYDVR0jBBgwFoAUcUYLiBHu+BQYVcwez45eyXdHkpIwDwYDVR0TAQH/
BAUwAwEB/zA3BgNVHREEMDAughB0ZXN0LmV4YW1wbGUuY29tghR3d3cudGVzdC5l
eGFtcGxlLmNvbYcEfwAAATANBgkqhkiG9w0BAQsFAAOCAQEAmXrURw98/QswFC+d
kcNQcyAivF9dg9oMHzQWAMb6chBoAYAdc8EVQ0GCumm1xejzZ7VOsr1MiI1MSFES
64EQblrsGEW+iiYF5hyTqJw7XMqqNDPXAyfBOSD3XTM9QNqNPcA4vVKISUVu9CoU
im41C6m7c97IQA5prhpGZwW1wLKpgC2ejf9XLdit9oD7H3QbcrIMji51/pU8bHz8
v7HyzOL1SombWp6DDLME8uqpHOPPuXxwLPLLHg6KXYq1oytVPSP/0UxDvDYX8GYx
aLE8WxcpVK3f/cLAJCbac8sDLTBVSzukAQP+zpvSyhrg4WCkzFHRIajcuqI0UZi/
TKlk2A==
-----END CERTIFICATE-----"#;

    #[test]
    fn test_parse_valid_pem() {
        let result = parse_pem(TEST_PEM);
        assert!(result.is_ok());

        let parsed = result.unwrap();
        assert!(!parsed.subject.is_empty());
        assert!(!parsed.issuer.is_empty());
    }

    #[test]
    fn test_parse_invalid_pem() {
        let result = parse_pem("invalid pem");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_san_entries() {
        let result = parse_pem(TEST_PEM);
        assert!(result.is_ok());

        let parsed: ParsedCertificate = result.unwrap();
        assert!(!parsed.san_entries.is_empty());
    }
}
