# milions-sys — Backlog Client

## Estado Atual (2026-04-25)

O contexto **client** foi significativamente refatorado. O service está wired no runtime (`startup.rs`), a `error_to_response` manual foi removida, e a arquitetura foi separada em `PgClientService` (concreto, injetado com location/contact services) + `ClientService<R>` (genérico, para testes unitários).

O `register_client` handler está marcado como `todo!()` — o input foi reestruturado (separando `RegisterClientLocationInput` e `RegisterClientContactInput`) mas o handler ainda não foi atualizado para usar a nova assinatura.

### Estado por Camada

| Camada | Status | Situação |
|--------|--------|----------|
| DB models / rows | ✅ | — |
| Repository (PG) | ✅ | Adicionados `LinkCreatedLocationToClient` + `LinkCreatedContactToClient` |
| Use case ports | ⚠️ | Expandidos: novos `UpdateClientEmailUseCase`, `UpdateClientPhoneUseCase`, `AddClientPhoneUseCase`, `UpdateClientLocationUseCase` — nenhum implementado no service ainda |
| Service impl | ⚠️ | `PgClientService` wired corretamente; `RegisterClientUseCase` refatorado mas handler está `todo!()`; genérico `ClientService<R>` mantido (duplicação parcial) |
| DTOs | 🔴 | Zero anotações `garde`, sem `#[derive(Validate)]` |
| Routes | ⚠️ | `error_to_response` removida ✅; usa `HttpResponse::from(e)` ✅; `register_client` é `todo!()` 🔴; demais handlers ainda usam `web::Json` sem `ValidatedJson` 🔴 |
| Wired in startup | ✅ | `pg_client_serv_build` criado e registrado com `location_service` e `contact_service` injetados |
| Hurl tests | 🔴 | Arquivo não existe |

### O que mudou no diff

| Arquivo | Mudança principal |
|---------|-------------------|
| `startup.rs` | `pg_client_serv_build` registrado; `client_routes::configure` adicionado ao scope `/api` |
| `application/mod.rs` | `pg_client_serv_build(pool, location_service, contact_service)` criado |
| `client_service.rs` | `ConcreteClientService` renomeado para `PgClientService`; injeção de `PgLocationService` e `PgContactService` via composição; `RegisterClientUseCase` refatorado com novo input estruturado |
| `client_error.rs` | Simplificado: removidos `ContactNotFound`, `LocationNotFound`, `EmailAlreadyExists`, `PhoneAlreadyExists`, `InvalidDoc`, `InvalidEmail`, `InvalidPhone`, `InvalidCep`; adicionados `NotImplemented`, `Location(LocationError)`, `Contact(ContactError)`, `InvalidDocument(DocError)` |
| `client_routes.rs` | `error_to_response` removida; todos os handlers usam `HttpResponse::from(e)`; `register_client` marcado como `todo!()` |
| `error_response.rs` | `From<ClientError>` atualizado para refletir o novo `ClientError` |
| `client_repository.rs` | Novos ports: `LinkCreatedLocationToClient`, `LinkCreatedContactToClient` |
| `pg_client_repository.rs` | Implementações dos 2 novos ports |
| `client_use_cases.rs` | `RegisterClientInput` reestruturado; novos inputs `RegisterClientLocationInput`, `RegisterClientContactInput`; novos use cases: `UpdateClientEmailUseCase`, `UpdateClientPhoneUseCase`, `AddClientPhoneUseCase`, `UpdateClientLocationUseCase` |

---

## ✅ Concluído

### T01 — Registrar client service no runtime ✅ 2026-04-25
- `pg_client_serv_build(pool, location_service, contact_service)` criado em `application/mod.rs`
- `PgClientService` registrado em `startup.rs` com `app_data`
- `client_routes::configure` adicionado ao scope `/api`

### T04 — Remover `error_to_response` e usar `From<ClientError> for HttpResponse` ✅ 2026-04-25
- `error_to_response` fn removida de `client_routes.rs`
- Todos os handlers usam `HttpResponse::from(e)`
- Testes unitários de rota atualizados para usar `HttpResponse::from(err)`

### T08 — Refactor arquitetura service (parcial) ✅ 2026-04-25
- `ConcreteClientService` → `PgClientService`
- Injeção de `PgLocationService` e `PgContactService` via composição (não mais hardcoded internamente)
- `PgClientService` usa `client_repo.link_created_location_to_client` e `client_repo.link_created_contact_to_client` em vez de chamar repos concretos diretamente

