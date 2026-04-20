# Projects Domain — Full Analysis

## Table of Contents

- [1. DTO Analysis](#1-dto-analysis)
- [2. Service Logic Analysis](#2-service-logic-analysis)
- [3. Hurl Test File](#3-hurl-test-file)

---

## 1. DTO Analysis

### 🔴 Critical: No `garde` validation on any project DTO

Every other domain in the project uses `#[derive(Validate)]` + `ValidatedJson<T>` extractor. The project DTOs have **none of this**.

| File | Issue |
|---|---|
| [project_dto.rs](file:///home/qqx/Programming/projects/milions/milions-backend/src/adapters/driving/models/dtos/project_dto.rs) | No `use garde::Validate`, no `#[derive(Validate)]`, no `#[garde(...)]` annotations on any request struct |
| [project_routes.rs](file:///home/qqx/Programming/projects/milions/milions-backend/src/adapters/driving/project_routes.rs) | Uses `web::Json<T>` instead of `ValidatedJson<T>` for all handlers |

### Missing annotations per DTO vs. DB schema

#### `CreateProjectRequest`

| Field | DB Constraint | Missing `garde` annotation |
|---|---|---|
| `name` | `VARCHAR(255) NOT NULL` | `#[garde(length(min = 1, max = 255))]` |
| `description` | `TEXT` (nullable) | `#[garde(skip)]` |
| `address_id` | `UUID NOT NULL` (FK) | `#[garde(skip)]` |
| `start_date` | `DATE` (nullable) | `#[garde(skip)]` |
| `estimated_end_date` | `DATE` (nullable) | `#[garde(skip)]` |
| `total_area_m2` | `NUMERIC(10,2)` (nullable) | `#[garde(skip)]` — no parse validation either |
| `estimated_cost` | `NUMERIC(14,2)` (nullable) | `#[garde(skip)]` — no parse validation either |
| `notes` | `TEXT` (nullable) | `#[garde(skip)]` |

#### `UpdateProjectRequest`

| Field | DB Constraint | Missing `garde` annotation |
|---|---|---|
| `name` | `VARCHAR(255)` | `#[garde(inner(length(min = 1, max = 255)))]` |
| All other fields | various | `#[garde(skip)]` |

#### `ProjectStatusRequest`

| Field | DB Constraint | Missing annotation |
|---|---|---|
| `status` | `CHECK ('in_progress','paused','completed','cancelled')` | `#[garde(pattern(...))]` — currently validated manually in the route handler, but returns plain text body instead of the JSON `{"error":"validation_error","message":"..."}` contract |

#### `CreateStageRequest`

| Field | DB Constraint | Missing annotation |
|---|---|---|
| `name` | `VARCHAR(255) NOT NULL` | `#[garde(length(min = 1, max = 255))]` |
| `order` | `INTEGER NOT NULL DEFAULT 1` | `#[garde(range(min = 1))]` |
| `description` | `TEXT` (nullable) | `#[garde(skip)]` |

#### `UpdateStageRequest`

| Field | DB Constraint | Missing annotation |
|---|---|---|
| `name` | `VARCHAR(255)` | `#[garde(inner(length(min = 1, max = 255)))]` |
| `status` | `CHECK ('pending','in_progress','completed','skipped')` | `#[garde(inner(pattern(...)))]` |
| `order` | `INTEGER` | `#[garde(skip)]` or range |

#### `CreateAllocationRequest`

| Field | DB Constraint | Missing annotation |
|---|---|---|
| `collaborator_id` | `UUID NOT NULL` (FK) | `#[garde(skip)]` |
| `work_date` | `DATE NOT NULL` | `#[garde(skip)]` |
| `hours_worked` | `NUMERIC(4,2)` (nullable) | `#[garde(skip)]` — no parse validation |
| `hourly_rate_snapshot` | `NUMERIC(10,2)` (nullable) | `#[garde(skip)]` — no parse validation |
| `present` | `BOOLEAN NOT NULL DEFAULT TRUE` | `#[garde(skip)]` |
| `notes` | `TEXT` (nullable) | `#[garde(skip)]` |

#### `UpdateAllocationRequest`

Same pattern — no annotations at all.

### 🔴 `error_to_response` function breaks project convention

Per the established convention, error handling should use `impl From<ProjectError> for HttpResponse`. Instead, [project_routes.rs](file:///home/qqx/Programming/projects/milions/milions-backend/src/adapters/driving/project_routes.rs#L172-L204) defines a manual `error_to_response` function.

### 🔴 `parse_bd` silently ignores invalid input

The [parse_bd](file:///home/qqx/Programming/projects/milions/milions-backend/src/adapters/driving/project_routes.rs#L62-L64) function uses `.ok()` on parse failures, turning invalid numeric strings like `"abc"` into `None` (interpreted as "don't update" for COALESCE). This means:
- `total_area_m2: "not_a_number"` → silently treated as `null`
- No 400/422 error is returned

---

## 2. Service Logic Analysis

### 🔴 Project service not wired in `startup.rs`

The [startup.rs](file:///home/qqx/Programming/projects/milions/milions-backend/src/startup.rs#L63-L93) file has the project service lines **commented out**:

```rust
// let project_service = web::Data::new(project::build(pool.clone()));
// .app_data(project_service.clone())
```

And the project routes are not registered in the `/api` scope. Additionally, there is no `pg_project_serv_build` function in [application/mod.rs](file:///home/qqx/Programming/projects/milions/milions-backend/src/application/mod.rs).

> [!CAUTION]
> **All project endpoints are currently unreachable.** They must be wired before any hurl tests can pass.

### 🟡 Missing status transition validation (state machine)

The status update use cases (`StartProjectUseCase`, `PauseProjectUseCase`, `CompleteProjectUseCase`, `CancelProjectUseCase`) only check "is it already in the target status?" (returning `AlreadyInStatus`). They do **not** validate allowed transitions:

| Current Status | Allowed Transitions (expected) | Actual Behavior |
|---|---|---|
| `planning` | → `in_progress`, `cancelled` | Can also go to `paused`, `completed` |
| `in_progress` | → `paused`, `completed`, `cancelled` | Can also go to `planning` (no endpoint but no restriction) |
| `paused` | → `in_progress`, `cancelled` | Can also go to `completed` |
| `completed` | → (none, terminal) | Can go to `in_progress`, `paused`, `cancelled` |
| `cancelled` | → (none, terminal) | Can go to `in_progress`, `paused`, `completed` |

> [!WARNING]
> A completed/cancelled project can be "started" again. This is likely unintentional.

### 🟡 `GetHistoryReport` returns empty `collaborator_name`

In [project_service.rs:532](file:///home/qqx/Programming/projects/milions/milions-backend/src/application/project_service.rs#L530-L537):

```rust
collaborator_name: String::new(),
```

The service never looks up the collaborator's name. The report contains an empty string.

### 🟡 No collaborator existence check in `CreateAllocationUseCase`

[project_service.rs:336-358](file:///home/qqx/Programming/projects/milions/milions-backend/src/application/project_service.rs#L335-L358): The service checks the project exists, but does **not** verify `collaborator_id` exists before inserting. This relies on the DB FK constraint which will produce an opaque `500 Internal Server Error` (wrapped as `Infra` error) instead of a clear `404 CollaboratorNotFound`.

### 🟡 Duplicate allocation uniqueness is DB-only

The `uq_allocation_collaborator_day` constraint on `(fk_project, fk_collaborator, dt_work_date)` is enforced only at the DB level. A duplicate insert will produce a `500` error from the `Infra` branch instead of a domain-level `409 Conflict`.

### 🟡 Invalid stage status silently defaults to `Pending`

In [project_service.rs:320-326](file:///home/qqx/Programming/projects/milions/milions-backend/src/application/project_service.rs#L320-L326):

```rust
_ => ProjectStageStatus::Pending,
```

An unrecognized status string like `"invalid"` is silently mapped to `Pending` instead of returning an error.

### 🟢 Correct patterns observed

- **Project existence check before update/delete/stage-create/allocation-create** ✅
- **Stage ownership check** (`current.fk_project != project_id`) ✅
- **Allocation ownership check** (`current.fk_project != project_id`) ✅
- **Report use cases check project existence** ✅

---

## 3. Hurl Test File

> [!IMPORTANT]
> This test file **cannot run** until:
> 1. `pg_project_serv_build` is added to `application/mod.rs`
> 2. The project service is registered in `startup.rs`
> 3. Project routes are added to the `/api` scope
> 4. A location must exist for `address_id` — we create one first
> 5. A collaborator must exist for allocation tests — we create one first

The hurl file below is designed to be **ready to use** once the wiring is done. It follows the exact patterns from `collaborators.hurl` and `location.hurl`.

### `hurl/projects.hurl`

```hurl
# =============================================================================
# SETUP: Create a location (needed for fk_address)
# =============================================================================
POST http://localhost:8000/api/locations
Content-Type: application/json
{
  "street": "Rua Projeto",
  "number": "100",
  "city": "Brasilia",
  "state": "DF",
  "zipcode": "70040-010",
  "complement": null,
  "public_space": "Rua",
  "unit": null,
  "neighborhood": "Asa Norte",
  "locality": "Brasilia",
  "region": "Centro-Oeste",
  "ibge": "5300108",
  "gia": null,
  "ddd": "61",
  "siafi": "9701"
}

HTTP 201
[Captures]
address_id: jsonpath "$.id"


# =============================================================================
# SETUP: Create a collaborator (needed for allocations)
# =============================================================================
POST http://localhost:8000/api/collaborators
Content-Type: application/json
{
  "name": "Collaborator Test",
  "cpf": "52998224725",
  "level": "painter",
  "contact": {
    "email": "collab.project@example.com",
    "phones": [{ "value": "+5561999990099" }]
  },
  "address": {
    "cep": "70040010",
    "street": "Esplanada dos Ministérios",
    "number": "S/N",
    "complement": null,
    "neighborhood": "Plano Piloto",
    "city": "Brasília",
    "state": "DF"
  }
}

HTTP 201
[Captures]
collaborator_id: jsonpath "$.id"


# =============================================================================
# POST /api/projects — Create Project (happy path)
# =============================================================================
POST http://localhost:8000/api/projects
Content-Type: application/json
{
  "name": "Obra Residencial Centro",
  "description": "Pintura completa da fachada",
  "address_id": "{{address_id}}",
  "start_date": "2026-05-01",
  "estimated_end_date": "2026-08-01",
  "total_area_m2": "350.50",
  "estimated_cost": "125000.00",
  "notes": "Cliente VIP"
}

HTTP 201
[Captures]
project_id: jsonpath "$.id"
[Asserts]
jsonpath "$.id" isString
jsonpath "$.name" == "Obra Residencial Centro"
jsonpath "$.description" == "Pintura completa da fachada"
jsonpath "$.status" == "planning"
jsonpath "$.start_date" == "2026-05-01"
jsonpath "$.estimated_end_date" == "2026-08-01"
jsonpath "$.total_area_m2" isString
jsonpath "$.estimated_cost" isString
jsonpath "$.notes" == "Cliente VIP"
jsonpath "$.active" == true
jsonpath "$.address_id" == {{address_id}}
jsonpath "$.created_at" isString
jsonpath "$.updated_at" isString


# =============================================================================
# POST /api/projects — Missing required field 'name' (body deserialization)
# =============================================================================
POST http://localhost:8000/api/projects
Content-Type: application/json
{
  "description": "no name",
  "address_id": "00000000-0000-7000-0000-000000000001"
}

HTTP 400
[Asserts]
jsonpath "$.error" isString
jsonpath "$.message" isString


# =============================================================================
# POST /api/projects — Missing required field 'address_id'
# =============================================================================
POST http://localhost:8000/api/projects
Content-Type: application/json
{
  "name": "Missing Address"
}

HTTP 400
[Asserts]
jsonpath "$.error" isString
jsonpath "$.message" isString


# =============================================================================
# POST /api/projects — FK address inexistente (500 — DB constraint)
# =============================================================================
POST http://localhost:8000/api/projects
Content-Type: application/json
{
  "name": "Fantasma",
  "address_id": "00000000-0000-7000-0000-ffffffffffff"
}

HTTP 500
[Asserts]
jsonpath "$.error" == "internal_error"
jsonpath "$.message" isString


# =============================================================================
# GET /api/projects — List Projects
# =============================================================================
GET http://localhost:8000/api/projects

HTTP 200
[Asserts]
jsonpath "$" isCollection
jsonpath "$[0].id" isString
jsonpath "$[0].name" isString
jsonpath "$[0].status" isString


# =============================================================================
# GET /api/projects/{uuid} — Get Project
# =============================================================================
GET http://localhost:8000/api/projects/{{project_id}}

HTTP 200
[Asserts]
jsonpath "$.id" == {{project_id}}
jsonpath "$.name" == "Obra Residencial Centro"
jsonpath "$.status" == "planning"
jsonpath "$.active" == true


# =============================================================================
# GET /api/projects/{uuid} — Not Found
# =============================================================================
GET http://localhost:8000/api/projects/00000000-0000-7000-0000-000000000000

HTTP 404
[Asserts]
jsonpath "$.error" == "not_found"
jsonpath "$.message" isString


# =============================================================================
# PUT /api/projects/{uuid} — Update Project (happy path)
# =============================================================================
PUT http://localhost:8000/api/projects/{{project_id}}
Content-Type: application/json
{
  "name": "Obra Centro Atualizada",
  "estimated_cost": "130000.00",
  "notes": "Nota atualizada"
}

HTTP 200
[Asserts]
jsonpath "$.id" == {{project_id}}
jsonpath "$.name" == "Obra Centro Atualizada"
jsonpath "$.notes" == "Nota atualizada"
jsonpath "$.status" == "planning"


# =============================================================================
# PUT /api/projects/{uuid} — Not Found
# =============================================================================
PUT http://localhost:8000/api/projects/00000000-0000-7000-0000-000000000000
Content-Type: application/json
{
  "name": "Ghost Update"
}

HTTP 404
[Asserts]
jsonpath "$.error" == "not_found"
jsonpath "$.message" isString


# =============================================================================
# PUT /api/projects/{uuid}/status — Start (planning → in_progress)
# =============================================================================
PUT http://localhost:8000/api/projects/{{project_id}}/status
Content-Type: application/json
{
  "status": "in_progress"
}

HTTP 200
[Asserts]
jsonpath "$.id" == {{project_id}}
jsonpath "$.status" == "in_progress"


# =============================================================================
# PUT /api/projects/{uuid}/status — Already in_progress (409 conflict)
# =============================================================================
PUT http://localhost:8000/api/projects/{{project_id}}/status
Content-Type: application/json
{
  "status": "in_progress"
}

HTTP 409
[Asserts]
jsonpath "$.error" == "conflict"
jsonpath "$.message" isString


# =============================================================================
# PUT /api/projects/{uuid}/status — Pause
# =============================================================================
PUT http://localhost:8000/api/projects/{{project_id}}/status
Content-Type: application/json
{
  "status": "paused"
}

HTTP 200
[Asserts]
jsonpath "$.id" == {{project_id}}
jsonpath "$.status" == "paused"


# =============================================================================
# PUT /api/projects/{uuid}/status — Already paused (409 conflict)
# =============================================================================
PUT http://localhost:8000/api/projects/{{project_id}}/status
Content-Type: application/json
{
  "status": "paused"
}

HTTP 409
[Asserts]
jsonpath "$.error" == "conflict"
jsonpath "$.message" isString


# =============================================================================
# PUT /api/projects/{uuid}/status — Resume (paused → in_progress)
# =============================================================================
PUT http://localhost:8000/api/projects/{{project_id}}/status
Content-Type: application/json
{
  "status": "in_progress"
}

HTTP 200
[Asserts]
jsonpath "$.id" == {{project_id}}
jsonpath "$.status" == "in_progress"


# =============================================================================
# PUT /api/projects/{uuid}/status — Complete
# =============================================================================
PUT http://localhost:8000/api/projects/{{project_id}}/status
Content-Type: application/json
{
  "status": "completed"
}

HTTP 200
[Asserts]
jsonpath "$.id" == {{project_id}}
jsonpath "$.status" == "completed"


# =============================================================================
# PUT /api/projects/{uuid}/status — Already completed (409 conflict)
# =============================================================================
PUT http://localhost:8000/api/projects/{{project_id}}/status
Content-Type: application/json
{
  "status": "completed"
}

HTTP 409
[Asserts]
jsonpath "$.error" == "conflict"
jsonpath "$.message" isString


# =============================================================================
# PUT /api/projects/{uuid}/status — Invalid status string (400)
# =============================================================================
PUT http://localhost:8000/api/projects/{{project_id}}/status
Content-Type: application/json
{
  "status": "suspended"
}

HTTP 400


# =============================================================================
# PUT /api/projects/{uuid}/status — Not Found
# =============================================================================
PUT http://localhost:8000/api/projects/00000000-0000-7000-0000-000000000000/status
Content-Type: application/json
{
  "status": "in_progress"
}

HTTP 404
[Asserts]
jsonpath "$.error" == "not_found"
jsonpath "$.message" isString


# =============================================================================
# POST /api/projects/{project_id}/stages — Create Stage (happy path)
# =============================================================================
POST http://localhost:8000/api/projects/{{project_id}}/stages
Content-Type: application/json
{
  "name": "Preparação de superfície",
  "description": "Lixamento e limpeza",
  "order": 1,
  "start_date": "2026-05-01",
  "end_date": "2026-05-15"
}

HTTP 201
[Captures]
stage_id: jsonpath "$.id"
[Asserts]
jsonpath "$.id" isString
jsonpath "$.project_id" == {{project_id}}
jsonpath "$.name" == "Preparação de superfície"
jsonpath "$.order" == 1
jsonpath "$.status" == "pending"
jsonpath "$.created_at" isString
jsonpath "$.updated_at" isString


# =============================================================================
# POST /api/projects/{project_id}/stages — Project not found (404)
# =============================================================================
POST http://localhost:8000/api/projects/00000000-0000-7000-0000-000000000000/stages
Content-Type: application/json
{
  "name": "Stage Ghost",
  "order": 1
}

HTTP 404
[Asserts]
jsonpath "$.error" == "not_found"
jsonpath "$.message" isString


# =============================================================================
# POST /api/projects/{project_id}/stages — Missing required field 'name'
# =============================================================================
POST http://localhost:8000/api/projects/{{project_id}}/stages
Content-Type: application/json
{
  "order": 1
}

HTTP 400
[Asserts]
jsonpath "$.error" isString
jsonpath "$.message" isString


# =============================================================================
# PUT /api/projects/{project_id}/stages/{stage_id} — Update Stage (happy)
# =============================================================================
PUT http://localhost:8000/api/projects/{{project_id}}/stages/{{stage_id}}
Content-Type: application/json
{
  "name": "Preparação Atualizada",
  "status": "in_progress"
}

HTTP 200
[Asserts]
jsonpath "$.id" == {{stage_id}}
jsonpath "$.project_id" == {{project_id}}
jsonpath "$.name" == "Preparação Atualizada"
jsonpath "$.status" == "in_progress"


# =============================================================================
# PUT /api/projects/{project_id}/stages/{stage_id} — Stage not found (404)
# =============================================================================
PUT http://localhost:8000/api/projects/{{project_id}}/stages/00000000-0000-7000-0000-000000000000
Content-Type: application/json
{
  "name": "Ghost Stage"
}

HTTP 404
[Asserts]
jsonpath "$.error" == "not_found"
jsonpath "$.message" isString


# =============================================================================
# PUT /api/projects/{project_id}/stages/{stage_id} — Project not found (404)
# =============================================================================
PUT http://localhost:8000/api/projects/00000000-0000-7000-0000-000000000000/stages/{{stage_id}}
Content-Type: application/json
{
  "name": "Orphan Stage"
}

HTTP 404
[Asserts]
jsonpath "$.error" == "not_found"
jsonpath "$.message" isString


# =============================================================================
# POST /api/projects/{project_id}/allocations — Create Allocation (happy)
# =============================================================================
POST http://localhost:8000/api/projects/{{project_id}}/allocations
Content-Type: application/json
{
  "collaborator_id": "{{collaborator_id}}",
  "work_date": "2026-05-01",
  "hours_worked": "8.00",
  "hourly_rate_snapshot": "45.00",
  "present": true,
  "notes": "Primeiro dia"
}

HTTP 201
[Captures]
allocation_id: jsonpath "$.id"
[Asserts]
jsonpath "$.id" isString
jsonpath "$.project_id" == {{project_id}}
jsonpath "$.collaborator_id" == {{collaborator_id}}
jsonpath "$.work_date" == "2026-05-01"
jsonpath "$.present" == true
jsonpath "$.created_at" isString
jsonpath "$.updated_at" isString


# =============================================================================
# POST /api/projects/{project_id}/allocations — Project not found (404)
# =============================================================================
POST http://localhost:8000/api/projects/00000000-0000-7000-0000-000000000000/allocations
Content-Type: application/json
{
  "collaborator_id": "{{collaborator_id}}",
  "work_date": "2026-05-02",
  "present": true
}

HTTP 404
[Asserts]
jsonpath "$.error" == "not_found"
jsonpath "$.message" isString


# =============================================================================
# POST /api/projects/{project_id}/allocations — Missing required fields (400)
# =============================================================================
POST http://localhost:8000/api/projects/{{project_id}}/allocations
Content-Type: application/json
{
  "notes": "missing everything"
}

HTTP 400
[Asserts]
jsonpath "$.error" isString
jsonpath "$.message" isString


# =============================================================================
# POST /api/projects/{project_id}/allocations — Duplicate (same day, 500 from DB)
# =============================================================================
POST http://localhost:8000/api/projects/{{project_id}}/allocations
Content-Type: application/json
{
  "collaborator_id": "{{collaborator_id}}",
  "work_date": "2026-05-01",
  "present": true
}

HTTP 500
[Asserts]
jsonpath "$.error" == "internal_error"
jsonpath "$.message" isString


# =============================================================================
# GET /api/projects/{project_id}/allocations — List Allocations
# =============================================================================
GET http://localhost:8000/api/projects/{{project_id}}/allocations

HTTP 200
[Asserts]
jsonpath "$" isCollection
jsonpath "$[0].id" isString
jsonpath "$[0].project_id" == {{project_id}}
jsonpath "$[0].work_date" isString


# =============================================================================
# GET /api/projects/{project_id}/allocations — Project not found (404)
# =============================================================================
GET http://localhost:8000/api/projects/00000000-0000-7000-0000-000000000000/allocations

HTTP 404
[Asserts]
jsonpath "$.error" == "not_found"
jsonpath "$.message" isString


# =============================================================================
# PUT /api/projects/{project_id}/allocations/{id} — Update Allocation (happy)
# =============================================================================
PUT http://localhost:8000/api/projects/{{project_id}}/allocations/{{allocation_id}}
Content-Type: application/json
{
  "hours_worked": "6.50",
  "notes": "Saiu mais cedo"
}

HTTP 200
[Asserts]
jsonpath "$.id" == {{allocation_id}}
jsonpath "$.project_id" == {{project_id}}
jsonpath "$.notes" == "Saiu mais cedo"


# =============================================================================
# PUT /api/projects/{project_id}/allocations/{id} — Allocation not found (404)
# =============================================================================
PUT http://localhost:8000/api/projects/{{project_id}}/allocations/00000000-0000-7000-0000-000000000000
Content-Type: application/json
{
  "present": false
}

HTTP 404
[Asserts]
jsonpath "$.error" == "not_found"
jsonpath "$.message" isString


# =============================================================================
# PUT /api/projects/{project_id}/allocations/{id} — Project not found (404)
# =============================================================================
PUT http://localhost:8000/api/projects/00000000-0000-7000-0000-000000000000/allocations/{{allocation_id}}
Content-Type: application/json
{
  "present": false
}

HTTP 404
[Asserts]
jsonpath "$.error" == "not_found"
jsonpath "$.message" isString


# =============================================================================
# GET /api/reports/projects/{project_id}/cost — Cost Report
# =============================================================================
GET http://localhost:8000/api/reports/projects/{{project_id}}/cost

HTTP 200
[Asserts]
jsonpath "$.project_id" == {{project_id}}
jsonpath "$.project_name" isString
jsonpath "$.actual_cost" isString
jsonpath "$.variance" isString
jsonpath "$.variance_pct" isString


# =============================================================================
# GET /api/reports/projects/{project_id}/cost — Not Found
# =============================================================================
GET http://localhost:8000/api/reports/projects/00000000-0000-7000-0000-000000000000/cost

HTTP 404
[Asserts]
jsonpath "$.error" == "not_found"
jsonpath "$.message" isString


# =============================================================================
# GET /api/reports/projects/{project_id}/progress — Progress Report
# =============================================================================
GET http://localhost:8000/api/reports/projects/{{project_id}}/progress

HTTP 200
[Asserts]
jsonpath "$.project_id" == {{project_id}}
jsonpath "$.project_name" isString
jsonpath "$.stages" isCollection
jsonpath "$.total_stages" isInteger
jsonpath "$.completed_stages" isInteger
jsonpath "$.progress_pct" isString


# =============================================================================
# GET /api/reports/projects/{project_id}/progress — Not Found
# =============================================================================
GET http://localhost:8000/api/reports/projects/00000000-0000-7000-0000-000000000000/progress

HTTP 404
[Asserts]
jsonpath "$.error" == "not_found"
jsonpath "$.message" isString


# =============================================================================
# GET /api/reports/collaborators/{id}/history — History Report
# =============================================================================
GET http://localhost:8000/api/reports/collaborators/{{collaborator_id}}/history

HTTP 200
[Asserts]
jsonpath "$.collaborator_id" == {{collaborator_id}}
jsonpath "$.allocations" isCollection
jsonpath "$.total_days" isInteger
jsonpath "$.total_hours" isString


# =============================================================================
# CLEANUP — Delete project (triggers CASCADE on stages/allocations)
# =============================================================================
DELETE http://localhost:8000/api/projects/{{project_id}}

HTTP 200
[Asserts]
jsonpath "$.id" == {{project_id}}


# =============================================================================
# DELETE /api/projects/{uuid} — Not Found (already deleted)
# =============================================================================
DELETE http://localhost:8000/api/projects/{{project_id}}

HTTP 404
[Asserts]
jsonpath "$.error" == "not_found"
jsonpath "$.message" isString


# =============================================================================
# CLEANUP — Delete collaborator
# =============================================================================
DELETE http://localhost:8000/api/collaborators/{{collaborator_id}}

HTTP 200


# =============================================================================
# CLEANUP — Delete location
# =============================================================================
DELETE http://localhost:8000/api/locations/{{address_id}}

HTTP 200
```

---

## Summary of Issues by Priority

| Priority | Issue | Fix Location |
|---|---|---|
| 🔴 Blocker | Project service not wired in `startup.rs` | `startup.rs` + `application/mod.rs` |
| 🔴 Critical | No `garde` validation on DTOs | `project_dto.rs` |
| 🔴 Critical | Routes use `web::Json` instead of `ValidatedJson` | `project_routes.rs` |
| 🔴 Critical | `error_to_response` fn instead of `From<Error> for HttpResponse` | `project_routes.rs` + `project_error.rs` |
| 🔴 Critical | `parse_bd` silently drops invalid numbers | `project_routes.rs` |
| 🟡 Medium | No state machine on status transitions | `project_service.rs` |
| 🟡 Medium | `collaborator_name` always empty in history report | `project_service.rs` |
| 🟡 Medium | No collaborator existence check before allocation | `project_service.rs` |
| 🟡 Medium | Duplicate allocation → 500 instead of 409 | `project_service.rs` |
| 🟡 Medium | Invalid stage status silently defaults to Pending | `project_service.rs` |
