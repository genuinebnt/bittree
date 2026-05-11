# BitTree — Learning Roadmap

> Each phase is a learning sprint.
>
> **Architecture:** BitTree is a **Full Microservices** system — one Rust binary per service, independently deployable, each with its own PostgreSQL schema. Services communicate asynchronously via NATS JetStream and synchronously via gRPC for selected high-frequency pairs (see ADR-003). The goal is not just to build features — it is to *encounter* and *internalise* specific Rust and system design concepts through real problems.

---

## Stack at a Glance

| Layer | Technology |
|---|---|
| Frontend | Leptos (SSR + WASM) + `cargo-leptos` |
| HTTP | Axum + Tower + Tokio |
| Database | PostgreSQL 20 (sqlx, JSONB, LTREE, recursive CTEs, native UUIDv7) |
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
| **Finite automaton (lexer)** | BEL lexer — hand-rolled state machine per token class | 14.1 |
| **Recursive descent parser** | BEL statement parser — `parse_filter`, `parse_formula` entry points | 14.2 |
| **Pratt parser (precedence climbing)** | BEL infix expression parser — binding power table for `OR < AND < NOT < cmp < +/- < */÷` | 14.2 |
| **Recursive enum / ADT** | `Expr` and `TypedExpr` — self-referential algebraic data type via `Box<Expr>` | 14.2 |
| **Post-order AST traversal** | BEL type checker — infer and propagate types bottom-up | 14.3 |
| **Type constraint propagation** | Unify `if(cond, then, else)` branches; resolve `prop()` types from schema | 14.3 |
| **NaN-boxing / tagged value repr** | `GcValue` — all VM values fit in one `u64`; inline scalars, `GcPtr` for heap objects | 14.4 |
| **Tri-color mark-and-sweep GC** | `GcHeap` — heap-allocates `String` and `List` values; write barrier; stop-the-world sweep | 14.5 |
| **Bytecode compiler** | `TypedExpr` → `Chunk { constants, code: Vec<Op> }` — type-specialized opcodes, jump fixup | 14.6 |
| **Stack machine VM** | Execute typed bytecode using GC-managed heap — `ADD_NUM`, `CONCAT_STR`, `CALL_BUILTIN` | 14.7 |
| **Tree transformation (transpiler)** | SQL transpiler — structural pattern matching on typed AST → WHERE clause | 14.8 |
| **Decision tree / discrimination tree** | Automation rules engine — skip unmatched rules without full evaluation | 14.11 |

### Trees

| Algorithm / Structure | Where in BitTree | Phase |
|---|---|---|
| **DFS (recursive + iterative)** | Traverse block tree to render a page; delete all descendants of a block | 1 |
| **BFS** | Find all pages within N levels of a root; breadth-first sidebar loading | 1 |
| **Tree deep copy** | Duplicate a page with all its blocks; clone a template | 1, 22 |
| **Trie** | Autocomplete on page titles and `@mention` lookup in search | 7 |
| **Segment tree** | Range queries on analytics events (sum edits in date range) | 18, 22 |
| **Fenwick tree (BIT)** | Prefix sum of block counts or view counts | 22 |
| **Interval tree** | Calendar view — O(log n + k) date-range overlap queries | 12 |
| **B-tree (conceptual)** | Understand PostgreSQL's on-disk B-tree index structure (used for all standard indexes) | 0 |
| **AVL / Red-Black (conceptual)** | Understand Tantivy's term index and sorted sets in Redis | 7 |
| **Rope** | Efficient string editing in CRDT text sync | 4 |

### Graphs

| Algorithm / Structure | Where in BitTree | Phase |
|---|---|---|
| **Graph DFS / BFS** | Traverse backlink graph; find all pages reachable from a given page | 8 |
| **Cycle detection** | Detect circular page references (page A links to B links to A) | 8 |
| **Topological sort** | Order saga steps by dependency; order ETL pipeline stages; rollup resolution order | 13, 18, 22 |
| **Shortest path (Dijkstra / BFS)** | "How many hops between page X and page Y?" — link distance feature | 8 |
| **Strongly connected components** | Detect clusters of heavily interlinked pages (knowledge clusters) | 8 |
| **Consistent hashing ring** | Distribute WebSocket sessions across collaboration-service instances | 20 |
| **Union-Find (Disjoint Set)** | Page connectivity queries — are A and B reachable from each other? | 8 |

### Dynamic Programming

| Algorithm / Problem | Where in BitTree | Phase |
|---|---|---|
| **Edit distance (Wagner-Fischer)** | Diff between two block tree snapshots — "what changed?" | 1 |
| **Myers diff algorithm** | Line-level diff for code blocks; snapshot diff viewer | 1 |
| **Longest common subsequence** | Merge base detection in CRDT undo (find common ancestor state) | 15 |
| **Knapsack (0-1)** | Optimal ETL batch scheduling: maximise processed events within memory budget | 18 |
| **Interval DP** | Optimal fractional index rebalancing: find the minimum re-keying operations | 1 |
| **Memoised tree traversal** | Cache subtree render results in Leptos for large block trees | 19 |
| **Memoised DAG reduce** | Rollup formula evaluation over cross-database relation chains | 13 |

### Backtracking

| Algorithm / Problem | Where in BitTree | Phase |
|---|---|---|
| **Parser combinators (`nom`)** | Import Markdown / Notion HTML export — backtrack on failed grammar rules | 22 |
| **Glob / wildcard matching** | Search filter: `page:*rust*` wildcard syntax with backtracking matcher | 7 |
| **Constraint satisfaction** | Assign roles in workspace invite: satisfy all RBAC constraints simultaneously | 3 |
| **Exhaustive path finding** | Find all paths between two pages in the backlink graph (within depth limit) | 8 |
| **Regex on block content** | Advanced search: regex match across rich-text spans | 7 |

### Greedy

| Algorithm / Problem | Where in BitTree | Phase |
|---|---|---|
| **Fractional indexing key gen** | Greedily pick the midpoint string between two sort keys | 1 |
| **Activity selection / interval scheduling** | Schedule ETL pipeline jobs to maximise throughput given time windows | 18 |
| **Huffman coding (conceptual)** | Understand Tantivy compression of repeated block content | 7 |
| **Greedy graph colouring** | Assign unique presence colours to collaborators on a page | 4 |
| **Token bucket / leaky bucket** | Rate limiting in API gateway — implement both and compare | 17 |
| **Exponential backoff with jitter** | Webhook retry scheduling — prove why pure exponential causes retry storms | 21 |
| **Sweep line** | Calendar conflict detection — sweep across sorted date intervals | 12 |

### Hash-Based Structures

| Structure | Where in BitTree | Phase |
|---|---|---|
| **HashMap / HashSet** | Block lookup by ID; dedup backlinks; notification dedup set | 1, 8 |
| **Bloom filter** | "Has this user viewed this page?" — space-efficient membership test | 22 |
| **HyperLogLog** | Approximate unique daily visitors per page | 22 |
| **Count-Min Sketch** | Top-K most edited pages without storing all counts | 22 |
| **Consistent hash ring** | Collaboration session routing (see Graphs above) | 20 |

### Heaps & Priority Queues

| Structure | Where in BitTree | Phase |
|---|---|---|
| **Min-heap** | Webhook retry queue — always process the next-due retry first | 21 |
| **Max-heap** | Top-N popular pages query without full sort | 22 |
| **Priority queue for events** | ETL pipeline: process events in `occurred_at` order across partitions | 18 |

### Sorting & Ordering

| Algorithm | Where in BitTree | Phase |
|---|---|---|
| **Merge sort** | Merge sorted block lists from multiple NATS partitions in analytics | 18 |
| **Counting sort / Radix sort** | Sort analytics events by timestamp bucket (fixed-range keys) | 18 |
| **Fractional / lexicographic ordering** | Maintain block order with string sort keys (no renumbering) | 1 |
| **External sort** | Sort analytics events larger than memory during ETL load step | 18 |

