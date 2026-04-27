# location

Bounded context for managing **addresses/locations** in the milions-sys construction project management system.

## Domain

Locations represent physical addresses with full Brazilian address data including CEP (postal code), street, neighborhood, city, state, and geographic coordinates. Addresses are deduplicated by hash to avoid duplicates.

## Architecture

Follows hexagonal architecture (ports and adapters):

```
location/
├── src/
│   ├── domain/
│   │   ├── errors/mod.rs          # LocationError enum
│   │   ├── models/db/             # LocationRow
│   │   └── ports/
│   │       ├── location_repository.rs   # FindById, FindByHash, FindAll, Create, Update, Delete
│   │       └── location_use_cases.rs    # RegisterLocation, FindLocation, ListLocations, UpdateLocation, DeleteLocation
│   ├── application/
│   │   └── location_service.rs    # LocationService — implements all use case traits
│   └── adapters/
│       ├── driven/
│       │   └── postgres/
│       │       └── pg_location_repository.rs  # PostgreSQL implementation
│       └── driving/
│           ├── dto.rs             # Request/response DTOs
│           └── routes.rs          # HTTP route handlers
└── Cargo.toml
```

## Dependencies

- `types` — shared value objects (`Cep`)

## API Endpoints

| Method | Route | Description |
|--------|-------|-------------|
| POST | `/api/locations` | Register a new location |
| GET | `/api/locations` | List all locations |
| GET | `/api/locations/{uuid}` | Find location by ID |
| PUT | `/api/locations/{uuid}` | Update a location |
| DELETE | `/api/locations/{uuid}` | Delete a location |

## Error Types

| Variant | HTTP Status | Description |
|---------|-------------|-------------|
| `NotFound` | 404 | Location not found |
| `AlreadyExists` | 409 | Location with same hash already exists |
| `InvalidField` | 422 | Validation error on a field |
| `Infra` | 500 | Infrastructure/database error |