### T09 — `AlreadyActive`/`AlreadyInactive` → `409` ✅ 2026-04-25
- Resolvido automaticamente com T04 — `From<ClientError>` já mapeia para `409`
- ⚠️ Os testes unitários de rota ainda verificam `400` (não foram atualizados) — ver T04b abaixo

---

## 🔴 P0 — Bloqueia o `POST /api/clients`

### T01b — Implementar o handler `register_client` (atualmente `todo!()`)

**Arquivo:** `src/adapters/driving/client_routes.rs`

O handler está marcado com `todo!()` desde a refatoração do `RegisterClientInput`. O `RegisterClientUseCase` mudou: o input agora recebe `location: Option<RegisterClientLocationInput>` e `contact: Option<RegisterClientContactInput>` — o handler precisa ser atualizado para construir esses tipos a partir do body DTO.

```rust
// Novo input esperado pelo use case:
RegisterClientInput {
    name: body.name.clone(),
    doc: body.document.clone(),
    status: ClientStatus::Active,
    location: body.address.as_ref().map(|a| RegisterClientLocationInput {
        street: a.street.clone(),
        number: a.number.clone(),
        city: a.city.clone(),
        state: a.state.clone(),
        zipcode: a.cep.clone(),
        complement: a.complement.clone().unwrap_or_default(),
        public_space: String::new(),
        unit: String::new(),
        neighborhood: a.neighborhood.clone(),
        locality: a.city.clone(),
        region: a.state.clone(),
        ibge: None, gia: None,
        ddd: String::new(), siafi: None,
    }),
    contact: body.contact.as_ref().map(|c| RegisterClientContactInput {
        email: c.email.clone(),
        phones: c.phones.clone(), // ou c.phones.iter().map(|p| p.value.clone()).collect() se PhoneEntry
    }),
}
```

- [ ] Remover `todo!()` e o bloco comentado
- [ ] Construir `RegisterClientInput` a partir do body DTO
- [ ] Usar `RegisterClientUseCase::execute(&**service, input).await`
- [ ] Verificar: o `RegisterClientRequest` DTO precisa atualizar `address` e `contact` para `Option` — atualmente são obrigatórios
- [ ] Adicionar teste de integração (hurl) após implementado

---

## 🔴 P1 — Conformidade com convenções do projeto

### T02 — Adicionar validação `garde` nos DTOs de client

Todos os outros contextos (collaborator, contact, location, project) usam `#[derive(Validate)]` + anotações `#[garde(...)]`. O contexto client é o único sem validação.

**Arquivo:** `src/adapters/driving/models/dtos/client_dto.rs`

- [ ] `use garde::Validate;`
- [ ] `RegisterClientRequest` → `#[derive(Debug, Deserialize, Validate)]`
  - `name`: `#[garde(length(min = 1, max = 64))]` (DB: `VARCHAR(64) NOT NULL`)
  - `document`: `#[garde(pattern(r"^\d{11}$|^\d{14}$"))]` (CPF 11 dígitos ou CNPJ 14 dígitos)
  - `contact`: `#[garde(dive)]` ou `#[garde(inner(dive))]` se `Option`
  - `address`: `#[garde(dive)]` ou `#[garde(inner(dive))]` se `Option`
- [ ] `UpdateClientRequest` → `#[derive(Debug, Deserialize, Validate)]`
  - `name`: `#[garde(inner(length(min = 1, max = 64)))]`
  - `document`: `#[garde(inner(pattern(r"^\d{11}$|^\d{14}$")))]`
  - `contact`: `#[garde(skip)]`
  - `address`: `#[garde(skip)]`
- [ ] `ContactDto` → `#[derive(Debug, Deserialize, Serialize, Clone, Validate)]`
  - `email`: `#[garde(email, length(max = 256))]`
  - `phones`: `#[garde(dive, length(min = 1))]` com wrapper `PhoneEntry { #[garde(pattern(r"^\+\d{8,16}$"))] value: String }`
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

Após T01b estar implementado, todos os handlers que recebem body devem usar `ValidatedJson`.

- [ ] `use crate::adapters::driving::utils::ValidatedJson;`
- [ ] Trocar em todos os handlers (após T01b):
  - `register_client`: `ValidatedJson(body): ValidatedJson<RegisterClientRequest>`
  - `update_client`: `ValidatedJson(body): ValidatedJson<UpdateClientRequest>`
  - `update_client_status`: `ValidatedJson(body): ValidatedJson<StatusRequest>`
- [ ] Remover o match manual `_ => return HttpResponse::BadRequest(...)` em `update_client_status` (garde já rejeita via pattern)

