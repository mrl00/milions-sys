# milions-sys

Backend system for managing construction and engineering projects in the Brazilian market.

## Tech Stack

- **Rust** (edition 2024)
- **tokio** async runtime
- **actix-web** 4.3 HTTP server
- **PostgreSQL** via `sqlx` 0.8 (compile-time query macros)
- **snafu** + **thiserror** for error handling
- **bigdecimal** for financial values
- **UUID v7** (time-ordered)

## Quick Start

```bash
# Start dependencies
docker-compose up -d postgres

# Run migrations and start server
cargo run

# Or with environment overrides
APP_ENVIRONMENT=development cargo run
```

Server starts at `http://{host}:{port}` (default: `localhost:8080`).

## Workspace Structure

```
milions-sys/          # Binary entrypoint (actix-web server)
├── src/              # HTTP routes, startup, wiring
├── client/           # Client bounded context
├── collaborator/     # Collaborator bounded context
├── contact/          # Contact bounded context
├── location/         # Location bounded context
├── project/          # Project bounded context
├── settings/         # Configuration (YAML + env vars)
├── types/            # Shared value objects (Phone, Email, Cpf, Cnpj, Cep)
├── viacep/           # ViaCEP API integration
└── migrations/       # SQL migrations
```

Each bounded context follows hexagonal architecture:

```
<crate>/src/
  domain/             # Ports, models, errors (zero infra imports)
  application/        # Use case implementations (services)
  adapters/           # Infrastructure (postgres repositories)
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

See `.ai/prompts/04-api-contracts.md` for full endpoint definitions.

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
# Check
cargo check

# Lint
cargo clippy

# Test
cargo test

# Format
cargo fmt
```

## Architecture

See `docs/sdd.md` for the full Software Design Document.

## License

Proprietary.
