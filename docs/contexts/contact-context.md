# contact

Bounded context for managing **contacts** and **phones** in the milions-sys construction project management system.

## Domain

Contacts represent communication channels (email, address reference) for clients and collaborators. Each contact can have multiple phone numbers. This is a shared context used by both `client` and `collaborator` contexts.

## Architecture

Follows hexagonal architecture (ports and adapters):

```
contact/
├── src/
│   ├── domain/
│   │   ├── errors/
│   │   │   └── contact_error.rs   # ContactError enum
│   │   ├── models/db/             # ContactRow, PhoneRow
│   │   └── ports/
│   │       ├── contact_repository.rs    # FindById, FindAll, Create, Update, Delete
│   │       ├── contact_use_cases.rs     # CreateContact, FindContact, ListContacts, UpdateContact, DeleteContact
│   │       └── phone_repository.rs      # FindById, FindByContactId, Create, Update, Delete
│   ├── application/
│   │   └── contact_service.rs       # ContactService — implements all use case traits
│   └── adapters/
│       ├── driven/
│       │   └── postgres/
│       │       ├── pg_contact_repository.rs   # PostgreSQL implementation for contacts
│       │       └── pg_phone_repository.rs     # PostgreSQL implementation for phones
│       └── driving/
│           ├── dto.rs               # Request/response DTOs
│           └── routes.rs            # HTTP route handlers
└── Cargo.toml
```

## Dependencies

- `types` — shared value objects (`Email`, `Phone`)

## API Endpoints

| Method | Route | Description |
|--------|-------|-------------|
| POST | `/api/contacts` | Create a new contact |
| GET | `/api/contacts` | List all contacts |
| GET | `/api/contacts/{uuid}` | Find contact by ID |
| PUT | `/api/contacts/{uuid}` | Update a contact |
| DELETE | `/api/contacts/{uuid}` | Delete a contact |
| POST | `/api/contacts/{uuid}/phones` | Add a phone to a contact |
| GET | `/api/contacts/{uuid}/phones` | List phones for a contact |
| PUT | `/api/phones/{uuid}` | Update a phone |
| DELETE | `/api/phones/{uuid}` | Delete a phone |

## Error Types

| Variant | HTTP Status | Description |
|---------|-------------|-------------|
| `NotFound` | 404 | Contact or phone not found |
| `InvalidField` | 422 | Validation error on a field |
| `Infra` | 500 | Infrastructure/database error |
