# milions-sys — Backlog Client

## Estado Atual (2026-04-23)

O contexto **client** possui código compilando (models, DTOs, ports, service, repository, routes, 16 unit tests passando), porém está **desconectado em runtime** — o service não está registrado no `startup.rs`, as rotas não estão no scope `/api`, e não existe builder function em `application/mod.rs`.

O `RegisterClientUseCase` é o mais complexo do sistema: ele orquestra uma transação que cria location, contact, phones, client, client_contact e client_address em sequência. Os demais use cases (CRUD, activate/deactivate) seguem o padrão padrão do projeto.

### Problemas por Camada

| Camada | Status | Problema |
|--------|--------|----------|
| DB models / rows | ✅ | — |
| Repository (PG) | ✅ | — |
| Use case ports | ✅ | — |
| Service impl | ⚠️ | `ConcreteClientService` usa `PgPool` direto + repos concretos na transação (viola hexagonal); `ClientService<R>` genérico nunca usado em runtime; status transition sem state machine |
| DTOs | 🔴 | Zero anotações `garde`, sem `#[derive(Validate)]`; usa `ContactDto`/`AddressDto` próprios em vez de reusar os do collaborator |
| Routes | 🔴 | Usa `web::Json` em vez de `ValidatedJson`; `error_to_response` fn manual (existe `From<ClientError> for HttpResponse` em `error_response.rs` mas não é usado); `AlreadyActive`/`AlreadyInactive` mapeados para `400` no handler manual mas para `409` no `From` (inconsistência) |
| Wired in startup | 🔴 | Comentado — `// let client_service = web::Data::new(client::build(pool.clone()));` — builder fn `client::build` não existe |
| Hurl tests | 🔴 | Arquivo não existe |

---

## 🔴 P0 — Bloqueia execução

### T01 — Registrar client service no runtime

- [ ] Criar `pg_client_serv_build(pool)` em `src/application/mod.rs`
  - Seguir o padrão de `pg_collaborator_serv_build`
  - `ConcreteClientService::new(PgClientRepository::new(pool.clone()), pool)`
- [ ] Em `src/startup.rs`:
  - Substituir `// let client_service = web::Data::new(client::build(pool.clone()));` por `let client_service = web::Data::new(pg_client_serv_build(pool.clone()));`
  - Descomentar `.app_data(client_service.clone())`
  - Adicionar `.configure(crate::adapters::driving::client_routes::configure)` no scope `/api`
- [ ] Verificar que os 6 endpoints respondem (status ≠ 404)

---

## 🔴 P1 — Conformidade com convenções do projeto

### T02 — Adicionar validação `garde` nos DTOs de client

Todos os outros contextos (collaborator, contact, location, project) usam `#[derive(Validate)]` + anotações `#[garde(...)]`. O contexto client é o único sem validação.

**Arquivo:** `src/adapters/driving/models/dtos/client_dto.rs`

- [ ] `use garde::Validate;`
- [ ] `RegisterClientRequest` → `#[derive(Debug, Deserialize, Validate)]`
  - `name`: `#[garde(length(min = 1, max = 64))]` (DB: `VARCHAR(64) NOT NULL`)
  - `document`: `#[garde(pattern(r"^\d{11}$|^\d{14}$"))]` (CPF 11 dígitos ou CNPJ 14 dígitos)
  - `contact`: `#[garde(dive)]` (requer `ContactDto` implementar `Validate`)
  - `address`: `#[garde(dive)]` (requer `AddressDto` implementar `Validate`)
- [ ] `UpdateClientRequest` → `#[derive(Debug, Deserialize, Validate)]`
  - `name`: `#[garde(inner(length(min = 1, max = 64)))]`
  - `document`: `#[garde(inner(pattern(r"^\d{11}$|^\d{14}$")))]`
  - `contact`: `#[garde(skip)]` (não usado no UpdateUseCase atualmente)
  - `address`: `#[garde(skip)]` (não usado no UpdateUseCase atualmente)
