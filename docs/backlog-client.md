# milions-sys — Backlog Client

## Estado Atual (2026-04-27)

O contexto **client** está funcional em runtime com a arquitetura refatorada: `ClientService<R, L, C>` genérico parametrizado por repositório, location service e contact service. O type alias `PgClientService = ClientService<PgClientRepository, PgLocationService, PgContactService>` é usado nas rotas e no startup.

Todos os 11 endpoints estão wired e respondendo. Validação garde está implementada em todos os DTOs. Handlers usam `ValidatedJson`. 32 testes passando (16 service + 16 routes).

### Estado por Camada

| Camada | Status | Situação |
|--------|--------|----------|
| DB models / rows | ✅ | — |
| Repository (PG) | ✅ | 10 port impls incluindo `LinkCreated*`, `FindContactByClientId`, `FindLocationByClientId` |
| Use case ports | ✅ | 12 use case traits (CRUD + Register + contact/location compostos) |
| Service impl | ✅ | `ClientService<R, L, C>` — single source of truth, zero duplicação |
| DTOs | ✅ | Garde em todos os DTOs (`RegisterClientRequest`, `UpdateClientRequest`, `ContactDto`, `AddressDto`, `ClientStatusRequest`, etc.) |
| Routes | ✅ | 11 endpoints com `ValidatedJson`; `HttpResponse::from(e)` para erros |
| Error mapping | ⚠️ | `Location(_)` → `500 internal_error` genérico; `Contact(_)` → `400 bad_request "contact error"` genérico |
| Wired in startup | ✅ | `pg_client_serv_build(pool, location_service, contact_service)` |
| Hurl tests | 🔴 | Arquivo não existe |

---

## ✅ Concluído

### T01 — Registrar client service no runtime ✅
- `pg_client_serv_build(pool, location_service, contact_service)` em `application/mod.rs`
- `PgClientService` registrado em `startup.rs` com `app_data`
- `client_routes::configure` adicionado ao scope `/api`

### T01b — Implementar handler `register_client` ✅
- Handler implementado com `ValidatedJson<RegisterClientRequest>`
- Constrói `RegisterClientInput` a partir do body DTO com `location: Option` e `contact: Option`

### T02 — Validação `garde` nos DTOs ✅
- Todos os DTOs com `#[derive(Validate)]` e anotações garde:
  - `RegisterClientRequest`: name, document (pattern CPF/CNPJ), contact (dive), address (dive)
  - `UpdateClientRequest`: name (inner), document (inner), contact/address (skip)
  - `ContactDto`: email (email), phones (dive com `PhoneEntry`)
  - `AddressDto`: cep (pattern), street, number, complement, neighborhood, city, state
  - `ClientStatusRequest`: status (pattern active|inactive)
  - DTOs adicionais: `UpdateEmailRequest`, `AddPhonesRequest`, `UpdatePhoneRequest`, `UpdateClientLocationRequest`

### T03 — Trocar `web::Json` por `ValidatedJson` ✅
- Todos os handlers de mutação usam `ValidatedJson`

### T04 — Remover `error_to_response` e usar `From<ClientError>` ✅
- Todos os handlers usam `HttpResponse::from(e)`

### T04b — Corrigir testes: `AlreadyActive`/`AlreadyInactive` → `409` ✅
- Testes verificam `assert_eq!(resp.status(), 409)` corretamente

### T05 — Implementar use cases de contact e location para o client ✅
- `UpdateClientEmailUseCase`, `UpdateClientPhoneUseCase`, `AddClientPhonesUseCase`, `UpdateClientLocationUseCase` implementados
- 4 rotas novas: `PATCH .../contact/email`, `POST .../contact/phones`, `PUT .../contact/phones/{phone}`, `PUT .../address`

### T08b — Refatorar arquitetura (Opção A) ✅
- `ClientService<R, L, C>` único struct genérico
- `PgClientService` é type alias
- Use cases CRUD: bounds `R: ClientRepository, L: Send + Sync, C: Send + Sync`
- Use cases compostos: bounds específicos por trait (`L: CreateLocationUseCase`, `C: RegisterContactUseCase + AddPhonesUseCase`, etc.)
- Testes usam `struct NoOp;` para L e C com helper `make_service(repo)`
- Zero duplicação de lógica

### T09 — `AlreadyActive`/`AlreadyInactive` → `409` ✅
- Resolvido com T04 — `From<ClientError>` mapeia para `409 Conflict`

### T10 — Unique doc no update ✅
- `UpdateClientUseCase` verifica `find_by_document` antes de fazer update
- Retorna `ClientError::DocumentAlreadyExists` se doc já registrado

---

## 🟡 P1 — Erros de contextos cruzados mapeados genericamente

### T06 — `Contact(ContactError)` e `Location(LocationError)` mapeados como `500`/`400` genéricos

