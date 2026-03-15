# BitTree — Learning Roadmap

> Each phase is a learning sprint. The goal is not just to build features — it is to *encounter* and *internalise* specific Rust and system design concepts through real problems.

---

## Stack at a Glance

| Layer | Technology |
|---|---|
| Frontend | Leptos (SSR + WASM) + `cargo-leptos` |
| HTTP | Axum + Tower + Tokio |
| Database | SurrealDB 2.x (document + graph) |
| Cache / Sessions | Redis |
| Messaging | NATS JetStream |
| Object Storage | MinIO (local) / S3 (cloud) |
| Search | Tantivy (in-process) |
| IaC | Pulumi (Rust SDK) |
| Observability | OpenTelemetry → Jaeger + Prometheus + Grafana |

---

## DSA Concepts Map

> Every major DSA category is encountered *naturally* through a real feature. This table is your checklist — not a separate exercise track, but a map of where each concept lives in BitTree.

### Trees

| Algorithm / Structure | Where in BitTree | Phase |
|---|---|---|
| **DFS (recursive + iterative)** | Traverse block tree to render a page; delete all descendants of a block | 3 |
| **BFS** | Find all pages within N levels of a root; breadth-first sidebar loading | 3 |
| **Tree deep copy** | Duplicate a page with all its blocks; clone a template | 3, 14 |
| **Trie** | Autocomplete on page titles and `@mention` lookup in search | 5 |
| **Segment tree** | Range queries on analytics events (sum edits in date range) | 9, 21 |
| **Fenwick tree (BIT)** | Prefix sum of block counts or view counts | 21 |
| **B-tree (conceptual)** | Understand SurrealDB's on-disk index structure | 0 |
| **AVL / Red-Black (conceptual)** | Understand Tantivy's term index and sorted sets in Redis | 5 |
| **Rope** | Efficient string editing in CRDT text sync | 4 |

### Graphs

| Algorithm / Structure | Where in BitTree | Phase |
|---|---|---|
| **Graph DFS / BFS** | Traverse backlink graph; find all pages reachable from a given page | 12 |
| **Cycle detection** | Detect circular page references (page A links to B links to A) | 12 |
| **Topological sort** | Order saga steps by dependency; order ETL pipeline stages | 19, 9 |
| **Shortest path (Dijkstra / BFS)** | "How many hops between page X and page Y?" — link distance feature | 12 |
| **Strongly connected components** | Detect clusters of heavily interlinked pages (knowledge clusters) | 12 |
| **Consistent hashing ring** | Distribute WebSocket sessions across collaboration-service instances | 18 |
| **Union-Find (Disjoint Set)** | Group collaboration sessions; detect isolated workspace subgraphs | 18 |

### Dynamic Programming

| Algorithm / Problem | Where in BitTree | Phase |
|---|---|---|
| **Edit distance (Wagner-Fischer)** | Diff between two block tree snapshots — "what changed?" | 3 |
| **Myers diff algorithm** | Line-level diff for code blocks; snapshot diff viewer | 3 |
| **Longest common subsequence** | Merge base detection in CRDT undo (find common ancestor state) | 22 |
| **Knapsack (0-1)** | Optimal ETL batch scheduling: maximise processed events within memory budget | 9 |
| **Interval DP** | Optimal fractional index rebalancing: find the minimum re-keying operations | 3 |
| **Memoised tree traversal** | Cache subtree render results in Leptos for large block trees | 24 |

### Backtracking

| Algorithm / Problem | Where in BitTree | Phase |
|---|---|---|
| **Parser combinators (`nom`)** | Import Markdown / Notion HTML export — backtrack on failed grammar rules | 23 |
| **Glob / wildcard matching** | Search filter: `page:*rust*` wildcard syntax with backtracking matcher | 5 |
| **Constraint satisfaction** | Assign roles in workspace invite: satisfy all RBAC constraints simultaneously | 2 |
| **Exhaustive path finding** | Find all paths between two pages in the backlink graph (within depth limit) | 12 |
| **Regex on block content** | Advanced search: regex match across rich-text spans | 5 |

### Greedy

| Algorithm / Problem | Where in BitTree | Phase |
|---|---|---|
| **Fractional indexing key gen** | Greedily pick the midpoint string between two sort keys | 3 |
| **Activity selection / interval scheduling** | Schedule ETL pipeline jobs to maximise throughput given time windows | 9 |
| **Huffman coding (conceptual)** | Understand SurrealDB / Tantivy compression of repeated block content | 5 |
| **Greedy graph colouring** | Assign unique presence colours to collaborators on a page | 4 |
| **Token bucket / leaky bucket** | Rate limiting in API gateway — implement both and compare | 8 |
| **Exponential backoff with jitter** | Webhook retry scheduling — prove why pure exponential causes retry storms | 16 |

### Hash-Based Structures