- [ ] `ContactDto` → `#[derive(Debug, Deserialize, Serialize, Clone, Validate)]`
  - `email`: `#[garde(email, length(max = 256))]`
  - `phones`: `#[garde(dive, length(min = 1))]` (requer wrapper `PhoneEntry` como no collaborator)
  - ⚠️ **Decisão:** reusar `ContactDto`/`AddressDto`/`PhoneEntry` do collaborator_dto OU manter cópia local com `Validate`. Recomendação: extrair para módulo compartilhado (ex: `shared_dto.rs`) para evitar duplicação.
- [ ] `AddressDto` → `#[derive(Debug, Deserialize, Serialize, Clone, Validate)]`
  - `cep`: `#[garde(pattern(r"^\d{8}$"))]`
  - `street`: `#[garde(length(min = 1, max = 128))]`
  - `number`: `#[garde(length(min = 1, max = 16))]`
  - `complement`: `#[garde(inner(length(max = 64)))]`
  - `neighborhood`: `#[garde(length(min = 1, max = 64))]`
  - `city`: `#[garde(length(min = 1, max = 64))]`
  - `state`: `#[garde(length(min = 2, max = 2), pattern(r"^[A-Z]{2}$"))]`
- [ ] `StatusRequest` → `#[derive(Debug, Deserialize, Validate)]`
  - `status`: `#[garde(pattern(r"^(active|inactive)$"))]`

### T03 — Trocar `web::Json` por `ValidatedJson` nos routes

**Arquivo:** `src/adapters/driving/client_routes.rs`

- [ ] `use crate::adapters::driving::utils::ValidatedJson;`
- [ ] Trocar em todos os handlers:
  - `register_client`: `body: web::Json<RegisterClientRequest>` → `ValidatedJson(body): ValidatedJson<RegisterClientRequest>`
  - `update_client`: idem para `UpdateClientRequest`
  - `update_client_status`: idem para `StatusRequest`
- [ ] Remover tratamento manual de status inválido no handler `update_client_status` (match `_ =>` — garde já rejeita via pattern)
- [ ] Adaptar acesso ao body: destructuring `ValidatedJson(body)` em todos os handlers

### T04 — Remover `error_to_response` e usar `From<ClientError> for HttpResponse`

A convenção do projeto é que cada domain error implemente `From<Error> for HttpResponse`. O contexto client já tem o `impl From<ClientError> for HttpResponse` em `error_response.rs`, porém os routes usam uma função manual `error_to_response` que tem mapeamentos inconsistentes.

**Inconsistências atuais:**
- `error_to_response`: `AlreadyActive`/`AlreadyInactive` → `400 Bad Request`
- `From<ClientError>`: `AlreadyActive`/`AlreadyInactive` → `409 Conflict`
- O padrão do projeto (ver collaborator) é `409 Conflict` para "already in status"

- [ ] Em `src/adapters/driving/client_routes.rs`:
  - Remover função `error_to_response`
  - Todos os handlers usam `Err(e) => HttpResponse::from(e)`
- [ ] Testes unitários de rota atualizados para refletir `409` em vez de `400` para `AlreadyActive`/`AlreadyInactive`
- [ ] Testes unitários existentes de `error_to_response` convertidos para testar `HttpResponse::from(err)`

### T05 — Converter phones de `Vec<String>` para `Vec<PhoneEntry>` no DTO

**Arquivo:** `src/adapters/driving/models/dtos/client_dto.rs`

O `ContactDto` do client usa `phones: Vec<String>`, sem validação garde. O collaborator usa `phones: Vec<PhoneEntry>` com wrapper validado.

- [ ] Opção A (recomendada): reusar `PhoneEntry` do collaborator_dto dentro do `ContactDto` do client
- [ ] Opção B: duplicar `PhoneEntry` no client_dto
- [ ] Atualizar extractor no `register_client` handler: `body.contact.phones.iter().map(|p| p.value.clone()).collect()`
- [ ] Garantir que o service recebe `Vec<String>` (valores puros) — a conversão `PhoneEntry → String` acontece no handler

---

## 🟡 P2 — Lógica de negócio incompleta

### T06 — Update não atualiza contact nem address

**Arquivo:** `src/application/client_service.rs` — `UpdateClientUseCase`

