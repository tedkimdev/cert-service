# Glossary

## PKI (Public Key Infrastructure)
The entire system of policies, roles, and technologies used to issue, manage, and verify digital certificates. Includes CAs, certificates, private/public keys, and revocation mechanisms.

## CA (Certificate Authority)
A trusted entity that signs and issues digital certificates. Acts like a government issuing passports — it vouches for the identity of the certificate holder.
- Examples: Let's Encrypt, DigiCert, internal CA

## Certificate (X.509)
A digitally signed document that binds a public key to an identity. Contains fields like Subject, Issuer, Expiration, SAN entries, and a cryptographic signature.

## CSR (Certificate Signing Request)
A request sent to a CA containing the applicant's public key and identity information. The CA verifies and signs it to produce a certificate.

## Private Key
A secret cryptographic key that must never leave the owner's system. Used to prove identity and decrypt data.

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
A cryptographic protocol for securing network communications. Only the server presents a certificate.

## mTLS (Mutual TLS)
A variant of TLS where both client and server present and verify certificates. Used for service-to-service authentication in microservices.

## HSM (Hardware Security Module)
A physical device designed to securely store and manage cryptographic keys. Used in high-security environments like banks and government.

## KMS (Key Management Service)
A cloud-managed service for creating, storing, and controlling cryptographic keys.
- Examples: AWS KMS, Google Cloud KMS, Azure Key Vault

## CRL (Certificate Revocation List)
A list of certificates that have been revoked before their expiration date.

## OCSP (Online Certificate Status Protocol)
A protocol for checking the revocation status of a certificate in real-time.

## Certificate Lifecycle
The stages a certificate goes through:
1. **Issuance** — CSR → CA signing → certificate issued
2. **Active** — certificate in use
3. **Rotation** — replacing an expiring certificate with a new one
4. **Revocation** — invalidating a certificate before expiry
5. **Expiration** — certificate reaches its `Not After` date

## UUID v7
A UUID format that includes a timestamp prefix, enabling time-ordered sorting. Better for database indexing than UUID v4 (random).

## Keyset Pagination
A pagination strategy using a cursor (last seen ID) instead of offset. More efficient for large datasets as it avoids scanning skipped rows.

## Service Mesh
Infrastructure layer that handles service-to-service communication. Provides mTLS, observability, and traffic management automatically.
- Examples: Istio, Linkerd

## Istio
A popular service mesh for Kubernetes. Automatically handles mTLS between services using sidecar proxies.

## Sidecar Proxy
A proxy container running alongside each service pod. Intercepts all network traffic to enforce mTLS, collect metrics, and manage routing.

## Ingress (Kubernetes)
A Kubernetes resource that manages external access to services. Acts as a reverse proxy with TLS termination and routing rules.

## HPA (Horizontal Pod Autoscaler)
A Kubernetes resource that automatically scales the number of pods based on CPU or memory usage.

## GitOps
A deployment methodology where Git is the single source of truth for infrastructure and application state.

## ArgoCD
A GitOps tool for Kubernetes that continuously syncs cluster state with a Git repository.

## App of Apps Pattern
An ArgoCD pattern where a root application manages multiple child applications, enabling centralized deployment management.

## SSR (Server-Side Rendering)
Rendering HTML on the server before sending it to the browser. Provides faster initial load and better SEO.

## SWR (Stale-While-Revalidate)
A data fetching strategy and React library. Shows cached data immediately while fetching fresh data in the background.

## Hydration
The process of React attaching event listeners and state to server-rendered HTML, making it interactive.

## Tokio
The async runtime for Rust. Enables non-blocking I/O and concurrent task execution.

## Axum
A Rust web framework built on top of Tokio and Tower. Provides ergonomic routing, extractors, and middleware.

## SQLx
An async Rust SQL toolkit with compile-time query verification. Ensures SQL queries are valid at compile time.

## Repository Pattern
A design pattern that abstracts data access logic behind an interface (trait). Separates business logic from storage concerns.

## Clean Architecture
A software architecture that organizes code into layers with strict dependency rules. Inner layers (domain) know nothing about outer layers (infrastructure).