| Structure | Where in BitTree | Phase |
|---|---|---|
| **HashMap / HashSet** | Block lookup by ID; dedup backlinks; notification dedup set | 3, 12 |
| **Bloom filter** | "Has this user viewed this page?" — space-efficient membership test | 21 |
| **HyperLogLog** | Approximate unique daily visitors per page | 21 |
| **Count-Min Sketch** | Top-K most edited pages without storing all counts | 21 |
| **Consistent hash ring** | Collaboration session routing (see Graphs above) | 18 |

### Heaps & Priority Queues

| Structure | Where in BitTree | Phase |
|---|---|---|
| **Min-heap** | Webhook retry queue — always process the next-due retry first | 16 |
| **Max-heap** | Top-N popular pages query without full sort | 21 |
| **Priority queue for events** | ETL pipeline: process events in `occurred_at` order across partitions | 9 |

### Sorting & Ordering

| Algorithm | Where in BitTree | Phase |
|---|---|---|
| **Merge sort** | Merge sorted block lists from multiple NATS partitions in analytics | 9 |
| **Counting sort / Radix sort** | Sort analytics events by timestamp bucket (fixed-range keys) | 9 |
| **Fractional / lexicographic ordering** | Maintain block order with string sort keys (no renumbering) | 3 |
| **External sort** | Sort analytics events larger than memory during ETL load step | 9 |

---

## Phase 0 — Foundation (Weeks 1–2)

### What You're Building
Workspace scaffold, shared `common` crate, local dev stack, CI.

### Rust Concepts
| Concept | Where It Appears |
|---|---|
| Cargo workspace & `[path]` deps | Linking `common` and `libs/shared` into all services |
| Feature flags (`cfg(feature)`) | Compile-time backend selection; `wasm32` gating in `libs/shared` |
| `tracing` + `tracing-subscriber` | Telemetry module in `common` |
| `config` crate + `serde` | `Settings` struct in `common` |
| `thiserror` — custom error types | `common::error` module |
| Newtype pattern | `UserId(Uuid)`, `PageId(Uuid)` in `libs/shared` |
| SurrealDB embedded (`Mem`) | Fast in-process DB for tests |

### System Design Concepts
- Monorepo vs polyrepo trade-offs
- 12-factor app configuration
- Observability: logs, metrics, traces (the three pillars)

### DevOps
- Docker multi-stage builds
- `docker compose` for local dependencies
- Git hooks with `cargo fmt --check` and `cargo clippy`

---

## Phase 1 — Auth Service (Weeks 3–5)

### What You're Building
Stateless JWT auth, refresh token rotation, OAuth2 login.

### Rust Concepts
| Concept | Where It Appears |
|---|---|
| `axum` extractors (`Extension`, `Json`, `Path`) | Route handlers |
| Tower `Layer` / `Service` trait | Auth middleware, rate limiting |
| Typestate pattern | `Request<Unauthenticated>` → `Request<Authenticated>` |
| `From`/`Into`/`TryFrom` | Converting DB rows → domain types |
| Async trait objects | Repository trait (`Box<dyn AuthRepo>`) |
| Constant-time comparisons | Password and token verification |

### System Design Concepts
- JWT anatomy (header, payload, signature) — RS256 vs HS256
- Refresh token rotation and family revocation
- OAuth2 PKCE flow
- Timing attacks on authentication systems
- Redis as a token blocklist

### Security
- Argon2id parameter tuning
- Rate limiting algorithms: token bucket, sliding window counter
- CSRF protection strategies

---

## Phase 2 — User & Workspace Service (Weeks 6–8)

### What You're Building
User profiles, workspace creation, membership, RBAC.

### Rust Concepts
| Concept | Where It Appears |
|---|---|
| Builder pattern | `WorkspaceBuilder`, `InviteBuilder` |
| Repository trait | `UserRepo`, `WorkspaceRepo`, `MemberRepo` |
| `SeaORM` entities and relations | User ↔ Workspace ↔ Member join table |
| `sqlx` migrations | Schema versioning |
| Enum-based RBAC | `Role::Owner`, `Role::Admin`, etc. |
| `TryFrom` for domain validation | Validating email, slug uniqueness |

### System Design Concepts
- Multi-tenancy models (row-level, schema-level, DB-level)
- RBAC vs ABAC
- Invitation token security (crypto-random, expiry, single-use)
- Soft delete patterns

---

## Phase 3 — Document Service (Weeks 9–13)

### What You're Building
The core: recursive block tree, CRUD, versioning.