O DTO `UpdateClientRequest` aceita `contact: Option<ContactDto>` e `address: Option<AddressDto>`, mas o `UpdateClientInput` só tem `name` e `doc`. Os campos contact e address são ignorados silenciosamente.

- [ ] Opção A (recomendada): Remover `contact` e `address` do `UpdateClientRequest` por enquanto — update do client só altera `name` e `doc`. Contact e address seriam atualizados via seus próprios endpoints.
- [ ] Opção B: Expandir `UpdateClientInput` para incluir email, phones (via contact_service), street, city, etc. (via location_service). Requer transação cross-context.
- [ ] Documentar decisão no backlog

### T07 — Client/Project association não implementada no service

**Arquivo:** `src/domain/ports/repositories/client_repository.rs`

Existem os port traits `CreateClientProject` e `FindProjectsByClientId` no repository, mas:
- Nenhum use case trait correspondente em `client_use_cases.rs`
- Nenhum handler/rota expondo esses endpoints
- Nenhum endpoint para associar/desassociar projetos a clientes

- [ ] Criar use case traits: `AssociateProjectUseCase`, `ListClientProjectsUseCase`, `DissociateProjectUseCase`
- [ ] Implementar no `ConcreteClientService` (e no genérico `ClientService<R>`)
- [ ] Criar rotas:
  - `POST /api/clients/{uuid}/projects` — associar projeto
  - `GET /api/clients/{uuid}/projects` — listar projetos do cliente
  - `DELETE /api/clients/{uuid}/projects/{project_uuid}` — desassociar projeto
- [ ] Validar existência do client e do project antes de criar a associação
- [ ] Tratar constraint `uq_fk_client_project` como `409 Conflict`
- [ ] Adicionar testes unitários

### T08 — `ConcreteClientService` viola padrão hexagonal

**Arquivo:** `src/application/client_service.rs`

O `ConcreteClientService` referencia diretamente `PgClientRepository`, `PgContactRepository`, `PgLocationRepository` e `PgPhoneRepository` dentro do `RegisterClientUseCase`. Isso acopla a camada de aplicação a implementações concretas.

Os demais use cases (Find, Update, Delete, Activate, Deactivate) estão duplicados: implementados **tanto** no `ConcreteClientService` quanto no genérico `ClientService<R>`.

- [ ] Opção A (pragmática): manter `ConcreteClientService` para o `RegisterClientUseCase` (que precisa de transação multi-repo) e usar `ClientService<R>` para os demais. Remover duplicação.
- [ ] Opção B (ideal): introduzir ports de transação (`UnitOfWork` ou `TransactionManager`) para permitir transação via traits genéricos
- [ ] Remover código duplicado — os 6 use cases estão implementados 2× cada

### T09 — `AlreadyActive`/`AlreadyInactive` deveria ser `409` (não `400`)

**Arquivo:** `src/adapters/driving/client_routes.rs` — `error_to_response`

O padrão do projeto (collaborator: `AlreadyActive`/`AlreadyInactive` → `409`) difere do client (→ `400`). O `From<ClientError> for HttpResponse` em `error_response.rs` já mapeia para `409` corretamente.

- [ ] Resolvido automaticamente ao implementar T04 (remover `error_to_response` e usar `From`)

### T10 — `tx_doc` constraint UNIQUE `uq_tx_doc` — update pode violar unicidade

**Arquivo:** `src/application/client_service.rs` — `UpdateClientUseCase`

Se o usuário passa um `doc` que já está registrado para outro client, o DB retorna unique violation → `500` opaco.

- [ ] Verificar se `doc` já existe (findByDocument) antes de fazer update, retornando `ClientError::DocumentAlreadyExists`
- [ ] Ou interceptar `sqlx::Error::Database` com código `23505` no repository e mapear para `DocumentAlreadyExists`
- [ ] Adicionar teste unitário

---

## 🟢 P3 — Testes de integração

### T11 — Criar e salvar `hurl/clients.hurl`

Os testes hurl devem depender dos contextos **location** e **contact** (que já estão wired e funcionando). O client context depende dessas entidades para o `RegisterClient` (que cria location e contact na transação).

#### Pré-requisitos

