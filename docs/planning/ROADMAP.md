# BitTree — Learning Roadmap

> Each phase is a learning sprint. The goal is not just to build features — it is to *encounter* and *internalise* specific Rust and system design concepts through real problems.

---

## Stack at a Glance

| Layer | Technology |
|---|---|
| Frontend | Leptos (SSR + WASM) + `cargo-leptos` |
| HTTP | Axum + Tower + Tokio |
| Database | PostgreSQL 16 (sqlx, JSONB, LTREE, recursive CTEs) |
| Cache / Sessions | Redis |
| Messaging | NATS JetStream |
| Object Storage | MinIO (local) / S3 (cloud) |
| Search | Tantivy (in-process) |
| IaC | Pulumi (Rust SDK) |
| gRPC | tonic + prost (api-gateway↔auth, doc↔collab, analytics ETL) |
| Observability | OpenTelemetry → Jaeger + Prometheus + Grafana |

---

## DSA Concepts Map

> Every major DSA category is encountered *naturally* through a real feature. This table is your checklist — not a separate exercise track, but a map of where each concept lives in BitTree.

### Compiler / Language

| Algorithm / Structure | Where in BitTree | Phase |
|---|---|---|
| **Finite automaton (lexer)** | BEL lexer — hand-rolled state machine per token class | 25.1 |
| **Recursive descent parser** | BEL statement parser — `parse_filter`, `parse_formula` entry points | 25.2 |
| **Pratt parser (precedence climbing)** | BEL infix expression parser — binding power table for `OR < AND < NOT < cmp < +/- < */÷` | 25.2 |
| **Recursive enum / ADT** | `Expr` and `TypedExpr` — self-referential algebraic data type via `Box<Expr>` | 25.2 |
| **Post-order AST traversal** | BEL type checker — infer and propagate types bottom-up | 25.3 |
| **Type constraint propagation** | Unify `if(cond, then, else)` branches; resolve `prop()` types from schema | 25.3 |
| **Tree transformation (transpiler)** | SQL transpiler — structural pattern matching on typed AST → WHERE clause | 25.4 |
| **Tree-walking interpreter** | Formula evaluator — recursive eval with short-circuit semantics | 25.5 |
| **Decision tree / discrimination tree** | Automation rules engine — skip unmatched rules without full evaluation | 25.8 |

### Trees

| Algorithm / Structure | Where in BitTree | Phase |
|---|---|---|
| **DFS (recursive + iterative)** | Traverse block tree to render a page; delete all descendants of a block | 3 |
| **BFS** | Find all pages within N levels of a root; breadth-first sidebar loading | 3 |
| **Tree deep copy** | Duplicate a page with all its blocks; clone a template | 3, 14 |
| **Trie** | Autocomplete on page titles and `@mention` lookup in search | 5 |
| **Segment tree** | Range queries on analytics events (sum edits in date range) | 9, 21 |
| **Fenwick tree (BIT)** | Prefix sum of block counts or view counts | 21 |
| **B-tree (conceptual)** | Understand PostgreSQL's on-disk B-tree index structure (used for all standard indexes) | 0 |
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
| **Huffman coding (conceptual)** | Understand Tantivy compression of repeated block content | 5 |
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

### Lock-Free & Concurrent Data Structures

> Encountered naturally as you eliminate mutex contention from hot paths. Each concept is tied to a specific performance problem you will hit.

| Structure / Concept | Where in BitTree | Phase |
|---|---|---|
| **`AtomicUsize` / `AtomicBool` + `Ordering`** | API gateway rate limit counters — understand SeqCst vs AcqRel vs Acquire/Release vs Relaxed; get it wrong and the counter races | 8 |
| **CAS loop (compare-and-swap)** | CRDT operation sequence number generator — atomically increment without a mutex | 4 |
| **`crossbeam::queue::SegQueue`** | Lock-free MPMC queue for fanout: NATS event → WebSocket connections in collaboration service | 4 |
| **`crossbeam::queue::ArrayQueue`** | Bounded lock-free ring buffer for CRDT operation batching before flush to PostgreSQL | 4 |
| **`dashmap`** | Session registry in collaboration service — many concurrent readers/writers; compare throughput to `RwLock<HashMap>` | 4, 18 |
| **Treiber stack (lock-free stack)** | Undo/redo stack in collaboration service — implement from scratch with CAS before using a library | 22 |
| **Epoch-based reclamation (`crossbeam-epoch`)** | CRDT operation log — safe concurrent access and deallocation of shared operation nodes without a GC | 4 |
| **`std::sync::atomic::fence`** | Understanding acquire/release fences — required before reasoning about any lock-free code | 4, 8 |
| **Seqlock** | Read-heavy analytics counters that rarely change — writers use a sequence number to signal readers to retry if a write races | 9 |

