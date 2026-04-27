# milions-sys — Backlog Client

## Estado Atual (2026-04-27)

O contexto **client** está funcional em runtime com a arquitetura `ClientService<R, L, C>` genérico parametrizado por repositório, location service e contact service. O type alias `PgClientService = ClientService<PgClientRepository, PgLocationService, PgContactService>` é usado nas rotas e no startup.

Todos os **14 endpoints** estão wired e respondendo. Validação garde em todos os DTOs. Handlers usam `ValidatedJson`. **35 testes passando** (16 service + 19 routes).

### Estado por Camada

| Camada | Status | Situação |
|--------|--------|----------|
| DB models / rows | ✅ | `ClientRow`, `ClientProjectRow`, `ClientAddressRow`, `ClientContactRow` |
| Repository (PG) | ✅ | 13 port impls incluindo `CreateClientProject`, `FindProjectsByClientId`, `DeleteClientProject` |
| Use case ports | ✅ | 15 use case traits (CRUD + Register + contact/location compostos + project association) |
| Service impl | ✅ | `ClientService<R, L, C>` — single source of truth, zero duplicação |
| DTOs | ✅ | Garde em todos os DTOs |
| Routes | ✅ | 14 endpoints com `ValidatedJson`; `HttpResponse::from(e)` para erros |
| Error mapping | ✅ | `Location(e)` / `Contact(e)` delegam para `From` correto; `ProjectAlreadyAssociated` → 409; `ProjectNotAssociated` → 404 |
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
- Todos os DTOs com `#[derive(Validate)]` e anotações garde

### T03 — Trocar `web::Json` por `ValidatedJson` ✅
- Todos os handlers de mutação usam `ValidatedJson`

### T04 — Remover `error_to_response` e usar `From<ClientError>` ✅
- Todos os handlers usam `HttpResponse::from(e)`

### T04b — Corrigir testes: `AlreadyActive`/`AlreadyInactive` → `409` ✅

### T05 — Implementar use cases de contact e location para o client ✅
- `UpdateClientEmailUseCase`, `UpdateClientPhoneUseCase`, `AddClientPhonesUseCase`, `UpdateClientLocationUseCase`
- 4 rotas: `PATCH .../contact/email`, `POST .../contact/phones`, `PUT .../contact/phones/{phone}`, `PUT .../address`

### T06 — Corrigir error mapping de `Contact`/`Location` ✅
- `Location(e) => HttpResponse::from(e)` — delega para `From<LocationError>`
- `Contact(e) => HttpResponse::from(e)` — delega para `From<ContactError>`
- Erros como `ContactError::EmailAlreadyExists` agora retornam `409` corretamente

### T07 — Client/Project association ✅
- 3 use case traits: `AssociateClientProjectUseCase`, `ListClientProjectsUseCase`, `DissociateClientProjectUseCase`
- 3 repo port traits: `CreateClientProject`, `FindProjectsByClientId`, `DeleteClientProject`
- Implementados em `ClientService<R, L, C>` (bounds: `R: ClientRepository, L/C: Send + Sync`)
- PG repo: `CreateClientProject` trata constraint `23505` → `ProjectAlreadyAssociated`; `DeleteClientProject` usa `fetch_optional` → `ProjectNotAssociated`
- 2 error variants: `ProjectAlreadyAssociated` (409), `ProjectNotAssociated` (404)
- 3 endpoints: `POST /api/clients/{uuid}/projects`, `GET /api/clients/{uuid}/projects`, `DELETE /api/clients/{uuid}/projects/{project_uuid}`
- `AssociateProjectRequest` DTO com garde, `ClientProjectResponse` DTO
- 3 route tests

### T08b — Refatorar arquitetura (Opção A) ✅
- `ClientService<R, L, C>` único struct genérico
- `PgClientService` é type alias
- Use cases CRUD: bounds `R: ClientRepository, L: Send + Sync, C: Send + Sync`
- Use cases compostos: bounds específicos por trait
- Testes usam `struct NoOp;` para L e C

### T09 — `AlreadyActive`/`AlreadyInactive` → `409` ✅

### T10 — Unique doc no update ✅

---

## 🟡 P1 — Cleanup

### T06b — Remover variante `NotImplemented` do `ClientError`

**Arquivo:** `src/domain/errors/client_error.rs`

A variante `NotImplemented` é um placeholder que não deveria existir em produção.

- [ ] Remover `NotImplemented` de `ClientError`
- [ ] Remover o mapeamento correspondente em `error_response.rs` (se existir)
- [ ] Verificar que não há uso em runtime

