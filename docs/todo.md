# milions-sys — TODO

## High Priority (blocking)

- [x] Fix Dockerfile — `sqlx prepare` runs before `COPY . .` (build-breaking bug)
- [x] Generate `.sqlx/` offline query cache (`SQLX_OFFLINE=true` build fails without it)
- [ ] Add bounded context crates as dependencies in root `Cargo.toml`
- [ ] Implement HTTP route controllers — clients CRUD (5 endpoints)
- [ ] Implement HTTP route controllers — projects CRUD + status (5 endpoints)
- [ ] Implement HTTP route controllers — stages create + update (2 endpoints)
- [ ] Implement HTTP route controllers — collaborators CRUD + status (5 endpoints)
- [ ] Implement HTTP route controllers — allocations create/list/update (3 endpoints)
- [ ] Implement HTTP route controllers — reports cost/progress/history (3 endpoints)
- [ ] Create request/response DTOs (serde structs) for all endpoints
- [ ] Implement error-to-HTTP response mapping (`ClientError`, `ProjectError`, etc → JSON + status code)
- [ ] Wire services into `startup.rs` and register all routes

## Medium Priority

- [ ] Implement viacep crate — HTTP client adapter for ViaCEP API
- [ ] Implement Stage CRUD — domain ports, service, repository (only models exist)
- [ ] Implement Allocation CRUD — domain ports, service, repository (only models exist)
- [ ] Implement Reports — cost, progress, history queries
- [ ] Add unit tests per bounded context

## Low Priority

- [ ] Add integration tests with `testcontainers`
- [ ] Add structured tracing/logging middleware
- [ ] Implement Keycloak JWT auth (currently standby per `05-security.md`)
- [ ] Implement pagination (`page`/`per_page`) for list endpoints

---

## Context

Domain and application layers are complete for 5/6 bounded contexts (client, collaborator, contact, location, project). The critical gap is the HTTP adapter layer — **0 of 23 API endpoints are wired**. The viacep crate is a skeleton (port + model only, no service/adapter).
