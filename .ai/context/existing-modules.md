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

**Composite trait:** `ClientRepository` (bundles all 6 traits + Send + Sync)

### Use Cases (`domain/ports/client_use_cases.rs`)

All use case traits end with `UseCase` suffix.

| Trait | Method |
|-------|--------|
| `RegisterClientUseCase` | `execute(input: RegisterClientInput) -> ClientRow` |
| `FindClientByIdUseCase` | `execute(uuid) -> ClientRow` |
| `FindClientByDocumentUseCase` | `execute(doc) -> Option<ClientRow>` |
| `ListClientsUseCase` | `execute() -> Vec<ClientRow>` |
| `UpdateClientUseCase` | `execute(uuid, input: UpdateClientInput) -> ClientRow` |
| `ActivateClientUseCase` | `execute(uuid) -> ClientRow` |
| `DeactivateClientUseCase` | `execute(uuid) -> ClientRow` |
| `DeleteClientUseCase` | `execute(uuid) -> ClientRow` |

**Input structs:** `RegisterClientInput`, `UpdateClientInput`

### Service (`application/client_service.rs`)

`ClientService<R>` — generic over repository traits. `ConcreteClientService` is a separate struct holding `PgClientRepository` and `PgPool`, enabling transactional full registration (contact, phones, location, join tables). 16 unit tests with `MockRepo`.

---

## collaborator

### Repository Ports (`domain/ports/collaborator_repository.rs`)

| Trait                      | Method                                                    |
| -------------------------- | --------------------------------------------------------- |
| `FindCollaboratorById`     | `find_by_id(uuid) -> Option<CollaboratorRow>`             |
| `FindCollaboratorByDocument` | `find_by_document(doc) -> Option<CollaboratorRow>`      |
| `FindAllCollaborators`     | `find_all() -> Vec<CollaboratorRow>`                      |
| `CreateCollaborator`       | `create(input: CreateCollaboratorRow) -> CollaboratorRow` |
| `UpdateCollaborator`       | `update(uuid, input) -> CollaboratorRow`                  |
| `DeleteCollaborator`       | `delete(uuid) -> CollaboratorRow`                         |

**Composite trait:** `CollaboratorRepository` (bundles all 6 traits + Send + Sync)

### Use Cases (`domain/ports/collaborator_use_cases.rs`)

All use case traits end with `UseCase` suffix.

| Trait | Method |
|-------|--------|
| `RegisterCollaboratorUseCase` | `execute(input: RegisterCollaboratorInput) -> CollaboratorRow` |
| `FindCollaboratorUseCase` | `execute(uuid) -> CollaboratorRow` |
| `FindCollaboratorByDocumentUseCase` | `execute(doc) -> Option<CollaboratorRow>` |
| `ListCollaboratorsUseCase` | `execute() -> Vec<CollaboratorRow>` |
| `UpdateCollaboratorUseCase` | `execute(uuid, input: UpdateCollaboratorInput) -> CollaboratorRow` |
| `ActivateCollaboratorUseCase` | `execute(uuid) -> CollaboratorRow` |
| `DeactivateCollaboratorUseCase` | `execute(uuid) -> CollaboratorRow` |
| `DeleteCollaboratorUseCase` | `execute(uuid) -> CollaboratorRow` |

**Input structs:** `RegisterCollaboratorInput`, `UpdateCollaboratorInput`

### Service (`application/collaborator_service.rs`)

`CollaboratorService<R>` — generic over repository traits. `ConcreteCollaboratorService` type alias for production (`CollaboratorService<PgCollaboratorRepository>`). 17 unit tests with `MockRepo`.

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

**Composite traits:** `ContactRepository` (5 traits + Send + Sync), `PhoneRepository` (7 traits + Send + Sync)

### Use Cases (`domain/ports/contact_use_cases.rs`)

All use case traits end with `UseCase` suffix.

**Contact:**