### Rust Concepts
| Concept | Where It Appears |
|---|---|
| Recursive types with `Box<T>` | `Block { children: Vec<Box<Block>> }` |
| Arena allocation (`typed-arena`, `slotmap`) | Efficient tree manipulation |
| `serde` adjacently-tagged enums | `BlockContent::Paragraph { ... }` |
| `jsonb` columns with `sqlx` | Storing block content in Postgres |
| Optimistic locking | `version` column on `Block` |
| Event sourcing basics | Publishing `BlockUpdated`, `PageCreated` events |
| Iterator adapters on trees | DFS/BFS traversal without recursion |

### System Design Concepts
- Tree data model in a relational DB (adjacency list vs nested sets vs LTREE)
- CRDT introduction: why distributed editing is hard
- Optimistic vs pessimistic concurrency control
- Event sourcing: append-only log of state changes
- NATS topics and event schema versioning

### Data Structures & Algorithms
- Tree traversal (DFS, BFS) — implementing without `Box` recursion
- Position encoding for ordered siblings (fractional indexing)

---

## Phase 4 — Collaboration Service (Weeks 14–18)

### What You're Building
Real-time WebSocket sessions, cursor presence, CRDT-based text sync.

### Rust Concepts
| Concept | Where It Appears |
|---|---|
| `tokio` tasks and `JoinHandle` | One task per WebSocket connection |
| `mpsc` + `broadcast` channels | Session ↔ connection fan-out |
| `Arc<RwLock<T>>` vs `Arc<Mutex<T>>` | Shared session state |
| `Pin<Box<dyn Stream>>` | WebSocket frame streams |
| `tokio::select!` | Multiplex incoming WS + NATS events |
| **Unsafe Rust** | CRDT rope internals (index arithmetic, raw slice ops) |
| `PhantomData` | Encoding CRDT operation ordering invariants |

### System Design Concepts
- CRDTs: G-Counter, LWW-Register, RGA/YATA for sequences
- Operational Transform (OT) vs CRDT trade-offs
- WebSocket session lifecycle and reconnect handling
- Backpressure in async message passing
- Presence protocols (awareness in Yjs)

### Data Structures & Algorithms
- YATA CRDT (Yet Another Transformation Approach)
- Rope data structure for efficient string editing
- Vector clocks and logical timestamps

---

## Phase 5 — Search Service (Weeks 19–21)

### What You're Building
Full-text search with Tantivy, event-driven index updates.

### Rust Concepts
| Concept | Where It Appears |
|---|---|
| Trait objects (`Box<dyn Trait>`) | Swappable search backend |
| Dynamic dispatch vs static dispatch | `IndexBackend` trait |
| `tantivy` internals | Schema, writer, searcher, collector |
| Thread pool (`rayon`) | Parallel indexing |
| `crossbeam` channels | Indexing worker ↔ NATS consumer |

### System Design Concepts
- Inverted index internals (postings list, TF-IDF, BM25)
- Near-real-time indexing via event stream
- Sharding and replication in search engines
- Relevance ranking and scoring

---

## Phase 6 — Storage Service (Weeks 22–23)

### What You're Building
Presigned upload URLs, file metadata, image pipelines.

### Rust Concepts
| Concept | Where It Appears |
|---|---|
| `async` streams and `tokio::io` | Streaming file uploads |
| `bytes::Bytes` | Zero-copy byte buffer handling |
| AWS SDK / `aws-sdk-rust` | S3 presigned URL generation |
| Custom `Display` | Human-readable file sizes |

### System Design Concepts
- Direct-to-storage upload (client → S3, bypass app server)
- Content-addressed storage (hash-based deduplication)
- CDN caching strategies for user-generated content
- Quota enforcement patterns

---

## Phase 7 — Notification Service (Weeks 24–25)

### What You're Building
In-app notifications, real-time delivery via WebSocket.

### Rust Concepts
| Concept | Where It Appears |
|---|---|
| `broadcast` channel | Fan-out to multiple WebSocket connections |
| `futures::StreamExt` | Combining multiple event streams |
| Idempotency via `SETNX` | Deduplication of notification delivery |

### System Design Concepts
- Fan-out-on-write vs fan-out-on-read
- At-least-once delivery and idempotency
- Dead letter queues

---

## Phase 8 — API Gateway (Weeks 26–28)

### What You're Building
Reverse proxy, JWT verification, rate limiting, circuit breakers.

### Rust Concepts
| Concept | Where It Appears |
|---|---|
| Tower `Service` trait | Every middleware layer |
| `tower::ServiceBuilder` | Composing layers |
| Type-erased services | `BoxCloneService` for dynamic routing |
| `hyper` client | Upstream proxy requests |
| Atomic counters | Lock-free rate limit counters |

### System Design Concepts
- API gateway vs service mesh
- Rate limiting algorithms: token bucket, leaky bucket, fixed/sliding window
- Circuit breaker states (closed → open → half-open)
- Distributed tracing context propagation (W3C Trace Context)

---

## Phase 9 — Analytics & ETL (Weeks 29–31)

### What You're Building
Event ingestion, transformation pipeline, aggregated metrics.

