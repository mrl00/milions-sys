# milions-sys — Backlog Client

## Estado Atual (2026-04-27)

O contexto **client** está funcional em runtime com 10 endpoints, 32 testes unitários (16 service + 16 routes), e um arquivo hurl cobrindo os 6 endpoints base. As tasks fundamentais (wiring, garde, ValidatedJson, register handler, error handling) foram todas concluídas. O foco agora é em lacunas de cobertura (hurl dos 4 endpoints novos, testes de already-active/already-inactive, unique doc no update) e evolução (client/project association).

### Estado por Camada

| Camada | Status | Situação |
|--------|--------|----------|
| DB models / rows | ✅ | — |
| Repository (PG) | ✅ | 10 port impls incluindo `FindContactByClientId`, `FindLocationByClientId` |
| Use case ports | ✅ | 12 traits definidos, todos implementados no `PgClientService` |
| Service impl | ✅ | `PgClientService` com `location_service` e `contact_service` injetados; `ClientService<R>` genérico para testes unitários |
| DTOs | ✅ | Garde em todos (Register, Update, Status, Email, Phones, Location) com `PhoneEntry` wrapper |
| Routes | ✅ | 10 endpoints usando `ValidatedJson` + `HttpResponse::from(e)` |
| Wired in startup | ✅ | `pg_client_serv_build(pool, location_service, contact_service)` |
| Hurl tests | ⚠️ | 6 endpoints base cobertos (17 cenários); 4 endpoints novos de contact/location **sem cobertura** |

---

## ✅ Concluído

### T01 — Registrar client service no runtime ✅
### T01b — Implementar handler `register_client` ✅
### T02 — Adicionar validação `garde` nos DTOs ✅
### T03 — Trocar `web::Json` por `ValidatedJson` ✅
### T04 — Remover `error_to_response`, usar `From<ClientError> for HttpResponse` ✅
### T04b — Corrigir testes unitários `AlreadyActive`/`AlreadyInactive` → `409` ✅
### T05 — Implementar use cases de contact e location ✅

Todos os 4 novos use cases implementados e roteados:
- `PATCH /api/clients/{uuid}/contact/email` → `UpdateClientEmailUseCase`
- `POST /api/clients/{uuid}/contact/phones` → `AddClientPhonesUseCase`
- `PUT /api/clients/{uuid}/contact/phones/{phone}` → `UpdateClientPhoneUseCase`
- `PUT /api/clients/{uuid}/address` → `UpdateClientLocationUseCase`

### T06 — Mapear `ContactNotFound`, `LocationNotFound`, `PhoneNotFound` para 404 ✅

Novos variants adicionados ao `ClientError`:
- `ContactNotFound { client_uuid }` → 404
- `LocationNotFound { client_uuid }` → 404
- `PhoneNotFound { phone, contact_uuid }` → 404

### T08 — Refator `ConcreteClientService` → `PgClientService` ✅
### T09 — `AlreadyActive`/`AlreadyInactive` → `409 Conflict` ✅
### T10 — Verificação de doc duplicado no `RegisterClientUseCase` ✅
### T11 — Criar `hurl/clients.hurl` (parcial) ✅

Arquivo criado com 17 cenários cobrindo os 6 endpoints base:
- POST register (happy path + 5 validation errors + document conflict)
- GET list, GET by id, GET not found
- PUT update (happy path + not found)
- PUT status (deactivate, activate, invalid status)
- DELETE (happy path + not found)

---

## 🔴 P1 — Lacunas de cobertura

### T11b — Expandir `hurl/clients.hurl` com cenários dos endpoints de contact/location

Os 4 endpoints novos não têm cobertura hurl.

**Cenários a adicionar:**

```
# ─── PATCH /api/clients/{uuid}/contact/email ────────────────────

# Happy path: atualizar email — 200
# Not found: client inexistente — 404
# Not found: client sem contact associado — 404
# Validation: email inválido (garde) — 400

# ─── POST /api/clients/{uuid}/contact/phones ────────────────────

# Happy path: adicionar phone — 200
# Not found: client inexistente — 404
# Not found: client sem contact — 404
# Validation: phone inválido (garde) — 400

# ─── PUT /api/clients/{uuid}/contact/phones/{phone} ─────────────

# Happy path: atualizar phone — 200
# Not found: client inexistente — 404
# Not found: phone inexistente — 404
# Validation: new_phone inválido (garde) — 400

# ─── PUT /api/clients/{uuid}/address ────────────────────────────

# Happy path: atualizar endereço — 200
# Not found: client inexistente — 404
# Not found: client sem address — 404
```

**Nota sobre ordem:** esses cenários devem ser inseridos **antes** do DELETE (cleanup), pois o DELETE remove o client. O fluxo deve ser:

```
1. POST register (happy path) → captura client_id
2. Validation errors do POST
3. Document conflict
4. GET list, GET by id, GET not found
5. PUT update + not found
6. PUT status (deactivate, activate, invalid)
7. ──── novos cenários de contact/location ────
8.   PATCH email (happy + errors)
9.   POST phones (happy + errors)
10.  PUT phone (happy + errors)
11.  PUT address (happy + errors)
12. ──── fim ────
13. DELETE (happy + not found)
```

### T11c — Adicionar cenários faltantes de erro nos 6 endpoints base

O hurl atual não cobre:

- [ ] `PUT /api/clients/{uuid}/status` — `409 AlreadyActive` (client já active, tentar activate de novo)
- [ ] `PUT /api/clients/{uuid}/status` — `409 AlreadyInactive` (client já inactive, tentar deactivate de novo)
- [ ] `PUT /api/clients/{uuid}/status` — `404` (uuid inexistente)
- [ ] `POST /api/clients` — `400` name vazio (garde: `length(min=1)`)
- [ ] `DELETE /api/clients/{client_id}` — `404` (segundo DELETE, idempotência — já deletado)

