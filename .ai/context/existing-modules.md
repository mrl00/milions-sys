# Existing Modules

Catalog of all port traits, use case traits, services, and adapters per bounded context.

## client

### Repository Ports (`domain/ports/client_repository.rs`)

| Trait                | Method                                              |
| -------------------- | --------------------------------------------------- |
| `FindById`           | `find_by_id(uuid) -> Option<ClientRow>`             |
| `FindByDocument`     | `find_by_document(doc) -> Option<ClientRow>`        |
| `FindAll`            | `find_all() -> Vec<ClientRow>`                      |
| `CreateClient`       | `create(input: CreateClientRow) -> ClientRow`       |
| `UpdateClient`       | `update(uuid, input: UpdateClientRow) -> ClientRow` |
| `DeleteClient`       | `delete(uuid) -> ClientRow`                         |
| `CreateClientWithTx` | `create_with_tx(tx, input) -> ClientRow`            |

**Super-traits:** `FindAndCreate`, `FindAndUpdate`, `FindAndDelete`

### Use Cases (`domain/ports/client_use_cases.rs`)

| Trait | Method |
|-------|--------|
| `RegisterClient` | `execute(input: RegisterClientInput) -> ClientRow` |
| `FindClientById` | `execute(uuid) -> ClientRow` |
| `FindClientByDocument` | `execute(doc) -> Option<ClientRow>` |
| `ListClients` | `execute() -> Vec<ClientRow>` |
| `UpdateClient` | `execute(uuid, input: UpdateClientInput) -> ClientRow` |
| `ActivateClient` | `execute(uuid) -> ClientRow` |
| `DeactivateClient` | `execute(uuid) -> ClientRow` |
| `DeleteClient` | `execute(uuid) -> ClientRow` |

**Input structs:** `RegisterClientInput`, `UpdateClientInput`

### Service (`application/client_service.rs`)

`ClientService` — implements all use case traits. Depends on `PgClientRepository`.

---

## collaborator

### Repository Ports (`domain/ports/collaborator_repository.rs`)

| Trait | Method |
|-------|--------|
| `FindCollaboratorById` | `find_by_id(uuid) -> Option<CollaboratorRow>` |
| `FindCollaboratorByCpf` | `find_by_cpf(cpf) -> Option<CollaboratorRow>` |
| `FindAllCollaborators` | `find_all() -> Vec<CollaboratorRow>` |
| `CreateCollaborator` | `create(input: CreateCollaboratorRow) -> CollaboratorRow` |
| `UpdateCollaborator` | `update(uuid, input) -> CollaboratorRow` |
| `DeleteCollaborator` | `delete(uuid) -> CollaboratorRow` |

**Super-traits:** `FindAndCreateCollaborator`, `FindAndUpdateCollaborator`, `FindAndDeleteCollaborator`

### Use Cases (`domain/ports/collaborator_use_cases.rs`)

| Trait | Method |
|-------|--------|
| `RegisterCollaborator` | `execute(input: RegisterCollaboratorInput) -> CollaboratorRow` |
| `FindCollaborator` | `execute(uuid) -> CollaboratorRow` |
| `FindCollaboratorByCpf` | `execute(cpf) -> Option<CollaboratorRow>` |
| `ListCollaborators` | `execute() -> Vec<CollaboratorRow>` |
| `UpdateCollaborator` | `execute(uuid, input: UpdateCollaboratorInput) -> CollaboratorRow` |
| `ActivateCollaborator` | `execute(uuid) -> CollaboratorRow` |
| `DeactivateCollaborator` | `execute(uuid) -> CollaboratorRow` |
| `DeleteCollaborator` | `execute(uuid) -> CollaboratorRow` |

**Input structs:** `RegisterCollaboratorInput`, `UpdateCollaboratorInput`

### Service (`application/collaborator_service.rs`)

`CollaboratorService` — implements all use case traits. Depends on `PgCollaboratorRepository`.

---

## contact

### Contact Repository Ports (`domain/ports/contact_repository.rs`)

| Trait | Method |
|-------|--------|
| `FindContactById` | `find_by_id(uuid) -> Option<ContactRow>` |
| `FindContactByEmail` | `find_by_email(email) -> Option<ContactRow>` |
| `FindAllContacts` | `find_all() -> Vec<ContactRow>` |
| `CreateContact` | `create(input: CreateContactRow) -> ContactRow` |
| `UpdateContactEmail` | `update_email(uuid, email) -> ContactRow` |

**Super-traits:** `FindAndCreateContact`, `FindAndUpdateContact`

### Phone Repository Ports (`domain/ports/phone_repository.rs`)

| Trait | Method |
|-------|--------|
| `FindPhoneById` | `find_by_id(uuid) -> Option<PhoneRow>` |
| `FindPhoneByContactId` | `find_by_contact_id(contact_id) -> Vec<PhoneRow>` |
| `CreatePhone` | `create(contact_id, phone) -> PhoneRow` |
| `CreateManyPhones` | `create_many(contact_id, phones) -> Vec<PhoneRow>` |
| `UpdatePhone` | `update(uuid, phone) -> PhoneRow` |
| `DeletePhone` | `delete(uuid) -> PhoneRow` |
| `FindNonexistentPhones` | `find_nonexistent_phones(phones) -> Vec<String>` |

