# milions-sys — TODO

## High Priority (blocking)

- [x] Fix Dockerfile — `sqlx prepare` runs before `COPY . .` (build-breaking bug)
- [x] Generate `.sqlx/` offline query cache (`SQLX_OFFLINE=true` build fails without it)
- [x] Add bounded context crates as dependencies in root `Cargo.toml`
- [x] Create request/response DTOs (serde structs) for all endpoints
- [x] Implement error-to-HTTP response mapping (`ClientError`, `ProjectError`, etc → JSON + status code)
- [x] Implement HTTP route controllers — clients CRUD (6 endpoints)
- [x] Implement HTTP route controllers — projects CRUD + status (6 endpoints)
- [x] Implement HTTP route controllers — collaborators CRUD + status (6 endpoints)
- [x] Implement HTTP route controllers — contacts CRUD + phone CRUD (7 endpoints)
- [x] Implement HTTP route controllers — locations CRUD (5 endpoints)
- [x] Wire services into `startup.rs` and register all routes
- [x] Implement HTTP route controllers — stages create + update (2 endpoints)
- [x] Implement HTTP route controllers — allocations create/list/update (3 endpoints)
- [x] Implement HTTP route controllers — reports cost/progress/history (3 endpoints)

## Medium Priority

- [ ] Implement viacep crate — HTTP client adapter for ViaCEP API
- [x] Implement Stage CRUD — domain ports, service, repository (only models existed)
- [x] Implement Allocation CRUD — domain ports, service, repository (only models existed)
- [x] Implement Reports — cost, progress, history queries
- [x] Add unit tests per bounded context (location: 16, contact: 21, collaborator: 17, client: 16, project: 14)
- [ ] Add structured tracing/logging middleware
- [ ] Implement Keycloak JWT auth (currently standby per `05-security.md`)
- [ ] Implement pagination (`page`/`per_page`) for list endpoints

## Low Priority (code quality)

- [ ] Refactor `ClientService` to use `Arc<dyn Trait>` instead of importing concrete repository types from other crates — restores hexagonal architecture dependency direction
- [ ] Implement complete collaborator registration (contact, phones, address, join tables)
- [ ] Fix `collaborator_name` in history report (project crate can't access collaborator data)
- [ ] Implement Service Types & Project Services CRUD (tables exist, zero code)
- [ ] Remove `dead_code = "allow"` lints and clean up unused code

---

## Context

All 29 API endpoints are wired and registering via `configure()` functions.
Startup wiring uses `pub fn build(pool: PgPool)` per context.
86 unit tests pass across all crates.
All error messages are in English.
Composite repository traits replace bloated trait bounds.
All multi-field UPDATE queries use COALESCE for partial updates.
