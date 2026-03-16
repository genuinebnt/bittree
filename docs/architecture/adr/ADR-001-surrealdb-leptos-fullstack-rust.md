# ADR-001 — SurrealDB as Primary Database + Leptos for Full-Stack Rust

**Date:** 2026-03-15
**Status:** Superseded by [ADR-002](ADR-002-switch-to-postgresql.md)
**Deciders:** @genuinebasilnt

---

## Context

BitTree is a collaborative block-based note-taking app. The core data challenges are:

1. **Hierarchical block tree** — pages contain blocks that contain blocks. Deep nesting, frequent reparenting and reordering.
2. **Graph relationships** — workspaces → members → users, pages → linked pages, block references.
3. **Real-time sync** — multiple clients editing the same document need change propagation.
4. **Full-stack Rust learning goal** — the project aims to teach intermediate-to-advanced Rust across the full stack, not just the backend.

Initially, the plan was PostgreSQL (relational) + a separate frontend framework. This ADR records the decision to switch to **SurrealDB** and **Leptos**.

---

## Decision

### 1. SurrealDB as the primary data store (replacing PostgreSQL for most services)

### 2. Leptos as the full-stack Rust frontend framework (WASM + SSR)

---

## Consequences

### SurrealDB — What You Gain

| Benefit | How It Helps BitTree |
|---|---|
| **Multi-model** (document + graph + relational) | Block trees map naturally to graph edges; no LTREE extension or adjacency-list hacks |
| **Native graph traversal** | `SELECT ->contains->block FROM page:uuid` instead of recursive CTEs |
| **LIVE SELECT** | Real-time push of DB changes to clients — replaces some NATS fan-out for simple cases |
| **Record links** | `page:uuid->contains->block:uuid` — typed, traversable edges are first-class |
| **SurrealQL** | Richer query language than SQL — computed fields, subqueries, graph selectors |
| **Rust SDK** | Native async Rust client (`surrealdb` crate) with typed queries via `#[derive(Serialize, Deserialize)]` |
| **Embedded mode** | Can run `Surreal::new::<Mem>(())` for unit/integration tests without Docker |
| **Namespace / Database isolation** | One SurrealDB instance, one namespace per environment, one database per service |

### SurrealDB — What You Lose / Trade-offs

| Cost | Mitigation |
|---|---|
| **Production maturity** | SurrealDB 2.x is stable but younger than Postgres. Evaluate on each phase; keep PostgreSQL as a fallback adapter behind the `Repository` trait |
| **Ecosystem** | No SeaORM support. Use the official `surrealdb` Rust SDK with raw SurrealQL or the typed query builder |
| **Operational knowledge** | Fewer community tutorials compared to Postgres. Lean on official docs and Discord |
| **ACID at scale** | Distributed transactions work differently. Study consistency model before Phase 3 |
| **Migration tooling** | No standard migration tool like `sqlx migrate`. Use schema definition scripts versioned in `migrations/` |

### Leptos — What You Gain

| Benefit | How It Helps BitTree |
|---|---|
| **Full-stack Rust** | Share domain types, validation, and even some business logic between frontend and backend in `common` or `libs/shared` |
| **SSR + hydration** | First-paint performance with full reactivity after load |
| **Reactive signals** | Fine-grained UI updates without a VDOM diff — well-suited to real-time block editor |
| **Server functions** | `#[server]` macro calls Axum handlers directly — blurs the client/server boundary cleanly |
| **WebAssembly** | Compiles to WASM — no JavaScript runtime needed for the UI logic |
| **Type safety end-to-end** | Request/response types shared between Leptos server functions and Axum handlers |

### Leptos — What You Lose / Trade-offs