### T04b — Corrigir testes unitários de rota: `AlreadyActive`/`AlreadyInactive` → `409`

**Arquivo:** `src/adapters/driving/client_routes.rs` — módulo `#[cfg(test)]`

Os testes ainda verificam `400` para `AlreadyActive`/`AlreadyInactive`, porém o `From<ClientError>` já mapeia para `409 Conflict`.

- [ ] `error_to_response_already_active`: `assert_eq!(resp.status(), 400)` → `assert_eq!(resp.status(), 409)`
- [ ] `error_to_response_already_inactive`: idem

---

## 🟡 P2 — Lógica de negócio incompleta

### T05 — Implementar use cases de contact e location para o client

**Arquivo:** `src/application/client_service.rs` + `src/adapters/driving/client_routes.rs`

Os seguintes use cases foram declarados em `client_use_cases.rs` mas **não possuem implementação no service**:

| Use Case | Trait | Status |
|----------|-------|--------|
| `UpdateClientEmailUseCase` | `execute(uuid, email)` | 🔴 Não implementado |
| `UpdateClientPhoneUseCase` | `execute(uuid, phone, new_phone)` | 🔴 Não implementado |
| `AddClientPhoneUseCase` | `execute(uuid, phone)` | 🔴 Não implementado |
| `UpdateClientLocationUseCase` | `execute(uuid, input)` | 🔴 Não implementado |

- [ ] Implementar cada use case no `PgClientService` (delegando para `self.contact_service` e `self.location_service`)
- [ ] Criar handlers e rotas correspondentes:
  - `PATCH /api/clients/{uuid}/contact/email` → `UpdateClientEmailUseCase`
  - `PATCH /api/clients/{uuid}/contact/phones` → `AddClientPhoneUseCase`
  - `PUT /api/clients/{uuid}/contact/phones/{phone}` → `UpdateClientPhoneUseCase`
  - `PUT /api/clients/{uuid}/address` → `UpdateClientLocationUseCase`
- [ ] Adicionar testes unitários em `client_service.rs`
- [ ] Cobrir no hurl (T11)

### T06 — `Contact(ContactError)` e `Location(LocationError)` mapeados como `500`/`400` genéricos

**Arquivo:** `src/adapters/driving/errors/error_response.rs`

Os novos variants `Contact(_)` e `Location(_)` no `ClientError` são mapeados com mensagens genéricas:
- `Location(_)` → `500 internal_error`
- `Contact(_)` → `400 bad_request "contact error"` (mensagem opaca)

Isso precisa propagar os erros corretamente ao cliente.

- [ ] `Location(e)` → re-propagar com `HttpResponse::from(e)` (delegando para `From<LocationError>`)
- [ ] `Contact(e)` → re-propagar com `HttpResponse::from(e)` (delegando para `From<ContactError>`)
- [ ] Garantir que `DocumentAlreadyExists` por violação de unique no update também retorna `409`

### T07 — Client/Project association não implementada no service

**Arquivo:** `src/domain/ports/repositories/client_repository.rs`

Existem os port traits `CreateClientProject` e `FindProjectsByClientId` no repository, mas:
- Nenhum use case trait correspondente em `client_use_cases.rs`
- Nenhum handler/rota expondo esses endpoints

- [ ] Criar use case traits: `AssociateClientProjectUseCase`, `ListClientProjectsUseCase`, `DissociateClientProjectUseCase`
- [ ] Implementar no `PgClientService`
- [ ] Criar rotas:
  - `POST /api/clients/{uuid}/projects` — associar projeto
  - `GET /api/clients/{uuid}/projects` — listar projetos do cliente
  - `DELETE /api/clients/{uuid}/projects/{project_uuid}` — desassociar projeto
- [ ] Validar existência do client e do project antes de criar a associação
- [ ] Tratar constraint `uq_fk_client_project` como `409 Conflict`
- [ ] Adicionar variante `ProjectAlreadyAssociated` no `ClientError`

### T08b — Remover duplicação `ClientService<R>` vs `PgClientService`

**Arquivo:** `src/application/client_service.rs`

Os 6 use cases básicos (Find, List, Update, Activate, Deactivate, Delete) ainda estão implementados **2×**: uma vez no `PgClientService` concreto e outra no genérico `ClientService<R>`.

- [ ] Remover duplicação: o `ClientService<R>` genérico pode ser mantido apenas para testes unitários e os impls do `PgClientService` devem delegar para ele, OU eliminar o genérico e manter apenas o concreto (que já é testado via mock no módulo de testes)
- [ ] Garantir que `MockRepo` no módulo de testes continua implementando os novos ports `LinkCreatedLocationToClient` e `LinkCreatedContactToClient` ✅ (já implementado no diff)

