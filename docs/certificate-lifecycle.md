# Certificate Lifecycle

## Overview

The complete journey of a certificate from creation to expiration or revocation.

```
Issuance → Active → Rotation → Revocation/Expiration
```

---

## Stages

### 1. Issuance

The process of creating and signing a new certificate.

**Real-world flow:**

```
Client
↓ Generate key pair (Private Key + Public Key)
↓ Create CSR (Public Key + identity info)
↓ Send CSR to CA
CA
↓ Verify CSR
↓ Sign with CA's Private Key
↓ Issue certificate
Client
↓ Store certificate + Private Key securely
```

**In this assessment (Dummy CA):**

```
Client
↓ POST /certificates (JSON metadata or PEM)
cert-service
↓ Parse & validate
↓ Generate UUID v7
↓ Store metadata in PostgreSQL
↓ Return CertificateResponse
```

> In production, the client would generate a key pair and CSR locally. The Private Key never leaves the client. Only the CSR is sent to the CA.

---

### 2. Active

The certificate is valid and in use.

- Certificate is within its validity period (`Not Before` → `Not After`)
- Used for TLS handshakes, mTLS, or identity verification
- Should be monitored for upcoming expiration

**In this assessment:**
- Certificate metadata stored in PostgreSQL
- `expiring_soon_count` tracks certificates expiring within 30 days
- Dashboard displays expiration status

---

### 3. Rotation

Replacing an expiring certificate with a new one before it expires.

**Why it matters:**
- Expired certificates cause service outages
- Manual rotation is error-prone at scale
- Automation is essential in microservice environments

**Rotation strategies:**

| Strategy | Description |
|---|---|
| Short-lived certificates | Issue certificates with short TTL (e.g., 24h) and auto-renew |
| cert-manager | Kubernetes tool that automates certificate rotation |
| ACME Protocol | Automated certificate renewal (used by Let's Encrypt) |
| Istio | Automatically rotates certificates for service mesh mTLS |

**In production:**

```
cert-manager monitors expiration
↓ 30 days before expiry → request new certificate
↓ New certificate issued by CA
↓ Old certificate replaced with zero downtime
↓ Old certificate archived
```

---

### 4. Revocation

Invalidating a certificate before its expiration date.

**When to revoke:**
- Private Key compromised
- Certificate issued incorrectly
- Service decommissioned

**Revocation mechanisms:**

| Mechanism | Description |
|---|---|
| CRL (Certificate Revocation List) | A list of revoked certificates published by the CA |
| OCSP (Online Certificate Status Protocol) | Real-time revocation status check |

**In production:**

```
Revocation request
↓ CA adds certificate to CRL
↓ OCSP responder updated
↓ Clients check OCSP before trusting certificate
```

---

### 5. Expiration

The certificate reaches its `Not After` date and becomes invalid.

**Consequences:**
- TLS handshakes fail
- Services can no longer authenticate
- Potential outage

**Prevention:**
- Monitor expiration dates
- Set up automated alerts (30 days, 7 days, 1 day before expiry)
- Implement automated rotation

---

## Key Storage

Private keys must be stored securely throughout the certificate lifecycle.

| Option | Description | Use Case |
|---|---|---|
| HSM | Physical hardware device | Highest security (banks, government) |
| KMS | Cloud-managed key service | Most production systems |
| Local file | Stored on disk | Development only |

> In production, Private Keys must never be stored in plaintext or in a database. Always use KMS or HSM.

---

## In This Assessment

| Stage | Implementation |
|---|---|
| Issuance | `POST /certificates` — JSON or PEM |
| Active | Metadata stored in PostgreSQL |
| Monitoring | `expiring_soon_count` in API response |
| Rotation | Documented only — would use cert-manager in production |
| Revocation | Documented only — would use CRL/OCSP in production |
| Key Storage | Documented only — would use AWS KMS or HSM in production |