### Cache-Conscious Design

> Cache misses are invisible until you profile. These concepts explain why your hot paths are slow and how to fix them.

| Concept | Where in BitTree | Phase |
|---|---|---|
| **Cache line size (64 bytes) and false sharing** | Per-connection state in collaboration service — two connections on adjacent cores thrash the same cache line; fix with `#[repr(align(64))]` padding | 4, 18 |
| **Structure of Arrays (SoA) vs Array of Structures (AoS)** | Block tree traversal — accessing only `block_type` across 1000 blocks is 10× faster with SoA layout; profile both before choosing | 3 |
| **`#[repr(C)]` and `#[repr(align(N))]`** | CRDT operation structs — control layout for SIMD alignment and interop with C FFI | 4, 25 |
| **Software prefetching (`core::arch::x86_64::_mm_prefetch`)** | Analytics prefix sum — prefetch the next cache line during the scan to hide memory latency | 9 |
| **Branch prediction hints (`std::hint::likely` / `cold`)** | BEL interpreter dispatch — mark error paths as `#[cold]` so the happy path stays in the branch predictor | 25 |
| **CPU cache hierarchy (L1/L2/L3)** | Understand before optimising any hot loop — measure with `perf stat -e cache-misses` or `cargo-flamegraph` | 3, 9 |

### SIMD

> Start with scalar, profile, then reach for SIMD. Each item below has a concrete function whose inner loop is a candidate for vectorisation.

| Technique | Where in BitTree | Phase |
|---|---|---|
| **Portable SIMD (`std::simd`)** | BEL lexer — scan for special characters (`(`, `)`, `"`, operators) 16 bytes at a time instead of byte-by-byte | 25.1 |
| **SIMD byte scanning (`memchr` crate)** | In-page KMP search — use `memchr` (which emits SIMD) for the first character scan before the full pattern match | 3 |
| **Auto-vectorisation + checking assembly** | Analytics prefix sum — write the scalar loop, compile with `--release`, check the LLVM IR / `cargo-asm` output; if LLVM didn't vectorise, understand why | 9 |
| **SIMD integer arithmetic (AVX2 `_mm256_add_epi64`)** | Analytics event count aggregation — sum 4 × u64 counters per instruction instead of one | 9 |
| **Tantivy SIMD internals (conceptual)** | Before calling `searcher.search()` — read how Tantivy uses SIMD for posting list intersection and BM25 scoring; understand what you're getting for free | 5 |
| **WASM SIMD (`wasm32` target)** | BEL WASM evaluator — `std::simd` emits WASM SIMD128 instructions on `wasm32`; profile in-browser formula evaluation on large databases | 25.6 |

### Memory Allocators

> The default allocator is correct; these alternatives are faster for specific allocation patterns you will encounter.

| Allocator / Concept | Where in BitTree | Phase |
|---|---|---|
| **Bump / arena allocator (`bumpalo`)** | Block tree construction during page load — allocate all blocks into a bump arena, build the tree, then serialise; the entire arena is freed in one call | 3 |
| **`typed-arena` / `slotmap`** | CRDT operation log — operations are allocated frequently, rarely freed individually; arena gives O(1) alloc with no fragmentation | 4 |
| **Slab allocator** | WebSocket connection objects — fixed-size slots, O(1) alloc/free, no fragmentation under churn | 4, 18 |
| **Pool allocator** | NATS message buffers — pre-allocate a pool of fixed-size `Bytes` buffers; reuse across messages to avoid per-message heap allocation | 7, 9 |
| **Custom `GlobalAlloc`** | Understand the trait before Phase 21 — implement a toy counting allocator that tracks live bytes; use it in tests to assert no unexpected allocations | 21 |
| **`MaybeUninit<T>`** | CRDT rope internals — initialise a `[MaybeUninit<Node>; N]` array without writing zeros, then selectively initialise slots | 4 |
| **`ManuallyDrop<T>`** | Lock-free data structures — prevent the destructor from running on a value that has been logically transferred to another thread | 4, 22 |

### Distributed Systems Protocols & Algorithms

> These are encountered *naturally* as you scale BitTree beyond a single process. Each concept is tied to a concrete problem you will hit.

