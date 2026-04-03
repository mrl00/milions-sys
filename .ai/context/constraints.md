# Constraints

Hard rules that must not be violated.

## Architecture

- Domain layer must never import from adapters or application layer
- No bounded context may import another context's `domain` module
- `domain::ports` must only define traits, not implementations
- Services must depend on port traits, never on concrete adapter structs
- Adapters must implement port traits, not define their own interface

## Dependencies

- `types` crate has zero dependencies on bounded contexts
- `viacep` crate has zero dependencies on bounded contexts
- Only `client` may depend on `location` and `viacep`
- No circular dependencies between crates
- No bounded context depends on another bounded context's domain

## Naming

- Repository port traits: short names (`FindById`, `CreateClient`)
- Use case traits: prefixed with entity (`FindClientById`, `RegisterContact`)
- Super-traits: `FindAnd*` pattern
- Composite repository traits: `<Entity>Repository` (e.g. `ClientRepository`)
- Services: `<Entity>Service`
- Adapters: `Pg<Entity>Repository`
- Errors: `<Entity>Error`

## Errors

- Each bounded context defines its own error enum
- `InfraError` is duplicated per context, never shared
- Error variants use `#[error(transparent)]` or `#[error("...")]`
- Error messages are in English
- sqlx errors must be wrapped in `InfraError::Database`

## SQL

- All queries use `sqlx::query_as!` macros (compile-time checked)
- Primary keys are UUID v7 (`Uuid::now_v7()`)
- Partial updates use COALESCE
- Generic executor parameter for transaction support

## Traits

- One method per trait (Interface Segregation)
- All async traits use `#[async_trait]` macro
- All port traits require `Send + Sync`
- Super-traits only compose, never add new methods
- Input structs live alongside use case traits in `domain/ports/`

## Files

- Use case traits live in `domain/ports/<entity>_use_cases.rs` (single file, not directory)
- Services live in `application/<entity>_service.rs` (single file)
- Repository ports live in `domain/ports/<entity>_repository.rs`
- No `domain/use_cases/` directory
