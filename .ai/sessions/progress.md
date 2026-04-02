# Session Progress: HTTP Routes + Wiring

**Task**: Implement all HTTP routes and wire into startup.rs
**Started**: 2026-04-02

## Plan

- [x] step 1 — Error-to-HTTP mapping — collaborator
- [x] step 2 — Error-to-HTTP mapping — project
- [x] step 3 — Error-to-HTTP mapping — contact
- [x] step 4 — Error-to-HTTP mapping — location
- [x] step 5 — Routes — collaborator CRUD + status (5 endpoints)
- [x] step 6 — Routes — project CRUD + status (5 endpoints)
- [x] step 7 — Routes — contact CRUD + phone CRUD (7 endpoints)
- [x] step 8 — Routes — location CRUD (5 endpoints)
- [x] step 9 — Wire all services into startup.rs
- [x] step 10 — Final workspace compilation check

## Summary

### Endpoints Wired (22 total)

**Client** (6): POST/GET /clients, GET/PUT/DELETE /clients/{uuid}, PUT /clients/{uuid}/status
**Collaborator** (6): POST/GET /collaborators, GET/PUT/DELETE /collaborators/{uuid}, PUT /collaborators/{uuid}/status
**Project** (6): POST/GET /projects, GET/PUT/DELETE /projects/{uuid}, PUT /projects/{uuid}/status
**Contact** (7): POST/GET /contacts, GET/PUT /contacts/{uuid}, POST/GET /contacts/{uuid}/phones, PUT/DELETE /phones/{uuid}
**Location** (5): POST/GET /locations, GET/PUT/DELETE /locations/{uuid}
**Health** (1): GET /health_check

### Not yet implemented (requires domain work)
- Stages (2 endpoints) — no use cases defined
- Allocations (3 endpoints) — no use cases defined
- Reports (3 endpoints) — no use cases defined

### Changes made
- Fixed client routes.rs trait disambiguation (10 compile errors)
- Created collaborator routes.rs + dto StatusRequest
- Created project routes.rs (fixed dto: address_id instead of address)
- Created contact routes.rs
- Created location routes.rs
- Added actix-web + serde_json to collaborator, contact, location, project Cargo.toml
- Wired all services and routes into startup.rs under /api scope