**Arquivo:** `src/adapters/driving/errors/error_response.rs`

Os variants `Contact(_)` e `Location(_)` no `ClientError` são mapeados com mensagens genéricas:
- `Location(_)` → `500 internal_error "internal server error"`
- `Contact(_)` → `400 bad_request "contact error"`

Isso perde o detalhe do erro original (ex: "email already registered" vira "contact error").

- [ ] `Location(e)` → fazer match detalhado nos sub-variants de `LocationError` ou re-propagar com `HttpResponse::from(LocationError)`
- [ ] `Contact(e)` → idem para `ContactError`
- [ ] Garantir que erros como `ContactError::EmailAlreadyExists` retornem `409` (não `400` genérico)
- [ ] Garantir que `ContactError::PhoneAlreadyExists` retorne `409`
- [ ] Adicionar testes unitários para esses mapeamentos

### T06b — Remover variante `NotImplemented` do `ClientError`

**Arquivo:** `src/domain/errors/client_error.rs`

A variante `NotImplemented` é um placeholder que não deveria existir em produção.

- [ ] Remover `NotImplemented` de `ClientError`
- [ ] Remover o mapeamento correspondente em `error_response.rs`
- [ ] Verificar que não há uso em runtime

---

## 🟡 P2 — Features pendentes

### T07 — Client/Project association não implementada no service

**Arquivo:** `src/domain/ports/repositories/client_repository.rs`

Existem os port traits `CreateClientProject` e `FindProjectsByClientId` no repository, mas:
- Nenhum use case trait correspondente em `client_use_cases.rs`
- Nenhum handler/rota expondo esses endpoints

- [ ] Criar use case traits: `AssociateClientProjectUseCase`, `ListClientProjectsUseCase`, `DissociateClientProjectUseCase`
- [ ] Implementar em `ClientService<R, L, C>` (bounds: `R: ClientRepository, L: Send + Sync, C: Send + Sync`)
- [ ] Criar rotas:
  - `POST /api/clients/{uuid}/projects` — associar projeto
  - `GET /api/clients/{uuid}/projects` — listar projetos do cliente
  - `DELETE /api/clients/{uuid}/projects/{project_uuid}` — desassociar
- [ ] Validar existência do client e do project antes de associar
- [ ] Tratar constraint `uq_fk_client_project` como `409 Conflict`
- [ ] Adicionar variante `ProjectAlreadyAssociated` no `ClientError`

---

## 🟢 P3 — Testes de integração

### T11 — Criar `hurl/clients.hurl`

#### Pré-requisitos

Todos implementados ✅. O hurl pode ser criado agora.

#### Endpoints a testar (11 rotas)

| # | Método | Rota | Descrição |
|---|--------|------|-----------|
| 1 | `POST` | `/api/clients` | Registrar client |
| 2 | `GET` | `/api/clients` | Listar clients |
| 3 | `GET` | `/api/clients/{uuid}` | Buscar por ID |
| 4 | `PUT` | `/api/clients/{uuid}` | Atualizar name/doc |
| 5 | `DELETE` | `/api/clients/{uuid}` | Remover client |
| 6 | `PUT` | `/api/clients/{uuid}/status` | Alterar status |
| 7 | `PATCH` | `/api/clients/{uuid}/contact/email` | Atualizar email |
| 8 | `POST` | `/api/clients/{uuid}/contact/phones` | Adicionar phones |
| 9 | `PUT` | `/api/clients/{uuid}/contact/phones/{phone}` | Atualizar phone |
| 10 | `PUT` | `/api/clients/{uuid}/address` | Atualizar endereço |

#### Fluxo do arquivo hurl