### Rust Concepts
| Concept | Where It Appears |
|---|---|
| Iterator adapters | Transformation pipeline stages |
| `rayon` parallel iterators | Parallel aggregation of large event batches |
| `serde` schema evolution | Handling old event formats gracefully |
| Scheduled async tasks | `tokio-cron-scheduler` |

### System Design Concepts
- Lambda architecture (batch + speed layer)
- Kappa architecture (stream-only, replayable)
- ETL vs ELT
- Append-only event log as source of truth
- Materialized views and CQRS projections

---

## Phase 10 — Observability & DevOps (Weeks 32–36)

### What You're Building
Production-grade telemetry, Kubernetes deployment, IaC.

### Concepts
- OpenTelemetry: traces, metrics, logs unified
- Prometheus + Grafana (RED method dashboards)
- Kubernetes: pods, deployments, services, ingress, HPA
- Pulumi Rust SDK for IaC
- GitHub Actions: multi-stage CI/CD pipeline
- SLI/SLO/SLA definitions for each service

---

## Extended Phases (Phases 11–24)

These build on the core services and are ordered by dependency, not strictly by time.

| Phase | Feature | Primary Learning |
|---|---|---|
| 11 | Comments & Discussions | Fan-out notifications, `@mention`, threaded tree |
| 12 | Backlinks & Bidirectional Refs | Graph traversal, eventual consistency, SurrealDB `RELATE` |
| 13 | Database Views (Kanban/Calendar) | CQRS projections, read models, sorting/filtering DSA |
| 14 | Page Templates | Deep tree clone, structural sharing, copy-on-write |
| 15 | Publish to Web + CDN | Cache invalidation strategies, public/private access split |
| 16 | Webhooks | Outbox pattern, at-least-once delivery, exponential backoff with jitter |
| 17 | Audit Log Service | Append-only log, event sourcing, GDPR pseudonymisation, hash chaining |
| 18 | Consistent Hashing for Collab | Consistent hashing ring, stateful scaling, session affinity |
| 19 | Saga: Workspace Deletion | Distributed transactions, saga choreography, compensating transactions |
| 20 | API Keys & External Access | OAuth2 scopes, constant-time comparison, key lifecycle |
| 21 | HyperLogLog & Approx Counting | Probabilistic data structures, Bloom filter, Count-Min Sketch |
| 22 | Undo / Redo | Command pattern, ring buffer, CRDT undo semantics |
| 23 | Import / Export Pipeline | Parser combinators (`nom`), ETL pipeline, idempotent imports |
| 24 | Full-Stack Frontend (Leptos) | Reactive signals, server functions, WASM, shared types |

---

## Learning Objectives Summary

| Domain | Key Things Learned |
|---|---|
| **Intermediate Rust** | Traits, generics, lifetimes, error handling, `serde`, async/await |
| **Advanced Rust** | Unsafe, raw pointers, CRDT internals, Tower traits, PhantomData, arena allocation |
| **Full-Stack Rust** | Leptos signals, server functions, WASM, `wasm32` feature gating, shared types |
| **Microservices** | Service decomposition, NATS events, API gateway, saga, webhooks, audit log |
| **System Design** | Caching, rate limiting, CDN, consistent hashing, circuit breakers, quotas |
| **Distributed Systems** | CRDTs, vector clocks, saga pattern, distributed locking, eventual consistency |
| **Security** | Argon2id, JWT RS256, OAuth2 PKCE, CSRF, timing attacks, RBAC, API keys |
| **Cloud** | SurrealDB Cloud, S3, ElastiCache, ECS/EKS, Pulumi Rust SDK |
| **DSA — Trees** | DFS/BFS iterative, trie (autocomplete), rope (CRDT text), segment tree, Fenwick tree, Myers diff (DP on trees) |
| **DSA — Graphs** | Cycle detection, BFS shortest path, SCC (Tarjan's/Kosaraju's), topological sort, consistent hashing ring, union-find |
| **DSA — DP** | Edit distance, Myers diff, LCS, 0-1 knapsack, interval DP (fractional index rebalancing), memoisation |
| **DSA — Backtracking** | Parser combinators (`nom`), wildcard/regex matching, exhaustive path finding with pruning |
| **DSA — Greedy** | Fractional indexing, activity selection, token/leaky bucket, greedy graph colouring, backoff with jitter |
| **DSA — Heaps/Probabilistic** | Min/max heap, k-way merge, HyperLogLog, Bloom filter, Count-Min Sketch |
| **DevOps** | Docker multi-stage, Kubernetes HPA, CI/CD, GitOps, observability (RED/USE) |
| **Data Modelling** | SurrealDB graph schema, record links, LIVE SELECT, schema evolution |
| **ETL** | Append-only log, Lambda/Kappa architecture, materialized views, `nom` parsers |