| Trait | Method |
|-------|--------|
| `RegisterContactUseCase` | `execute(input: RegisterContactInput) -> ContactRow` |
| `FindContactUseCase` | `execute(uuid) -> ContactRow` |
| `ListContactsUseCase` | `execute() -> Vec<ContactRow>` |
| `UpdateContactEmailUseCase` | `execute(uuid, email) -> ContactRow` |

**Phone:**

| Trait | Method |
|-------|--------|
| `FindPhoneUseCase` | `execute(uuid) -> PhoneRow` |
| `ListPhonesUseCase` | `execute(contact_id) -> Vec<PhoneRow>` |
| `AddPhoneUseCase` | `execute(contact_id, phone) -> PhoneRow` |
| `AddPhonesUseCase` | `execute(contact_id, phones) -> Vec<PhoneRow>` |
| `UpdatePhoneUseCase` | `execute(uuid, phone) -> PhoneRow` |
| `RemovePhoneUseCase` | `execute(uuid) -> PhoneRow` |

**Input structs:** `RegisterContactInput`

### Service (`application/contact_service.rs`)

`ContactService<C, P>` — generic over contact and phone repository traits. `ConcreteContactService` type alias for production (`ContactService<PgContactRepository, PgPhoneRepository>`). 21 unit tests with `MockContactRepo` and `MockPhoneRepo`.

---

## location

### Repository Ports (`domain/ports/location_repository.rs`)

| Trait | Method |
|-------|--------|
| `FindLocationById` | `find_by_id(uuid) -> Option<LocationRow>` |
| `FindAllLocations` | `find_all() -> Vec<LocationRow>` |
| `CreateLocation` | `create(input: CreateLocationRow) -> LocationRow` |
| `UpdateLocation` | `update(uuid, input) -> LocationRow` |
| `DeleteLocation` | `delete(uuid) -> LocationRow` |

**Composite trait:** `LocationRepository` (bundles all 5 traits + Send + Sync)

### Use Cases (`domain/ports/location_use_cases.rs`)

All use case traits end with `UseCase` suffix.

| Trait | Method |
|-------|--------|
| `FindLocationUseCase` | `execute(uuid) -> LocationRow` |
| `ListLocationsUseCase` | `execute() -> Vec<LocationRow>` |
| `CreateLocationUseCase` | `execute(input: CreateLocationInput) -> LocationRow` |
| `UpdateLocationUseCase` | `execute(uuid, input: UpdateLocationInput) -> LocationRow` |
| `DeleteLocationUseCase` | `execute(uuid) -> LocationRow` |

**Input structs:** `CreateLocationInput` (no `hash` field — hash is `GENERATED ALWAYS AS` in DB), `UpdateLocationInput`

### Service (`application/location_service.rs`)

`LocationService<R>` — generic over repository traits. `ConcreteLocationService` type alias for production (`LocationService<PgLocationRepository>`). 14 unit tests with `MockRepo`.

### Location Hash

`nr_hash` is a database-generated column: `GENERATED ALWAYS AS (hashtext(concat_ws('|', tx_street, tx_number, tx_city, tx_state, tx_zipcode))) STORED`. The `PgLocationRepository::find_or_create_with_executor` method uses `ON CONFLICT (nr_hash) DO UPDATE` for atomic find-or-create in a single query.

---

## project

### Repository Ports (`domain/ports/project_repository.rs`)

