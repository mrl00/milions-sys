# collaborator

Bounded context for managing **collaborators** (workers/employees) in the milions-sys construction project management system.

## Domain

Collaborators are the workforce assigned to projects. Each collaborator has a name, CPF, professional level, status, and can be associated with contacts and locations.

## Architecture

Follows hexagonal architecture (ports and adapters):

```
collaborator/
├── src/
│   ├── domain/
│   │   ├── errors/
│   │   │   └── collaborator_error.rs  # CollaboratorError enum
│   │   ├── models/db/                 # CollaboratorRow, CollaboratorContactRow, CollaboratorLocationRow
│   │   └── ports/
│   │       ├── collaborator_repository.rs   # FindById, FindByCpf, FindAll, Create, Update, Delete
│   │       └── collaborator_use_cases.rs    # RegisterCollaborator, FindCollaborator, ListCollaborators, UpdateCollaborator, DeleteCollaborator, ChangeCollaboratorStatus
│   ├── application/
│   │   └── collaborator_service.rs    # CollaboratorService — implements all use case traits
│   └── adapters/
│       ├── driven/
│       │   └── postgres/
│       │       └── pg_collaborator_repository.rs  # PostgreSQL implementation
│       └── driving/
│           ├── dto.rs                 # Request/response DTOs
│           └── routes.rs              # HTTP route handlers
└── Cargo.toml
```

## Dependencies

- `types` — shared value objects (`Cep`, `Cpf`, `Email`, `Phone`)

## API Endpoints

| Method | Route | Description |
|--------|-------|-------------|
| POST | `/api/collaborators` | Register a new collaborator |
| GET | `/api/collaborators` | List all collaborators |
| GET | `/api/collaborators/{uuid}` | Find collaborator by ID |
| PUT | `/api/collaborators/{uuid}` | Update a collaborator |
| DELETE | `/api/collaborators/{uuid}` | Delete a collaborator |
| PUT | `/api/collaborators/{uuid}/status` | Change collaborator status |

## Error Types

| Variant | HTTP Status | Description |
|---------|-------------|-------------|
| `NotFound` | 404 | Collaborator not found |
| `AlreadyInStatus` | 409 | Collaborator already has the requested status |
| `InvalidField` | 422 | Validation error on a field |
| `CpfAlreadyRegistered` | 409 | CPF already in use |
| `Infra` | 500 | Infrastructure/database error |
