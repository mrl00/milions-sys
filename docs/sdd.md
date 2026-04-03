# Software Design Document

**Project:** milions-sys
**Version:** 0.1.0
**Last updated:** 2026-04-02

---

## 1. Introduction

### 1.1 Purpose

milions-sys is a backend system for managing construction and engineering projects in the Brazilian market. It handles the full lifecycle of clients, collaborators, contacts, locations, and projects — including financial tracking, workforce allocation, and progress reporting.

### 1.2 Scope

The system provides:

- Client registration and management (individuals and companies)
- Collaborator management with skill levels
- Contact and phone number handling
- Location management with Brazilian postal data (CEP/ViaCEP)
- Project management with stages, services, and daily workforce allocation
- Cost and progress reporting

Out of scope:

- Authentication/authorization (standby — see §9)
- Frontend — API-only backend
- Payment processing
- Document/file storage

### 1.3 Definitions

| Term | Definition |
|------|------------|
| Client | Company or individual who contracts construction services |
| Collaborator | Worker/employee assigned to construction projects |
| Contact | Communication record (email + phones) linked to a client or collaborator |
| Location | Physical address with Brazilian postal data |
| Project | Construction project owned by a client, with stages, services, and workforce |
| Stage | Ordered phase within a project (foundation, structure, finishing) |
| Service Type | Catalog of service categories (m², linear meter, hour, unit) |
| Daily Allocation | Collaborator assigned to a project on a specific work date |
| CPF | Cadastro de Pessoa Física (individual tax ID, 11 digits) |
| CNPJ | Cadastro Nacional de Pessoa Jurídica (company tax ID, 14 digits) |
| CEP | Código de Endereçamento Postal (postal code, 8 digits) |

---

## 2. System Overview

### 2.1 High-Level Architecture

```
┌─────────────┐
│   Client     │
│  (Browser)   │
└──────┬───────┘
       │ HTTPS
       ▼
┌──────────────┐     ┌─────────────┐
│  actix-web   │────▶│  Keycloak   │  (standby)
│  HTTP Server │     │  JWT JWKS   │
└──────┬───────┘     └─────────────┘
       │
       ▼
┌──────────────┐
│   Services   │  (application layer)
│  Use Cases   │
└──────┬───────┘
       │
       ▼
┌──────────────┐
│  Adapters    │  (postgres repositories)
│  sqlx        │
└──────┬───────┘
       │
       ▼
┌──────────────┐
│  PostgreSQL  │
└──────────────┘
```

### 2.2 Technology Stack

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
| Auth | Keycloak JWT (standby) |

---

## 3. Architecture

### 3.1 Hexagonal Architecture

Every bounded context follows a hexagonal (ports and adapters) layout:

```
<crate>/src/
  domain/
    errors/
      mod.rs                      context-specific error enum
    models/
      db/
        mod.rs                    module registry
        <entity>_row.rs           sqlx FromRow structs
    ports/
      mod.rs                      module registry
      <entity>_repository.rs      repository port traits
      <entity>_use_cases.rs       use case traits + input structs
  application/
    <entity>_service.rs           implements use case traits
  adapters/
    driven/
      postgres/
        mod.rs
        pg_<entity>_repository.rs implements port traits using sqlx
    driving/
        mod.rs                    (ACL adapters for cross-context calls)
```

### 3.2 Layer Responsibilities

| Layer | Responsibility | Dependencies |
|-------|---------------|--------------|
| `domain` | Contracts (ports, use cases), models, errors | None (zero infra imports) |
| `application` | Orchestration, validation, business rules | Domain ports only |
| `adapters` | Infrastructure implementation (SQL, HTTP, external APIs) | Domain ports only |
| `src/` (binary) | Startup, wiring, HTTP server | All crates |

### 3.3 Workspace Crates

| Crate | Role |
|-------|------|
| `milions-sys` | Binary entrypoint (actix-web server) |
| `settings` | Configuration from YAML + env vars |
| `types` | Shared value objects (Phone, Email, Cpf, Cnpj, Doc, Cep) |
| `viacep` | ViaCEP external API integration |
| `client` | Client bounded context |
| `collaborator` | Collaborator bounded context |
| `contact` | Contact bounded context |
| `location` | Location bounded context |
| `project` | Project bounded context |

