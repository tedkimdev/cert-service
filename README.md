# cert-service

Certificate Issuance and Inventory Microservice built with Rust.

## Stack

- **Backend:** Rust (Axum, SQLx, Tokio)
- **Frontend:** Next.js (SSR, SWR)
- **Database:** PostgreSQL
- **Infrastructure:** Docker, Kubernetes, Istio

## Project Structure
```
cert-service/
backend/    ← Rust REST API
frontend/   ← Next.js
docs/       ← High-level design, diagrams
```
## Getting Started

```bash
# Generate TLS certificates
cd backend && make certs

# Start all services
# Migrations run automatically on startup
docker compose up
```

## Development

```bash
cd backend

# Run database only
make db

# Run migrations
make migrate

# Run backend (without Docker)
make run
```

## Environment Variables

Copy `.env.example` to `.env` and update values:

```bash
cp backend/.env.example backend/.env
```

## Documentation

See [docs/high-level-design.md](docs/high-level-design.md) for system design.
