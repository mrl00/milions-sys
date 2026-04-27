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
| Validation | garde (declarative) |
| Config | YAML + env vars via `config` crate |
| Errors | thiserror |
| UUIDs | uuid v7 (time-ordered) |
| Math | bigdecimal (financial values) |
| External APIs | ViaCEP (address lookup) |

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

## Architecture

The project follows **hexagonal architecture** (ports & adapters) with a flat module layout:

```
src/
├── domain/
│   ├── models/db/          Row structs (sqlx::FromRow)
│   ├── value_objects/      Doc, Phone, Email, Cep, etc.
│   ├── errors/             Domain error enums (thiserror)
│   └── ports/
│       ├── repositories/   Repository trait definitions
│       └── use_cases/      Use case trait definitions + input models
├── application/
│   ├── client_service.rs   ClientService<R, L, C> (generic)
│   ├── project_service.rs  ProjectService<R> (generic)
│   ├── collaborator_service.rs
│   ├── contact_service.rs
│   ├── location_service.rs
│   └── mod.rs              Service builders (pg_*_serv_build)
├── adapters/
│   ├── driven/             PostgreSQL repositories
│   └── driving/
│       ├── *_routes.rs     HTTP handlers (actix-web)
│       ├── models/dtos/    Request/Response DTOs (garde + serde)
│       ├── errors/         From<DomainError> for HttpResponse
│       └── utils.rs        ValidatedJson extractor
├── startup.rs              App wiring (services → routes)
└── main.rs                 Server entrypoint
```

External crates:

```
viacep/         ViaCEP API client (address autocomplete)
settings/       Configuration (YAML + env vars)
```

### Service Generics

Services are generic over their dependencies for testability:

```rust
// Production: PgClientService = ClientService<PgClientRepository, PgLocationService, PgContactService>
pub struct ClientService<R, L, C> { repo: R, location_service: L, contact_service: C }
pub type PgClientService = ClientService<PgClientRepository, PgLocationService, PgContactService>;

// Tests: ClientService<MockRepo, NoOp, NoOp>
let service = ClientService::new(mock_repo, NoOp, NoOp);
```

## API

All routes prefixed with `/api`. JSON request/response bodies. Input validation via `garde` + `ValidatedJson`.

### Clients (14 endpoints)

| Method | Route | Description |
|--------|-------|-------------|
| `POST` | `/clients` | Register client (with optional contact + address) |
| `GET` | `/clients` | List all clients |
| `GET` | `/clients/{uuid}` | Get client by ID |
| `PUT` | `/clients/{uuid}` | Update client name/doc |
| `DELETE` | `/clients/{uuid}` | Delete client |
| `PUT` | `/clients/{uuid}/status` | Activate / Deactivate |
| `PATCH` | `/clients/{uuid}/contact/email` | Update email |
| `POST` | `/clients/{uuid}/contact/phones` | Add phones |
| `PUT` | `/clients/{uuid}/contact/phones/{phone}` | Update phone |
| `PUT` | `/clients/{uuid}/address` | Update address |
| `POST` | `/clients/{uuid}/projects` | Associate project |
| `GET` | `/clients/{uuid}/projects` | List client projects |
| `DELETE` | `/clients/{uuid}/projects/{project_uuid}` | Dissociate project |

### Projects (13 endpoints)

| Method | Route | Description |
|--------|-------|-------------|
| `POST` | `/projects` | Create project |
| `GET` | `/projects` | List projects |
| `GET` | `/projects/{uuid}` | Get project |
| `PUT` | `/projects/{uuid}` | Update project |
| `DELETE` | `/projects/{uuid}` | Delete project |
| `PUT` | `/projects/{uuid}/status` | Transition status (start/pause/complete/cancel) |
| `POST` | `/projects/{id}/stages` | Create stage |
| `PUT` | `/projects/{id}/stages/{stage_id}` | Update stage |
| `POST` | `/projects/{id}/allocations` | Create allocation |
| `GET` | `/projects/{id}/allocations` | List allocations |
| `PUT` | `/projects/{id}/allocations/{id}` | Update allocation |
| `GET` | `/reports/projects/{id}/cost` | Cost report |
| `GET` | `/reports/projects/{id}/progress` | Progress report |

### Collaborators (5 endpoints)

| Method | Route | Description |
|--------|-------|-------------|
| `POST` | `/collaborators` | Register collaborator |
| `GET` | `/collaborators` | List collaborators |
| `GET` | `/collaborators/{uuid}` | Get collaborator |
| `PUT` | `/collaborators/{uuid}` | Update collaborator |
| `DELETE` | `/collaborators/{uuid}` | Delete collaborator |
| `PUT` | `/collaborators/{uuid}/status` | Activate / Deactivate |

### Contacts (7 endpoints)

| Method | Route | Description |
|--------|-------|-------------|
| `POST` | `/contacts` | Register contact |
| `GET` | `/contacts` | List contacts |
| `GET` | `/contacts/{uuid}` | Get contact |
| `POST` | `/contacts/{uuid}/phones` | Add phone |
| `GET` | `/contacts/{uuid}/phones` | List phones |
| `PUT` | `/phones/{uuid}` | Update phone |
| `DELETE` | `/phones/{uuid}` | Remove phone |

### Locations (4 endpoints)

| Method | Route | Description |
|--------|-------|-------------|
| `POST` | `/locations` | Create location |
| `GET` | `/locations` | List locations |
| `GET` | `/locations/{uuid}` | Get location |
| `PUT` | `/locations/{uuid}` | Update location |

### Health

| Method | Route | Description |
|--------|-------|-------------|
| `GET` | `/health` | Health check |

## Database

Schemas: `clients`, `collaborators`, `contacts`, `locations`, `project`.

Tables:
- `clients`: `tb_client`, `tb_client_contact`, `tb_client_address`, `tb_client_project`
- `project`: `tb_project`, `tb_project_stage`, `tb_project_allocation`
- `collaborators`: `tb_collaborator`
- `contacts`: `tb_contact`, `tb_phone`
- `locations`: `tb_location`

Migrations run automatically at startup via `sqlx::migrate!()`.

## Configuration

Source priority:
1. `settings/base.yaml` — defaults
2. `settings/{environment}.yaml` — environment overrides
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

## Development

```bash
cargo check          # Type check
cargo clippy         # Lint
cargo test           # Run tests
cargo fmt            # Format code

# E2E tests (requires running server)
hurl --test hurl/collaborators.hurl
hurl --test hurl/projects.hurl
```

Use `SQLX_OFFLINE=true` to build without a running Postgres (uses `.sqlx/` cache).

### Backlogs

- [`docs/backlog-client.md`](docs/backlog-client.md) — Client context tasks
- [`docs/backlog-projects.md`](docs/backlog-projects.md) — Project context tasks

## License

Proprietary.
