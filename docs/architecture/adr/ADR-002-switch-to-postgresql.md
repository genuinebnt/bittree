# ADR-002 — Switch Primary Database from SurrealDB to PostgreSQL

**Date:** 2026-03-16
**Status:** Accepted
**Supersedes:** [ADR-001](ADR-001-surrealdb-leptos-fullstack-rust.md)
**Deciders:** @genuinebasilnt

---

## Context

ADR-001 selected SurrealDB as the primary database for its multi-model graph capabilities and LIVE SELECT feature. After planning the full 25-phase roadmap, the decision was re-evaluated against the primary project goal: **maximum learning ROI**.

The question is not which database fits the domain model better — SurrealDB's graph model does fit the block tree naturally. The question is which database teaches more transferable, career-relevant skills.

---

## Decision

Switch the primary database from **SurrealDB** to **PostgreSQL 16**, using `sqlx` as the Rust database library.

---

## Rationale

### PostgreSQL teaches more transferable skills

| Skill | With SurrealDB | With PostgreSQL |
|---|---|---|
| Used in production at scale | Rare | Everywhere |
| On backend job descriptions | Almost never | Almost always |
| Compile-time query verification | No | Yes (`sqlx` query macros) |
| MVCC + transaction isolation | Abstracted | First-class |
| Query planning (`EXPLAIN ANALYZE`) | Limited | Industry-standard |
| Schema migrations | Manual scripts | `sqlx migrate` (versioned, transactional) |
| Connection pooling | Limited | `pgBouncer`, `deadpool-postgres` |
| Real-time subscriptions | LIVE SELECT | `LISTEN/NOTIFY` (production-grade) |

### Graph algorithms are MORE educational with PostgreSQL

With SurrealDB, `SELECT ->contains->block` performs the traversal. With PostgreSQL, you load the adjacency list and write BFS/DFS/SCC/Union-Find yourself in Rust. The DSA learning objective is better served because you implement the algorithms, not just call the query.

### `sqlx` is uniquely valuable in Rust

`sqlx` verifies SQL queries against the real schema at compile time. Every `query!` macro call is a compiler error if the schema changes. No other language/DB combination provides this level of type safety at the DB boundary. This is a Rust-specific advantage that significantly improves learning value.

---

## Consequences

### Architecture changes

- **Schema layout:** one Postgres instance; one schema per service (`auth`, `users`, `docs`, `storage`, `notifications`, `analytics`, `audit`)
- **Tree structures:** adjacency list (`parent_id` column) + LTREE extension for page hierarchy path queries
- **Graph edges:** explicit join tables (`block_references`, `workspace_members`, `user_favorites`)
- **Block content:** JSONB columns (was SurrealDB objects)
- **Real-time sync:** `LISTEN/NOTIFY` (same-instance) + NATS JetStream (cross-instance, already planned)
- **Schema migrations:** `sqlx migrate` — versioned SQL files in `migrations/` per service
- **Integration tests:** `#[sqlx::test]` macro — automatically creates and tears down a real Postgres database per test function; no Docker required for unit-level integration tests

### New DSA concepts encountered via PostgreSQL

- **Recursive CTEs** — implementing tree traversal directly in SQL, understanding when to push traversal into the DB vs application layer
- **LTREE** — hierarchical path queries with GiST indexing
- **EXPLAIN ANALYZE** — reading query plans, identifying seq scans, understanding index usage
- **MVCC** — transaction isolation levels, phantom reads, write skew, serialisation anomalies
- **WAL** — write-ahead log durability guarantees (directly observable via PostgreSQL internals)
- **`LISTEN/NOTIFY`** — lightweight pub/sub, the production pattern for change notifications

### What is preserved from ADR-001

- Leptos (full-stack Rust frontend) — unchanged
- NATS JetStream as the event bus — unchanged
- Ports & adapters architecture — `PostgresPageRepo implements PageRepo`; the domain layer is unaffected
- All DSA learning targets — graph algorithms are now implemented in Rust, not delegated to the DB

### Trade-offs accepted

- Recursive CTEs for tree traversal are more verbose than SurrealQL graph queries
- Modelling graph edges requires explicit join tables (not native graph syntax)
- `pg_trgm` / `tsvector` for basic FTS (Tantivy still handles Phase 5+ full-text search)

---

## Alternatives Reconsidered

| Alternative | Reason Not Chosen |
|---|---|
| SurrealDB (ADR-001) | Lower learning ROI; skills don't transfer; graph traversal delegated to DB instead of implemented in Rust |
| CockroachDB | PostgreSQL-compatible but adds distributed complexity without added learning value at this stage |
| SQLite (for simplicity) | No production relevance for microservice backends; missing Postgres-specific concepts |

---

## Resources to Study Before Phase 0

| Resource | What to Learn |
|---|---|
| [sqlx docs](https://docs.rs/sqlx) | `query!`, `query_as!`, `FromRow`, `PgPool`, `#[sqlx::test]` |
| [sqlx GitHub examples](https://github.com/launchbadge/sqlx/tree/main/examples) | Real-world patterns for Postgres + sqlx |
| [Zero To Production In Rust — Ch. 3-5](https://www.zero2prod.com/) | Database integration with sqlx — the gold standard reference |
| [DDIA Chapter 3](https://dataintensive.net/) | B-trees, WAL, storage engine internals |
| [DDIA Chapter 7](https://dataintensive.net/) | Transaction isolation levels, MVCC, write skew |
| [PostgreSQL EXPLAIN docs](https://www.postgresql.org/docs/current/sql-explain.html) | Reading query plans |
| [PostgreSQL LTREE docs](https://www.postgresql.org/docs/current/ltree.html) | Hierarchical data with path queries |
