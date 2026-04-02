# API Contracts

REST API routes for milions-sys.

## Conventions

- All routes prefixed with `/api`
- JSON request/response bodies
- Path params: `{uuid}` = UUID v7
- Status changes use dedicated `PUT .../status` endpoints
- Aggregates return the entity with nested relationships

---

## Clients

| Method | Route | Description |
|--------|-------|-------------|
| POST | `/api/clients` | Register client |
| GET | `/api/clients` | List clients |
| GET | `/api/clients/{uuid}` | Get client (aggregate: contact, address) |
| PUT | `/api/clients/{uuid}` | Update client |
| DELETE | `/api/clients/{uuid}` | Remove client |

### Request Bodies

**POST `/api/clients`**
```json
{
  "name": "string",
  "document": "string (CPF or CNPJ)",
  "contact": {
    "email": "string",
    "phones": ["string"]
  },
  "address": {
    "cep": "string",
    "street": "string",
    "number": "string",
    "complement": "string | null",
    "neighborhood": "string",
    "city": "string",
    "state": "string"
  }
}
```

**PUT `/api/clients/{uuid}`**
```json
{
  "name": "string | null",
  "document": "string | null",
  "contact": { ... } | null,
  "address": { ... } | null
}
```

---

## Projects

| Method | Route | Description |
|--------|-------|-------------|
| POST | `/api/projects` | Create project |
| GET | `/api/projects` | List projects |
| GET | `/api/projects/{uuid}` | Get project (aggregate: stages, services, allocations) |
| PUT | `/api/projects/{uuid}` | Update project |
| PUT | `/api/projects/{uuid}/status` | Change status (planning → in_progress → paused/completed/cancelled) |

### Request Bodies

**POST `/api/projects`**
```json
{
  "name": "string",
  "description": "string | null",
  "client_id": "uuid",
  "address": {
    "cep": "string",
    "street": "string",
    "number": "string",
    "complement": "string | null",
    "neighborhood": "string",
    "city": "string",
    "state": "string"
  },
  "start_date": "date | null",
  "estimated_end_date": "date | null",
  "total_area_m2": "number | null",
  "estimated_cost": "number | null"
}
```

**PUT `/api/projects/{uuid}/status`**
```json
{
  "status": "in_progress | paused | completed | cancelled"
}
```

---

## Stages

| Method | Route | Description |
|--------|-------|-------------|
| POST | `/api/projects/{uuid}/stages` | Create stage |
| PUT | `/api/projects/{uuid}/stages/{uuid}` | Update stage |

### Request Bodies

**POST `/api/projects/{project_id}/stages`**
```json
{
  "name": "string",
  "description": "string | null",
  "order": "i32",
  "start_date": "date | null",
  "end_date": "date | null"
}
```

**PUT `/api/projects/{project_id}/stages/{uuid}`**
```json
{
  "name": "string | null",
  "description": "string | null",
  "order": "i32 | null",
  "status": "pending | in_progress | completed | skipped | null",
  "start_date": "date | null",
  "end_date": "date | null"
}
```

---

## Collaborators

| Method | Route | Description |
|--------|-------|-------------|
| POST | `/api/collaborators` | Register collaborator |
| GET | `/api/collaborators` | List collaborators |
| GET | `/api/collaborators/{uuid}` | Get collaborator (aggregate: contact, address) |
| PUT | `/api/collaborators/{uuid}` | Update collaborator |
| PUT | `/api/collaborators/{uuid}/status` | Activate/deactivate |

### Request Bodies

**POST `/api/collaborators`**
```json
{
  "name": "string",
  "cpf": "string",
  "level": "P0 | P1 | P2 | P3",
  "contact": {
    "email": "string",
    "phones": ["string"]
  },
  "address": {
    "cep": "string",
    "street": "string",
    "number": "string",
    "complement": "string | null",
    "neighborhood": "string",
    "city": "string",
    "state": "string"
  }
}
```

**PUT `/api/collaborators/{uuid}/status`**
```json
{
  "status": "active | inactive"
}
```

---

## Allocations

| Method | Route | Description |
|--------|-------|-------------|
| POST | `/api/projects/{uuid}/allocations` | Allocate collaborator |
| GET | `/api/projects/{uuid}/allocations` | List project allocations |
| PUT | `/api/projects/{uuid}/allocations/{uuid}` | Update allocation |

### Request Bodies

**POST `/api/projects/{project_id}/allocations`**
```json
{
  "collaborator_id": "uuid",
  "work_date": "date",
  "hours_worked": "number | null",
  "hourly_rate_snapshot": "number | null",
  "present": "bool",
  "notes": "string | null"
}
```

**PUT `/api/projects/{project_id}/allocations/{uuid}`**
```json
{
  "hours_worked": "number | null",
  "hourly_rate_snapshot": "number | null",
  "present": "bool | null",
  "notes": "string | null"
}
```

---

## Reports

| Method | Route | Description |
|--------|-------|-------------|
| GET | `/api/reports/projects/{uuid}/cost` | Actual vs estimated cost |
| GET | `/api/reports/projects/{uuid}/progress` | Progress by stage |
| GET | `/api/reports/collaborators/{uuid}/history` | Allocation history |

### Response Bodies

**GET `/api/reports/projects/{uuid}/cost`**
```json
{
  "project_id": "uuid",
  "project_name": "string",
  "estimated_cost": "number | null",
  "actual_cost": "number",
  "variance": "number",
  "variance_pct": "number"
}
```

**GET `/api/reports/projects/{uuid}/progress`**
```json
{
  "project_id": "uuid",
  "project_name": "string",
  "stages": [
    {
      "stage_id": "uuid",
      "name": "string",
      "order": "i32",
      "status": "pending | in_progress | completed | skipped",
      "start_date": "date | null",
      "end_date": "date | null"
    }
  ],
  "total_stages": "i32",
  "completed_stages": "i32",
  "progress_pct": "number"
}
```

**GET `/api/reports/collaborators/{uuid}/history`**
```json
{
  "collaborator_id": "uuid",
  "collaborator_name": "string",
  "allocations": [
    {
      "allocation_id": "uuid",
      "project_id": "uuid",
      "project_name": "string",
      "work_date": "date",
      "hours_worked": "number | null",
      "hourly_rate_snapshot": "number | null",
      "present": "bool"
    }
  ],
  "total_days": "i32",
  "total_hours": "number"
}
```
