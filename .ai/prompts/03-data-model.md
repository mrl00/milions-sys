# Data Model

Database schemas and entity relationships.

## Naming Convention

- Tables: `tb_<entity>`
- Primary key: `pk_<entity>` (UUID)
- Index: `idx_<entity>` (auto-increment)
- Foreign key: `fk_<referenced_entity>`
- Timestamps: `ts_<entity>_created_at`, `ts_<entity>_updated_at`
- Text: `tx_<field>`
- Number: `nr_<field>`
- Boolean: `bl_<field>`
- Date: `dt_<field>`

## Schema: `clients`

### tb_client

| Column | Type | Notes |
|--------|------|-------|
| pk_client | UUID | PK |
| idx_client | i32 | auto-increment |
| tx_name | String | |
| tx_status | String | enum: active, inactive |
| tx_doc | String | CPF or CNPJ |
| ts_client_created_at | NaiveDateTime | |
| ts_client_updated_at | NaiveDateTime | |

### tb_client_contact

| Column | Type | Notes |
|--------|------|-------|
| pk_client_contact | UUID | PK |
| idx_client_contact | i64 | |
| fk_client | UUID | → tb_client |
| fk_contact | UUID | → contacts.tb_contact |
| ts_*_created_at | NaiveDateTime | |
| ts_*_updated_at | NaiveDateTime | |

### tb_client_address

| Column | Type | Notes |
|--------|------|-------|
| pk_client_address | UUID | PK |
| idx_client_address | i32 | |
| fk_client | UUID | → tb_client |
| fk_address | UUID | → locations.tb_location |
| ts_*_created_at | NaiveDateTime | |
| ts_*_updated_at | NaiveDateTime | |

### tb_project

| Column | Type | Notes |
|--------|------|-------|
| pk_project | UUID | PK |
| idx_project | i32 | |
| tx_name | String | |
| tx_description | Option<String> | |
| tx_status | String | enum: planning, in_progress, paused, completed, cancelled |
| dt_start_date | Option<NaiveDate> | |
| dt_estimated_end_date | Option<NaiveDate> | |
| dt_actual_end_date | Option<NaiveDate> | |
| nr_total_area_m2 | Option<BigDecimal> | |
| nr_estimated_cost | Option<BigDecimal> | |
| nr_actual_cost | Option<BigDecimal> | |
| tx_notes | Option<String> | |
| bl_active | bool | |
| fk_client | UUID | → tb_client |
| fk_address | UUID | → locations.tb_location |
| ts_*_created_at | NaiveDateTime | |
| ts_*_updated_at | NaiveDateTime | |

### tb_project_stage

| Column | Type | Notes |
|--------|------|-------|
| pk_project_stage | UUID | PK |
| idx_project_stage | i32 | |
| fk_project | UUID | → tb_project |
| tx_name | String | |
| tx_description | Option<String> | |
| nr_order | i32 | sort order |
| tx_status | String | enum: pending, in_progress, completed, skipped |
| dt_start_date | Option<NaiveDate> | |
| dt_end_date | Option<NaiveDate> | |
| ts_*_created_at | NaiveDateTime | |
| ts_*_updated_at | NaiveDateTime | |

### tb_service_type (catalog)

| Column | Type | Notes |
|--------|------|-------|
| pk_service_type | UUID | PK |
| idx_service_type | i32 | |
| tx_name | String | |
| tx_description | Option<String> | |
| tx_unit | String | enum: m2, m_linear, unit, hour |
| nr_default_unit_price | Option<BigDecimal> | |
| bl_active | bool | |
| ts_*_created_at | NaiveDateTime | |
| ts_*_updated_at | NaiveDateTime | |

### tb_project_service

| Column | Type | Notes |
|--------|------|-------|
| pk_project_service | UUID | PK |
| idx_project_service | i32 | |
| fk_project | UUID | → tb_project |
| fk_project_stage | Option<Uuid> | → tb_project_stage |
| fk_service_type | UUID | → tb_service_type |
| tx_description | Option<String> | |
| nr_quantity | BigDecimal | |
| nr_unit_price | BigDecimal | |
| nr_total_price | Option<BigDecimal> | |
| tx_status | String | enum: pending, in_progress, completed |
| ts_*_created_at | NaiveDateTime | |
| ts_*_updated_at | NaiveDateTime | |