### Lock-Free & Concurrent Data Structures

> Encountered naturally as you eliminate mutex contention from hot paths. Each concept is tied to a specific performance problem you will hit.

| Structure / Concept | Where in BitTree | Phase |
|---|---|---|
| **`AtomicUsize` / `AtomicBool` + `Ordering`** | API gateway rate limit counters — understand SeqCst vs AcqRel vs Acquire/Release vs Relaxed; get it wrong and the counter races | 17 |
| **CAS loop (compare-and-swap)** | CRDT operation sequence number generator — atomically increment without a mutex | 4 |
| **`crossbeam::queue::SegQueue`** | Lock-free MPMC queue for fanout: NATS event → WebSocket connections in collaboration service | 4 |
| **`crossbeam::queue::ArrayQueue`** | Bounded lock-free ring buffer for CRDT operation batching before flush to PostgreSQL | 4 |
| **`dashmap`** | Session registry in collaboration service — many concurrent readers/writers; compare throughput to `RwLock<HashMap>` | 4, 20 |
| **Treiber stack (lock-free stack)** | Undo/redo stack in collaboration service — implement from scratch with CAS before using a library | 15 |
| **Epoch-based reclamation (`crossbeam-epoch`)** | CRDT operation log — safe concurrent access and deallocation of shared operation nodes without a GC | 4 |
| **`std::sync::atomic::fence`** | Understanding acquire/release fences — required before reasoning about any lock-free code | 4, 17 |
| **Seqlock** | Read-heavy analytics counters that rarely change — writers use a sequence number to signal readers to retry if a write races | 18 |

### Cache-Conscious Design

> Cache misses are invisible until you profile. These concepts explain why your hot paths are slow and how to fix them.

| Concept | Where in BitTree | Phase |
|---|---|---|
| **Cache line size (64 bytes) and false sharing** | Per-connection state in collaboration service — two connections on adjacent cores thrash the same cache line; fix with `#[repr(align(64))]` padding | 4, 20 |
| **Structure of Arrays (SoA) vs Array of Structures (AoS)** | Block tree traversal — accessing only `block_type` across 1000 blocks is 10× faster with SoA layout; profile both before choosing | 1 |
| **`#[repr(C)]` and `#[repr(align(N))]`** | CRDT operation structs — control layout for SIMD alignment and interop with C FFI | 4, 14 |
| **Software prefetching (`core::arch::x86_64::_mm_prefetch`)** | Analytics prefix sum — prefetch the next cache line during the scan to hide memory latency | 18 |
| **Branch prediction hints (`std::hint::likely` / `cold`)** | BEL interpreter dispatch — mark error paths as `#[cold]` so the happy path stays in the branch predictor | 14 |
| **CPU cache hierarchy (L1/L2/L3)** | Understand before optimising any hot loop — measure with `perf stat -e cache-misses` or `cargo-flamegraph` | 1, 18 |

### SIMD

> Start with scalar, profile, then reach for SIMD. Each item below has a concrete function whose inner loop is a candidate for vectorisation.

| Technique | Where in BitTree | Phase |
|---|---|---|
| **Portable SIMD (`std::simd`)** | BEL lexer — scan for special characters (`(`, `)`, `"`, operators) 16 bytes at a time instead of byte-by-byte | 14.1 |
| **SIMD byte scanning (`memchr` crate)** | In-page KMP search — use `memchr` (which emits SIMD) for the first character scan before the full pattern match | 1 |
| **Auto-vectorisation + checking assembly** | Analytics prefix sum — write the scalar loop, compile with `--release`, check the LLVM IR / `cargo-asm` output; if LLVM didn't vectorise, understand why | 18 |
| **SIMD integer arithmetic (AVX2 `_mm256_add_epi64`)** | Analytics event count aggregation — sum 4 × u64 counters per instruction instead of one | 18 |
| **Tantivy SIMD internals (conceptual)** | Before calling `searcher.search()` — read how Tantivy uses SIMD for posting list intersection and BM25 scoring; understand what you're getting for free | 7 |
| **WASM SIMD (`wasm32` target)** | BEL WASM evaluator — `std::simd` emits WASM SIMD128 instructions on `wasm32`; profile in-browser formula evaluation on large databases | 14.6 |

### Memory Allocators

> The default allocator is correct; these alternatives are faster for specific allocation patterns you will encounter.

| Allocator / Concept | Where in BitTree | Phase |
|---|---|---|
| **Bump / arena allocator (`bumpalo`)** | Block tree construction during page load — allocate all blocks into a bump arena, build the tree, then serialise; the entire arena is freed in one call | 1 |
| **`typed-arena` / `slotmap`** | CRDT operation log — operations are allocated frequently, rarely freed individually; arena gives O(1) alloc with no fragmentation | 4 |
| **Slab allocator** | WebSocket connection objects — fixed-size slots, O(1) alloc/free, no fragmentation under churn | 4, 20 |
| **Pool allocator** | NATS message buffers — pre-allocate a pool of fixed-size `Bytes` buffers; reuse across messages to avoid per-message heap allocation | 10, 18 |
| **Custom `GlobalAlloc`** | Understand the trait before Phase 21 — implement a toy counting allocator that tracks live bytes; use it in tests to assert no unexpected allocations | 22 |
| **`MaybeUninit<T>`** | CRDT rope internals — initialise a `[MaybeUninit<Node>; N]` array without writing zeros, then selectively initialise slots | 4 |
| **`ManuallyDrop<T>`** | Lock-free data structures — prevent the destructor from running on a value that has been logically transferred to another thread | 4, 15 |

### Distributed Systems Protocols & Algorithms

> These are encountered *naturally* as you scale BitTree beyond a single process. Each concept is tied to a concrete problem you will hit.

| Protocol / Concept | Where in BitTree | Phase |
|---|---|---|
| **Leader election (Redis SETNX + TTL)** | ETL scheduler: only one `analytics-service` instance runs the hourly aggregation; others stand by | 18 |
| **Distributed lock + fencing token** | Webhook delivery worker mutual exclusion — prevent two workers delivering the same outbox row; ETL job lock | 18, 21 |
| **Heartbeat + failure detector (φ accrual)** | Collaboration instance registry — instances write a heartbeat key with TTL; absence = failure; gateway rehashes | 20 |
| **Gossip protocol (conceptual)** | How NATS JetStream propagates cluster membership and stream metadata across nodes — understand before scaling NATS | 20 |
| **Raft consensus (conceptual)** | How NATS JetStream achieves durable, ordered, exactly-once delivery — read the Raft paper after Phase 4 | 4, 10 |
| **CAP theorem** | Choosing consistency model per service: page permissions → CP (must be consistent); backlink index → AP (tolerate stale) | 1, 8 |
| **PACELC theorem** | PostgreSQL replication lag trade-offs: under Partition → AP or CP; Else → Latency or Consistency — evaluate before Phase 3 | 1 |
| **Anti-entropy / read repair** | Backlink index reconciliation after a network partition — replay missed NATS events to rebuild the references graph | 8 |
| **Chandy-Lamport distributed snapshot** | Capture a consistent global state of all collaboration-service instances for crash recovery and debugging | 20 |
| **Two-generals problem (conceptual)** | Why you cannot achieve exactly-once over an async channel — the theoretical basis for at-least-once + idempotency | 10, 21 |
| **Quorum reads/writes (conceptual)** | PostgreSQL replication — understand w + r > n requirement; relevant when reasoning about RDS Multi-AZ read replicas | 1 |
| **Vector clocks / logical timestamps** | Causal ordering of CRDT operations across collaboration-service instances | 4 |
| **Write-ahead log (WAL) (conceptual)** | How PostgreSQL and NATS JetStream guarantee durability — understand before trusting their crash recovery | 0, 1 |