```
# ═══════════════════════════════════════════════════════════════════
# SETUP: Nenhum — RegisterClient cria location, contact e client
#        via composição (PgLocationService + PgContactService)
# ═══════════════════════════════════════════════════════════════════

# ─── HAPPY PATH ────────────────────────────────────────────────────

# 1. POST /api/clients — 201 Created (PF com CPF)
#    Body: { name, document, contact: { email, phones: [{ value }] }, address: { cep, ... } }
#    Capture: client_id
#    Assert: id isString, name, status=="active", document, created_at, updated_at

# 2. GET /api/clients — 200 OK
#    Assert: isCollection, $[0].id isString

# 3. GET /api/clients/{client_id} — 200 OK
#    Assert: id == client_id, name, status == "active", document

# 4. PUT /api/clients/{client_id} — 200 OK
#    Body: { "name": "Nome Atualizado" }
#    Assert: name == "Nome Atualizado"

# 5. PUT /api/clients/{client_id}/status — 200 (deactivate)
#    Body: { "status": "inactive" }
#    Assert: status == "inactive"

# 6. PUT /api/clients/{client_id}/status — 200 (activate)
#    Body: { "status": "active" }
#    Assert: status == "active"

# 7. PATCH /api/clients/{client_id}/contact/email — 200
#    Body: { "email": "novo@example.com" }
#    Assert: 200

# 8. POST /api/clients/{client_id}/contact/phones — 200
#    Body: { "phones": [{ "phone": "+5511988887777" }] }
#    Assert: 200

# 9. PUT /api/clients/{client_id}/contact/phones/%2B5511988887777 — 200
#    Body: { "new_phone": "+5511966665555" }
#    Assert: 200

# 10. PUT /api/clients/{client_id}/address — 200
#     Body: { "cep": "01310100", "street": "Av Paulista", ... }
#     Assert: 200

# ─── ERROS: POST /api/clients ──────────────────────────────────────

# 11. POST — 400 name vazio (garde)
#     Assert: error == "validation_error"

# 12. POST — 400 document formato inválido "123" (garde)
#     Assert: error == "validation_error"

# 13. POST — 422 CPF com dígitos verificadores inválidos "12345678900" (value object)
#     Assert: error == "validation_error"

# 14. POST — 400 email inválido (garde)
#     Assert: error == "validation_error"

# 15. POST — 400 cep formato inválido (garde)
#     Assert: error == "validation_error"

# 16. POST — 409 document já existe (conflict)
#     Body: mesmos dados do #1
#     Assert: error == "conflict"

# ─── ERROS: GET/PUT/DELETE ─────────────────────────────────────────

# 17. GET /api/clients/00000000-0000-7000-0000-000000000000 — 404
#     Assert: error == "not_found"

# 18. PUT /api/clients/00000000-0000-7000-0000-000000000000 — 404
#     Body: { "name": "Ghost" }
#     Assert: error == "not_found"

# 19. PUT /api/clients/{client_id}/status — 409 (já active)
#     Body: { "status": "active" }
#     Assert: error == "conflict"

# 20. PUT /api/clients/{client_id}/status — 400 status inválido (garde)
#     Body: { "status": "suspended" }
#     Assert: error == "validation_error"

# 21. PUT /api/clients/00000000-0000-7000-0000-000000000000/status — 404
#     Body: { "status": "active" }
#     Assert: error == "not_found"

# 22. DELETE /api/clients/00000000-0000-7000-0000-000000000000 — 404
#     Assert: error == "not_found"

# ─── ERROS: contact/location ──────────────────────────────────────

# 23. PATCH /api/clients/{client_id}/contact/email — 400 email inválido (garde)
#     Body: { "email": "nao-eh-email" }
#     Assert: error == "validation_error"

# 24. POST /api/clients/{client_id}/contact/phones — 400 phone inválido (garde)
#     Body: { "phones": [{ "phone": "123" }] }
#     Assert: error == "validation_error"

# ─── CLEANUP ───────────────────────────────────────────────────────

# 25. DELETE /api/clients/{client_id} — 200
#     Assert: id == client_id

# 26. DELETE /api/clients/{client_id} — 404 (idempotência)
#     Assert: error == "not_found"
```

#### Dados de teste sugeridos

```json
{
  "name": "Maria da Silva",
  "document": "52998224725",
  "contact": {
    "email": "maria.silva@example.com",
    "phones": [{ "value": "+5561999990101" }]
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

#### Comando de execução

```bash
hurl --test hurl/clients.hurl
```

---

## Ordem de execução recomendada

```
T06 + T06b (error mapping — pode ser feito a qualquer momento)
 ↓
T07 (client/project association — nova feature)
 ↓
T11 (hurl tests — pode começar agora para os 10 endpoints existentes)
```

---

## Referências

| Arquivo | Descrição |
|---------|-----------|
| `src/application/client_service.rs` | `ClientService<R, L, C>` genérico + `PgClientService` type alias; 12 use cases |
| `src/application/mod.rs` | `pg_client_serv_build(pool, location_service, contact_service)` |
| `src/adapters/driven/pg_client_repository.rs` | Repository PG — 10 port impls |
| `src/adapters/driving/client_routes.rs` | 11 endpoints com `ValidatedJson` |
| `src/adapters/driving/models/dtos/client_dto.rs` | 9 DTOs com garde |
| `src/adapters/driving/errors/error_response.rs` | `From<ClientError>` — `Contact`/`Location` mapeados genericamente ⚠️ |
| `src/domain/errors/client_error.rs` | 12 variantes |
| `src/domain/ports/use_cases/client_use_cases.rs` | 12 use case traits |
| `src/domain/ports/repositories/client_repository.rs` | 10 ports + super trait `ClientRepository` |
| `src/startup.rs` | `PgClientService` wired |
| `migrations/20260222180343_create_clients_schema.sql` | 4 tabelas |
