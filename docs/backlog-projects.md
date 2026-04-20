# milions-sys — Backlog Projects

## Estado Atual (2026-04-20)

O contexto **project** possui todo o código compilando (models, DTOs, ports, service, repository, routes, 32 unit tests passando), porém está **desconectado em runtime** — o service não está registrado no `startup.rs` e as rotas não estão no scope `/api`.

### Problemas por Camada

| Camada | Status | Problema |
|--------|--------|----------|
| DB models / rows | ✅ | — |
| Repository (PG) | ✅ | — |
| Use case ports | ✅ | — |
| Service impl | ⚠️ | Falta state machine, collaborator_name vazio, status inválido aceito silenciosamente |
| DTOs | 🔴 | Zero anotações `garde`, sem `#[derive(Validate)]` |
| Routes | 🔴 | Usa `web::Json` em vez de `ValidatedJson`, `error_to_response` fn manual, `parse_bd` engole erros |
| Wired in startup | 🔴 | Comentado, `pg_project_serv_build` não existe |
| Hurl tests | 🔴 | Arquivo não existe |

---

## 🔴 P0 — Bloqueia execução

### T01 — Registrar project service no runtime

- [x] Criar `pg_project_serv_build(pool)` em `src/application/mod.rs`
  - Seguir o padrão de `pg_collaborator_serv_build`
  - `PgProjectService::new(PgProjectRepository::new(pool))`
- [x] Em `src/startup.rs`:
  - Descomentar/criar `let project_service = web::Data::new(pg_project_serv_build(pool.clone()));`
  - Adicionar `.app_data(project_service.clone())`
  - Adicionar `.configure(crate::adapters::driving::project_routes::configure)` no scope `/api`
- [x] Verificar que os 14 endpoints respondem (status != 404)

---

## 🔴 P1 — Conformidade com convenções do projeto

### T02 — Adicionar validação `garde` nos DTOs de project

Todos os outros contextos (collaborator, contact, location) usam `#[derive(Validate)]` + anotações `#[garde(...)]`. O contexto project é o único sem validação.

**Arquivo:** `src/adapters/driving/models/dtos/project_dto.rs`

- [x] `use garde::Validate;`
- [x] `CreateProjectRequest` → `#[derive(Debug, Deserialize, Validate)]`
  - `name`: `#[garde(length(min = 1, max = 255))]` (DB: `VARCHAR(255) NOT NULL`)
  - `description`: `#[garde(skip)]`
  - `address_id`: `#[garde(skip)]`
  - `start_date`: `#[garde(skip)]`
  - `estimated_end_date`: `#[garde(skip)]`
  - `total_area_m2`: `#[garde(skip)]`
  - `estimated_cost`: `#[garde(skip)]`
  - `notes`: `#[garde(skip)]`
- [x] `UpdateProjectRequest` → `#[derive(Debug, Deserialize, Validate)]`
  - `name`: `#[garde(inner(length(min = 1, max = 255)))]`
  - Todos os demais: `#[garde(skip)]`
- [x] `ProjectStatusRequest` → `#[derive(Debug, Deserialize, Validate)]`
  - `status`: `#[garde(pattern(r"^(in_progress|paused|completed|cancelled)$"))]`
- [x] `CreateStageRequest` → `#[derive(Debug, Deserialize, Validate)]`
  - `name`: `#[garde(length(min = 1, max = 255))]`
  - `order`: `#[garde(range(min = 1))]`
  - `description`, `start_date`, `end_date`: `#[garde(skip)]`
- [x] `UpdateStageRequest` → `#[derive(Debug, Deserialize, Validate)]`
  - `name`: `#[garde(inner(length(min = 1, max = 255)))]`
  - `status`: `#[garde(inner(pattern(r"^(pending|in_progress|completed|skipped)$")))]`
  - Demais: `#[garde(skip)]`
- [x] `CreateAllocationRequest` → `#[derive(Debug, Deserialize, Validate)]`
  - Todos os campos: `#[garde(skip)]`
- [x] `UpdateAllocationRequest` → `#[derive(Debug, Deserialize, Validate)]`
  - Todos os campos: `#[garde(skip)]`

### T03 — Trocar `web::Json` por `ValidatedJson` nos routes

**Arquivo:** `src/adapters/driving/project_routes.rs`