---


---
0 — Foundation

### What You're Building
Workspace scaffold, `libs/infra` (runtime infrastructure) and `libs/domain` (domain primitives), local dev stack, CI.

### 🎓 Prerequisite Study (Complete *before* coding this phase)
- **DDIA:** Read Chapters 1–3 (Foundations).
- **MIT 6.824:** Watch Lecture 3 (GFS) and MapReduce.
- **Gossip Glomers:** Complete Challenges 1 & 2 (Echo & Unique ID) in Rust.
- **CMU 15-445 (Andy Pavlo):** Watch B+Tree and Concurrency Control lectures to understand PostgreSQL internals.
- **Jon Gjengset:** Watch "Crust of Rust: Lifetime Annotations" (essential for tree data structures).
- **Paper:** *The Log: What every software engineer should know about real-time data's unifying abstraction* (Jay Kreps).
- **Paper:** *The Google File System* (Ghemawat et al.) — understand distributed storage fundamentals before standing up MinIO.

### Workspace Crate Layout
```
libs/infra/      telemetry bootstrap, config loading, AppError/ApiError, define_id! macro
libs/domain/     domain primitives (newtypes, DTOs, events) — wasm32-compatible
libs/bel/        BitTree Expression Language — lexer, parser, type checker, backends
libs/proto/      protobuf definitions (tonic + prost)
libs/test-utils/ Testcontainers wrappers, mock builders, TestContext
services/…       one binary crate per microservice
```

### Status
| Task | Status |
|---|---|
| Cargo workspace configured (`resolver = "3"`) | ✅ Done |
| `libs/infra` crate created | ✅ Done |
| `libs/domain` crate created | ✅ Done |
| `libs/test-utils` crate created | ✅ Done |
| `libs/proto` crate created | ✅ Done |
| `infra::telemetry` — `get_subscriber` + `init_subscriber` (bunyan JSON) | ✅ Done |
| `infra::error` — `AppError` (internal error type, `thiserror`) | ✅ Done |
| `infra::config` — generic `get_configuration::<T>()` | ✅ Done |
| `infra::macros` — `define_id!` newtype macro (`paste`) | ✅ Done |
| `docker-compose.yml` — PostgreSQL, Redis, NATS, MinIO | ✅ Done |
| `infra/init-db.sql` — initial schema bootstrap | 🔄 In progress |
| Domain ID newtypes in `libs/domain` | ⬜ Todo |
| `libs/test-utils` — `TestContext` + testcontainers setup | ⬜ Todo |
| `services/document-service` skeleton (health endpoint) | ⬜ Todo |
| CI — `cargo fmt --check` + `cargo clippy` + `cargo test` | ⬜ Todo |
| Git hooks — pre-commit fmt + clippy | ⬜ Todo |

### Rust Concepts
| Concept | Where It Appears |
|---|---|
| Cargo workspace & `[path]` deps | Linking `libs/infra` and `libs/domain` into all services |
| Feature flags (`cfg(feature)`) | Compile-time backend selection; `wasm32` gating in `libs/domain` and `libs/bel` |
| `tracing` + `tracing-subscriber` + `tracing-bunyan-formatter` | `infra::telemetry` — `get_subscriber` / `init_subscriber` |
| `config` crate + `serde` + 12-factor env override | `infra::config::get_configuration::<T>()` |
| `thiserror` | `infra::error` — `AppError` (internal error type only; HTTP boundary lives in each service) |
| Newtype pattern + `paste` macro | `define_id!` in `infra::macros` — generates `UserId`, `PageId`, etc. |
| `#[sqlx::test]` macro | Creates a real Postgres DB per test, tears it down after |

### System Design Concepts
- Monorepo vs polyrepo trade-offs
- 12-factor app configuration (env vars override file config)
- Observability: logs, metrics, traces (the three pillars)
- Error boundary design: internal errors never leak details to HTTP clients

### DevOps
- Docker multi-stage builds
- `docker compose` for local dependencies (Postgres, Redis, NATS, MinIO)
- Git hooks with `cargo fmt --check` and `cargo clippy`

---


---

## Phase 1 — Document Service

### What You're Building
The core: recursive block tree, CRUD, versioning.

