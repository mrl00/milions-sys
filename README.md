# milions-sys

[![CI](https://github.com/mrl00/milions-sys/actions/workflows/ci.yml/badge.svg)](https://github.com/mrl00/milions-sys/actions/workflows/ci.yml)
[![Rust](https://img.shields.io/badge/rust-edition%202024-orange)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-proprietary-red)](#license)

Backend system for managing construction and engineering projects in the Brazilian market.

## Tech Stack

| Layer | Technology |
|-------|------------|
| Language | Rust (edition 2024) |
| Runtime | tokio (async) |
| HTTP | actix-web 4.3 |
| Database | PostgreSQL |
| DB Driver | sqlx 0.8 (compile-time macros) |
| Config | YAML + env vars via `config` crate |
| Errors | thiserror + snafu |
| UUIDs | uuid v7 (time-ordered) |
| Math | bigdecimal (financial values) |

## Quick Start

```bash
# Start Postgres
docker run -d --name milions-pg \
  -e POSTGRES_DB=milions_db \
  -e POSTGRES_USER=postgres \
  -e POSTGRES_PASSWORD=postgres \
  -p 5432:5432 postgres:16

# Run migrations and start server
cargo run

# Or with environment overrides
APP_ENVIRONMENT=development cargo run
```

Server starts at `http://localhost:8000`.

## Workspace Structure

```
milions-sys/          Binary entrypoint (actix-web server)
├── src/              HTTP routes, startup, wiring
├── client/           Client bounded context
├── collaborator/     Collaborator bounded context
├── contact/          Contact bounded context
├── location/         Location bounded context
├── project/          Project bounded context
├── settings/         Configuration (YAML + env vars)
├── types/            Shared value objects (Phone, Email, Cpf, Cnpj, Cep)
├── viacep/           ViaCEP API integration
├── migrations/       SQL migrations
└── .sqlx/            Offline query cache
```

Each bounded context follows hexagonal architecture:

```
<crate>/src/
  domain/             Ports, models, errors (zero infra imports)
  application/        Use case implementations (services)
  adapters/           Infrastructure (postgres repositories)
```

## API

All routes prefixed with `/api`. JSON request/response bodies.

| Group | Endpoints |
|-------|-----------|
| Clients | CRUD (5 endpoints) |
| Projects | CRUD + status (5 endpoints) |
| Stages | create, update (2 endpoints) |
| Collaborators | CRUD + status (5 endpoints) |
| Allocations | create, list, update (3 endpoints) |
| Reports | cost, progress, history (3 endpoints) |
| Health | `GET /health` |

See [API Contracts](.ai/prompts/04-api-contracts.md) for full endpoint definitions.

## Configuration

Source priority:
1. `files/app_config/base.yaml` — defaults
2. `files/app_config/{environment}.yaml` — environment overrides
3. Environment variables (`APP_*`) — highest priority

| Variable | Override |
|----------|----------|
| `APP_ENVIRONMENT` | Environment name |
| `APP__APPLICATION__HOST` | Server host |
| `APP__APPLICATION__PORT` | Server port |
| `APP__DATABASE__HOST` | Database host |
| `APP__DATABASE__PORT` | Database port |
| `APP__DATABASE__USERNAME` | Database user |
| `APP__DATABASE__PASSWORD` | Database password |
| `APP__DATABASE__DATABASE_NAME` | Database name |
| `APP__DATABASE__REQUIRE_SSL` | Require SSL |

## Database

Schemas: `clients`, `collaborators`, `contacts`, `locations`.

Migrations run automatically at startup via `sqlx::migrate!()`.

## Development

```bash
cargo check          # Type check
cargo clippy         # Lint
cargo test           # Run tests
cargo fmt            # Format code
```

Use `SQLX_OFFLINE=true` to build without a running Postgres (uses `.sqlx/` cache).

## Architecture

See [Software Design Document](docs/sdd.md) for the full architecture, domain model, and API contracts.

## License

Proprietary.