| Trait                   | Method                                                                |
| ----------------------- | --------------------------------------------------------------------- |
| `FindProjectById`       | `find_by_id(uuid) -> Option<ProjectRow>`                              |
| `FindProjectByClientId` | `find_by_client_id(client_id) -> Vec<ProjectRow>`                     |
| `FindAllProjects`       | `find_all() -> Vec<ProjectRow>`                                       |
| `CreateProject`         | `create(input: CreateProjectRow) -> ProjectRow`                       |
| `UpdateProject`         | `update(uuid, input) -> ProjectRow`                                   |
| `DeleteProject`         | `delete(uuid) -> ProjectRow`                                          |
| `FindStageById`         | `find_stage_by_id(uuid) -> Option<ProjectStageRow>`                   |
| `CreateStage`           | `create_stage(input: CreateProjectStageRow) -> ProjectStageRow`       |
| `UpdateStage`           | `update_stage(uuid, input: UpdateProjectStageRow) -> ProjectStageRow` |
| `FindAllocationById`    | `find_allocation_by_id(uuid) -> Option<ProjectDailyAllocationRow>`    |
| `FindAllocationsByProjectId` | `find_allocations_by_project_id(project_id) -> Vec<ProjectDailyAllocationRow>` |
| `CreateAllocation`      | `create_allocation(input: CreateProjectDailyAllocationRow) -> ProjectDailyAllocationRow` |
| `UpdateAllocation`      | `update_allocation(uuid, input: UpdateProjectDailyAllocationRow) -> ProjectDailyAllocationRow` |
| `FindStagesByProjectId` | `find_stages_by_project_id(project_id) -> Vec<ProjectStageRow>` |
| `FindAllocationsByCollaboratorId` | `find_allocations_by_collaborator_id(collaborator_id) -> Vec<AllocationWithProjectName>` |

**Composite trait:** `ProjectRepository` (bundles all 15 traits + Send + Sync)

### Use Cases (`domain/ports/project_use_cases.rs`)

All use case traits end with `UseCase` suffix.

| Trait                  | Method                                                                      |
| ---------------------- | --------------------------------------------------------------------------- |
| `FindProjectUseCase`          | `execute(uuid) -> ProjectRow`                                               |
| `ListProjectsUseCase`         | `execute() -> Vec<ProjectRow>`                                              |
| `CreateProjectUseCase`        | `execute(input: CreateProjectInput) -> ProjectRow`                          |
| `UpdateProjectUseCase`        | `execute(uuid, input: UpdateProjectInput) -> ProjectRow`                    |
| `StartProjectUseCase`         | `execute(uuid) -> ProjectRow`                                               |
| `PauseProjectUseCase`         | `execute(uuid) -> ProjectRow`                                               |
| `CompleteProjectUseCase`      | `execute(uuid) -> ProjectRow`                                               |
| `CancelProjectUseCase`        | `execute(uuid) -> ProjectRow`                                               |
| `DeleteProjectUseCase`        | `execute(uuid) -> ProjectRow`                                               |
| `CreateStageUseCase`          | `execute(project_id, input: CreateStageInput) -> ProjectStageRow`           |
| `UpdateStageUseCase`          | `execute(project_id, stage_id, input: UpdateStageInput) -> ProjectStageRow` |
| `CreateAllocationUseCase`     | `execute(project_id, input: CreateAllocationInput) -> ProjectDailyAllocationRow` |
| `ListAllocationsUseCase`      | `execute(project_id) -> Vec<ProjectDailyAllocationRow>`                     |
| `UpdateAllocationUseCase`     | `execute(project_id, allocation_id, input: UpdateAllocationInput) -> ProjectDailyAllocationRow` |
| `GetCostReportUseCase`        | `execute(project_id) -> CostReportData`                                     |
| `GetProgressReportUseCase`    | `execute(project_id) -> ProgressReportData`                                 |
| `GetHistoryReportUseCase`     | `execute(collaborator_id) -> HistoryReportData`                             |

**Input structs:** `CreateProjectInput`, `UpdateProjectInput`, `CreateStageInput`, `UpdateStageInput`, `CreateAllocationInput`, `UpdateAllocationInput`

**Output structs:** `CostReportData`, `ProgressReportData`, `HistoryReportData`, `AllocationHistoryEntry`

### Service (`application/project_service.rs`)

`ProjectService<R>` — generic over repository traits. `ConcreteProjectService` type alias for production (`ProjectService<PgProjectRepository>`). 14 unit tests with `MockRepo`.