### 🎓 Prerequisite Study (Complete *before* coding this phase)
- **DDIA:** Read Chapter 2 (Data Models) — adjacency list vs nested sets vs LTREE; understand why you're choosing LTREE for the block tree.
- **Zero To Production Ch. 3–5:** sqlx migrations, `#[sqlx::test]`, connection pooling — essential before writing any database queries.
- **Jon Gjengset:** Watch "Crust of Rust: Iterators" — required before implementing iterative DFS/BFS tree traversal without recursion.
- **Figma Blog:** [Realtime Editing of Ordered Sequences](https://www.figma.com/blog/realtime-editing-of-ordered-sequences/) — fractional indexing explained; read before implementing block ordering.
- **Paper:** *An O(ND) Difference Algorithm and Its Variations* (Myers, 1986) — the exact algorithm powering the snapshot diff viewer in this phase.

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
| `std::simd::u8x16`, SIMD linear scan | Block ID intern table in `libs/domain` — compare 16-byte UUIDs 16 bytes at a time; beats `HashMap` for small sets due to cache locality |

### System Design Concepts
- Tree data model in a relational DB (adjacency list vs nested sets vs LTREE)
- CRDT introduction: why distributed editing is hard
- Optimistic vs pessimistic concurrency control
- Event sourcing: append-only log of state changes
- NATS topics and event schema versioning

### Data Structures & Algorithms
- Tree traversal (DFS, BFS) — implementing without `Box` recursion
- Position encoding for ordered siblings (fractional indexing)
- Myers diff — edit distance between two block tree snapshots

---


---

## Phase 2 — Auth Service

### What You're Building
Stateless JWT auth, refresh token rotation, OAuth2 login.

### 🎓 Prerequisite Study (Complete *before* coding this phase)
- **Jon Gjengset:** Watch "Crust of Rust: Smart Pointers and Interior Mutability" (crucial for sharing state in Axum extractors).
- **Paper:** *Macaroons: Cookies with Contextual Caveats for Decentralized Authorization in the Cloud* (Birgisson et al.) — understand capability-based tokens before reaching for JWT.
- **Paper:** *OAuth 2.0 Security Best Current Practice* (IETF RFC 9700, 2023) — mandatory reading before implementing the PKCE flow; covers token leakage, redirect URI validation, and PKCE requirements.

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

---

## Phase 3 — User & Workspace Service

### What You're Building
User profiles, workspace creation, membership, RBAC.

### 🎓 Prerequisite Study (Complete *before* coding this phase)
- **Paper:** *Role-Based Access Controls* (Ferraiolo & Kuhn, NIST, 1992) — the original RBAC model; read before designing the `roles` table so you understand Owner/Admin/Editor/Viewer semantically, not just as enum variants.
- **OWASP:** [Authentication Cheat Sheet](https://cheatsheetseries.owasp.org/cheatsheets/Authentication_Cheat_Sheet.html) — covers invitation token security (crypto-random, single-use, expiry) and session management.
- **Zero To Production Ch. 6–8:** middleware, error handling in Axum — relevant before wiring up membership guards.

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


---

## Phase 4 — Collaboration Service

### What You're Building
Real-time WebSocket sessions, cursor presence, CRDT-based text sync.

### 🎓 Prerequisite Study (Complete *before* coding this phase)
- **Tokio Tutorial:** Complete the official `mini-redis` tutorial to master `tokio::spawn`, `select!`, and TCP framing before attempting WebSockets.
- **DDIA:** Read Chapter 7 (Transactions).
- **Martin Kleppmann:** Watch his Cambridge University lecture on CRDTs (YouTube).
- **Gossip Glomers:** Complete Challenge 4 (Grow-Only Counter / CRDT).
- **Paper:** *Time, Clocks, and the Ordering of Events in a Distributed System* (Leslie Lamport, 1978).
- **Paper:** *Conflict-Free Replicated Data Types* (Shapiro et al., 2011).
- **Paper:** *A Conflict-Free Replicated JSON Datatype* (Kleppmann & Beresford, 2017) — closest to BitTree's rich-text block structure; more concrete than the general CRDT survey above.

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
| `[AtomicU64; N]` bitset + CAS + `u64x4` SIMD OR | `FixedBitSet` block presence map — replace `HashSet<BlockId>` with a lock-free bitset; scan active blocks with a SIMD OR reduction |

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

### Custom Binary Wire Protocol

Replace JSON encoding of `Op` messages on the WebSocket connection with a compact hand-rolled binary format.

| Concept | Where It Appears |
|---|---|
| `bytes::Bytes` zero-copy slicing | Decode op messages directly from the incoming frame buffer — no intermediate heap copy |
| `bytemuck::cast_slice` | Zero-copy cast of homogeneous arrays (e.g., batch of `u64` cursor positions) |
| VarInt encoding | Compact variable-length integers for block offsets and text lengths |
| Binary framing (length prefix + CRC32) | Frame delimiting and integrity verification before passing to CRDT |
| `#[repr(C, packed)]` pitfalls | Alignment hazards when casting raw bytes directly to structs |

**Low-level lesson:** JSON at 1M ops/s allocates a heap string per message. A binary format with `Bytes::slice` makes zero allocations on the decode path. Measure the difference with a counting `GlobalAlloc`.

### Write-Ahead Log for Op Buffer

Persist in-flight ops to a binary WAL file before acknowledging to the client — do not rely solely on PostgreSQL for op durability.

| Concept | Where It Appears |
|---|---|
| `O_APPEND` + `sync_data()` vs `sync_all()` | Sequential WAL writes; `sync_data` flushes data pages only, `sync_all` also flushes inode metadata — measure the latency difference |
| CRC32 (`crc32fast`) | Per-record checksum to detect torn writes (partial records written before a `SIGKILL`) |
| Binary framing | Each record: `[4-byte length][op bytes][4-byte crc32]` |
| Log rotation | Rename active segment to `wal.{seq}.log` once it exceeds 64 MB; delete after PostgreSQL confirms durability |
| Crash recovery | Kill the process with `SIGKILL` mid-write in a test; verify the WAL reader skips torn records and replays all acknowledged ops |

**Low-level lesson:** Sequential appends are 10–100× faster than random PostgreSQL writes for an op log. PostgreSQL, Kafka, and NATS JetStream all use an append-only WAL as their durability primitive — this is why they all survive crashes reliably.

---

---

## Phase 5 — Dockerize & Deploy v1

### What You're Building
Containerize the services, set up CI/CD, ship it.

### 🎓 Prerequisite Study (Complete *before* coding this phase)
- **[`cargo-chef` README](https://github.com/LukeMathWalker/cargo-chef)** — Rust-specific Docker layer caching; without it your CI builds recompile all dependencies on every push.
- **Docker Docs:** [Multi-stage builds](https://docs.docker.com/build/building/multi-stage/) — understand build-stage vs runtime-stage before writing the Dockerfile.
- **[The Twelve-Factor App](https://12factor.net/)** — read all 12 factors; the config (III) and build/release/run (V) factors are directly applicable to this phase.
- **GitHub Actions Docs:** [Building and testing Rust](https://docs.github.com/en/actions/use-cases-and-examples/building-and-testing/building-and-testing-rust) — reference before writing the CI workflow.

### Concepts
| Concept | Where It Appears |
|---|---|
| Docker multi-stage builds | `cargo-chef` + distroless runtime |
| Docker Compose production | Full stack deployment |
| GitHub Actions CI/CD | fmt → clippy → test → build → push |
| Structured logging | `tracing-bunyan-formatter` JSON |
| 12-factor configuration | `config` crate + env overrides |

---

## Phase 6 — Storage Service

### What You're Building
Presigned upload URLs, file metadata, image pipelines.

### 🎓 Prerequisite Study (Complete *before* coding this phase)
- **PingCAP Talent Plan:** Complete the `kvs` (Key-Value Store) project in Rust. It teaches you how to build a networked storage engine from scratch and is the ultimate prep for this phase.
- **Paper:** *The Google File System* (Ghemawat et al.) — already read in Phase 0; revisit the section on chunk servers and write-once semantics before designing the upload pipeline.

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


---

## Phase 7 — Search Service

### What You're Building
Full-text search with Tantivy, event-driven index updates.

### 🎓 Prerequisite Study (Complete *before* coding this phase)
- **Jon Gjengset:** Watch "Crust of Rust: Channels" to understand how to safely pass indexing tasks to worker threads.
- **Paper:** *The Anatomy of a Large-Scale Hypertextual Web Search Engine* (Brin & Page, 1998) — understand inverted indexes, TF-IDF, and BM25 scoring before calling `searcher.search()`; Tantivy implements these algorithms.

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


---

## Phase 8 — Comments & Inline Discussions

### What You're Building
Block-level comment threads, `@mention` notifications, `[[page]]` backlink syntax, the bidirectional reference graph, and the anti-entropy reconciliation job that repairs it after network partitions.

### 🎓 Prerequisite Study (Complete *before* coding this phase)
- **DDIA:** Read Chapter 5 (Replication) — specifically the sections on anti-entropy, read repair, and eventual consistency; the backlink index is deliberately AP and needs a reconciliation strategy.
- **Paper:** *Epidemic Algorithms for Replicated Database Maintenance* (Demers et al., 1987) — the theoretical basis for anti-entropy; gossip-based reconciliation is the same pattern the nightly backlink reconciliation job uses.
- **CP-Algorithms:** [Disjoint Set Union](https://cp-algorithms.com/data_structures/disjoint_set_union.html) — Union-Find with path compression and union-by-rank; required before implementing the page connectivity queries (`GET /pages/connected`).
- **CP-Algorithms:** [Strongly Connected Components](https://cp-algorithms.com/graph/strongly-connected-components.html) — Tarjan's and Kosaraju's algorithms; required before building the knowledge cluster feature.

### Rust Concepts
| Concept | Where It Appears |
|---|---|
| Recursive `Box<T>` for comment trees | `Comment { replies: Vec<Comment> }` — self-referential via `Vec` |
| `HashSet` dedup | Backlink deduplication — a page linked from N blocks counts once |
| Graph algorithms on adjacency table | BFS/DFS on `block_references` join table |
| NATS async event fan-out | `BacklinkCreated` events trigger async index update |
| Union-Find (`petgraph` or hand-rolled) | Page connectivity queries — `FIND(a) == FIND(b)` |

### System Design Concepts
- Reverse index via explicit `block_references` adjacency table
- Eventual consistency for the backlink index — AP design, updates trail writes
- Anti-entropy: nightly reconciliation scan re-derives truth from block content
- Notification fan-out on `@mention` — pub/sub vs direct delivery

### Data Structures & Algorithms
- Graph DFS/BFS on `block_references` adjacency list
- Cycle detection (DFS + colour marking) for circular page references
- Strongly connected components (Tarjan's / Kosaraju's) for knowledge clusters
- Union-Find (path compression + union-by-rank) for O(α) page connectivity queries

---


---

## Phase 9 — Page-Level Permissions & Access Control

### What You're Building
Per-page permission overrides, guest access, page locking, favorites and recents.

### 🎓 Prerequisite Study (Complete *before* coding this phase)
- **OWASP:** [Access Control Cheat Sheet](https://cheatsheetseries.owasp.org/cheatsheets/Access_Control_Cheat_Sheet.html) — covers least privilege, permission escalation prevention, and the difference between authentication and authorisation.
- **Paper:** *Protection in Operating Systems* (Lampson, 1974) — the original access matrix / capability model; read before designing the permission resolution order (`page-level → workspace-role → deny`).

### Rust Concepts
| Concept | Where It Appears |
|---|---|
| Typestate pattern | Permission resolution state machine (`Unresolved` → `Resolved`) |
| `bitflags` | Capability sets for fine-grained permission checks |
| `TryFrom` | Converting permission request → validated `ResolvedPermission` |
| Caching hot paths | Redis TTL cache for (user, page) resolved permissions |
| `AtomicU64` permission word + CAS + `u64x4` | SIMD permission bitset — encode capabilities as a `u64` bitmask; batch-check 4 pages per instruction; CAS update on permission change events |

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


---

## Phase 10 — Notification Service

### What You're Building
In-app notifications, real-time delivery via WebSocket.

### 🎓 Prerequisite Study (Complete *before* coding this phase)
- **Gossip Glomers:** Complete Challenge 3 (Broadcast) in Rust to master network partition tolerance.
- **Paper:** *Epidemic Algorithms for Replicated Database Maintenance* (Demers et al.) — already read in Phase 8; revisit the fan-out section before designing notification delivery.

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


---

## Phase 11 — Observability & Monitoring

### What You're Building
Production telemetry: distributed traces, metrics scraping, RED method dashboards, SLI/SLO definitions.

### 🎓 Prerequisite Study (Complete *before* coding this phase)
- **Google SRE Book:** Read [Chapter 6 — Monitoring Distributed Systems](https://sre.google/sre-book/monitoring-distributed-systems/) — the four golden signals (latency, traffic, errors, saturation) and the difference between SLI, SLO, and SLA.
- **OpenTelemetry:** Read [Concepts](https://opentelemetry.io/docs/concepts/) — traces, spans, context propagation, and the OTel data model before wiring `tracing` to Jaeger.
- **[The Rust Performance Book](https://nnethercote.github.io/perf-book/)** — covers `cargo-flamegraph`, `perf`, `criterion`, and `tokio-console`; you will need all of these to produce meaningful performance data for the dashboards.

### Concepts
| Concept | Where It Appears |
|---|---|
| OpenTelemetry | Distributed tracing → Jaeger |
| Prometheus + Grafana | RED method dashboards |
| `tracing` spans | Per-module instrumentation |
| SLI/SLO | Targets for each module |

---


---

## Phase 12 — Database Views (Kanban, Calendar, Table, Gallery)

### What You're Building
Four projections of the same database block rows: Table, Board/Kanban, Calendar, Gallery. Calendar view uses an interval tree for date-range overlap queries; sweep line for conflict detection.

### 🎓 Prerequisite Study (Complete *before* coding this phase)
- **Martin Fowler:** [CQRS](https://martinfowler.com/bliki/CQRS.html) — understand command/query separation before designing view projections; each view is a different read model over the same write model.
- **DDIA:** Read Chapter 12 (The Future of Data Systems) — derived data, materialized views, and the relationship between CQRS and event-sourced projections.
- **CLRS Ch. 14:** Augmenting Data Structures — the interval tree section; required before implementing the Calendar view's O(log n + k) overlap query.
- **CP-Algorithms:** [Segment Tree](https://cp-algorithms.com/data_structures/segment_tree.html) — the augmented BST used for interval overlap queries.

### Rust Concepts
| Concept | Where It Appears |
|---|---|
| Trait objects for view renderers | `Box<dyn ViewRenderer>` — swap Table/Board/Calendar/Gallery at runtime |
| `HashMap`-based groupings | Kanban columns — group rows by `select` property value |
| CQRS projections | View config = query spec stored as JSONB; DB rows = write model |
| `serde` for view config evolution | Deserialise stored filter/sort config across schema versions |

### System Design Concepts
- CQRS — separate read model per view type; rebuild projections from events
- Materialized views for derived data (formula columns, rollup previews)
- Formula evaluation at query time — never store stale computed values

### Data Structures & Algorithms
- Interval tree — O(log n + k) overlap query for Calendar date-range queries
- Sweep line for conflict detection (overlapping date intervals in Calendar)
- Sorting by arbitrary property types (multi-key comparators)

---


---

## Phase 13 — Database Relations & Rollups

### What You're Building
`relation` and `rollup` property types, bidirectional cross-database links, linked database views embedded in pages, formula evaluation across related records.

### 🎓 Prerequisite Study (Complete *before* coding this phase)
- **GitHub:** [graphql/dataloader](https://github.com/graphql/dataloader) — the DataLoader pattern (batch + deduplicate within a request window); the fix for the N+1 problem you will hit immediately when loading related rows.
- **DDIA:** Read Chapter 2 (Data Models) — the joins and relationships section; understand why the relational model handles this and how to replicate it in a document-adjacent schema.
- **Paper:** *A Note on Distributed Computing* (Waldo et al., 1994) — why distributed object transparency fails; directly relevant before designing cross-database relation resolution where rows may live in different services.

### Rust Concepts
| Concept | Where It Appears |
|---|---|
| `Arc<dyn Trait>` resolvers | Multi-database row fetches — one resolver per target database |
| `futures::join_all` | Parallel fetching of related rows within one request |
| Recursive resolution with depth bound | Relation chains A → B → C — bound at 5 levels to prevent cycles |
| `HashMap` memoisation | Cache already-resolved rows within a single request to avoid re-fetching |

### System Design Concepts
- N+1 query problem and the DataLoader batching solution
- Bidirectional reference integrity — `RowDeleted` NATS event propagates to all referencing databases
- Lazy vs eager relation loading trade-offs (render time vs query count)
- Rollup computed at query time — never stored, always fresh

### Data Structures & Algorithms
- DAG traversal with cycle detection and depth bound (relation chain resolution)
- Topological sort for ordering rollup resolution steps across chained relations
- Memoised DAG reduce — cache subtree values during rollup aggregation

---


---

## Extended Phases (14–22)

| Phase | Feature | Primary Learning |
|---|---|---|
| 14 | BEL Expression Language | Compiler: lexer → parser → type checker → eval |
| 15 | Undo / Redo & History | Command pattern, CRDT undo, monotonic stack |
| 16 | ☁️ Kubernetes & IaC | K8s, HPA, Pulumi Rust SDK, GitOps |
| 17 | API Gateway & Rate Limiting | Tower Service, token/leaky/sliding window |
| 18 | Analytics & ETL | SIMD aggregation, leader election, fencing tokens |
| 19 | Frontend (Leptos) | Reactive signals, WASM, shared types |
| 20 | ☁️ Microservice Extraction | Consistent hashing, session routing |
| 21 | Webhooks & Audit Log | Outbox pattern, append-only log |
| 22 | Templates, Publishing, Import/Export | Deep clone, CDN, `nom`, HyperLogLog |

---


---

## Phase 14 — BitTree Expression Language (BEL)

### What You're Building
A strongly-typed, VM-based expression language with a mark-and-sweep garbage collector. BEL compiles source text all the way to typed bytecode and executes it on a stack machine. Heap-allocated values (`String`, `List`) are managed by a tri-color GC. One pipeline, three backends: VM execution (primary), SQL filter transpiler, and WASM (same VM compiled to `wasm32`). Powers database view filters, formula properties, automation trigger conditions, and inline formula evaluation in the browser.

### 🎓 Prerequisite Study (Complete *before* coding this phase)
- **Crafting Interpreters (Robert Nystrom):** Read **all three parts** — free at [craftinginterpreters.com](https://craftinginterpreters.com/). Part I (scanning/parsing), Part II (tree-walk, skimmed), Part III (bytecode VM + GC) is the primary reference for this phase.
- **Crafting Interpreters Ch. 26 — Garbage Collection:** The clox mark-and-sweep GC is the direct model for BEL's GC; work through it before writing a single line of `GcHeap`.
- **Amos (fasterthanli.me):** Read his deep-dive on Rust Macros if you plan to use `logos`/`nom`.
- **Paper:** *A Unified Theory of Garbage Collection* (Bacon, Cheng & Rajan, 2004) — proves that tracing GC and reference counting are mathematical duals; read before choosing an algorithm so you understand the design space.
- **Paper:** *A Theory of Type Polymorphism in Programming* (Milner, 1978) — Hindley-Milner type inference; read after building the type checker to understand the theoretical framework you're approximating.
- **Paper:** *Simple Generational Garbage Collection and Fast Allocation* (Appel, 1989) — optional; read after Phase 14.5 to understand why generational collection is the obvious next step.

### Compiler Pipeline
```
Source string (UTF-8)
    ↓ Lexer (FSM)                    → Vec<Token>                    [14.1]
    ↓ Parser (R.D. + Pratt)          → Expr (recursive enum)         [14.2]
    ↓ Type Checker (post-order walk) → TypedExpr (every node typed)  [14.3]
    ↓
    ├─ Bytecode Compiler             → Chunk { constants, Vec<Op> }  [14.6]
    │   ↓
    │  VM + GC Heap                  → GcValue (primary eval path)   [14.7]
    │   ↓
    │  WASM target                   → same VM to wasm32             [14.9]
    │
    └─ SQL Transpiler                → (WHERE fragment, bound params) [14.8]
```

### Instruction Set (typed — no runtime type dispatch)
```
// Scalars (inline, no allocation)
Nil, True, False, Const(u16)      // push constants[idx]

// Arithmetic — type resolved at compile time
AddNum, SubNum, MulNum, DivNum, NegNum
ConcatStr                          // allocates GcString on heap → triggers GC if needed

// Comparisons — type-specialized variants emitted by compiler
EqNum, EqStr, EqBool,  NeNum, NeStr
LtNum, LteNum, GtNum, GteNum
LtDate, LteDate, GtDate, GteDate

// Logic
Not, And, Or                       // And/Or emit JumpIfFalse for short-circuit

// Control flow
Jump(i16), JumpIfFalse(i16), JumpIfNull(i16)   // relative offsets; patched at compile time

// Data
LoadProp(u16)                      // push row.properties[name] — name interned in constants
CallBuiltin(u8, u8)                // builtin_id + arity; resolved at compile time, no dynamic dispatch

Pop, Return
```

### Value Representation & GC

**Phase 14.4 — Value Representation:**
Represent all VM values in a single `u64` using NaN-boxing. IEEE 754 quiet NaNs have 51 bits of payload — enough to encode every non-`f64` value type without an extra tag word.

```
f64 (number)   → stored as-is when not NaN
Null           → 0x7FFC_0000_0000_0000
Bool(false)    → 0x7FFC_0000_0000_0001
Bool(true)     → 0x7FFC_0000_0000_0002
GcPtr(u32)     → 0x7FFE_0000_PPPP_PPPP   (lower 32 bits = index into GcHeap)
```

- All values fit in a register — the VM value stack is `Vec<u64>`, not `Vec<Box<dyn Any>>`
- `GcPtr` is a 32-bit index into `GcHeap`, not a raw pointer — safe to move during compaction

**Phase 14.5 — Tri-Color Mark-and-Sweep GC:**
```
GcHeap {
    objects: Vec<GcObject>,   // the heap
    free_list: Vec<u32>,      // recycled slots
    bytes_allocated: usize,
    gc_threshold: usize,      // trigger at 2× last collection size
}

GcObject { header: GcHeader, value: GcPayload }
GcHeader { color: Color, ... }   // White | Gray | Black
GcPayload { String(String) | List(Vec<GcValue>) }
```

- **Roots:** the VM's value stack + any live `GcPtr`s in the constants table
- **Mark:** start from roots; push to gray worklist; pop gray → trace children → mark black
- **Write barrier:** when a black object stores a white pointer, re-gray the black object (prevents the tri-color invariant from being violated during incremental phases)
- **Sweep:** iterate `objects`; reclaim `White` slots back to `free_list`; reset survivors to `White`
- **Trigger:** `bytes_allocated > gc_threshold` — checked after every `ConcatStr` / `BuildList` instruction

**Key invariant:** *No black object holds a reference to a white object.* The write barrier enforces this.

### Rust Concepts
| Concept | Where It Appears |
|---|---|
| Recursive enums + `Box<T>` | `Expr` and `TypedExpr` AST nodes |
| Pattern matching | Every compiler pass |
| `thiserror` with source spans | `LexError`, `ParseError`, `TypeError` all carry `Span { start, end }` |
| `#[repr(u8)]` opcode enum | `Op` — flat bytecode array; `#[cold]` on error dispatch |
| `u64` NaN-boxing | `GcValue` — entire value stack is `Vec<u64>`; no heap allocation per value |
| `unsafe` pointer provenance | NaN-boxing tag extraction — transmute `u64` ↔ `f64`; encapsulate in safe `GcValue` API |
| `Vec<GcObject>` as arena | `GcHeap` — index-stable slot array; `free_list: Vec<u32>` for recycling |
| `wasm32`-compatible crate | `libs/bel` compiles to both native and WASM; GC works in WASM (no threads needed) |

### DSA Concepts
| Concept | Where It Appears |
|---|---|
| Finite automaton | Lexer — each byte transitions state |
| Recursive descent + Pratt | Parser — statements + infix precedence table |
| Post-order AST traversal | Type checker walks `Expr` bottom-up |
| Type constraint propagation | `if(cond, then, else)` branch unification; `prop()` type from schema |
| NaN-boxing | Value representation — all values in one `u64` |
| Tri-color mark-and-sweep | GC — white/gray/black invariant, write barrier, sweep |
| Bytecode compiler | `TypedExpr` → `Chunk`; jump offsets patched in two passes |
| Stack machine | VM — push/pop operands, typed opcodes, no dynamic dispatch |
| Tree transformation | SQL transpiler — pattern match `TypedExpr` → parameterized WHERE |
| Decision tree | Automation rules engine — skip unmatched rules without full evaluation |

### Sub-phases
| Sub-phase | What You Build |
|---|---|
| 14.1 | `libs/bel` — Lexer (FSM, `Iterator<Token>`, spans) |
| 14.2 | `libs/bel` — Parser & `Expr` AST (recursive descent + Pratt) |
| 14.3 | `libs/bel` — Type Checker (`TypedExpr`, `PropertySchema`, type errors with spans) |
| 14.4 | `libs/bel` — Value Representation (`GcValue` NaN-boxing; benchmark vs tagged union) |
| 14.5 | `libs/bel` — GC (`GcHeap`, tri-color mark-and-sweep, write barrier, threshold trigger) |
| 14.6 | `libs/bel` — Bytecode Compiler (`TypedExpr` → `Chunk`; constant folding; jump fixup) |
| 14.7 | `libs/bel` — VM (stack machine, typed dispatch, GC integration, short-circuit control flow) |
| 14.8 | `libs/bel` — SQL Transpiler (`TypedExpr` → parameterized WHERE clause; no interpolation) |
| 14.9 | `libs/bel` — WASM Build (`wasm32` target; export `bel_eval` callable from Leptos) |
| 14.10 | `bel-service` — REST API: `/bel/validate`, `/bel/explain`, `/bel/autocomplete` |
| 14.11 | `bel-service` — Automation rules engine (trigger-action, decision tree over compiled triggers) |

### Resources
| Resource | What to Learn |
|---|---|
| [Crafting Interpreters](https://craftinginterpreters.com/) — Robert Nystrom | **Part III (clox)** is the primary reference — bytecode, VM, GC; free online |
| [Pratt Parsers — Made Simple (matklad)](https://matklad.github.io/2020/04/13/simple-but-powerful-pratt-parsing.html) | The canonical Rust-flavoured Pratt explainer |
| *A Unified Theory of Garbage Collection* (Bacon et al., 2004) | Tracing vs reference counting as mathematical duals — read before choosing GC algorithm |
| [logos](https://docs.rs/logos/) crate | Lexer generator — study after hand-rolling yours to see what it generates |
| [chumsky](https://docs.rs/chumsky/) crate | Parser combinator — study after recursive descent to appreciate the abstraction |

---


---

## Phase 15 — Undo / Redo & Operation History

### What You're Building
Per-user, per-session undo/redo stack, operation collapsing (monotonic stack), server-side history log with point-in-time restore.

### 🎓 Prerequisite Study (Complete *before* coding this phase)
- **Refactoring Guru:** [Command Pattern](https://refactoring.guru/design-patterns/command) — understand how an operation becomes a first-class object that can be inverted before touching the undo stack.
- **Jon Gjengset:** Watch "Crust of Rust: Atomics and Locks" — required before implementing the lock-free Treiber stack for the undo history.
- **Paper:** *Undo Support in Cooperative Work* (Prakash & Knister, 1994) — why undo in a collaborative environment is fundamentally different from single-user undo; the theoretical basis for CRDT-based undo semantics.

### Rust Concepts
| Concept | Where It Appears |
|---|---|
| `VecDeque` for bounded history | Per-session undo ring buffer — drop oldest when depth limit reached |
| Command pattern | Each `Op` is invertible: `insert` ↔ `delete`, `format` ↔ `unformat` |
| Treiber stack (lock-free) | Undo/redo stack — implement from scratch with CAS before reaching for a library |
| `ManuallyDrop<T>` | Lock-free stack node ownership transfer across threads |
| Monotonic stack invariant | Operation collapsing — only push when new op breaks the monotone condition |

### System Design Concepts
- Client-side vs server-side undo history (who owns the stack?)
- Per-user undo in a collaborative session — other users' ops interleave
- Inverse operation semantics: an undo is just another op, sent to the collaboration service
- CRDT undo: the inverse op must be valid even if concurrent ops have changed the document

### Data Structures & Algorithms
- Stack / ring buffer for bounded undo history
- Monotonic stack — collapse adjacent compatible operations (5 char inserts → 1 word insert)
- LCS for merge-base detection in CRDT undo (find common ancestor state)

---


---

## Phase 16 — Kubernetes & Infrastructure as Code

### What You're Building
K8s manifests for all services, HPA, rolling deployments, Pulumi Rust SDK for cloud infrastructure (VPC, RDS, ElastiCache, S3, EKS), and GitHub Actions CD.

### 🎓 Prerequisite Study (Complete *before* coding this phase)
- **Kubernetes Docs:** [Concepts](https://kubernetes.io/docs/concepts/) — Pods, Deployments, Services, ConfigMaps, Secrets, HPA; read before writing a single manifest.
- **Pulumi Rust SDK Docs:** [Get started with Pulumi and Rust](https://www.pulumi.com/docs/languages-sdk/rust/) — understand how IaC in a real programming language differs from HCL/YAML.
- **Paper:** *Large-scale cluster management at Google with Borg* (Verma et al., 2015) — the direct predecessor to Kubernetes; read to understand *why* K8s makes the design choices it does (pods, resource requests, scheduling).

### Concepts
| Concept | Where It Appears |
|---|---|
| Kubernetes Deployments | One `Deployment` per service binary |
| ConfigMaps + Secrets | 12-factor config injected into pods |
| Horizontal Pod Autoscaler | Scale `collaboration-service` on WebSocket connection count |
| Rolling deployment | Health check gates before traffic shifts |
| Pulumi Rust SDK | VPC, RDS PostgreSQL, ElastiCache Redis, S3 bucket, EKS cluster |
| GitHub Actions CD | test → build → push → `kubectl rollout` |

### Cloud Concepts
- Pets vs cattle: stateless services scale horizontally; stateful services (PostgreSQL, Redis, NATS) stay outside K8s or use StatefulSets
- Resource requests vs limits — why setting both matters for the scheduler
- Ingress with TLS termination (cert-manager + Let's Encrypt)
- GitOps: the cluster state is declared in git; CI applies it

---


---

## Phase 17 — API Gateway

### What You're Building
Reverse proxy, JWT verification, rate limiting, circuit breakers.

### 🎓 Prerequisite Study (Complete *before* coding this phase)
- **CodeCrafters:** Complete "Build Your Own Redis" in Rust. Perfect prep for TCP multiplexing and rate limiting.
- **Paper:** *Your Server as a Function* (Marius Eriksen, Twitter, 2013) — the paper that inspired Rust's `tower` ecosystem; read before touching `tower::Service`.
- **Paper:** *Maglev: A Fast and Reliable Software Network Load Balancer* (Eisenbud et al., Google, 2016) — consistent hashing ring in a real production load balancer; directly relevant before Phase 20 session routing.

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


---

## Phase 18 — Analytics & ETL

### What You're Building
Event ingestion, transformation pipeline, aggregated metrics.

### 🎓 Prerequisite Study (Complete *before* coding this phase)
- **Jon Gjengset:** Watch "Crust of Rust: Async/Await" to deeply understand what Tokio is doing with your ETL tasks.
- **DDIA:** Read Chapter 9 (Consensus) and Chapter 11 (Stream Processing).
- **MIT 6.824:** Watch Lectures 6–8 (Raft).
- **Gossip Glomers:** Complete Challenge 5 (Kafka-style Log).
- **Paper:** *In Search of an Understandable Consensus Algorithm (Raft)* (Ongaro & Ousterhout, 2014).
- **Paper:** *MapReduce: Simplified Data Processing on Large Clusters* (Dean & Ghemawat, 2004).
- **Paper:** *Questioning the Lambda Architecture* (Jay Kreps, 2014) — argues for the log-centric Kappa architecture over Lambda; directly shapes the architectural choice you make in this phase.

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

### Memory-Mapped Event Log Scan

Replace `BufReader<File>` in the ETL Transform step with a `memmap2::Mmap` view of the raw event log file.

| Concept | Where It Appears |
|---|---|
| `memmap2::MmapOptions` | Read-only `Mmap` over the analytics event log — virtual address space mapping, no explicit `read()` |
| `madvise(MADV_SEQUENTIAL)` | Tell the OS to prefetch pages ahead of the scan — measure with `perf stat -e page-faults` |
| Page faults and demand paging | First access to each 4 KB page triggers a fault; the OS loads it from disk on demand |
| SIMD over `mmap` slice | Run `u64x4` aggregation directly over the memory-mapped `&[u8]` — no intermediate heap copy |
| `unsafe` + append-only invariant | `Mmap` dereference is `unsafe`; the file must not be truncated while the mapping is live |

**Low-level lesson:** `mmap` is not always faster than `BufReader` for a single sequential scan — with a large buffer they're often equivalent. `mmap` wins when the file is scanned repeatedly (OS page cache amortises I/O) or when you need zero-copy access to arbitrary slices. Benchmark `BufReader` vs `mmap` vs `mmap + MADV_SEQUENTIAL` on a 500 MB event log with `criterion` + `perf` before committing to either.

---


---

## Phase 19 — Full-Stack Frontend (Leptos)

### What You're Building
The full client: reactive page tree sidebar, block editor, collaborative cursor overlay, search modal (`Cmd+K`), database view switcher, notifications bell, settings pages, dark mode, and drag-and-drop block reordering (fractional index key generation compiled to WASM).

### 🎓 Prerequisite Study (Complete *before* coding this phase)
- **Leptos Book:** Read [The Leptos Book](https://book.leptos.dev/) — signals, server functions, SSR + hydration, and the WASM target; read before writing a single component.
- **[Leptos Examples](https://github.com/leptos-rs/leptos/tree/main/examples)** — every Leptos pattern in practice; reference while building components.
- **[cargo-leptos README](https://github.com/leptos-rs/cargo-leptos)** — hot-reload, WASM bundling, and the `--release` build pipeline; understand the toolchain before starting.
- **Paper:** *A Spreadsheet with a Formula Language* (Bricklin) — optional but illuminating on reactive cell evaluation, the conceptual ancestor of reactive signals.

### Rust Concepts
| Concept | Where It Appears |
|---|---|
| Leptos signals + derived signals | Reactive block tree — signal per block's content, derived signal for render output |
| Server functions (`#[server]`) | Data fetching without a separate API client — same Rust types on both sides |
| `wasm32` feature gating | `libs/bel` formula evaluator compiled to WASM for client-side evaluation |
| Shared DTOs via `libs/domain` | Same `Block`, `Page`, `UserId` types used in both server handlers and WASM components |
| `Suspense` + `Resource` | Async data loading with streaming SSR |
| SSR + hydration | Server renders HTML, WASM rehydrates interactivity — understand the split |

### System Design Concepts
- SSR + hydration: when the server renders, what the client rehydrates, and why the split matters for time-to-interactive
- Optimistic UI updates — apply block edits locally before the server round-trip
- WASM bundle size budget — `cargo-leptos --release` with `wasm-opt`; audit imports to keep under 500 KB
- Drag-and-drop block reordering runs fractional index key generation entirely in WASM

---


---

## Phase 20 — Distributed Session Routing

### What You're Building
Consistent hashing of WebSocket sessions across `collaboration-service` instances, instance heartbeat registry, failure detection, and Chandy-Lamport global snapshots.

### 🎓 Prerequisite Study (Complete *before* coding this phase)
- **DDIA:** Read Chapter 8 (The Trouble with Distributed Systems) — network partitions, unreliable clocks, process pauses; the theoretical grounding for every design decision in this phase.
- **Paper:** *Dynamo: Amazon's Highly Available Key-Value Store* (DeCandia et al., Amazon, 2007) — consistent hashing + virtual nodes + eventual consistency in production; the conceptual groundwork for session routing and the failure detector.
- **Paper:** *Chord: A Scalable Peer-to-peer Lookup Service for Internet Applications* (Stoica et al., 2001) — consistent hashing ring from first principles; read alongside Dynamo to understand the trade-offs.

### System Design Concepts
- Consistent hashing ring — minimal rehashing on node add/remove
- Heartbeat + TTL failure detection → φ accrual failure detector (conceptual upgrade)
- Gossip protocol for membership propagation
- Chandy-Lamport distributed snapshot — consistent global state without stopping the world

---


---

## Phase 21 — Webhooks & Audit Log

### What You're Building
Outbox-pattern webhook delivery with HMAC signing, exponential backoff retry queue, and an append-only hash-chained audit log.

### 🎓 Prerequisite Study (Complete *before* coding this phase)
- **DDIA:** Read Chapter 8 (The Trouble with Distributed Systems) — the Two Generals problem section; understand why exactly-once delivery is impossible before designing the outbox.
- **Stripe Engineering Blog:** [Idempotency Keys](https://stripe.com/blog/idempotency) — practical patterns for idempotent API design; the outbox + idempotency key is the same pattern Stripe uses for payment retries.

### System Design Concepts
- Outbox pattern — write event atomically with DB change; poll separately
- At-least-once delivery + idempotent receivers (the only achievable guarantee)
- Exponential backoff with full jitter — why pure exponential causes thundering herds
- Append-only audit log with hash chaining — tamper evidence

---


---

## Learning Objectives Summary

| Domain | Key Things Learned |
|---|---|
| **Intermediate Rust** | Traits, generics, lifetimes, error handling, `serde`, async/await |
| **Advanced Rust** | Unsafe, raw pointers, NaN-boxing (`u64` transmute), CRDT internals, Tower traits, PhantomData, arena allocation, `MaybeUninit`, `ManuallyDrop`, custom `GlobalAlloc` |
| **Garbage Collection** | Tri-color mark-and-sweep, write barrier, GC roots (VM stack), `GcPtr` index-stable heap, GC threshold + stress-test mode, stop-the-world vs incremental trade-offs |
| **Lock-Free & Concurrent** | `Atomic*` + memory ordering (SeqCst/AcqRel/Acquire/Release/Relaxed), CAS loops, `crossbeam` epoch reclamation, `dashmap`, Treiber stack, seqlock, lock-free queues |
| **SIMD & Vectorisation** | `std::simd` portable SIMD, `memchr`, auto-vectorisation, AVX2 intrinsics, WASM SIMD128, checking assembly output |
| **Cache-Conscious Design** | False sharing + `#[repr(align(64))]`, SoA vs AoS, `#[repr(C)]`, software prefetching, branch prediction hints, L1/L2/L3 cache hierarchy |
| **Memory Allocators** | Bump/arena (`bumpalo`), `typed-arena`, `slotmap`, slab, pool, custom `GlobalAlloc` |
| **Full-Stack Rust** | Leptos signals, server functions, WASM, `wasm32` feature gating, shared types |
| **Microservices** | Service decomposition, NATS events, API gateway, saga, webhooks, audit log |
| **System Design** | Caching, rate limiting, CDN, consistent hashing, circuit breakers, quotas, CQRS (read/write model separation), N+1 problem + DataLoader batching |
| **Distributed Systems** | CRDTs, vector clocks, saga, leader election, distributed locks + fencing tokens, gossip (conceptual), Raft (conceptual), CAP/PACELC, anti-entropy, Chandy-Lamport snapshots, two-generals problem, quorum, WAL |
| **Security** | Argon2id, JWT RS256, OAuth2 PKCE, CSRF, timing attacks, RBAC, API keys |
| **Cloud** | RDS PostgreSQL, S3, ElastiCache, ECS/EKS, Pulumi Rust SDK |
| **DSA — Trees** | DFS/BFS iterative, trie (autocomplete), rope (CRDT text), segment tree, Fenwick tree, interval tree (calendar), Myers diff (DP on trees) |
| **DSA — Graphs** | Cycle detection, BFS shortest path, SCC (Tarjan's/Kosaraju's), topological sort, consistent hashing ring, union-find |
| **DSA — DP** | Edit distance, Myers diff, LCS, 0-1 knapsack, interval DP (fractional index rebalancing), memoisation |
| **DSA — Backtracking** | Parser combinators (`nom`), wildcard/regex matching, exhaustive path finding with pruning |
| **DSA — Strings & Searching** | KMP (single-pattern), Aho-Corasick (multi-pattern), Rabin-Karp (rolling hash), binary search (snapshot lookup), monotonic stack (block sibling nav, undo collapsing), sliding window + two-pointer (notification dedup, rate limiting, pagination) |
| **Compiler / Language** | Lexer (FSM), recursive descent, Pratt parsing, AST design, type inference, NaN-boxing, tri-color mark-and-sweep GC, bytecode compiler, stack machine VM, SQL transpiler, WASM target |
| **DSA — Greedy** | Fractional indexing, activity selection, token/leaky bucket, greedy graph colouring, backoff with jitter, sweep line |
| **DSA — Heaps/Probabilistic** | Min/max heap, k-way merge, reservoir sampling, HyperLogLog, Bloom filter, Count-Min Sketch |
| **DevOps** | Docker multi-stage, Kubernetes HPA, CI/CD, GitOps, observability (RED/USE) |
| **Data Modelling** | PostgreSQL schema design, JSONB, LTREE, adjacency list for trees, recursive CTEs, LISTEN/NOTIFY, sqlx migrations |
| **ETL** | Append-only log, Lambda/Kappa architecture, materialized views, `nom` parsers |