### T10 — `tx_doc` constraint UNIQUE — update pode violar unicidade

**Arquivo:** `src/application/client_service.rs` — `UpdateClientUseCase`

Se o usuário passa um `doc` que já está registrado para outro client, o DB retorna unique violation → `500` opaco.

- [ ] Verificar se `doc` já existe (findByDocument) antes de fazer update, retornando `ClientError::DocumentAlreadyExists`
- [ ] Ou interceptar `sqlx::Error::Database` com código `23505` no repository e mapear para `DocumentAlreadyExists`
- [ ] Adicionar teste unitário

---

## 🟢 P3 — Testes de integração

### T11 — Criar e salvar `hurl/clients.hurl`

#### Pré-requisitos

| Task | Impacto no hurl |
|------|-----------------|
| T01b (handler `register_client`) | **Bloqueante** — sem isso, `POST /api/clients` retorna panic |
| T02 + T03 (garde/ValidatedJson) | Status codes de validação (garde → `400`, value object → `422`) |
| T04b (testes de rota) | Sem impacto no hurl, apenas nos testes unitários |
| T06 (re-propagar Location/Contact errors) | Status codes dos erros de location/contact |

#### Endpoints a testar (6 rotas base + 4 de T05)

| # | Método | Rota | Descrição |
|---|--------|------|-----------|
| 1 | `POST` | `/api/clients` | Registrar client |
| 2 | `GET` | `/api/clients` | Listar clients |
| 3 | `GET` | `/api/clients/{uuid}` | Buscar client por ID |
| 4 | `PUT` | `/api/clients/{uuid}` | Atualizar name/doc |
| 5 | `DELETE` | `/api/clients/{uuid}` | Remover client |
| 6 | `PUT` | `/api/clients/{uuid}/status` | Alterar status |
| 7* | `PATCH` | `/api/clients/{uuid}/contact/email` | Atualizar email (T05) |
| 8* | `PATCH` | `/api/clients/{uuid}/contact/phones` | Adicionar phone (T05) |
| 9* | `PUT` | `/api/clients/{uuid}/address` | Atualizar endereço (T05) |

> \* Dependem de T05 estar implementado.

#### Fluxo do arquivo hurl

```
# ═══════════════════════════════════════════════════════════════════
# SETUP: Nenhum — RegisterClient cria location, contact e client
#        via composição (PgLocationService + PgContactService)
# ═══════════════════════════════════════════════════════════════════

# ─── HAPPY PATH ────────────────────────────────────────────────────

# 1. POST /api/clients — 201 Created (PF com CPF)
#    Body: { name, document, contact: { email, phones }, address: { cep, street, number, ... } }
#    Capture: client_id
#    Assert: id isString, name, status=="active", document, created_at, updated_at

# 2. GET /api/clients — 200 OK
#    Assert: isCollection, $[0].id isString, $[0].name isString, $[0].status isString

# 3. GET /api/clients/{client_id} — 200 OK
#    Assert: id == client_id, name, status == "active", document

# 4. PUT /api/clients/{client_id} — 200 OK
#    Body: { "name": "Nome Atualizado", "document": null }
#    Assert: id == client_id, name == "Nome Atualizado"

# 5. PUT /api/clients/{client_id}/status — 200 (deactivate)
#    Body: { "status": "inactive" }
#    Assert: status == "inactive"

# 6. PUT /api/clients/{client_id}/status — 200 (activate)
#    Body: { "status": "active" }
#    Assert: status == "active"

# ─── ERROS: POST /api/clients ──────────────────────────────────────

# 7. POST — 400 name vazio (garde)
#    Assert: error == "validation_error"

# 8. POST — 400 document formato inválido "123" (garde — fora do padrão \d{11}|\d{14})
#    Assert: error == "validation_error"

# 9. POST — 422 CPF com dígitos verificadores inválidos "12345678900" (value object)
#    Assert: error == "validation_error"

# 10. POST — 400 email inválido (garde)
#     Assert: error == "validation_error"

# 11. POST — 400 cep inválido formato (garde — não é \d{8})
#     Assert: error == "validation_error"

# 12. POST — 409 document já existe (conflict)
#     Body: mesmos dados do #1
#     Assert: error == "conflict"

# ─── ERROS: GET/PUT/DELETE ─────────────────────────────────────────

# 13. GET /api/clients/00000000-0000-7000-0000-000000000000 — 404
#     Assert: error == "not_found"

# 14. PUT /api/clients/00000000-0000-7000-0000-000000000000 — 404
#     Body: { "name": "Ghost" }
#     Assert: error == "not_found"

# 15. PUT /api/clients/{client_id}/status — 409 (já active)
#     Body: { "status": "active" }   (client está active após #6)
#     Assert: error == "conflict"

# 16. PUT /api/clients/{client_id}/status — 400 status inválido (garde)
#     Body: { "status": "suspended" }
#     Assert: error == "validation_error"

# 17. PUT /api/clients/00000000-0000-7000-0000-000000000000/status — 404
#     Body: { "status": "active" }
#     Assert: error == "not_found"

# 18. DELETE /api/clients/00000000-0000-7000-0000-000000000000 — 404
#     Assert: error == "not_found"

# ─── CLEANUP ───────────────────────────────────────────────────────

# 19. DELETE /api/clients/{client_id} — 200
#     Assert: id == client_id

# 20. DELETE /api/clients/{client_id} — 404 (idempotência)
#     Assert: error == "not_found"
```

