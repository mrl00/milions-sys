# Non-Functional Requirements

Quality attributes and operational constraints for milions-sys.

---

## Performance

| Requirement | Target |
|-------------|--------|
| P95 latency (single entity read) | < 100ms |
| P95 latency (list endpoints) | < 300ms |
| P95 latency (aggregate/report) | < 500ms |
| Throughput | 500 req/s sustained |
| DB connection pool | 10 connections (configurable) |

### Notes

- No N+1 queries — use joins or batch fetches for aggregates
- Paginate all list endpoints (default 20, max 100)
- BigDecimal queries should avoid full table scans (indexed columns only)

---

## Availability

| Requirement | Target |
|-------------|--------|
| Uptime | 99.5% (no SLA, internal tool) |
| Graceful shutdown | Drain in-flight requests on SIGTERM, max 30s |
| Health check | `GET /health` — returns 200 if process alive |

### Notes

- This is an internal system, not customer-facing — high availability is not critical
- Database downtime = service downtime (no offline queue)

---

## Scalability

| Axis | Strategy |
|------|----------|
| Horizontal | Stateless binary — scale via multiple instances behind LB |
| Vertical | Tokio async runtime — efficient for I/O-bound workloads |
| Database | Connection pooling via sqlx — shared pool per instance |

### Constraints

- No distributed transactions — single Postgres instance
- No read replicas — single source of truth

---

## Observability

### Logging

- Structured logging via `tracing` crate
- Log levels: `ERROR` (failures), `WARN` (retries, slow queries), `INFO` (startup, shutdown), `DEBUG` (request lifecycle)
- Include in every log: `request_id`, `method`, `path`, `status`, `duration_ms`
- No PII in logs (no CPF, email, phone values)

### Metrics (future)

- Request count by endpoint + status
- Request duration histogram
- DB query duration
- Pool utilization

### Health

- `GET /health` — process liveness (no dependency check)

---

## Security

| Requirement | Detail |
|-------------|--------|
| TLS | Terminate at reverse proxy (nginx/traefik), not in binary |
| Secrets | Via env vars, never in config files or logs |
| Database credentials | `secrecy::SecretString` — redacted in Debug |
| CORS | Restrict to known frontends (configurable origins) |
| Rate limiting | Out of scope — handled by infra |
| Auth | JWT via Keycloak (see `05-security.md`) — standby |

### Constraints

- `.env` file must be in `.gitignore`
- No hardcoded credentials in source code
- `APP_DATABASE__PASSWORD` never logged

---

## Reliability

| Requirement | Detail |
|-------------|--------|
| Migrations | Applied at startup via `sqlx::migrate!()` |
| Migration failure | Fatal — abort startup |
| DB connection failure | Fatal — abort startup (fail fast) |
| Lazy connection | Not allowed — validate pool on boot |

### Error Handling

- All user-facing errors return JSON: `{ "error": "code", "message": "human readable" }`
- Internal errors return `500` with generic message — details in logs only
- Validation errors return `422` with field-level messages

---

## Maintainability

| Requirement | Detail |
|-------------|--------|
| Rust edition | 2024 |
| Clippy | Deny warnings in CI |
| Format | `rustfmt` enforced |
| Dead code | Allowed during development (`dead_code = "allow"`) |
| Tests | Unit tests per crate |
| CI | `cargo check`, `cargo clippy`, `cargo test`, `cargo fmt --check` |

### Documentation

- No rustdoc requirement — code is self-documenting via naming
- `.ai/` directory serves as living documentation
- `PROJECT_OVERVIEW.md` kept in sync with architecture changes

---

## Deployment

| Aspect | Detail |
|--------|--------|
| Container | Dockerfile in repo root |
| Base image | Multi-stage build (rust builder + debian slim runtime) |
| Env config | All via environment variables (`APP_*`) |
| DB | External Postgres — not containerized in production |
| Migrations | Run on container start, not as separate step |

### Environment Tiers

| Environment | Config file | DB |
|-------------|-------------|-----|
| Development | `files/app_config/base.yaml` + `development.yaml` | Local Postgres |
| Production | `files/app_config/base.yaml` + `production.yaml` | Managed Postgres |

---

## Data

| Requirement | Detail |
|-------------|--------|
| Primary keys | UUID v7 (time-ordered) |
| Timestamps | `NaiveDateTime` via chrono, set by DB or app |
| Soft delete | Not implemented — hard delete |
| Pagination | Cursor-based or offset (decide per endpoint) |
| Encoding | UTF-8 throughout |
| Timezone | UTC in DB, convert on API boundary if needed |

---

## Internationalization

- Error messages in **Brazilian Portuguese** (pt-BR)
- No i18n framework — messages are hardcoded
- API field names in English (e.g., `name`, `status`, `created_at`)