- [x] `use crate::adapters::driving::utils::ValidatedJson;`
- [x] Trocar em todos os handlers:
  - `create_project`: `body: web::Json<CreateProjectRequest>` → `ValidatedJson(body): ValidatedJson<CreateProjectRequest>`
  - `update_project`: idem para `UpdateProjectRequest`
  - `update_project_status`: idem para `ProjectStatusRequest`
  - `create_stage`: idem para `CreateStageRequest`
  - `update_stage`: idem para `UpdateStageRequest`
  - `create_allocation`: idem para `CreateAllocationRequest`
  - `update_allocation`: idem para `UpdateAllocationRequest`
- [x] Remover tratamento manual de status inválido no handler `update_project_status` (substituído por `unreachable!` — garde já rejeita)
- [x] Adaptar acesso ao body: destructuring `ValidatedJson(body)` em todos os handlers

### T04 — Substituir `error_to_response` por `impl From<ProjectError> for HttpResponse`

A convenção do projeto é que cada domain error implemente `From<Error> for HttpResponse`. O contexto project usa uma função manual.

- [x] Em `src/adapters/driving/errors/error_response.rs`: implementar `impl From<ProjectError> for HttpResponse`
  - `NotFound` / `StageNotFound` / `AllocationNotFound` / `CollaboratorNotFound` → `404` com `{"error":"not_found","message":"..."}`
  - `AlreadyInStatus` → `409` com `{"error":"conflict","message":"..."}`
  - `InvalidField` → `422` com `{"error":"validation_error","message":"..."}`
  - `Infra(_)` → `500` com `{"error":"internal_error","message":"internal server error"}`
- [x] Em `src/adapters/driving/project_routes.rs`:
  - Função `error_to_response` removida
  - Todos os handlers usam `Err(e) => HttpResponse::from(e)`
- [x] Testes unitários atualizados para usar `HttpResponse::from(err)` diretamente

### T05 — Corrigir `parse_bd` que engole erros silenciosamente

**Arquivo:** `src/adapters/driving/project_routes.rs`

A função `parse_bd` usa `.ok()` em falhas de parse, fazendo `"abc"` virar `None` silenciosamente.

- [x] Opção A (implementada): parse movido para o service via `fn parse_decimal(val: Option<String>, field: &'static str) -> Result<Option<BigDecimal>, ProjectError>` em `project_service.rs`
  - Se `Some(s)` mas `s.parse::<BigDecimal>()` falha → `ProjectError::InvalidField { field, reason: "'...' is not a valid decimal number" }` → `422`
  - `parse_bd` removida de `project_routes.rs`
  - Import `sqlx::types::BigDecimal` removido das rotas
- [x] Campos numéricos inválidos retornam `422`
- [x] 3 testes unitários adicionados para `parse_decimal` (válido, inválido, None)

---

## 🟡 P2 — Lógica de negócio incompleta

### T06 — Implementar state machine para transições de status

**Arquivo:** `src/application/project_service.rs`

Atualmente qualquer transição é permitida desde que o status atual ≠ status alvo. Um projeto `completed` pode ser "started" novamente.

- [x] Definir transições válidas via `fn valid_transition(from: &str, to: &ProjectStatus) -> bool`:
  - `planning` → `in_progress`, `cancelled`
  - `in_progress` → `paused`, `completed`, `cancelled`
  - `paused` → `in_progress`, `cancelled`
  - `completed` → (terminal, nenhuma transição)
  - `cancelled` → (terminal, nenhuma transição)
- [x] Adicionada variante `InvalidTransition { from, to }` em `ProjectError`
- [x] `fn check_transition(uuid, from, target)` centraliza a lógica: mesmo status → `AlreadyInStatus`, transição inválida → `InvalidTransition`
- [x] Start/Pause/Complete/Cancel usam `check_transition` em vez de verificação manual
- [x] `InvalidTransition` mapeado para `409 Conflict` no `From<ProjectError> for HttpResponse`
- [x] 12 testes unitários adicionados (transições válidas, inválidas, estados terminais, via use cases)

### T07 — Verificar existência do collaborator antes de criar allocation

**Arquivo:** `src/application/project_service.rs` — `CreateAllocationUseCase`

Atualmente o service verifica que o project existe, mas **não** verifica o `collaborator_id`. FK violation no DB gera `500 Internal Server Error` genérico.

- [ ] Opção A (recomendada): adicionar port `FindCollaboratorById` no `ProjectRepository` (ou via ACL adapter) e verificar no service, retornando `ProjectError::CollaboratorNotFound`
- [ ] Opção B: interceptar `sqlx::Error` de FK violation no repository e mapear para `CollaboratorNotFound`
- [ ] Adicionar teste unitário

### T08 — Tratar conflito de allocation duplicada como `409`

**Arquivo:** `src/application/project_service.rs` ou `src/adapters/driven/pg_project_repository.rs`

