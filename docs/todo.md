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
- [ ] Implement HTTP route controllers — reports cost/progress/history (3 endpoints) — blocked: no use cases

## Medium Priority

- [ ] Implement viacep crate — HTTP client adapter for ViaCEP API
- [x] Implement Stage CRUD — domain ports, service, repository (only models existed)
- [x] Implement Allocation CRUD — domain ports, service, repository (only models existed)
- [ ] Implement Reports — cost, progress, history queries
- [ ] Add unit tests per bounded context

## Low Priority

- [ ] Add integration tests with `testcontainers`
- [ ] Add structured tracing/logging middleware
- [ ] Implement Keycloak JWT auth (currently standby per `05-security.md`)
- [ ] Implement pagination (`page`/`per_page`) for list endpoints

---

## Context

25 of 29 API endpoints are wired and compiling. Routes are registered under `/api` scope in `startup.rs`. Remaining 3 endpoints (reports) require domain use cases to be implemented first.