| Protocol / Concept | Where in BitTree | Phase |
|---|---|---|
| **Leader election (Redis SETNX + TTL)** | ETL scheduler: only one `analytics-service` instance runs the hourly aggregation; others stand by | 9 |
| **Distributed lock + fencing token** | Webhook delivery worker mutual exclusion — prevent two workers delivering the same outbox row; ETL job lock | 9, 16 |
| **Heartbeat + failure detector (φ accrual)** | Collaboration instance registry — instances write a heartbeat key with TTL; absence = failure; gateway rehashes | 18 |
| **Gossip protocol (conceptual)** | How NATS JetStream propagates cluster membership and stream metadata across nodes — understand before scaling NATS | 18 |
| **Raft consensus (conceptual)** | How NATS JetStream achieves durable, ordered, exactly-once delivery — read the Raft paper after Phase 4 | 4, 7 |
| **CAP theorem** | Choosing consistency model per service: page permissions → CP (must be consistent); backlink index → AP (tolerate stale) | 3, 12 |
| **PACELC theorem** | PostgreSQL replication lag trade-offs: under Partition → AP or CP; Else → Latency or Consistency — evaluate before Phase 3 | 3 |
| **Anti-entropy / read repair** | Backlink index reconciliation after a network partition — replay missed NATS events to rebuild the references graph | 12 |
| **Chandy-Lamport distributed snapshot** | Capture a consistent global state of all collaboration-service instances for crash recovery and debugging | 18 |
| **Two-generals problem (conceptual)** | Why you cannot achieve exactly-once over an async channel — the theoretical basis for at-least-once + idempotency | 7, 16 |
| **Quorum reads/writes (conceptual)** | PostgreSQL replication — understand w + r > n requirement; relevant when reasoning about RDS Multi-AZ read replicas | 3 |
| **Vector clocks / logical timestamps** | Causal ordering of CRDT operations across collaboration-service instances | 4 |
| **Write-ahead log (WAL) (conceptual)** | How PostgreSQL and NATS JetStream guarantee durability — understand before trusting their crash recovery | 0, 3 |

---

## Phase 0 — Foundation (Weeks 1–2)

### What You're Building
Workspace scaffold, shared `common` crate, local dev stack, CI.

### Workspace Crate Layout
```
common/          shared config, telemetry, error types, newtype macros
libs/shared/     domain primitives (newtypes, DTOs) — wasm32-compatible
libs/bel/        BitTree Expression Language — lexer, parser, type checker, backends
libs/proto/      protobuf definitions (tonic + prost)
libs/test-utils/ Testcontainers wrappers, mock builders, TestContext
services/…       one binary crate per microservice
```

### Rust Concepts
| Concept | Where It Appears |
|---|---|
| Cargo workspace & `[path]` deps | Linking `common` and `libs/shared` into all services |
| Feature flags (`cfg(feature)`) | Compile-time backend selection; `wasm32` gating in `libs/shared` and `libs/bel` |
| `tracing` + `tracing-subscriber` | Telemetry module in `common` |
| `config` crate + `serde` | `Settings` struct in `common` |
| `thiserror` — custom error types | `common::error` module |
| Newtype pattern | `UserId(Uuid)`, `PageId(Uuid)` in `libs/shared` |
| `#[sqlx::test]` macro | Creates a real Postgres DB per test, tears it down after |

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
| `tonic` server, proto3 schema, gRPC unary RPC | `ValidateToken` RPC called by `api-gateway` on every request (`libs/proto/proto/auth.proto`) |

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
| `sqlx` typed queries, compile-time query checking | User ↔ Workspace ↔ Member join table |
| `sqlx migrate` migrations | Schema versioning |
| Enum-based RBAC | `Role::Owner`, `Role::Admin`, etc. |
| `TryFrom` for domain validation | Validating email, slug uniqueness |

### System Design Concepts
- Multi-tenancy models (row-level, schema-level, DB-level)
- RBAC vs ABAC
- Invitation token security (crypto-random, expiry, single-use)
- Soft delete patterns

---

## Phase 2.5 — Page-Level Permissions & Access Control (Weeks 8–9)

### What You're Building
Per-page permission overrides, guest access, page locking, favorites and recents.

### Rust Concepts
| Concept | Where It Appears |
|---|---|
| Typestate pattern | Permission resolution state machine (`Unresolved` → `Resolved`) |
| `bitflags` | Capability sets for fine-grained permission checks |
| `TryFrom` | Converting permission request → validated `ResolvedPermission` |
| Caching hot paths | Redis TTL cache for (user, page) resolved permissions |