### tb_project_daily_allocation

| Column | Type | Notes |
|--------|------|-------|
| pk_project_daily_allocation | UUID | PK |
| idx_project_daily_allocation | i32 | |
| fk_project | UUID | → tb_project |
| fk_collaborator | UUID | → collaborators.tb_collaborator |
| dt_work_date | NaiveDate | |
| nr_hours_worked | Option<BigDecimal> | |
| nr_hourly_rate_snapshot | Option<BigDecimal> | |
| tx_notes | Option<String> | |
| bl_present | bool | |
| ts_*_created_at | NaiveDateTime | |
| ts_*_updated_at | NaiveDateTime | |

## Schema: `collaborators`

### tb_collaborator

| Column | Type | Notes |
|--------|------|-------|
| pk_collaborator | UUID | PK |
| idx_collaborator | i64 | |
| tx_name | String | |
| tx_cpf | String | individual tax ID |
| tx_level | String | enum: P0, P1, P2, P3 |
| tx_status | String | enum: active, inactive |
| ts_*_created_at | NaiveDateTime | |
| ts_*_updated_at | NaiveDateTime | |

### tb_collaborator_contact

| Column | Type | Notes |
|--------|------|-------|
| pk_collaborator_contact | UUID | PK |
| idx_collaborator_contact | i64 | |
| fk_collaborator | UUID | → tb_collaborator |
| fk_contact | UUID | → contacts.tb_contact |
| ts_*_created_at | NaiveDateTime | |
| ts_*_updated_at | NaiveDateTime | |

### tb_collaborator_address

| Column | Type | Notes |
|--------|------|-------|
| pk_collaborator_address | UUID | PK |
| idx_collaborator_address | i64 | |
| fk_collaborator | UUID | → tb_collaborator |
| fk_address | UUID | → locations.tb_location |
| ts_*_created_at | NaiveDateTime | |
| ts_*_updated_at | NaiveDateTime | |

## Schema: `contacts`

### tb_contact

| Column | Type | Notes |
|--------|------|-------|
| pk_contact | UUID | PK |
| idx_contact | i64 | |
| tx_email | Option<String> | |
| ts_*_created_at | NaiveDateTime | |
| ts_*_updated_at | NaiveDateTime | |

### tb_phone

| Column | Type | Notes |
|--------|------|-------|
| pk_phone | UUID | PK |
| idx_phone | i64 | |
| tx_phone | String | |
| fk_contact | UUID | → tb_contact |
| ts_*_created_at | NaiveDateTime | |
| ts_*_updated_at | NaiveDateTime | |

## Schema: `locations`

### tb_location

| Column | Type | Notes |
|--------|------|-------|
| pk_location | UUID | PK |
| idx_location | i64 | |
| tx_public_space | String | ViaCEP: logradouro |
| tx_address_complement | Option<String> | ViaCEP: complemento |
| tx_unit | String | ViaCEP: unidade |
| tx_neighborhood | String | ViaCEP: bairro |
| tx_locality | String | ViaCEP: localidade |
| tx_region | String | ViaCEP: uf |
| tx_ibge | Option<String> | IBGE municipality code |
| tx_gia | Option<String> | São Paulo GIA code |
| tx_ddd | String | area code |
| tx_siafi | Option<String> | federal treasury code |
| tx_street | String | |
| tx_number | String | |
| tx_city | String | |
| tx_state | String | |
| tx_zipcode | String | |
| nr_hash | i64 | deduplication hash |
| ts_*_created_at | NaiveDateTime | |
| ts_*_updated_at | NaiveDateTime | |

## Relationships

```
client ──1:N──→ project
client ──1:1──→ client_contact ──→ contact
client ──1:N──→ client_address ──→ location

collaborator ──1:1──→ collaborator_contact ──→ contact
collaborator ──1:N──→ collaborator_address ──→ location

contact ──1:N──→ phone

project ──1:N──→ project_stage
project ──1:N──→ project_service ──→ service_type
project ──1:N──→ project_daily_allocation ──→ collaborator

client_address.location       = locations.tb_location
collaborator_address.location = locations.tb_location
project.address               = locations.tb_location
```
