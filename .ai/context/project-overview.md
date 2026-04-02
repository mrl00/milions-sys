# milions-sys — Project Overview

## Purpose

Backend system for managing construction/engineering projects in the Brazilian market. Handles clients, collaborators, contacts, locations, and projects with financial tracking and workforce allocation.

## Tech Stack

- **Rust** (edition 2024)
- **tokio** async runtime
- **actix-web** HTTP server
- **Postgres** via `sqlx` (compile-time query macros)
- **snafu** for error context
- **thiserror** for error derivation
- **async-trait** for async trait support

## Workspace Crates

| Crate | Role |
|-------|------|
| `milions-sys` | Binary entrypoint (actix-web server) |
| `api` | HTTP routes, middleware, startup |
| `settings` | YAML config + env vars |
| `types` | Shared value objects (Phone, Email, Cpf, Doc, Cep) |
| `viacep` | ViaCEP external API integration |
| `client` | Bounded context |
| `collaborator` | Bounded context |
| `contact` | Bounded context |
| `location` | Bounded context |
| `project` | Bounded context |

## Bounded Context Structure

Every bounded context follows the same hexagonal layout:

```
<crate>/src/
  domain/
    errors/
      mod.rs                  context-specific error enum
    models/db/
      mod.rs                  sqlx FromRow structs
    ports/
      mod.rs                  module exports
      <entity>_repository.rs  repository port traits
      <entity>_use_cases.rs   use case traits + input structs
  application/
    <entity>_service.rs       implements use case traits
  adapters/
    driven/
      postgres/
        mod.rs
        pg_<entity>_repository.rs  implements port traits
    driving/
      mod.rs                  (ACL adapters for cross-context calls)
```

## Domain Rules

- **Port traits** are granular — one trait per method (Interface Segregation)
- **Super-traits** group common combinations (`FindAndCreate`, `FindAndUpdate`, `FindAndDelete`)
- **Use case traits** define a single `async fn execute(...)` method
- **Input structs** live alongside use case traits in `domain/ports/`
- **Services** implement use case traits, depend on port traits (not concrete adapters)
- **Adapters** implement repository port traits using `sqlx` static queries
- **Errors** are context-owned — no shared error crate
- **InfraError** is duplicated per context (not shared from `types`)

## Cross-Context Dependencies

```
client ──→ location   (LocationRow)
client ──→ viacep     (ViaCepPort)
```

All other contexts are independent. No context imports another context's domain layer.

## Types Crate

Shared value objects with pure validation logic. Zero SQL, zero business rules, zero state.

```
types/src/
  phone.rs      Phone, PhoneError
  email.rs      Email, EmailError
  cpf.rs        Cpf, CpfError
  cnpj.rs       Cnpj, CnpjError
  doc.rs        Doc, DocError
  cep.rs        Cep, CepError
  alphabetic.rs
  alphanumeric.rs
  numeric.rs
  errors/       InfraError (reusable by contexts)
```

## Database Schemas

| Schema | Entities |
|--------|----------|
| `clients` | tb_client, tb_client_contact, tb_client_address, tb_project, tb_project_stage, tb_project_service, tb_project_daily_allocation |
| `collaborators` | tb_collaborator, tb_collaborator_contact, tb_collaborator_address |
| `contacts` | tb_contact, tb_phone |
| `locations` | tb_location |

## Key Patterns

- **UUID v7** for time-ordered primary keys
- **Generic executor** in adapters (`E: sqlx::Executor<'a, Database = sqlx::Postgres>`) for transaction support
- **Static helper methods** on adapter structs for transaction-bound operations (create_contact, create_address)
- **`as _` imports** for port traits to avoid name collisions with use case traits