### System Design Concepts
- Hierarchical permission inheritance (page → parent page → workspace)
- Capability-based vs role-based access (when each is appropriate)
- Cache invalidation on permission change events
- Guest access without workspace membership — scoped JWTs

### Security
- Principle of least privilege: most-specific permission wins
- Preventing permission escalation via forged page-permission grants
- Guest token single-use expiry and revocation

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
| `tonic` bidirectional streaming, gRPC interceptors | `SyncOps` bidi streaming RPC — `document-service` ↔ `collaboration-service` op delivery (`libs/proto/proto/collab.proto`) |

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
| `tonic` client-streaming, batched ingestion | `IngestEvents` client-streaming RPC — ETL batch push to ingestion endpoint (`libs/proto/proto/analytics.proto`) |

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

## Extended Phases (Phases 11–25)

These build on the core services and are ordered by dependency, not strictly by time.

| Phase | Feature | Primary Learning |
|---|---|---|
| 11 | Comments & Discussions | Fan-out notifications, `@mention`, threaded tree |
| 12 | Backlinks & Bidirectional Refs | Graph traversal, eventual consistency, `block_references` adjacency table |
| 12.5 | Database Relations & Rollups | DAG traversal, N+1 / DataLoader, relation integrity, formula eval |
| 13 | Database Views (Kanban/Calendar) | CQRS projections, interval tree, sweep line, sorting/filtering DSA |
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
| 25 | BitTree Expression Language (BEL) | Compiler pipeline: lexer → parser → type checker → multi-backend eval |

---

## Phase 25 — BitTree Expression Language (BEL)

### What You're Building
A small, safe, statically-typed expression language embedded in BitTree. One parser, four evaluation backends: SQL filter transpiler, tree-walking formula interpreter, WASM client-side evaluator, and a search query parser. Powers database view filters, formula properties, automation trigger conditions, and structured search.

### Rust Concepts
| Concept | Where It Appears |
|---|---|
| Recursive enums + `Box<T>` | `Expr` and `TypedExpr` AST nodes |
| Pattern matching on enum variants | Every pass of the compiler pipeline |
| `thiserror` with source spans | `LexError`, `ParseError`, `TypeError`, `EvalError` all carry `Span { start, end }` |
| `Display` for pretty-printing | Error messages with source highlights |
| `From`/`Into` for IR lowering | `Expr` → `TypedExpr` during type checking |
| `wasm32`-compatible crate | `libs/bel` compiles to both native and WASM; no I/O or threads |
| `Iterator` on lexer | `Lexer` implements `Iterator<Item = Result<Token, LexError>>` |
| Newtype for `Span` | `Span { start: usize, end: usize }` on every token and AST node |

### Compiler Pipeline
```
Source string
    ↓ Lexer (finite automaton)       → Vec<Token>
    ↓ Parser (recursive descent      → Expr (recursive enum, Box<Expr>)
             + Pratt for precedence)
    ↓ Type Checker (post-order walk) → TypedExpr (every node annotated)
    ↓
    ├─ SQL Transpiler                → (WHERE fragment, bound params)
    ├─ Tree-Walking Interpreter      → Value (formula eval on a database row)
    ├─ WASM Evaluator                → same interpreter, compiled to wasm32
    └─ Search Query Parser           → structured search spec
```

### DSA Concepts
| Concept | Where It Appears |
|---|---|
| Finite automaton | Lexer state machine — each character transitions state |
| Recursive descent | Statement-level parser (`parse_filter`, `parse_formula`) |
| Pratt parsing (precedence climbing) | Infix expression parser — binding power table |
| Post-order AST traversal | Type checker walks bottom-up, infers types |
| Type constraint propagation | `if(cond, then, else)` branch unification |
| Tree transformation | SQL transpiler — AST → target IR via structural pattern matching |
| Tree-walking interpreter | Formula evaluator with short-circuit semantics |
| Decision tree | Automation rules engine — skip unmatched rules fast |

### Sub-phases
| Sub-phase | What You Build |
|---|---|
| 25.1 | `libs/bel` — Lexer (FSM, `Iterator<Token>`, spans) |
| 25.2 | `libs/bel` — Parser & `Expr` AST (recursive descent + Pratt) |
| 25.3 | `libs/bel` — Type Checker (`TypedExpr`, `PropertySchema`, type errors) |
| 25.4 | `libs/bel` — SQL Transpiler (filter → parameterised WHERE clause) |
| 25.5 | `libs/bel` — Tree-Walking Interpreter (formula eval on `DatabaseRow`) |
| 25.6 | `libs/bel` — WASM build; export `bel_eval` callable from Leptos |
| 25.7 | `bel-service` — REST API: `/bel/validate`, `/bel/explain`, `/bel/autocomplete` |
| 25.8 | `bel-service` — Automation rules engine (trigger-action, decision tree) |