### 3.4 Cross-Context Dependencies

```
client ──→ location    (ClientAddress → LocationRow)
client ──→ viacep      (ViaCepPort for CEP lookup)
```

All other contexts are independent. No context imports another context's domain layer.

---

## 4. Domain Model

### 4.1 Entity Relationship Diagram

```
┌──────────┐       ┌───────────────┐       ┌──────────┐
│  Client  │1────N │ClientContact  │N────1  │ Contact  │
│          │       └───────────────┘       │          │
│          │1────N ┌───────────────┐       │          │1────N ┌───────┐
│          │       │ClientAddress  │N────1  │          │       │ Phone │
└────┬─────┘       └───────────────┘       └──────────┘       └───────┘
     │                      │
     │1              ┌──────┴─────┐
     │N              ▼            ▼
┌────┴─────┐   ┌──────────┐  ┌──────────┐
│ Project  │   │ Location │  │ Location │
│          │   └──────────┘  └──────────┘
│          │1────N ┌──────────────┐
│          │       │ProjectStage  │
│          │1────N └──────────────┘
│          │       ┌───────────────────┐
│          │       │ProjectService     │──→ ServiceType
│          │1────N └───────────────────┘
│          │       ┌──────────────────────────┐
│          │       │ProjectDailyAllocation    │──→ Collaborator
└──────────┘       └──────────────────────────┘

┌──────────────┐       ┌─────────────────────┐       ┌──────────┐
│ Collaborator │1────N │CollaboratorContact  │N────1  │ Contact  │
│              │       └─────────────────────┘       └──────────┘
│              │1────N ┌─────────────────────┐
│              │       │CollaboratorAddress  │N────1 ──→ Location
└──────────────┘       └─────────────────────┘
```

### 4.2 Entities

#### Client

| Field | Type | Notes |
|-------|------|-------|
| id | UUID v7 | PK |
| name | String | |
| status | ClientStatus | active, inactive |
| document | String | CPF or CNPJ |
| created_at | NaiveDateTime | |
| updated_at | NaiveDateTime | |

#### Collaborator

| Field | Type | Notes |
|-------|------|-------|
| id | UUID v7 | PK |
| name | String | |
| cpf | String | individual tax ID |
| level | CollaboratorLevel | P0, P1, P2, P3 |
| status | CollaboratorStatus | active, inactive |
| created_at | NaiveDateTime | |
| updated_at | NaiveDateTime | |

#### Contact

| Field | Type | Notes |
|-------|------|-------|
| id | UUID v7 | PK |
| email | Option<String> | |
| created_at | NaiveDateTime | |
| updated_at | NaiveDateTime | |

#### Phone

| Field | Type | Notes |
|-------|------|-------|
| id | UUID v7 | PK |
| phone | String | |
| contact_id | UUID | FK → Contact |
| created_at | NaiveDateTime | |
| updated_at | NaiveDateTime | |

#### Location

| Field | Type | Notes |
|-------|------|-------|
| id | UUID v7 | PK |
| public_space | String | ViaCEP: logradouro |
| address_complement | Option<String> | ViaCEP: complemento |
| unit | String | ViaCEP: unidade |
| neighborhood | String | ViaCEP: bairro |
| locality | String | ViaCEP: localidade |
| region | String | ViaCEP: uf |
| ibge | Option<String> | IBGE municipality code |
| gia | Option<String> | São Paulo GIA code |
| ddd | String | area code |
| siafi | Option<String> | federal treasury code |
| street | String | |
| number | String | |
| city | String | |
| state | String | |
| zipcode | String | |
| hash | i64 | deduplication hash |
| created_at | NaiveDateTime | |
| updated_at | NaiveDateTime | |

#### Project

| Field | Type | Notes |
|-------|------|-------|
| id | UUID v7 | PK |
| name | String | |
| description | Option<String> | |
| status | ProjectStatus | planning, in_progress, paused, completed, cancelled |
| start_date | Option<NaiveDate> | |
| estimated_end_date | Option<NaiveDate> | |
| actual_end_date | Option<NaiveDate> | |
| total_area_m2 | Option<BigDecimal> | |
| estimated_cost | Option<BigDecimal> | |
| actual_cost | Option<BigDecimal> | |
| notes | Option<String> | |
| active | bool | |
| client_id | UUID | FK → Client |
| address_id | UUID | FK → Location |
| created_at | NaiveDateTime | |
| updated_at | NaiveDateTime | |

