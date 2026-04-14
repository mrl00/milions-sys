# project

Bounded context for managing **projects**, **stages**, **allocations**, and **reports** in the milions-sys construction project management system.

## Domain

Projects represent construction/engineering works commissioned by clients. Each project has stages (phases), daily collaborator allocations (time tracking), and cost/progress reports.

### Sub-entities

- **Project** — Main entity with status lifecycle (draft → in_progress → paused/completed/cancelled), costs, dates, and area
- **Stage** — Project phase with order, status (pending/in_progress/completed/skipped), and dates
- **Allocation** — Daily collaborator time tracking with hours worked, hourly rate, and attendance
- **Reports** — Read-only queries for cost variance, progress by stage, and collaborator history

## Architecture

Follows hexagonal architecture (ports and adapters):

```
project/
├── src/
│   ├── domain/
│   │   ├── errors/mod.rs          # ProjectError enum
│   │   ├── models/db/             # ProjectRow, ProjectStageRow, ProjectDailyAllocationRow + create/update variants
│   │   └── ports/
│   │       ├── project_repository.rs   # 14 repository port traits (project + stage + allocation)
│   │       └── project_use_cases.rs    # 18 use case traits (project + stage + allocation + reports)
│   ├── application/
│   │   └── project_service.rs     # ProjectService — implements all use case traits
│   └── adapters/
│       ├── driven/
│       │   └── postgres.rs        # PostgreSQL implementation (all repository traits)
│       └── driving/
│           ├── dto.rs             # Request/response DTOs
│           └── routes.rs          # HTTP route handlers
└── Cargo.toml
```

## Dependencies

- `types` — shared value objects (`BigDecimal` for monetary values)

## API Endpoints

| Method | Route | Description |
|--------|-------|-------------|
| POST | `/api/projects` | Create a new project |
| GET | `/api/projects` | List all projects |
| GET | `/api/projects/{uuid}` | Find project by ID |
| PUT | `/api/projects/{uuid}` | Update a project |
| DELETE | `/api/projects/{uuid}` | Delete a project |
| PUT | `/api/projects/{uuid}/status` | Change project status |
| POST | `/api/projects/{project_id}/stages` | Create a stage |
| PUT | `/api/projects/{project_id}/stages/{stage_id}` | Update a stage |
| POST | `/api/projects/{project_id}/allocations` | Create an allocation |
| GET | `/api/projects/{project_id}/allocations` | List allocations for a project |
| PUT | `/api/projects/{project_id}/allocations/{allocation_id}` | Update an allocation |
| GET | `/api/reports/projects/{project_id}/cost` | Cost report (actual vs estimated) |
| GET | `/api/reports/projects/{project_id}/progress` | Progress report (by stage) |
| GET | `/api/reports/collaborators/{collaborator_id}/history` | Collaborator allocation history |

## Error Types

| Variant | HTTP Status | Description |
|---------|-------------|-------------|
| `NotFound` | 404 | Project not found |
| `AlreadyInStatus` | 409 | Project already has the requested status |
| `StageNotFound` | 404 | Stage not found |
| `AllocationNotFound` | 404 | Allocation not found |
| `CollaboratorNotFound` | 404 | Collaborator not found |
| `InvalidField` | 422 | Validation error on a field |
| `Infra` | 500 | Infrastructure/database error |