---

## 🟢 P3 — Testes de integração

### T11 — Criar `hurl/clients.hurl`

#### Endpoints a testar (14 rotas)

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
| 11 | `POST` | `/api/clients/{uuid}/projects` | Associar projeto |
| 12 | `GET` | `/api/clients/{uuid}/projects` | Listar projetos do client |
| 13 | `DELETE` | `/api/clients/{uuid}/projects/{project_uuid}` | Desassociar projeto |

#### Fluxo do arquivo hurl

```
# ═══════════════════════════════════════════════════════════════════
# SETUP: Precisa de um projeto existente para testar associação.
#        Criar projeto via POST /api/projects antes dos testes de client/project.
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

# 8. POST /api/clients/{client_id}/contact/phones — 200
#    Body: { "phones": [{ "phone": "+5511988887777" }] }

# 9. PUT /api/clients/{client_id}/contact/phones/%2B5511988887777 — 200
#    Body: { "new_phone": "+5511966665555" }

# 10. PUT /api/clients/{client_id}/address — 200
#     Body: { "cep": "01310100", "street": "Av Paulista", ... }

# ─── HAPPY PATH: Client/Project ────────────────────────────────────

# 11. POST /api/projects — 201 (SETUP: criar projeto)
#     Capture: project_id

# 12. POST /api/clients/{client_id}/projects — 201
#     Body: { "project_id": "{project_id}" }
#     Assert: client_id, project_id

# 13. GET /api/clients/{client_id}/projects — 200
#     Assert: isCollection, length == 1

# 14. POST /api/clients/{client_id}/projects — 409 (duplicate)
#     Body: { "project_id": "{project_id}" }
#     Assert: error == "conflict"

# 15. DELETE /api/clients/{client_id}/projects/{project_id} — 200
#     Assert: project_id

# 16. DELETE /api/clients/{client_id}/projects/{project_id} — 404 (idempotência)
#     Assert: error == "not_found"

# ─── ERROS: POST /api/clients ──────────────────────────────────────

# 17. POST — 400 name vazio (garde)
# 18. POST — 400 document formato inválido "123" (garde)
# 19. POST — 422 CPF com dígitos verificadores inválidos "12345678900" (value object)
# 20. POST — 400 email inválido (garde)
# 21. POST — 400 cep formato inválido (garde)
# 22. POST — 409 document já existe (conflict)

# ─── ERROS: GET/PUT/DELETE ─────────────────────────────────────────

# 23. GET /api/clients/00000000-... — 404
# 24. PUT /api/clients/00000000-... — 404
# 25. PUT /api/clients/{client_id}/status — 409 (já active)
# 26. PUT /api/clients/{client_id}/status — 400 status inválido (garde)
# 27. DELETE /api/clients/00000000-... — 404

# ─── ERROS: contact/location ──────────────────────────────────────

# 28. PATCH .../contact/email — 400 email inválido (garde)
# 29. POST .../contact/phones — 400 phone inválido (garde)

# ─── CLEANUP ───────────────────────────────────────────────────────

# 30. DELETE /api/projects/{project_id} — cleanup
# 31. DELETE /api/clients/{client_id} — 200
# 32. DELETE /api/clients/{client_id} — 404 (idempotência)
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
T06b (cleanup — rápido, pode ser feito a qualquer momento)
 ↓
T11 (hurl tests — pode começar agora, todos os endpoints estão prontos)
```

---

## Referências

| Arquivo | Descrição |
|---------|-----------|
| `src/application/client_service.rs` | `ClientService<R, L, C>` genérico + `PgClientService` type alias; 15 use cases |
| `src/application/mod.rs` | `pg_client_serv_build(pool, location_service, contact_service)` |
| `src/adapters/driven/pg_client_repository.rs` | Repository PG — 13 port impls |
| `src/adapters/driving/client_routes.rs` | 14 endpoints com `ValidatedJson` |
| `src/adapters/driving/models/dtos/client_dto.rs` | 9 DTOs com garde |
| `src/adapters/driving/errors/error_response.rs` | `From<ClientError>` — 12 match arms |
| `src/domain/errors/client_error.rs` | 14 variantes |
| `src/domain/ports/use_cases/client_use_cases.rs` | 15 use case traits |
| `src/domain/ports/repositories/client_repository.rs` | 13 ports + super trait `ClientRepository` |
| `src/startup.rs` | `PgClientService` wired |
| `migrations/20260222180343_create_clients_schema.sql` | 4 tabelas |