#### ProjectStage

| Field | Type | Notes |
|-------|------|-------|
| id | UUID v7 | PK |
| project_id | UUID | FK → Project |
| name | String | |
| description | Option<String> | |
| order | i32 | sort order |
| status | ProjectStageStatus | pending, in_progress, completed, skipped |
| start_date | Option<NaiveDate> | |
| end_date | Option<NaiveDate> | |
| created_at | NaiveDateTime | |
| updated_at | NaiveDateTime | |

#### ServiceType (catalog)

| Field | Type | Notes |
|-------|------|-------|
| id | UUID v7 | PK |
| name | String | |
| description | Option<String> | |
| unit | ProjectServiceUnit | m2, m_linear, unit, hour |
| default_unit_price | Option<BigDecimal> | |
| active | bool | |
| created_at | NaiveDateTime | |
| updated_at | NaiveDateTime | |

#### ProjectService

| Field | Type | Notes |
|-------|------|-------|
| id | UUID v7 | PK |
| project_id | UUID | FK → Project |
| project_stage_id | Option<UUID> | FK → ProjectStage |
| service_type_id | UUID | FK → ServiceType |
| description | Option<String> | |
| quantity | BigDecimal | |
| unit_price | BigDecimal | |
| total_price | Option<BigDecimal> | |
| status | ProjectServiceStatus | pending, in_progress, completed |
| created_at | NaiveDateTime | |
| updated_at | NaiveDateTime | |

#### ProjectDailyAllocation

| Field | Type | Notes |
|-------|------|-------|
| id | UUID v7 | PK |
| project_id | UUID | FK → Project |
| collaborator_id | UUID | FK → Collaborator |
| work_date | NaiveDate | |
| hours_worked | Option<BigDecimal> | |
| hourly_rate_snapshot | Option<BigDecimal> | |
| notes | Option<String> | |
| present | bool | |
| created_at | NaiveDateTime | |
| updated_at | NaiveDateTime | |

### 4.3 State Machines

#### Project Status

```
                  ┌──────────┐
                  │ planning │
                  └────┬─────┘
                       │ start
                       ▼
                 ┌───────────┐
            ┌────│in_progress│────┐
            │    └───────────┘    │
            │ pause               │
            ▼                     │
       ┌─────────┐               │
       │ paused  │               │
       └────┬────┘               │
            │ resume (→ in_progress)
            │                     │ complete
            ▼                     ▼
                     ┌───────────┐    ┌───────────┐
                     │ completed │    │ cancelled │
                     └───────────┘    └───────────┘
```

#### ProjectStage Status

```
pending → in_progress → completed
                   ╲
                    └→ skipped
```

#### Collaborator/Client Status

```
active ↔ inactive
```

---

## 5. API

See `.ai/prompts/04-api-contracts.md` for full endpoint definitions.

### 5.1 Summary

| Group | Endpoints |
|-------|-----------|
| Clients | 5 (CRUD) |
| Projects | 5 (CRUD + status) |
| Stages | 2 (create, update) |
| Collaborators | 5 (CRUD + status) |
| Allocations | 3 (create, list, update) |
| Reports | 3 (cost, progress, history) |
| Health | 1 (GET /health) |

### 5.2 Error Response Format

```json
{
  "error": "not_found",
  "message": "client not found: 018e3a7b-..."
}
```

### 5.3 Pagination

List endpoints accept:

| Param | Default | Max |
|-------|---------|-----|
| `page` | 1 | — |
| `per_page` | 20 | 100 |

---

## 6. Database

### 6.1 Schemas

| Schema | Tables |
|--------|--------|
| `clients` | tb_client, tb_client_contact, tb_client_address, tb_project, tb_project_stage, tb_service_type, tb_project_service, tb_project_daily_allocation |
| `collaborators` | tb_collaborator, tb_collaborator_contact, tb_collaborator_address |
| `contacts` | tb_contact, tb_phone |
| `locations` | tb_location |

### 6.2 Naming Conventions

