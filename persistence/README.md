# Pulsgram Persistance Backend

Structured backend foundation with a focus on clean persistence architecture, strong database guarantees, and long-term maintainability.

---

## Overview

This project is designed as a long-term backend foundation intended to support:

- SaaS-style services
- Telegram automation backends
- Infrastructure tooling
- Experimental systems (trading, analytics, etc.)

The goal is not rapid feature iteration, but **architectural durability**.

---

## Persistence Layer

The persistence layer is intentionally structured and hardened:

- PostgreSQL
- `sqlx` for async database access
- Repository pattern (per-table repositories)
- Central `Repositories` container
- Explicit SQL queries (no `SELECT *`)
- Strong DB constraints (PRIMARY KEY, UNIQUE, NOT NULL)
- Integration tests using `#[sqlx::test]`
- Isolated `PersistenceError` boundary (no `sqlx::Error` leakage)

### Design Goals

- No panics in production code
- No SQL leakage outside repositories
- Explicit error mapping
- Deterministic CRUD behavior
- Migration discipline

---

## Testing Strategy

Integration tests run against a temporary PostgreSQL database created per test using:

```rust
#[sqlx::test]
```