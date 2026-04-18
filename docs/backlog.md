# milions-sys — Backlog

## Current State (2026-04-16)

**Architecture:** Rust (edition 2024) + actix-web 4.3 + sqlx 0.8 + PostgreSQL. Hexagonal architecture.
Previously a multi-crate workspace (one crate per bounded context), now consolidated into a single crate (`milions-sys`) with module-based organization under `src/`. Only `settings` and `viacep` remain as separate workspace crates.

**What compiles and passes:** 149 unit tests pass. Zero integration tests execute.

**What's wired at runtime:** Only the **location** context is active in `startup.rs`. Client, collaborator, contact, and project services are **commented out** — meaning 24 out of 29 API endpoints are unreachable despite their route/service/repo code existing and compiling.

### Code Completeness per Layer

| Layer | client | collaborator | contact | location | project |
|-------|--------|--------------|---------|----------|---------|
| DB models | ✅ | ✅ | ✅ | ✅ | ✅ |
| DTOs | ✅ | ✅ | ✅ | ✅ | ✅ |
| Repository ports | ✅ | ✅ | ✅ (+ phone) | ✅ | ✅ |
| Use case ports | ✅ | ✅ | ✅ | ✅ | ✅ |
| Error types | ✅ | ✅ | ✅ | ✅ | ✅ |
| Service impl | ✅ | ✅ | ✅ | ✅ | ✅ |
| PG repository | ✅ | ✅ | ✅ (+ phone) | ✅ | ✅ |
| HTTP routes | ✅ | ✅ | ✅ | ✅ | ✅ |
| **Wired in startup** | ❌ | ❌ | ❌ | ✅ | ❌ |

### Test Breakdown (149 passing)

| Module | Count |
|--------|-------|
| application/contact_service | 21 |
| adapters/driving/project_routes | 18 |
| application/collaborator_service | 17 |
| application/client_service | 16 |
| application/project_service | 14 |
| application/location_service | 14 |
| adapters/driving/contact_routes | 13 |
| adapters/driving/client_routes | 12 |
| adapters/driving/collaborator_routes | 11 |
| adapters/driving/location_routes | 9 |
| domain/value_objects/text | 3 |
| routes/health_check | 1 |

### Value Objects (all implemented)

`Alphabetic`, `Alphanumeric`, `Cep`, `Cnpj`, `Cpf`, `Doc`, `Email`, `Numeric`, `Phone`, `Text` — in `src/domain/value_objects/`.

---

## High Priority — Wire remaining contexts

All code exists but is dead at runtime. Each context needs a `build(pool)` function in `application/mod.rs` and must be uncommented/registered in `startup.rs`.

- [ ] Wire **client** context into `startup.rs`
  - Create `pg_client_serv_build(pool)` in `application/mod.rs` (like `pg_location_serv_build`)
  - Uncomment `client_service` in `startup.rs`, register `client_routes::configure`
  - Verify the 6 client endpoints respond (POST/GET/PUT/DELETE /clients, GET /clients/{uuid}, PUT /clients/{uuid}/status)
- [ ] Wire **collaborator** context into `startup.rs`
  - Create `pg_collaborator_serv_build(pool)` in `application/mod.rs`
  - Register `collaborator_routes::configure`
  - Verify 6 endpoints
- [ ] Wire **contact** context into `startup.rs`
  - Create `pg_contact_serv_build(pool)` — note: `ContactService` depends on both `ContactRepository` and `PhoneRepository`
  - Register `contact_routes::configure`
  - Verify 7 endpoints (contact CRUD + phone CRUD)
- [ ] Wire **project** context into `startup.rs`
  - Create `pg_project_serv_build(pool)` in `application/mod.rs`
  - Register `project_routes::configure`
  - Verify 14 endpoints (project CRUD + status + stages + allocations + reports)
- [ ] Smoke-test all 29 endpoints with hurl or curl against a running server + PostgreSQL

## Medium Priority — Missing features

- [ ] Add structured tracing/logging middleware
  - Add `tracing`, `tracing-subscriber`, `tracing-actix-web` dependencies
  - Configure subscriber in `main.rs` with JSON output
  - Replace `println!` with `tracing::info!`
  - No PII in logs
- [ ] Implement pagination (`page`/`per_page`) for list endpoints
  - Add `PaginationParams` query struct (page default 1, per_page default 20, max 100)
  - Update all `find_all` repository methods to accept LIMIT/OFFSET
  - Update all `ListXxxUseCase` traits and impls
  - Update route handlers to extract query params
- [ ] Add integration tests with `sqlx::test` + real PostgreSQL
  - Integration test files exist in `src/tests/` (client: 15, location: 6, contact: 13, project: 22, collaborator: 17)
  - Currently **not executed** — need `#[cfg(test)]` integration test harness or separate test binary
  - Previous blocker: `ColumnDecode` on timestamp columns in Docker PostgreSQL (locale/encoding issue)
- [ ] Implement Keycloak JWT auth
  - `keycloak` crate already in workspace deps
  - See `docs/adr/002-auth-strategy.md` for strategy
  - Middleware in actix-web, exclude `/health` from auth
  - Signature + issuer verification only (no roles initially)

## Low Priority — Code quality & completeness

- [ ] Update `docs/sdd.md` and `README.md` to reflect single-crate architecture
  - Both still reference old multi-crate workspace structure (client/, collaborator/, contact/, location/, project/, types/ as separate crates)
  - Actual structure is `src/{domain,application,adapters,routes}/` with modules
  - Only settings and viacep remain as workspace crates
- [ ] Implement Service Types & Project Services CRUD
  - Tables exist in migration `20260222180342_create_project_schema.sql` (`tb_service_type`, `tb_project_service`)
  - Zero Rust code: no models, ports, service, repo, or routes
- [ ] Implement complete collaborator registration (contact, phones, address, join tables)
  - Join table models exist (`collaborator_contact_row.rs`, `collaborator_location_row.rs`) but no orchestration in service
- [ ] Fix `collaborator_name` in history report
  - Project service can't access collaborator data (cross-context boundary issue)
  - Options: ACL adapter, denormalized column, or SQL JOIN in report query
- [ ] Remove `#[allow(dead_code)]` lint in `collaborator_use_cases.rs`
- [ ] Fix compiler warning: unused `NotFound` variant in `location_service::tests::FindByIdResult`
- [ ] Extract duplicated `error_to_response` functions from route modules into shared middleware
  - Each route module (client, collaborator, contact, location, project) has its own `error_to_response` fn
  - Consider a trait-based approach or generic actix error handler
- [ ] Add more hurl integration test files
  - Only location has hurl tests (`hurl/location.hurl`, `hurl/location.errors.hurl`)
  - Add for client, collaborator, contact, project

---

## Context

Architecture: single Rust crate with hexagonal module layout (`domain/`, `application/`, `adapters/`).
Only `settings` (config) and `viacep` (external API client) are separate workspace crates.
149 unit tests pass (service logic + route existence + error mapping).
All 5 route modules compile and have `configure()` functions ready for `web::scope("/api")`.
All multi-field UPDATE queries use COALESCE for partial updates.
Value object validation (CPF, CNPJ, CEP, Email, Phone, Doc) is comprehensive.
Accent removal (`unicode-normalization`) applied to text inputs.
ViaCEP HTTP client adapter has `wiremock` contract tests (3 tests in separate crate).
CI pipeline (`.github/workflows/ci.yml`) runs fmt + clippy + migrate + test on PR to `dev`.
Config: YAML files in `settings/app_config/` + env var overrides (`APP_*`).
6 SQL migrations applied at startup via `sqlx::migrate!()`.
