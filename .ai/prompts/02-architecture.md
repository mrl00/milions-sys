# Architecture Prompt

Detailed guide for implementing new features following the hexagonal architecture.

## Step-by-Step: Adding a New Entity

### 1. Define Models (`domain/models/db/`)

Create `<entity>_row.rs` with:
- `<Entity>Row` — sqlx `FromRow` struct (DB representation)
- `Create<Entity>Row` — input for inserts
- `Update<Entity>Row` — input for updates (Option fields)
- Add `pub mod <entity>_row;` to `models/db/mod.rs`

### 2. Define Repository Port (`domain/ports/<entity>_repository.rs`)

One trait per method:

```rust
#[async_trait]
pub trait Find<Entity>ById: Send + Sync {
    async fn find_by_id(&self, uuid: Uuid) -> Result<Option<<Entity>Row>, <Entity>Error>;
}
```

Add super-traits for common combinations:
```rust
pub trait FindAndCreate<Entity>: Find<Entity>ByX + Create<Entity> {}
pub trait FindAndUpdate<Entity>: Find<Entity>ById + Update<Entity> {}
pub trait FindAndDelete<Entity>: Find<Entity>ById + Delete<Entity> {}
```

Register in `domain/ports/mod.rs`.

### 3. Define Use Case Traits (`domain/ports/<entity>_use_cases.rs`)

One trait per business operation, single `execute` method:

```rust
pub struct Register<Entity>Input {
    pub name: String,
    // ... fields
}

#[async_trait]
pub trait Register<Entity>: Send + Sync {
    async fn execute(&self, input: Register<Entity>Input) -> Result<<Entity>Row, <Entity>Error>;
}
```

Register in `domain/ports/mod.rs`.

### 4. Implement Adapter (`adapters/driven/postgres/pg_<entity>_repository.rs`)

```rust
pub struct Pg<Entity>Repository {
    pool: PgPool,
}

impl Pg<Entity>Repository {
    pub fn new(pool: PgPool) -> Self { Self { pool } }
}

fn sqlx_err(action: &'static str) -> impl FnOnce(sqlx::Error) -> <Entity>Error {
    move |e| <Entity>Error::Infra {
        source: InfraError::Database { action, source: e },
    }
}

#[async_trait]
impl Find<Entity>ById for Pg<Entity>Repository {
    async fn find_by_id(&self, uuid: Uuid) -> Result<Option<<Entity>Row>, <Entity>Error> {
        sqlx::query_as!(<Entity>Row, "SELECT ...", &uuid)
            .fetch_optional(&self.pool)
            .await
            .map_err(sqlx_err("buscar <entity> por id"))
    }
}

impl FindAndCreate<Entity> for Pg<Entity>Repository {}
impl FindAndUpdate<Entity> for Pg<Entity>Repository {}
impl FindAndDelete<Entity> for Pg<Entity>Repository {}
```

### 5. Implement Service (`application/<entity>_service.rs`)

```rust
pub struct <Entity>Service {
    repo: Pg<Entity>Repository,
}

impl <Entity>Service {
    pub fn new(pool: PgPool) -> Self {
        Self { repo: Pg<Entity>Repository::new(pool) }
    }
}

#[async_trait]
impl Register<Entity> for <Entity>Service {
    async fn execute(&self, input: Register<Entity>Input) -> Result<<Entity>Row, <Entity>Error> {
        // validate
        // check uniqueness
        self.repo.create(Create<Entity>Row { ... }).await
    }
}
```

### 6. Define Error (`domain/errors/mod.rs`)

```rust
#[derive(Debug, thiserror::Error)]
pub enum <Entity>Error {
    #[error("<entity> não encontrado: {uuid}")]
    NotFound { uuid: Uuid },

    #[error("<field> já cadastrado")]
    AlreadyExists { field: String },

    #[error(transparent)]
    Infra { source: InfraError },
}
```

## Import Pattern

### Application layer imports

```rust
// Port traits — use `as _` to avoid collision with use case traits
use crate::domain::ports::<entity>_repository::Find<Entity>ById as _;
use crate::domain::ports::<entity>_repository::Create<Entity> as _;

// Use case traits — import directly
use crate::domain::ports::<entity>_use_cases::{Register<Entity>, Find<Entity>};
```

### Adapter imports

```rust
use crate::domain::ports::<entity>_repository::*;
use types::errors::infra_error::InfraError;
// Never import from application or other contexts
```

## Transaction Support

For operations spanning multiple tables:

```rust
// Static helper on adapter struct
impl Pg<Entity>Repository {
    pub async fn create_related<'a, E>(
        executor: E,
        ...
    ) -> Result<..., sqlx::Error>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        sqlx::query_as!(..., "INSERT ...", ...)
            .fetch_one(executor)
            .await
    }
}
```

Then in the service:

```rust
let mut tx = pool.begin().await.map_err(...)?;
let main = self.repo.create_with_tx(&mut *tx, input).await?;
Pg<Entity>Repository::create_related(&mut *tx, main.id, ...).await?;
tx.commit().await.map_err(...)?;
```

## Checklist

- [ ] Model structs in `domain/models/db/`
- [ ] Error variants in `domain/errors/`
- [ ] Repository port traits in `domain/ports/<entity>_repository.rs`
- [ ] Use case traits + inputs in `domain/ports/<entity>_use_cases.rs`
- [ ] Adapter in `adapters/driven/postgres/pg_<entity>_repository.rs`
- [ ] Service in `application/<entity>_service.rs`
- [ ] All modules registered in their respective `mod.rs`
- [ ] `cargo check -p <crate>` compiles
