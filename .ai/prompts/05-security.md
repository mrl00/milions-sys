# Security

JWT validation strategy for milions-sys.

## Status: Standby

This document is not to be implemented now. It serves as a reference for future integration.

---

## Overview

Lightweight JWT validation using a middleware that verifies tokens issued by Keycloak. No role-based access control, no token introspection — only signature and issuer verification.

## Flow

```
Client → actix-web middleware → route handler
              │
              ├─ Extract `Authorization: Bearer <token>` header
              ├─ Decode JWT header to get `kid`
              ├─ Fetch Keycloak JWKS (cached)
              ├─ Validate signature (RS256)
              ├─ Validate claims: iss, exp, nbf
              └─ If valid → proceed. If not → 401 Unauthorized
```

## Keycloak Endpoints

| Purpose | URL Pattern |
|---------|-------------|
| JWKS | `{keycloak_url}/realms/{realm}/protocol/openid-connect/certs` |
| Issuer | `{keycloak_url}/realms/{realm}` |
| Discovery | `{keycloak_url}/realms/{realm}/.well-known/openid-configuration` |

## Configuration

Add to `settings` crate:

```rust
pub struct KeycloakSettings {
    pub url: String,           // e.g. "https://keycloak.example.com"
    pub realm: String,         // e.g. "milions"
}
```

Environment variables:
- `APP_KEYCLOAK__URL`
- `APP_KEYCLOAK__REALM`

## Middleware Design

### Requirements

- Async middleware (actix-web `Transform` / `Service`)
- JWKS fetched once at startup, refreshed on `kid` miss (with TTL cache, e.g. 10 min)
- No database calls
- No role/permission checks
- Rejects requests with missing or invalid tokens → `401`
- Passes no user data downstream (lean — no identity injection)

### Validation Rules

| Claim | Rule |
|-------|------|
| `iss` | Must equal `{keycloak_url}/realms/{realm}` |
| `exp` | Must be in the future |
| `nbf` | Must be in the past (if present) |
| `alg` | Must be `RS256` |
| `typ` | Must be `JWT` |

### Rejected Responses

```
401 Unauthorized
{
  "error": "unauthorized",
  "message": "Token inválido ou expirado"
}
```

## Dependencies (when implemented)

| Crate | Purpose |
|-------|---------|
| `jsonwebtoken` | JWT decode + signature verification |
| `reqwest` | Fetch JWKS from Keycloak |
| `jwks-client` | JWKS parsing and key resolution (optional — can use `jsonwebtoken` directly) |

## Crate Placement

```
security/
  Cargo.toml
  src/
    lib.rs
    middleware.rs       actix-web middleware impl
    jwks.rs             JWKS fetcher + cache
    token.rs            Claims struct + validation
```

- New crate `security` in workspace
- Depends on: `actix-web`, `jsonwebtoken`, `reqwest`, `serde`
- Does NOT depend on any bounded context
- Bounded contexts do NOT import `security` — the middleware is wired in `src/startup.rs`

## Wiring

In `src/startup.rs`:

```rust
use security::JwtMiddleware;

let server = HttpServer::new(move || {
    App::new()
        .wrap(JwtMiddleware::new(keycloak_settings.clone()))
        .service(health_check)           // public — excluded from auth
        // .service(...)                 // protected routes
        .app_data(web::Data::new(pool.clone()))
})
```

### Excluding Routes

The middleware should skip validation for:

- `GET /health` — health check (no auth)
- Future: `POST /api/auth/*` — if login proxy is added

Pass excluded paths as configuration to the middleware.

## Open Questions (resolve before implementation)

- [ ] Which Keycloak realm + client to use?
- [ ] Should JWKS cache use in-memory TTL or external cache (Redis)?
- [ ] Should health check and future auth routes be excluded by path prefix or exact match?
- [ ] Is `reqwest` acceptable as HTTP client, or use `ureq` for sync simplicity?