- T01 (wire startup) deve estar implementado — caso contrário, todos os endpoints retornam 404
- T02 + T03 (garde/ValidatedJson) implementados — caso contrário, os status codes de validação serão diferentes (o body inválido pode não ser rejeitado)
- T04 implementado — caso contrário, `AlreadyActive`/`AlreadyInactive` retornam `400` em vez de `409`

#### Endpoints a testar (6 rotas)

| # | Método | Rota | Descrição |
|---|--------|------|-----------|
| 1 | `POST` | `/api/clients` | Registrar client |
| 2 | `GET` | `/api/clients` | Listar clients |
| 3 | `GET` | `/api/clients/{uuid}` | Buscar client por ID |
| 4 | `PUT` | `/api/clients/{uuid}` | Atualizar client |
| 5 | `DELETE` | `/api/clients/{uuid}` | Remover client |
| 6 | `PUT` | `/api/clients/{uuid}/status` | Alterar status (activate/deactivate) |

#### Fluxo do arquivo hurl

Seguir o padrão do `hurl/collaborators.hurl`: happy path primeiro, depois testes de erro de cada endpoint, e cleanup ao final.

```
# ═══════════════════════════════════════════════════════════════════
# SETUP: Nenhum — RegisterClient cria location, contact e client
#        na mesma transação
# ═══════════════════════════════════════════════════════════════════

# ─── HAPPY PATH ────────────────────────────────────────────────────

# 1. POST /api/clients — 201 Created (PF com CPF)
#    Capture: client_id
#    Assert: id isString, name, status="active", document, created_at, updated_at

# 2. GET /api/clients — 200 OK (lista)
#    Assert: collection, pelo menos 1 item com os campos esperados

# 3. GET /api/clients/{client_id} — 200 OK
#    Assert: id == client_id, name, status, document

# 4. PUT /api/clients/{client_id} — 200 OK (atualizar nome)
#    Body: { "name": "Updated Name", "document": null }
#    Assert: name == "Updated Name"

# 5. PUT /api/clients/{client_id}/status — 200 OK (deactivate)
#    Body: { "status": "inactive" }
#    Assert: status == "inactive"

# 6. PUT /api/clients/{client_id}/status — 200 OK (activate)
#    Body: { "status": "active" }
#    Assert: status == "active"

# ─── CASOS DE ERRO ─────────────────────────────────────────────────

# 7. POST /api/clients — 400 (name vazio, garde validation)
#    Body: { "name": "", "document": "52998224725", "contact": {...}, "address": {...} }
#    Assert: error == "validation_error", message isString

# 8. POST /api/clients — 400 (document formato inválido, garde)
#    Body: { "name": "X", "document": "123", "contact": {...}, "address": {...} }
#    Assert: error == "validation_error", message isString

# 9. POST /api/clients — 422 (CPF dígitos verificadores inválidos, value object)
#    Body: { "name": "X", "document": "12345678900", "contact": {...}, "address": {...} }
#    Assert: error == "validation_error", message isString

# 10. POST /api/clients — 400 (email inválido, garde)
#     Body: { "name": "X", "document": "52998224725", "contact": {"email":"nao-eh-email","phones":[...]}, "address": {...} }
#     Assert: error == "validation_error", message isString

# 11. POST /api/clients — 400 (cep inválido, garde)
#     Body: { "name": "X", "document": "52998224725", "contact": {...}, "address": {"cep":"123",...} }
#     Assert: error == "validation_error", message isString

# 12. POST /api/clients — 409 (document já existe, conflict)
#     Body: mesmos dados do registro #1 (mesmo doc)
#     Assert: error == "conflict", message isString

# 13. GET /api/clients/{uuid_inexistente} — 404
#     Assert: error == "not_found", message isString

# 14. PUT /api/clients/{uuid_inexistente} — 404
#     Body: { "name": "Ghost" }
#     Assert: error == "not_found", message isString

# 15. PUT /api/clients/{client_id}/status — 409 (já ativo)
#     Body: { "status": "active" }   (client está active após #6)
#     Assert: error == "conflict", message isString

# 16. PUT /api/clients/{client_id}/status — 400 (status inválido, garde)
#     Body: { "status": "suspended" }
#     Assert: error == "validation_error", message isString

# 17. PUT /api/clients/{client_id}/status — 404 (uuid inexistente)
#     Body: { "status": "active" }
#     Assert: error == "not_found", message isString

# 18. DELETE /api/clients/{uuid_inexistente} — 404
#     Assert: error == "not_found", message isString

# ─── CLEANUP ───────────────────────────────────────────────────────

# 19. DELETE /api/clients/{client_id} — 200 OK
#     Assert: id == client_id

# 20. DELETE /api/clients/{client_id} — 404 (já deletado)
#     Assert: error == "not_found"
```