#### Dados de teste sugeridos

```json
{
  "name": "Maria da Silva",
  "document": "52998224725",
  "contact": {
    "email": "maria.silva@example.com",
    "phones": ["55619999901011"]
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

> **Nota:** O campo `phones` pode precisar ser `[{ "value": "+5561999990101" }]` se T02 for implementado com wrapper `PhoneEntry`. Ajustar conforme o formato final do DTO.

> **Nota:** O status de erro `422` nos cenários #9 depende da validação do value object `Doc` (dígitos verificadores). Se o garde rejeitar primeiro (antes do value object ser instanciado), o status pode ser `400`. Verificar no teste.

#### Comando de execução

```bash
hurl --test hurl/clients.hurl
```

---

## Ordem de execução recomendada

```
T01b (handler register_client — desbloqueia POST /api/clients)
 ↓
T04b (corrigir assert 400→409 nos testes unitários)
 ↓
T02 (garde DTOs) + T03 (ValidatedJson) — podem ser feitas juntas
 ↓
T06 (re-propagar Location/Contact errors no From<ClientError>)
 ↓
T08b (remover duplicação ClientService<R>)
 ↓
T10 (unique doc no update)
 ↓
T05 (implementar UpdateClientEmail/Phone/Location use cases + rotas)
 ↓
T07 (client/project association endpoints)
 ↓
T11 (hurl tests) ← só depois que T01b estiver implementado (mínimo)
```

---

## Referências

| Arquivo | Descrição |
|---------|-----------|
| `src/adapters/driven/pg_client_repository.rs` | Repository PG (6 port impls + `LinkCreatedLocationToClient` + `LinkCreatedContactToClient`) |
| `src/adapters/driving/client_routes.rs` | 6 endpoints; `register_client` está `todo!()`; todos usam `HttpResponse::from(e)` |
| `src/adapters/driving/models/dtos/client_dto.rs` | DTOs (sem garde) |
| `src/adapters/driving/errors/error_response.rs` | `From<ClientError>` atualizado; `Contact`/`Location` mapeados genericamente |
| `src/application/client_service.rs` | `PgClientService` + `ClientService<R>` genérico (parcialmente duplicado) |
| `src/application/mod.rs` | `pg_client_serv_build(pool, location_service, contact_service)` |
| `src/domain/errors/client_error.rs` | 9 variantes: `NotImplemented`, `AlreadyExists`, `NotFound`, `AlreadyActive`, `AlreadyInactive`, `DocumentAlreadyExists`, `InvalidDocument`, `Location`, `Contact`, `ViaCep`, `Infra` |
| `src/domain/models/db/client_row.rs` | `ClientRow`, `CreateClientRow`, `ClientStatus` |
| `src/domain/models/db/client_contact_row.rs` | Associação client ↔ contact |
| `src/domain/models/db/client_address_row.rs` | Associação client ↔ location |
| `src/domain/models/db/client_project_row.rs` | Associação client ↔ project |
| `src/domain/ports/use_cases/client_use_cases.rs` | 12 use case traits (8 existentes + 4 novos de contact/location) |
| `src/domain/ports/repositories/client_repository.rs` | 8 ports: 6 base + `LinkCreatedLocationToClient` + `LinkCreatedContactToClient` |
| `src/domain/value_objects/doc.rs` | Value object CPF/CNPJ |
| `src/startup.rs` | `PgClientService` wired com `pg_client_serv_build` |
| `migrations/20260222180343_create_clients_schema.sql` | 4 tabelas: `tb_client`, `tb_client_contact`, `tb_client_address`, `tb_client_project` |