| Prefix | Meaning |
|--------|---------|
| `tb_` | table |
| `pk_` | primary key (UUID) |
| `idx_` | auto-increment index |
| `fk_` | foreign key |
| `ts_` | timestamp |
| `tx_` | text |
| `nr_` | number |
| `bl_` | boolean |
| `dt_` | date |

### 6.3 Migrations

Applied at startup via `sqlx::migrate!()`. Located in `migrations/`:

| File | Schema |
|------|--------|
| `20260222180028_create_location_schema.sql` | locations |
| `20260222180340_create_contacts_schema.sql` | contacts |
| `20260222180341_create_collaborators_schema.sql` | collaborators |
| `20260222180342_create_clients_schema.sql` | clients |

---

## 7. Configuration

### 7.1 Source Priority

1. `files/app_config/base.yaml` — defaults
2. `files/app_config/{environment}.yaml` — environment overrides
3. Environment variables (`APP_*`) — highest priority

### 7.2 Settings Structure

```rust
Settings {
    application: ApplicationSettings {
        host: String,
        port: u16,
    },
    database: DatabaseSettings {
        host: String,
        port: u16,
        username: String,
        password: SecretString,
        database_name: String,
        require_ssl: bool,
    },
}
```

### 7.3 Environment Variables

| Variable | Override |
|----------|----------|
| `APP_ENVIRONMENT` | `application.environment` |
| `APP__APPLICATION__HOST` | `application.host` |
| `APP__APPLICATION__PORT` | `application.port` |
| `APP__DATABASE__HOST` | `database.host` |
| `APP__DATABASE__PORT` | `database.port` |
| `APP__DATABASE__USERNAME` | `database.username` |
| `APP__DATABASE__PASSWORD` | `database.password` |
| `APP__DATABASE__DATABASE_NAME` | `database.database_name` |
| `APP__DATABASE__REQUIRE_SSL` | `database.require_ssl` |

---

## 8. Non-Functional Requirements

See `.ai/prompts/06-nfr.md` for full details.

| Attribute | Target |
|-----------|--------|
| P95 latency (read) | < 100ms |
| P95 latency (list) | < 300ms |
| Throughput | 500 req/s |
| Uptime | 99.5% |
| Pool size | 10 connections |
| Graceful shutdown | 30s drain |
| Logging | structured via `tracing`, no PII |
| Errors | en-US messages, generic to client |

---

## 9. Security (Standby)

See `.ai/prompts/05-security.md` for full details. Not implemented yet.

- JWT validation via Keycloak JWKS
- Signature + issuer verification only (no roles)
- Middleware in new `security` crate
- Health check excluded from auth

---

## 10. Deployment

### 10.1 Container

- Multi-stage Dockerfile: Rust build → Debian slim runtime
- All configuration via environment variables
- Migrations run on container start

### 10.2 Environments

| Environment | Config | Database |
|-------------|--------|----------|
| Development | `base.yaml` + `development.yaml` | Local Postgres |
| Production | `base.yaml` + `production.yaml` | Managed Postgres |

---

## 11. Design Decisions

| # | Decision | Rationale |
|---|----------|-----------|
| 1 | Hexagonal architecture | Clean separation of concerns, testable domain logic |
| 2 | One trait per method (ISP) | Flexible composition, easy mocking |
| 3 | Super-traits (`FindAnd*`) | Convenient grouping without forcing interface bloat |
| 3b | Composite repository traits | Replace bloated trait bounds across 50+ impl blocks, reducing ~630 lines |
| 4 | `InfraError` duplicated per context | Avoids shared error crate coupling |
| 5 | UUID v7 | Time-ordered, good B-tree index performance |
| 6 | `sqlx::query_as!` | Compile-time SQL validation |
| 7 | Generic executor | Enables transactions without separate methods |
| 8 | `secrecy::SecretString` | Redacts credentials in Debug output |
| 9 | `BigDecimal` for money | Exact decimal arithmetic, no floating point |
| 10 | COALESCE for partial updates | Avoids overwriting fields with NULL |
| 11 | `pub fn build(pool)` per context | Encapsulates service wiring, simplifies startup.rs |
| 12 | Executor-based repo methods | Enables cross-entity transactions without tight coupling |