#### Dados de teste sugeridos

```json
{
  "name": "Maria da Silva",
  "document": "52998224725",
  "contact": {
    "email": "maria.silva@example.com",
    "phones": ["+5561999990101"]
  },
  "address": {
    "cep": "70040010",
    "street": "Esplanada dos Ministérios",
    "number": "100",
    "complement": "Bloco A",
    "neighborhood": "Plano Piloto",
    "city": "Brasília",
    "state": "DF"
  }
}
```

> **Nota:** O campo `phones` pode precisar ser `[{ "value": "+5561999990101" }]` se T05 for implementado (wrapper `PhoneEntry`). Caso contrário, permanece `["+5561999990101"]`.

#### Comando de execução

```bash
hurl --test hurl/clients.hurl
```

> **Nota:** T11 deve ser a última task pois os testes hurl validam o comportamento final de todos os endpoints. Se features como `From<ClientError>` (T04) ou garde (T02) forem implementadas depois, os status codes esperados no hurl mudarão.

---

## Ordem de execução recomendada

```
T01 (wire startup)
 ↓
T02 (garde DTOs) + T03 (ValidatedJson) + T05 (PhoneEntry) ← podem ser feitas juntas
 ↓
T04 (remover error_to_response, usar From)  →  T09 (resolvido automaticamente)
 ↓
T06 (update contact/address → decisão)
 ↓
T08 (refactor duplicação ConcreteClientService vs ClientService<R>)
 ↓
T10 (unique doc no update)
 ↓
T07 (client/project association endpoints)
 ↓
T11 (hurl tests) ← só depois que tudo acima estiver implementado
```

---

## Referências

| Arquivo | Descrição |
|---------|-----------|
| `src/adapters/driven/pg_client_repository.rs` | Repository PostgreSQL (6 port impls + `create_contact`/`create_address` estáticos) |
| `src/adapters/driving/client_routes.rs` | Handlers HTTP (6 endpoints) + `error_to_response` manual |
| `src/adapters/driving/models/dtos/client_dto.rs` | Request/Response DTOs (sem garde) |
| `src/application/client_service.rs` | `ConcreteClientService` (7 use cases) + `ClientService<R>` genérico (6 use cases duplicados) |
| `src/domain/errors/client_error.rs` | Error enum (14 variantes incluindo VO errors) |
| `src/domain/models/db/client_row.rs` | DB row struct + `ClientStatus` enum |
| `src/domain/models/db/client_contact_row.rs` | Associação client ↔ contact |
| `src/domain/models/db/client_address_row.rs` | Associação client ↔ location |
| `src/domain/models/db/client_project_row.rs` | Associação client ↔ project |
| `src/domain/models/db/client_projects_row.rs` | Models do contexto project (vivem no namespace errado — são models de project, não de client) |
| `src/domain/ports/use_cases/client_use_cases.rs` | 8 use case traits |
| `src/domain/ports/repositories/client_repository.rs` | Repository traits (6 base + 2 cross-context: `CreateClientProject`, `FindProjectsByClientId`) |
| `src/domain/value_objects/doc.rs` | Value object CPF/CNPJ |
| `src/adapters/driving/errors/error_response.rs` | `impl From<ClientError> for HttpResponse` (já existe, não está sendo usado) |
| `migrations/20260222180343_create_clients_schema.sql` | 4 tabelas: `tb_client`, `tb_client_contact`, `tb_client_address`, `tb_client_project` |
| `src/application/mod.rs` | Builder functions (falta `pg_client_serv_build`) |
| `src/startup.rs` | Wiring (client comentado) |
