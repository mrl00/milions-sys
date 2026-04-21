# =============================================================================
# SETUP: Create a location (needed for fk_address)
# =============================================================================
POST http://localhost:8000/api/locations
Content-Type: application/json
{
  "street": "Rua das Flores",
  "number": "42",
  "city": "Brasilia",
  "state": "DF",
  "zipcode": "70040-010",
  "complement": "Bloco A",
  "public_space": "Rua",
  "unit": "Apt 101",
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
# POST /api/projects/{project_id}/allocations — Duplicate (same collaborator + day = 409)
# =============================================================================
POST http://localhost:8000/api/projects/{{project_id}}/allocations
Content-Type: application/json
{
  "collaborator_id": "{{collaborator_id}}",
  "work_date": "2026-05-01",
  "present": true
}

HTTP 409
[Asserts]
jsonpath "$.error" == "conflict"
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
