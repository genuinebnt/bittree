# BitTree

A collaborative, block-based note-taking application built entirely in Rust — inspired by Notion.

Documents are trees of typed blocks. Multiple users edit simultaneously. Everything is scoped to workspaces with role-based access control. Built as a structured learning project covering intermediate-to-advanced Rust, microservice architecture, distributed systems, and DSA from first principles.

---

## What it looks like (domain model)

```
Workspace
 ├── Page
 │    ├── Block (paragraph)
 │    ├── Block (heading_1)
 │    ├── Block (toggle)
 │    │    └── Block (paragraph)          ← nested children
 │    └── Block (database)
 │         ├── database_row  { Status: "Todo",  Due: 2026-04-01 }
 │         └── database_row  { Status: "Done",  Due: 2026-03-10 }
 └── Page
      └── ...
```

---

## Stack

| Layer | Technology |
|---|---|
| HTTP framework | [Axum](https://docs.rs/axum) + [Tower](https://docs.rs/tower) + [Tokio](https://tokio.rs/tokio/tutorial) |
| Database | [PostgreSQL 16](https://www.postgresql.org/) + [`sqlx`](https://docs.rs/sqlx) (compile-time checked queries, JSONB, LTREE) |
| Cache / Sessions | [Redis](https://redis.io/) via [`fred`](https://docs.rs/fred) |
| Messaging | [NATS JetStream](https://docs.nats.io/nats-concepts/jetstream) via [`async-nats`](https://docs.rs/async-nats) |
| Object storage | MinIO (local) / S3 (cloud) via `aws-sdk-s3` |
| Search | [Tantivy](https://docs.rs/tantivy) (in-process full-text) |
| Frontend | [Leptos](https://book.leptos.dev/) (SSR + WASM, full-stack Rust) |
| Expression language | BitTree Expression Language (BEL) — custom lexer + Pratt parser + type checker |
| IaC | [Pulumi Rust SDK](https://www.pulumi.com/docs/languages-sdk/rust/) |
| Observability | OpenTelemetry → Jaeger + Prometheus + Grafana |

---

## Services

| Service | Port | Responsibility |
|---|---|---|
| `api-gateway` | 8000 | JWT validation, rate limiting, circuit breaker, WebSocket routing |
| `auth-service` | 8001 | JWT (RS256), Argon2id, OAuth2 PKCE, refresh token rotation |
| `user-service` | 8002 | Profiles, workspaces, RBAC, invites |
| `document-service` | 8003 | Pages, blocks, snapshots, versioning, in-page search |
| `collaboration-service` | 8004 | WebSocket sessions, CRDT text sync, presence awareness |
| `search-service` | 8005 | Full-text search, autocomplete trie, phrase search |
| `storage-service` | 8006 | Presigned uploads, file metadata, image pipeline |
| `notification-service` | 8007 | In-app notifications, burst dedup, WebSocket push |
| `analytics-service` | 8008 | ETL pipeline, prefix-sum range queries, reservoir sampling |
| `webhook-service` | 8009 | Outbox pattern, HMAC delivery, exponential backoff |
| `audit-service` | 8010 | Append-only log, hash chaining, GDPR pseudonymisation |
| `template-service` | 8011 | Deep clone, page/workspace templates |
| `bel-service` | 8012 | BEL expression validation, autocomplete, automation rules |
| `frontend` | 3000 | Leptos SSR + WASM |

---

## Repository layout

```
bittree/
├── Cargo.toml                  ← workspace root
├── common/                     ← shared config, errors, telemetry, domain primitives
├── libs/
│   ├── shared/                 ← DTOs, newtype IDs — compiles to wasm32 + native
│   ├── bel/                    ← BitTree Expression Language (lexer/parser/eval)
│   ├── proto/                  ← protobuf definitions (tonic + prost)
│   └── test-utils/             ← TestContext, Testcontainers wrappers, mock builders
├── services/
│   ├── auth-service/
│   ├── user-service/
│   ├── document-service/
│   ├── collaboration-service/
│   ├── search-service/
│   ├── storage-service/
│   ├── notification-service/
│   ├── analytics-service/
│   ├── webhook-service/
│   ├── audit-service/
│   ├── template-service/
│   ├── bel-service/
│   └── frontend/
├── infra/                      ← Pulumi IaC (Rust SDK)
├── docs/
│   ├── planning/
│   │   ├── ROADMAP.md          ← phase-by-phase learning plan + DSA concepts map
│   │   └── FEATURE_LIST.md     ← all features with endpoints, DSA targets, learning notes
│   └── architecture/
│       ├── ARCHITECTURE.md     ← Mermaid service + event flow diagrams
│       ├── DATA_MODEL.md       ← Mermaid ER diagram + full PostgreSQL schema
│       ├── CLOUD_PORTABILITY.md← ports & adapters, local vs cloud stack
│       ├── GLOSSARY.md         ← ubiquitous language
│       └── adr/                ← Architecture Decision Records
└── docker-compose.yml          ← PostgreSQL, Redis, NATS, MinIO, Jaeger, Prometheus, Grafana
```

---

## Local dev prerequisites

- Rust (stable, latest) — install via [rustup](https://rustup.rs/)
- Docker + Docker Compose
- [`cargo-leptos`](https://github.com/leptos-rs/cargo-leptos) — `cargo install cargo-leptos`
- [`cargo-audit`](https://github.com/rustsec/rustsec/tree/main/cargo-audit) — `cargo install cargo-audit`
- [`wasm-pack`](https://rustwasm.github.io/wasm-pack/) — for WASM builds

## Start the infrastructure

```bash
docker compose up -d
```

This starts PostgreSQL, Redis, NATS, MinIO, Jaeger, Prometheus, and Grafana with defaults matching every service's `config.yaml`.

## Run a service

```bash
cargo run -p auth-service
```

## Run all tests

```bash
cargo test --workspace
```

Integration tests use `#[sqlx::test]` — automatically creates and tears down a real Postgres database per test — no Docker required for tests.

## Lint and format

```bash
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo audit
```

---

## Configuration

Each service has a single `config.yaml` with safe local defaults. Secrets and environment-specific overrides are injected via environment variables using double-underscore nesting:

```bash
APP__DATABASE__HOST=mydb.cluster.us-east-1.rds.amazonaws.com
APP__JWT__PRIVATE_KEY_PEM="$(cat private.pem)"
```

Secrets are never committed. Store them in a git-ignored `.env` file locally; use AWS SSM / Secrets Manager in production.

---

## Architecture principles

**Clean architecture** — domain logic has zero external dependencies. Infrastructure implements domain traits. Swapping PostgreSQL or NATS → SQS requires changes only in the infrastructure layer.

**Ports & adapters** — every external dependency (database, cache, broker, storage) is behind a Rust trait. The concrete implementation is wired at startup from config.

**Single config file per service** — `config.yaml` holds local defaults. Environment variables override at runtime. No `local.yaml`, no `test.yaml`. See [`docs/architecture/CLOUD_PORTABILITY.md`](docs/architecture/CLOUD_PORTABILITY.md).

**Event-driven** — services communicate via NATS JetStream domain events. HTTP is used only for synchronous request/response (user-facing API calls). See the event flow diagram in [`docs/architecture/ARCHITECTURE.md`](docs/architecture/ARCHITECTURE.md).

---

## Data model

See [`docs/architecture/DATA_MODEL.md`](docs/architecture/DATA_MODEL.md) for the full Mermaid ER diagram and PostgreSQL schema definitions.

Key design decisions:
- Block ordering uses **fractional indexing** (lexicographic sort keys on `sort_key` column) — no renumbering on insert
- Concurrent writes use **optimistic locking** (`version` field on `page` and `block`)
- Database view rows share one `block` table — views are query specifications, not data copies
- Relations between database rows are stored as explicit join/adjacency tables (e.g. `block_references`, `workspace_members`)

---

## Learning roadmap

This project is built phase-by-phase, each phase targeting specific Rust and system design concepts through real features.

| Phase | What gets built | Key concepts |
|---|---|---|
| 0 | Workspace scaffold, `common` crate, Docker Compose, CI | Cargo workspace, `config`, `thiserror`, `tracing` |
| 1 | Auth service — JWT, Argon2id, OAuth2 | Axum extractors, Tower layers, typestate, timing attacks |
| 2 | Users, workspaces, RBAC, invites | Repository trait, builder pattern, `From`/`Into` |
| 2.5 | Page permissions, guest access, page locking | Hierarchical permission resolution, capability model |
| 3 | Document service — pages, blocks, snapshots | Recursive types, arena allocation, DFS/BFS, binary search, KMP, monotonic stack |
| 4 | Real-time collaboration — CRDT, WebSocket, presence | `tokio` tasks, channels, `Arc<RwLock<T>>`, unsafe, YATA CRDT |
| 5 | Search — full-text, autocomplete trie, phrase search | Tantivy, trait objects, Aho-Corasick, sliding window |
| 6 | Storage — presigned URLs, file pipeline | `async` streams, `bytes::Bytes`, content-addressed storage |
| 7 | Notifications — in-app, WebSocket push, burst dedup | `broadcast` channels, sliding window counter, two pointers |
| 8 | API gateway — rate limiting, circuit breaker | Tower `Service`, token bucket / leaky bucket / sliding window |
| 9 | Analytics — ETL, prefix sum range queries, reservoir sampling | Iterator adapters, `rayon`, prefix sum, Algorithm R |
| 10 | Observability, Kubernetes, Pulumi IaC | OpenTelemetry, RED method, GitOps |
| 11 | Comments and inline discussions | Fan-out notifications, threaded tree traversal |
| 12 | Backlinks — BFS/DFS, SCC, Union-Find | Graph algorithms, eventual consistency |
| 12.5 | Database relations & rollups | DAG traversal, N+1 / DataLoader, type-driven aggregation |
| 13 | Database views — kanban, calendar, interval tree | CQRS, interval tree, sweep line |
| 14–19 | Templates, publish, webhooks, audit log, saga, consistent hashing | Outbox, GDPR, compensating transactions, DSU |
| 20–23 | API keys, HLL, undo/redo, import/export | Probabilistic structures, ring buffer, `nom`, Rabin-Karp |
| 24 | Full-stack frontend (Leptos) | Reactive signals, server functions, WASM, SSR hydration |
| 25 | BitTree Expression Language (BEL) | Lexer (FSM), Pratt parser, AST, type inference, tree-walking interpreter, SQL transpiler |

Full details in [`docs/planning/ROADMAP.md`](docs/planning/ROADMAP.md) and [`docs/planning/FEATURE_LIST.md`](docs/planning/FEATURE_LIST.md).

---

## DSA coverage

Every major DSA category is encountered through a real production feature — not a toy exercise.

| Category | Where in BitTree |
|---|---|
| **Trees** — DFS, BFS, trie, segment tree, interval tree, Myers diff | Block traversal, autocomplete, analytics, calendar view, snapshot diff |
| **Graphs** — SCC, cycle detection, BFS shortest path, topological sort, Union-Find | Backlinks, knowledge clusters, relation chains, workspace deletion saga |
| **Dynamic programming** — edit distance, LCS, interval DP, knapsack | Snapshot diff, CRDT undo, fractional key rebalancing, ETL scheduling |
| **Greedy** — fractional indexing, token/leaky bucket, activity selection | Block ordering, rate limiting, ETL jobs, webhook retry |
| **Strings** — KMP, Aho-Corasick, Rabin-Karp | In-page search, multi-term highlight, duplicate import detection |
| **Sliding window / two pointers** — notification dedup, rate limiting, pagination | Notification service, API gateway, search |
| **Heaps** — min-heap retry queue, k-way merge, max-heap top-N | Webhook retries, analytics partition merge, popular pages |
| **Probabilistic** — HyperLogLog, Bloom filter, Count-Min Sketch, reservoir sampling | Analytics: unique visitors, membership test, top-K, stream sampling |
| **Distributed systems** — leader election, fencing tokens, failure detectors, gossip (conceptual), Raft (conceptual), CAP/PACELC, anti-entropy, Chandy-Lamport snapshots, two-generals problem | ETL scheduler lock, collaboration scaling, backlink reconciliation, PostgreSQL replication trade-offs |
| **Lock-free & concurrent** — `Atomic*` + memory ordering, CAS loops, `crossbeam` epoch reclamation, `dashmap`, `SegQueue`, Treiber stack, seqlock | CRDT op log, session registry, rate limit counters, undo/redo stack |
| **SIMD & vectorisation** — `std::simd`, `memchr`, AVX2 intrinsics, auto-vectorisation, WASM SIMD128 | Analytics batch aggregation, BEL lexer scanning, in-page search, prefix sum |
| **Cache-conscious design** — false sharing, `#[repr(align(64))]`, SoA vs AoS, software prefetch, branch hints | Block tree traversal, rate limit counters, analytics prefix sum |
| **Memory allocators** — bump/arena (`bumpalo`), `typed-arena`, slab, pool, `MaybeUninit`, `ManuallyDrop` | CRDT op log, block tree construction, WebSocket connections |
| **Compiler** — FSM (lexer), Pratt parser, type inference, tree-walking interpreter | BitTree Expression Language (BEL) |
| **Hashing** — consistent hash ring, rolling hash, HMAC | Collaboration routing, import dedup, webhook signatures |

---

## Documentation index

| Document | What it covers |
|---|---|
| [`docs/planning/ROADMAP.md`](docs/planning/ROADMAP.md) | Phase-by-phase plan with DSA concepts map |
| [`docs/planning/FEATURE_LIST.md`](docs/planning/FEATURE_LIST.md) | All features, endpoints, DSA targets, learning notes |
| [`docs/architecture/ARCHITECTURE.md`](docs/architecture/ARCHITECTURE.md) | Service map, NATS event flow, request flow diagrams |
| [`docs/architecture/DATA_MODEL.md`](docs/architecture/DATA_MODEL.md) | Mermaid ER diagram, full PostgreSQL schema |
| [`docs/architecture/CLOUD_PORTABILITY.md`](docs/architecture/CLOUD_PORTABILITY.md) | Ports & adapters, local vs cloud stack, config strategy |
| [`docs/architecture/GLOSSARY.md`](docs/architecture/GLOSSARY.md) | Ubiquitous language — domain terms and naming conventions |
| [`docs/architecture/adr/`](docs/architecture/adr/) | Architecture Decision Records |
| [`docs/architecture/rfc/`](docs/architecture/rfc/) | Requests for Comments |

---

## Key resources

| Resource | Why it matters here |
|---|---|
| [Zero To Production In Rust](https://www.zero2prod.com/) | Production Rust web services — the closest book to what this project builds |
| [Designing Data-Intensive Applications](https://dataintensive.net/) | Distributed systems foundation — replication, CRDT, saga, event sourcing |
| [Crafting Interpreters](https://craftinginterpreters.com/) | Prerequisite for Phase 25 (BEL) — lexer, Pratt parser, interpreter |
| [Rust Design Patterns](https://rust-unofficial.github.io/patterns/) | Newtype, typestate, builder, repository — all used throughout |
| [sqlx docs](https://docs.rs/sqlx) | Primary database library — `query!`, `query_as!`, `FromRow`, `PgPool`, `#[sqlx::test]` |
| [Leptos Book](https://book.leptos.dev/) | Frontend — signals, server functions, SSR, WASM |
| [Tokio tutorial](https://tokio.rs/tokio/tutorial) | Async runtime — tasks, channels, `select!` |
| [matklad's blog](https://matklad.github.io/) | Rust idioms + the canonical Pratt parser walkthrough |
| [Axum examples](https://github.com/tokio-rs/axum/tree/main/examples) | Extractors, middleware, WebSocket patterns |
| [The Rust Performance Book](https://nnethercote.github.io/perf-book/) | Profiling, SIMD, cache-conscious design, allocator tuning — prerequisite before any low-level optimisation |
| [Crust of Rust — Atomics and Locks](https://www.youtube.com/watch?v=rMGWeSjctlY) — Jon Gjengset | `Atomic*`, memory ordering, implementing a mutex from scratch |
