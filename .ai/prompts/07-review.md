# Code Review Checklist

Use this checklist when reviewing PRs or before merging changes to milions-sys.

---

## Architecture

- [ ] Domain layer does not import from `adapters/` or `application/`
- [ ] No bounded context imports another context's `domain/` module
- [ ] Port traits define only signatures — no implementations
- [ ] Services depend on port traits, not concrete adapter structs
- [ ] Adapters implement port traits — no custom interfaces
- [ ] Cross-context dependency follows the allowed graph:
  - `client → location`
  - `client → viacep`
  - All others: independent

## Naming

- [ ] Repository traits: short names (`FindById`, `CreateClient`)
- [ ] Use case traits: prefixed (`FindClientById`, `RegisterCollaborator`)
- [ ] Super-traits: `FindAnd*` pattern, no new methods
- [ ] Services: `<Entity>Service`
- [ ] Adapters: `Pg<Entity>Repository`
- [ ] Errors: `<Entity>Error`
- [ ] Input structs: `Register<Entity>Input`, `Update<Entity>Input`

## Traits

- [ ] One method per trait (Interface Segregation)
- [ ] All async traits use `#[async_trait]` macro
- [ ] All port traits require `Send + Sync`
- [ ] Super-traits only compose — no additional methods
- [ ] Input structs live in `domain/ports/`, not in models
- [ ] `as _` imports used when port and use case trait names collide

## Errors

- [ ] Each bounded context defines its own error enum
- [ ] `InfraError` is duplicated per context — never shared across contexts
- [ ] Error variants use `#[error(transparent)]` or `#[error("...")]`
- [ ] sqlx errors wrapped in `InfraError::Database`
- [ ] No `unwrap()` or `expect()` in business logic (only in `main.rs` boot)
- [ ] User-facing errors return JSON: `{ "error": "code", "message": "..." }`
- [ ] Internal error details never exposed to client — only in logs

## SQL & Database

- [ ] All queries use `sqlx::query_as!` macros (compile-time checked)
- [ ] Primary keys use `Uuid::now_v7()`
- [ ] Partial updates use `COALESCE`
- [ ] Generic executor pattern for transaction support: `E: sqlx::Executor<'a, Database = Postgres>`
- [ ] No N+1 queries — joins or batch fetches for aggregates
- [ ] Indexes exist for foreign keys used in WHERE/JOIN
- [ ] No raw SQL strings outside `query_as!` macros

## File Structure

- [ ] Use case traits in `domain/ports/<entity>_use_cases.rs` (single file, not directory)
- [ ] Services in `application/<entity>_service.rs` (single file)
- [ ] Repository ports in `domain/ports/<entity>_repository.rs`
- [ ] No `domain/use_cases/` directory
- [ ] Models in `domain/models/db/<entity>_row.rs`
- [ ] Errors in `domain/errors/mod.rs`
- [ ] All modules registered in parent `mod.rs`

## Type Safety

- [ ] Value objects from `types` crate used where applicable (`Cpf`, `Phone`, `Email`, `Cep`, `Doc`)
- [ ] No `String` for validated fields — use typed wrappers
- [ ] `secrecy::SecretString` for credentials — never plain `String`
- [ ] `BigDecimal` for monetary values — never `f64`
- [ ] `NaiveDate` / `NaiveDateTime` for dates — never `String`

## Security

- [ ] No credentials in source code or config files committed to repo
- [ ] `.env` in `.gitignore`
- [ ] `APP_DATABASE__PASSWORD` never logged
- [ ] No PII (CPF, email, phone) in log output
- [ ] CORS origins restricted to known frontends

## Performance

- [ ] List endpoints support pagination (default 20, max 100)
- [ ] No synchronous blocking calls in async context
- [ ] Connection pool configured (max 10, idle timeout 5s)
- [ ] Lazy connections not used — pool validated on boot

## Testing

- [ ] Unit tests for domain logic (validation, error mapping)
- [ ] Tests cover error paths, not only happy paths
- [ ] No tests depend on shared mutable state

## Documentation

- [ ] `.ai/context/existing-modules.md` updated if ports/use cases/services changed
- [ ] `.ai/context/project-overview.md` updated if architecture changed
- [ ] `.ai/prompts/03-data-model.md` updated if tables/columns changed
- [ ] `.ai/prompts/04-api-contracts.md` updated if routes changed
- [ ] Migrations in `migrations/` — numbered, descriptive filename

## CI (when available)

- [ ] `cargo check --workspace` passes
- [ ] `cargo clippy --workspace -- -D warnings` passes
- [ ] `cargo test --workspace` passes
- [ ] `cargo fmt --check` passes