**Super-traits:** `FindAndCreatePhone`, `FindAndUpdatePhone`, `FindAndDeletePhone`

### Use Cases (`domain/ports/contact_use_cases.rs`)

**Contact:**
| Trait | Method |
|-------|--------|
| `RegisterContact` | `execute(input: RegisterContactInput) -> ContactRow` |
| `FindContact` | `execute(uuid) -> ContactRow` |
| `ListContacts` | `execute() -> Vec<ContactRow>` |
| `UpdateContactEmail` | `execute(uuid, email) -> ContactRow` |

**Phone:**
| Trait | Method |
|-------|--------|
| `FindPhone` | `execute(uuid) -> PhoneRow` |
| `ListPhones` | `execute(contact_id) -> Vec<PhoneRow>` |
| `AddPhone` | `execute(contact_id, phone) -> PhoneRow` |
| `AddPhones` | `execute(contact_id, phones) -> Vec<PhoneRow>` |
| `UpdatePhone` | `execute(uuid, phone) -> PhoneRow` |
| `RemovePhone` | `execute(uuid) -> PhoneRow` |

**Input structs:** `RegisterContactInput`

### Service (`application/contact_service.rs`)

`ContactService` — implements all use case traits (contact + phone). Depends on `PgContactRepository` + `PgPhoneRepository`.

---

## location

### Repository Ports (`domain/ports/location_repository.rs`)

| Trait | Method |
|-------|--------|
| `FindLocationById` | `find_by_id(uuid) -> Option<LocationRow>` |
| `FindLocationByHash` | `find_by_hash(hash) -> Option<LocationRow>` |
| `FindAllLocations` | `find_all() -> Vec<LocationRow>` |
| `CreateLocation` | `create(input: CreateLocationRow) -> LocationRow` |
| `UpdateLocation` | `update(uuid, input) -> LocationRow` |
| `DeleteLocation` | `delete(uuid) -> LocationRow` |

**Super-traits:** `FindOrCreateLocation`, `FindAndUpdateLocation`, `FindAndDeleteLocation`

### Use Cases (`domain/ports/location_use_cases.rs`)

| Trait | Method |
|-------|--------|
| `FindLocation` | `execute(uuid) -> LocationRow` |
| `ListLocations` | `execute() -> Vec<LocationRow>` |
| `CreateLocation` | `execute(input: CreateLocationInput) -> LocationRow` |
| `FindOrCreateLocation` | `execute(input: CreateLocationInput) -> LocationRow` |
| `UpdateLocation` | `execute(uuid, input: UpdateLocationInput) -> LocationRow` |
| `DeleteLocation` | `execute(uuid) -> LocationRow` |

**Input structs:** `CreateLocationInput`, `UpdateLocationInput`

### Service (`application/location_service.rs`)

`LocationService` — implements all use case traits. Depends on `PgLocationRepository`.

---

## project

### Repository Ports (`domain/ports/project_repository.rs`)

| Trait | Method |
|-------|--------|
| `FindProjectById` | `find_by_id(uuid) -> Option<ProjectRow>` |
| `FindProjectByClientId` | `find_by_client_id(client_id) -> Vec<ProjectRow>` |
| `FindAllProjects` | `find_all() -> Vec<ProjectRow>` |
| `CreateProject` | `create(input: CreateProjectRow) -> ProjectRow` |
| `UpdateProject` | `update(uuid, input) -> ProjectRow` |
| `DeleteProject` | `delete(uuid) -> ProjectRow` |

| `FindStageById` | `find_stage_by_id(uuid) -> Option<ProjectStageRow>` |
| `CreateStage` | `create_stage(input: CreateProjectStageRow) -> ProjectStageRow` |
| `UpdateStage` | `update_stage(uuid, input: UpdateProjectStageRow) -> ProjectStageRow` |

**Super-traits:** `FindAndCreateProject`, `FindAndUpdateProject`, `FindAndDeleteProject`, `FindAndCreateStage`, `FindAndUpdateStage`

### Use Cases (`domain/ports/project_use_cases.rs`)

| Trait | Method |
|-------|--------|
| `FindProject` | `execute(uuid) -> ProjectRow` |
| `ListProjects` | `execute() -> Vec<ProjectRow>` |
| `ListProjectsByClient` | `execute(client_id) -> Vec<ProjectRow>` |
| `CreateProject` | `execute(input: CreateProjectInput) -> ProjectRow` |
| `UpdateProject` | `execute(uuid, input: UpdateProjectInput) -> ProjectRow` |
| `StartProject` | `execute(uuid) -> ProjectRow` |
| `PauseProject` | `execute(uuid) -> ProjectRow` |
| `CompleteProject` | `execute(uuid) -> ProjectRow` |
| `CancelProject` | `execute(uuid) -> ProjectRow` |
| `DeleteProject` | `execute(uuid) -> ProjectRow` |
| `CreateStage` | `execute(project_id, input: CreateStageInput) -> ProjectStageRow` |
| `UpdateStage` | `execute(project_id, stage_id, input: UpdateStageInput) -> ProjectStageRow` |

**Input structs:** `CreateProjectInput`, `UpdateProjectInput`, `CreateStageInput`, `UpdateStageInput`

### Service (`application/project_service.rs`)

`ProjectService` — implements all use case traits (project + stage). Depends on `PgProjectRepository`.
