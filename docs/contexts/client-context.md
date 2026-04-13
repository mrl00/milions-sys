# client

Bounded context for managing **clients** (customers/contractors) in the milions-sys construction project management system.

## Domain

Clients represent the customers who commission construction projects. Each client has a name, a document (CPF or CNPJ), a status, and can be associated with addresses, contacts, and projects.

## Architecture

Follows hexagonal architecture (ports and adapters):

```
client/
├── src/
│   ├── domain/
│   │   ├── errors/mod.rs          # ClientError enum
│   │   ├── models/db/             # ClientRow, ClientAddressRow, ClientContactRow, ClientProjectsRow
│   │   └── ports/
│   │       ├── client_repository.rs   # FindById, FindByDoc, FindAll, Create, Update, Delete
│   │       └── client_use_cases.rs    # RegisterClient, FindClient, ListClients, UpdateClient, DeleteClient, ChangeClientStatus
│   ├── application/
│   │   └── client_service.rs      # ClientService — implements all use case traits
│   └── adapters/
│       ├── driven/
│       │   └── postgres/
│       │       └── pg_client_repository.rs  # PostgreSQL implementation
│       └── driving/
│           ├── dto.rs             # Request/response DTOs
│           └── routes.rs          # HTTP route handlers
└── Cargo.toml
```

## Dependencies

- `location` — for address management
- `viacep` — for CEP lookup (standby)
- `types` — shared value objects (`Cep`, `Cpf`, `Cnpj`, `Email`, `Phone`)

## API Endpoints

| Method | Route | Description |
|--------|-------|-------------|
| POST | `/api/clients` | Register a new client |
| GET | `/api/clients` | List all clients |
| GET | `/api/clients/{uuid}` | Find client by ID |
| PUT | `/api/clients/{uuid}` | Update a client |
| DELETE | `/api/clients/{uuid}` | Delete a client |
| PUT | `/api/clients/{uuid}/status` | Change client status |

## Error Types

| Variant | HTTP Status | Description |
|---------|-------------|-------------|
| `NotFound` | 404 | Client not found |
| `AlreadyInStatus` | 409 | Client already has the requested status |
| `InvalidField` | 422 | Validation error on a field |
| `InvalidCep` | 422 | Invalid CEP format |
| `InvalidDoc` | 422 | Invalid CPF or CNPJ |
| `DocAlreadyRegistered` | 409 | Document already in use |
| `Infra` | 500 | Infrastructure/database error |