### Resources
| Resource | What to Learn |
|---|---|
| [Crafting Interpreters](https://craftinginterpreters.com/) — Robert Nystrom | Full compiler pipeline from scanning through evaluation; free online |
| [Pratt Parsers — Made Simple (matklad)](https://matklad.github.io/2020/04/13/simple-but-powerful-pratt-parsing.html) | The canonical Rust-flavoured Pratt explainer |
| [logos](https://docs.rs/logos/) crate | Lexer generator — study after hand-rolling yours to understand what it generates |
| [chumsky](https://docs.rs/chumsky/) crate | Parser combinator library — study after writing recursive descent to appreciate the abstraction |

---

## Learning Objectives Summary

| Domain | Key Things Learned |
|---|---|
| **Intermediate Rust** | Traits, generics, lifetimes, error handling, `serde`, async/await |
| **Advanced Rust** | Unsafe, raw pointers, CRDT internals, Tower traits, PhantomData, arena allocation, `MaybeUninit`, `ManuallyDrop`, custom `GlobalAlloc` |
| **Lock-Free & Concurrent** | `Atomic*` + memory ordering (SeqCst/AcqRel/Acquire/Release/Relaxed), CAS loops, `crossbeam` epoch reclamation, `dashmap`, Treiber stack, seqlock, lock-free queues |
| **SIMD & Vectorisation** | `std::simd` portable SIMD, `memchr`, auto-vectorisation, AVX2 intrinsics, WASM SIMD128, checking assembly output |
| **Cache-Conscious Design** | False sharing + `#[repr(align(64))]`, SoA vs AoS, `#[repr(C)]`, software prefetching, branch prediction hints, L1/L2/L3 cache hierarchy |
| **Memory Allocators** | Bump/arena (`bumpalo`), `typed-arena`, `slotmap`, slab, pool, custom `GlobalAlloc` |
| **Full-Stack Rust** | Leptos signals, server functions, WASM, `wasm32` feature gating, shared types |
| **Microservices** | Service decomposition, NATS events, API gateway, saga, webhooks, audit log |
| **System Design** | Caching, rate limiting, CDN, consistent hashing, circuit breakers, quotas |
| **Distributed Systems** | CRDTs, vector clocks, saga, leader election, distributed locks + fencing tokens, gossip (conceptual), Raft (conceptual), CAP/PACELC, anti-entropy, Chandy-Lamport snapshots, two-generals problem, quorum, WAL |
| **Security** | Argon2id, JWT RS256, OAuth2 PKCE, CSRF, timing attacks, RBAC, API keys |
| **Cloud** | RDS PostgreSQL, S3, ElastiCache, ECS/EKS, Pulumi Rust SDK |
| **DSA — Trees** | DFS/BFS iterative, trie (autocomplete), rope (CRDT text), segment tree, Fenwick tree, Myers diff (DP on trees) |
| **DSA — Graphs** | Cycle detection, BFS shortest path, SCC (Tarjan's/Kosaraju's), topological sort, consistent hashing ring, union-find |
| **DSA — DP** | Edit distance, Myers diff, LCS, 0-1 knapsack, interval DP (fractional index rebalancing), memoisation |
| **DSA — Backtracking** | Parser combinators (`nom`), wildcard/regex matching, exhaustive path finding with pruning |
| **Compiler / Language** | Lexer (FSM), recursive descent, Pratt parsing, AST design, type inference, tree-walking interpreter, SQL transpiler, WASM-compatible crate |
| **DSA — Greedy** | Fractional indexing, activity selection, token/leaky bucket, greedy graph colouring, backoff with jitter |
| **DSA — Heaps/Probabilistic** | Min/max heap, k-way merge, HyperLogLog, Bloom filter, Count-Min Sketch |
| **DevOps** | Docker multi-stage, Kubernetes HPA, CI/CD, GitOps, observability (RED/USE) |
| **Data Modelling** | PostgreSQL schema design, JSONB, LTREE, adjacency list for trees, recursive CTEs, LISTEN/NOTIFY, sqlx migrations |
| **ETL** | Append-only log, Lambda/Kappa architecture, materialized views, `nom` parsers |
