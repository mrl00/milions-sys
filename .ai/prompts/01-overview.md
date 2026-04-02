# Project Overview Prompt

Load this when starting a new session to understand the project quickly.

## Read These Files

1. `.ai/context/project-overview.md` — full architecture
2. `.ai/context/glossary.md` — domain terms
3. `.ai/context/existing-modules.md` — current modules
4. `.ai/context/constraints.md` — hard rules

## Quick Reference

### What This Is

Rust backend for construction project management (Brazilian market). Hexagonal architecture with 5 bounded contexts: client, collaborator, contact, location, project.

### Crate Layout

```
<crate>/src/
  domain/
    errors/           context-specific error enum
    models/db/        sqlx FromRow structs
    ports/
      *_repository.rs port traits (one method per trait)
      *_use_cases.rs  use case traits + input structs
  application/
    *_service.rs      implements use case traits
  adapters/
    driven/postgres/
      pg_*_repository.rs  implements port traits
```

### Port Traits (data access)

Short names: `FindById`, `CreateClient`, `UpdateCollaborator`
Super-traits: `FindAndCreate`, `FindAndUpdate`, `FindAndDelete`

### Use Case Traits (business operations)

Prefixed: `FindClientById`, `RegisterCollaborator`, `AddPhone`
Single `execute(...)` method each.

### Services (application layer)

Struct like `ClientService { repo: PgClientRepository }`.
Implements use case traits, delegates to port traits.

### Key Rules

- Domain never imports adapters
- No cross-context domain imports
- One trait per method
- InfraError duplicated per context
- UUID v7 for all primary keys
- sqlx::query_as! for all queries