---

## 🟡 P2 — Lógica de negócio pendente

### T06b — `Location(LocationError)` e `Contact(ContactError)` mapeados genericamente

**Arquivo:** `src/adapters/driving/errors/error_response.rs`

Os variants wrapping `Location(LocationError)` e `Contact(ContactError)` têm mapeamentos genéricos:
- `Location(_)` → `500 internal_error "internal server error"` (deveria propagar status do `LocationError`)
- `Contact(_)` → `400 bad_request "contact error"` (mensagem opaca, deveria propagar do `ContactError`)

- [ ] `Location(e)` → delegar para `HttpResponse::from(e)` (re-propagar `From<LocationError>`)
- [ ] `Contact(e)` → delegar para `HttpResponse::from(e)` (re-propagar `From<ContactError>`)
- [ ] Adicionar testes unitários para esses mapeamentos

### T07 — Client/Project association não implementada

**Arquivo:** `src/domain/ports/repositories/client_repository.rs`

Os port traits `CreateClientProject` e `FindProjectsByClientId` existem no repository, mas:
- Nenhum use case trait correspondente
- Nenhum handler/rota

- [ ] Criar use case traits: `AssociateClientProjectUseCase`, `ListClientProjectsUseCase`, `DissociateClientProjectUseCase`
- [ ] Implementar no `PgClientService`
- [ ] Criar rotas:
  - `POST /api/clients/{uuid}/projects` — associar projeto
  - `GET /api/clients/{uuid}/projects` — listar projetos do cliente
  - `DELETE /api/clients/{uuid}/projects/{project_uuid}` — desassociar
- [ ] Validar existência do client e do project
- [ ] Tratar constraint `uq_fk_client_project` como `409 Conflict`
- [ ] Adicionar testes unitários + hurl

### T08b — Remover duplicação `ClientService<R>` vs `PgClientService`

**Arquivo:** `src/application/client_service.rs`

Os 6 use cases básicos (Find, List, Update, Activate, Deactivate, Delete) estão implementados **2×**: no `PgClientService` (L128–L242) e no `ClientService<R>` (L349–L464). O genérico é usado apenas nos testes unitários.

- [ ] Remover duplicação e manter apenas o necessário

### T10b — `tx_doc` constraint UNIQUE — **update** pode violar unicidade

**Arquivo:** `src/application/client_service.rs` — `UpdateClientUseCase`

O `RegisterClientUseCase` já verifica doc duplicado ✅, mas o `UpdateClientUseCase` não. Se o usuário faz PUT com um `doc` já registrado para outro client, o DB retorna unique violation → `500` opaco.

- [ ] No `UpdateClientUseCase`: se `input.doc.is_some()`, verificar com `find_by_document` se já existe para outro client, retornando `ClientError::DocumentAlreadyExists`
- [ ] Ou interceptar `sqlx::Error::Database` código `23505` no repository
- [ ] Adicionar teste unitário
- [ ] Adicionar cenário hurl: PUT com doc existente → 409

### T12 — Resolver `NotImplemented` variant em `ClientError`

**Arquivo:** `src/domain/errors/client_error.rs`

O variant `NotImplemented` existe (~L10) e é mapeado para `500`. Se não é mais usado em nenhum lugar, remover. Se ainda é referenciado, verificar onde e resolver.

- [ ] `grep -r "NotImplemented" src/` → verificar uso
- [ ] Remover se não usado, ou implementar o que falta

---

## Ordem de execução recomendada

```
T11b + T11c (completar cobertura hurl) — podem ser feitos imediatamente
 ↓
T06b (propagar Location/Contact errors)
 ↓
T10b (unique doc no update)
 ↓
T12 (RemoveNotImplemented)
 ↓
T08b (remover duplicação genérico)
 ↓
T07 (client/project association)
```

---

## Referências

| Arquivo | Descrição |
|---------|-----------|
| `src/adapters/driven/pg_client_repository.rs` | Repository PG — 10 port impls |
| `src/adapters/driving/client_routes.rs` | 10 endpoints, todos com `ValidatedJson` + `HttpResponse::from(e)` |
| `src/adapters/driving/models/dtos/client_dto.rs` | DTOs com garde: `RegisterClientRequest`, `UpdateClientRequest`, `ClientStatusRequest`, `UpdateEmailRequest`, `AddPhonesRequest`, `UpdatePhoneRequest`, `UpdateClientLocationRequest` |
| `src/adapters/driving/errors/error_response.rs` | `From<ClientError>` — 12 variants mapeados |
| `src/application/client_service.rs` | `PgClientService` (12 use cases) + `ClientService<R>` (6 duplicados) — 16 testes unitários |
| `src/application/mod.rs` | `pg_client_serv_build(pool, location_service, contact_service)` |
| `src/domain/errors/client_error.rs` | 12 variants: `NotImplemented`, `AlreadyExists`, `NotFound`, `AlreadyActive`, `AlreadyInactive`, `DocumentAlreadyExists`, `ContactNotFound`, `LocationNotFound`, `PhoneNotFound`, `InvalidDocument`, `Location`, `Contact`, `ViaCep`, `Infra` |
| `src/domain/ports/use_cases/client_use_cases.rs` | 12 use case traits |
| `src/domain/ports/repositories/client_repository.rs` | 12 traits: 6 base + `LinkCreatedLocation/Contact` + `FindContact/LocationByClientId` + `CreateClientProject` + `FindProjectsByClientId` |
| `hurl/clients.hurl` | 17 cenários cobrindo 6 endpoints base |
| `src/startup.rs` | `PgClientService` wired |
| `migrations/20260222180343_create_clients_schema.sql` | 4 tabelas |
