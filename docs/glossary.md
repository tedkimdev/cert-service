# Glossary

## X.509
The international standard that defines the format of public key certificates. Specifies what fields a certificate must contain — Subject, Issuer, Expiration, SAN, Public Key, and a cryptographic signature.

## PKI (Public Key Infrastructure)
The entire system of policies, roles, and technologies used to issue, manage, and verify digital certificates. Includes CAs, certificates, private/public keys, and revocation mechanisms.

## CA (Certificate Authority)
A trusted entity that signs and issues digital certificates. Acts like a government issuing passports — it vouches for the identity of the certificate holder.
- Examples: Let's Encrypt, DigiCert, HashiCorp Vault PKI

## CSR (Certificate Signing Request)
A request sent to a CA containing the applicant's public key and identity information. The CA verifies and signs it to produce a certificate.

```
Client
↓ Generate key pair
↓ Create CSR (Public Key + identity info)
↓ Send CSR to CA
CA
↓ Verify & Sign
↓ Return certificate
```

## Private Key
A secret cryptographic key that must never leave the owner's system. Used to prove identity and decrypt data. Must be stored in KMS or HSM in production.

## Public Key
A cryptographic key that can be shared openly. Included in a CSR and embedded in the final certificate.

## CN (Common Name)
The human-readable name identifying the entity in a certificate.
- Subject CN: who the certificate belongs to (e.g., `example.com`)
- Issuer CN: who signed the certificate (e.g., `MyRootCA`)

## SAN (Subject Alternative Name)
A certificate extension listing additional domains or IP addresses the certificate is valid for.
- Examples: `DNS:example.com`, `DNS:www.example.com`, `IP:192.168.1.1`

## PEM (Privacy Enhanced Mail)
A Base64-encoded format for storing certificates and keys, wrapped in header/footer lines.

```
-----BEGIN CERTIFICATE-----
...
-----END CERTIFICATE-----
```

## DER (Distinguished Encoding Rules)
The binary format of a certificate. PEM is Base64-encoded DER.

## TLS (Transport Layer Security)
A cryptographic protocol for securing network communications. Only the server presents a certificate — the client is anonymous.

## mTLS (Mutual TLS)
A variant of TLS where both client and server present and verify certificates. Used for service-to-service authentication in microservices.

## CRL (Certificate Revocation List)
A list of revoked certificates published by the CA. Clients download periodically and check against it. Not real-time — there's a delay between revocation and list update.

## OCSP (Online Certificate Status Protocol)
A protocol for real-time certificate revocation status checking. Client asks the OCSP responder: "Is this certificate still valid?"

## HSM (Hardware Security Module)
A physical device designed to securely store and manage cryptographic keys. Used in high-security environments like banks and government. Private Keys never leave the device.

## KMS (Key Management Service)
A cloud-managed service for creating, storing, and controlling cryptographic keys.
- Examples: AWS KMS, Google Cloud KMS, Azure Key Vault
- All signing operations performed inside KMS boundary — Private Key never exported.

## cert-manager
An open-source Kubernetes tool that automates certificate issuance and rotation. Integrates with CAs like HashiCorp Vault PKI and Let's Encrypt via `ClusterIssuer` configuration.

## Certificate Lifecycle
The stages a certificate goes through:
1. **Issuance** — CSR → CA signing → certificate issued
2. **Active** — certificate in use, expiration monitored
3. **Rotation** — replacing an expiring certificate with a new one (new key pair)
4. **Revocation** — invalidating a certificate before expiry (CRL/OCSP)
5. **Expiration** — certificate reaches its `Not After` date

## UUID v7
A UUID format that includes a timestamp prefix, enabling time-ordered sorting. Better for database B-tree indexing than UUID v4 (fully random).

## Keyset Pagination
A pagination strategy using a cursor (last seen ID) instead of offset. More efficient for large datasets as it avoids scanning skipped rows. No duplicate or missing records on concurrent inserts.