A constraint `uq_allocation_collaborator_day(fk_project, fk_collaborator, dt_work_date)` causa unique violation no DB → erro opaco `500`.

- [ ] Adicionar variante `AllocationConflict { project_id, collaborator_id, work_date }` em `ProjectError`
- [ ] Interceptar `sqlx::Error::Database` com código `23505` no repository e mapear para o novo erro
- [ ] Mapear para `409 Conflict` no `From<ProjectError> for HttpResponse`
- [ ] Adicionar teste hurl

### T09 — Rejeitar status de stage inválido em vez de defaultar para Pending

**Arquivo:** `src/application/project_service.rs` — `UpdateStageUseCase`

```rust
_ => ProjectStageStatus::Pending, // ← silencia erro
```

- [ ] Retornar `ProjectError::InvalidField { field: "status", reason: "..." }` quando o valor não é um dos 4 válidos
- [ ] Se T02 (garde) for implementado primeiro, valores inválidos já serão rejeitados com 400 no DTO — este match se torna unreachable
- [ ] Considerar usar `FromStr` no enum `ProjectStageStatus` em vez de match manual

### T10 — Preencher `collaborator_name` no history report

**Arquivo:** `src/application/project_service.rs` — `GetHistoryReportUseCase`

Atualmente retorna `String::new()`.

- [ ] Opção A: fazer JOIN com `collaborators.tb_collaborator` na query do repository (adicionar `collaborator_name` ao `AllocationWithProjectName` ou criar novo row type)
- [ ] Opção B: adicionar port cross-context via ACL para buscar nome do collaborator
- [ ] Atualizar testes

---

## 🟢 P3 — Testes de integração

### T11 — Criar e salvar `hurl/projects.hurl`

- [ ] Criar o arquivo com cenários de happy path e erros para todos os 14 endpoints (ver análise completa em `analysis_results.md` da conversa `b5ccad4d`)
- [ ] O arquivo depende de setup (criar location + collaborator antes) e cleanup (DELETE ao final)
- [ ] Cenários obrigatórios:
  - Project CRUD (POST 201, GET 200, PUT 200, DELETE 200)
  - Project not found (GET/PUT/DELETE 404)
  - Missing required fields (POST 400)
  - FK address inexistente (POST 500 → idealmente 422 após T05)
  - Status transitions (4 use cases × happy + already-in-status 409)
  - Invalid status (400)
  - Stage CRUD (POST 201, PUT 200, not found 404)
  - Allocation CRUD (POST 201, PUT 200, GET list 200, not found 404)
  - Duplicate allocation (500 → idealmente 409 após T08)
  - Reports: cost, progress, history (200 + not found 404)
  - Cleanup de todos os recursos criados
- [ ] Executar `hurl --test hurl/projects.hurl` e garantir que todos passam

---

## Ordem de execução recomendada

```
T01 (wire startup)
 ↓
T02 (garde DTOs) + T03 (ValidatedJson) ← podem ser feitas juntas
 ↓
T04 (From<Error> for HttpResponse)
 ↓
T05 (parse_bd)
 ↓
T06 (state machine) + T07 (collaborator check) + T08 (allocation conflict) + T09 (stage status)
 ↓
T10 (collaborator_name)
 ↓
T11 (hurl tests) ← só depois que tudo acima estiver implementado
```

> **Nota:** T11 deve ser a última task pois os testes hurl validam o comportamento final de todos os endpoints. Se features como state machine (T06) ou conflict handling (T08) forem implementadas depois, os status codes esperados no hurl mudarão.

---

## Referências

| Arquivo | Descrição |
|---------|-----------|
| `src/adapters/driven/pg_project_repository.rs` | Repository PostgreSQL |
| `src/adapters/driving/project_routes.rs` | Handlers HTTP (14 endpoints) |
| `src/adapters/driving/models/dtos/project_dto.rs` | Request/Response DTOs |
| `src/application/project_service.rs` | Service com 16 use cases |
| `src/domain/errors/project_error.rs` | Error enum (6 variantes) |
| `src/domain/models/db/project_rows.rs` | DB row structs e enums |
| `src/domain/ports/use_cases/project_use_cases.rs` | Use case traits |
| `src/domain/ports/repositories/project_repository.rs` | Repository traits (14 port traits) |
| `migrations/20260222180342_create_project_schema.sql` | 5 tabelas: project, stage, service_type, project_service, daily_allocation |
| `src/application/mod.rs` | Builder functions (falta `pg_project_serv_build`) |
| `src/startup.rs` | Wiring (project comentado) |