| Cost | Mitigation |
|---|---|
| **Ecosystem immaturity** | Fewer UI component libraries than React/Vue. Build primitives yourself (good for learning) |
| **Compile times** | WASM builds are slower. Use `cargo-leptos` with `--hot-reload` for dev |
| **Learning curve** | Reactive signals are a different mental model. Study [Leptos book](https://book.leptos.dev/) before starting Phase 3 frontend |
| **WASM bundle size** | Use `wasm-opt` and route-based code splitting |

---

## Architecture Impact

### New Service: `frontend`

A new crate `services/frontend` runs as a Leptos SSR application (using `axum` as the underlying server). It:
- Serves the initial HTML (SSR)
- Hydrates on the client (WASM)
- Calls backend services via `#[server]` functions or direct REST/WebSocket
- Shares types from `libs/shared` crate

### Shared Types Crate: `libs/shared`

A new `libs/shared` crate (compiled for both native and `wasm32` targets) contains:
- Domain newtype wrappers (`PageId`, `BlockId`, `UserId`)
- Request/response DTO structs
- Validation logic (e.g., `Email::try_from`)
- Shared error types

### SurrealDB Schema Pattern

Instead of SQL `CREATE TABLE`, use SurrealQL definitions:

```surql
-- Example (for illustration — you write the actual SurrealQL)
DEFINE TABLE user SCHEMAFULL;
DEFINE FIELD email ON user TYPE string ASSERT string::is::email($value);
DEFINE FIELD display_name ON user TYPE string;

DEFINE TABLE page SCHEMAFULL;
DEFINE FIELD title ON page TYPE string;
DEFINE FIELD workspace ON page TYPE record<workspace>;

DEFINE TABLE contains SCHEMALESS;  -- graph edge: page->contains->block
DEFINE TABLE child_of SCHEMALESS;  -- graph edge: block->child_of->block
```

### Adapter Trait Remains

The `Repository` trait pattern is preserved. The concrete implementation changes from `PostgresPageRepo` to `SurrealPageRepo`, but the domain never knows the difference.

---

## Alternatives Considered

| Alternative | Reason Rejected |
|---|---|
| PostgreSQL + LTREE | Works well but misses the graph learning opportunity; no full-stack Rust story |
| PostgreSQL + JSONB for blocks | Querying nested JSON is awkward; SurrealDB's graph model is more natural |
| MongoDB | Not Rust-native, misses the graph model, weaker type safety |
| React/TypeScript frontend | Breaks the full-stack Rust learning goal |
| Dioxus | Also good, but Leptos has stronger SSR story and `#[server]` integration |
| EdgeDB | Interesting, but smaller community and no embedded mode for tests |

---

## Resources to Study Before Implementing

| Resource | What to Learn |
|---|---|
| [SurrealDB Docs — Rust SDK](https://surrealdb.com/docs/sdk/rust) | Connection, queries, typed records |
| [SurrealDB Docs — SurrealQL](https://surrealdb.com/docs/surrealql) | Graph traversal, LIVE SELECT, schema definition |
| [SurrealDB Docs — Graph Relations](https://surrealdb.com/docs/surrealql/statements/relate) | `RELATE`, `->`, `<-` syntax |
| [Leptos Book](https://book.leptos.dev/) | Signals, resources, server functions, SSR |
| [Leptos Examples](https://github.com/leptos-rs/leptos/tree/main/examples) | Real patterns for every Leptos feature |
| [cargo-leptos](https://github.com/leptos-rs/cargo-leptos) | Build tool for Leptos (hot reload, WASM bundling) |
| [Sharing types between Leptos client/server](https://book.leptos.dev/server/25_server_functions.html) | Server functions and type sharing |

---

## Follow-up Actions

- [ ] Create `libs/shared` crate (wasm32-compatible)
- [ ] Create `services/frontend` crate (Leptos SSR)
- [ ] Update Docker Compose to include `surrealdb/surrealdb` container
- [ ] Add `surrealdb` crate to `common` or `libs/surrealdb-store`
- [ ] Write ADR-002 for NATS vs SurrealDB LIVE SELECT (decide when to use each)
- [ ] Update `CLOUD_PORTABILITY.md` with SurrealDB Cloud / self-hosted options
