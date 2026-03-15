# BitTree — Feature List

> **What is BitTree?**
> A collaborative, block-based note-taking application inspired by Notion. Documents are composed of a tree of typed blocks (paragraphs, headings, lists, code, embeds, databases). Multiple users can edit the same document simultaneously. Everything is organized into workspaces with role-based access control.

---

## Services Overview

| Service | Port | Responsibility |
|---|---|---|
| `api-gateway` | 8000 | Routing, rate limiting, auth verification, request fan-out |
| `auth-service` | 8001 | JWT issuance, OAuth2, sessions, token refresh |
| `user-service` | 8002 | User profiles, workspace membership, RBAC |
| `document-service` | 8003 | Pages, blocks, tree CRUD, version snapshots |
| `collaboration-service` | 8004 | Real-time WebSocket sessions, CRDT sync |
| `search-service` | 8005 | Full-text and property-based search |
| `storage-service` | 8006 | File uploads, presigned URLs, object metadata |
| `notification-service` | 8007 | In-app and push notifications, event fan-out |
| `analytics-service` | 8008 | Usage events, ETL pipeline, aggregation |
| `webhook-service` | 8009 | External webhook delivery with retry and backoff |
| `audit-service` | 8010 | Append-only audit log, compliance queries |
| `template-service` | 8011 | Page/workspace templates, deep-clone operations |
| `bel-service` | 8012 | BEL expression lexer, parser, type checker, evaluator |
| `frontend` | 3000 | Leptos SSR + WASM full-stack UI |

---

## Phase 0 — Foundation & Tooling

> **Rust concepts:** Workspace, crate organization, feature flags, `tracing`, `config`, `thiserror`/`anyhow`

- [x] Cargo workspace scaffold (`common`, `libs/`, `services/`)
- [ ] `common` crate: shared config, error types, telemetry, newtype macros, domain primitives
- [ ] `libs/proto` crate: protobuf definitions (tonic + prost)
- [ ] `libs/test-utils` crate: test helpers, `Testcontainers` wrappers, mock builders
- [ ] Docker Compose: SurrealDB, Redis, NATS, MinIO, Jaeger, Prometheus, Grafana
- [ ] CI pipeline: `cargo fmt`, `cargo clippy`, `cargo test`, `cargo audit`
- [ ] Git hooks: pre-commit lint + format

---

## Phase 1 — Auth Service

> **Rust concepts:** `axum` extractors, `Tower` middleware, `thiserror` error hierarchy, newtype pattern, typestate for auth flows
> **System design:** Stateless JWT, refresh token rotation, OAuth2 PKCE, timing-safe comparisons

### Core
- [ ] `POST /auth/register` — email + password registration (Argon2id hashing)
- [ ] `POST /auth/login` — credential validation, JWT + refresh token issuance
- [ ] `POST /auth/refresh` — rotate refresh token, issue new access token
- [ ] `POST /auth/logout` — revoke refresh token (add to Redis blocklist)
- [ ] `GET  /auth/me` — decode JWT, return claims

### OAuth2
- [ ] `GET  /auth/oauth/:provider` — redirect to GitHub / Google
- [ ] `GET  /auth/oauth/:provider/callback` — exchange code, issue tokens

### Security
- [ ] Argon2id password hashing (`argon2` crate)
- [ ] JWT RS256 signing (asymmetric keys, `jsonwebtoken` crate)
- [ ] Refresh token family rotation (detect reuse = full family revocation)
- [ ] Rate limiting on login endpoint (Tower layer, Redis sliding window)
- [ ] CSRF protection on cookie-based flows

---

## Phase 2 — User & Workspace Service

> **Rust concepts:** Repository trait pattern, `From`/`Into` conversions, builder pattern, `SeaORM` entities
> **System design:** Multi-tenancy, RBAC, invitation flows

### Users
- [ ] `GET  /users/me` — current user profile
- [ ] `PATCH /users/me` — update display name, avatar
- [ ] `DELETE /users/me` — soft delete, cascade workspace membership removal

### Workspaces
- [ ] `POST /workspaces` — create workspace (creator becomes Owner)
- [ ] `GET  /workspaces` — list workspaces for current user
- [ ] `GET  /workspaces/:id` — workspace detail + members
- [ ] `PATCH /workspaces/:id` — update name, icon (Owner/Admin only)
- [ ] `DELETE /workspaces/:id` — soft delete (Owner only)

### Membership & Invites
- [ ] `POST /workspaces/:id/invites` — send invite link/email
- [ ] `POST /workspaces/:id/invites/:token/accept` — join via invite token
- [ ] `PATCH /workspaces/:id/members/:user_id` — change role (Owner/Admin only)
- [ ] `DELETE /workspaces/:id/members/:user_id` — remove member

### Roles
| Role | Can Read | Can Edit | Can Invite | Can Manage | Can Delete Workspace |
|---|---|---|---|---|---|
| Viewer | ✓ | | | | |
| Commenter | ✓ | comments only | | | |
| Editor | ✓ | ✓ | | | |
| Admin | ✓ | ✓ | ✓ | ✓ | |
| Owner | ✓ | ✓ | ✓ | ✓ | ✓ |

---

## Phase 2.5 — Page-Level Permissions & Access Control

> **Rust concepts:** Typestate for permission resolution, `bitflags` for capability sets, `From`/`TryFrom` for permission models
> **System design:** Hierarchical permission inheritance, capability-based access, guest access patterns
> **Security:** Principle of least privilege, permission escalation prevention

Every document-service read/write path gates on the resolved permission for (user, page) — not just (user, workspace). This phase must be complete before Phase 3 is usable in production.

### Permission Model

Permissions resolve in this order (most specific wins):

```
Page-level override  →  Team Space role  →  Workspace role  →  Guest grant  →  Deny
```

| Permission level | Who |
|---|---|
| `private` | Creator only — invisible to all other workspace members |
| `workspace` | Inherits workspace RBAC role (default) |
| `custom` | Specific members/groups assigned explicit roles on this page |
| `public` | Unauthenticated read (covered in Phase 15) |

### Page Permissions API
- [ ] `GET  /pages/:id/permissions` — list all explicit grants on a page
- [ ] `PUT  /pages/:id/permissions` — set page visibility (`private`, `workspace`, `custom`)
- [ ] `POST /pages/:id/permissions/members` — grant a specific member a role on this page
- [ ] `DELETE /pages/:id/permissions/members/:user_id` — revoke a member's page-level override
- [ ] Permission inheritance: child pages inherit parent page permissions by default
- [ ] `POST /pages/:id/permissions/inherit` — reset page to inherit from parent (remove overrides)

### Guest Access
- [ ] `POST /pages/:id/guests` — invite an external email to a single page (no workspace membership required)
- [ ] Guest receives a magic-link email (single-use token, expires in 7 days)
- [ ] Guest JWT has `scope: guest` claim and is restricted to allowed page IDs only
- [ ] `GET  /pages/:id/guests` — list active guest grants
- [ ] `DELETE /pages/:id/guests/:guest_id` — revoke guest access
- [ ] Guest access does not appear in workspace member list

### Page Locking
- [ ] `POST /pages/:id/lock` — lock page (Owner/Admin only); prevents all edits including by Editors
- [ ] `DELETE /pages/:id/lock` — unlock page
- [ ] Locked status returned in `GET /pages/:id` response
- [ ] Collaboration-service rejects `Op` messages for locked pages

### Favorites & Recents
- [ ] `POST /pages/:id/favorite` — star a page for the current user
- [ ] `DELETE /pages/:id/favorite` — unstar
- [ ] `GET /users/me/favorites` — ordered list of starred pages
- [ ] `GET /users/me/recents` — last 20 pages visited (written to Redis sorted set on every `GET /pages/:id`)

### Private Pages
- [ ] `PATCH /pages/:id` accepts `visibility: "private"` — hides page from all workspace members except creator
- [ ] `GET /workspaces/:wid/pages` must filter out pages the requester cannot see
- [ ] Private pages do not appear in search results for other users

**Security lesson:** Permission resolution is a hot path — every document read calls it. Explore caching resolved permissions in Redis with a short TTL and invalidating on permission change events.

---

## Phase 3 — Document Service

> **Rust concepts:** Recursive tree types, `Box<T>`, arena allocation, `serde` with enums/adjacently-tagged, `sqlx` or `SeaORM` with `jsonb`
> **System design:** Block-tree data model, optimistic locking, event sourcing basics, CRDT preparation

### Pages
- [ ] `POST /workspaces/:wid/pages` — create root page
- [ ] `GET  /workspaces/:wid/pages` — list top-level pages (sidebar tree)
- [ ] `GET  /pages/:id` — fetch page with full block tree
- [ ] `PATCH /pages/:id` — update title, icon, cover, parent
- [ ] `DELETE /pages/:id` — soft delete + cascade child pages
- [ ] `POST /pages/:id/duplicate` — deep clone page + blocks

### Blocks
- [ ] `POST /pages/:id/blocks` — insert block at position
- [ ] `GET  /blocks/:id` — fetch block + children
- [ ] `PATCH /blocks/:id` — update block content/properties
- [ ] `DELETE /blocks/:id` — delete block + subtree
- [ ] `POST /blocks/:id/move` — reparent block, reorder siblings
- [ ] `POST /blocks/:id/convert` — change block type (paragraph → heading)

### Block Types
| Type | Content | Has Children |
|---|---|---|
| `paragraph` | Rich text spans | No |
| `heading_1/2/3` | Rich text | No |
| `bulleted_list` | Rich text | Yes |
| `numbered_list` | Rich text | Yes |
| `toggle` | Rich text (summary) | Yes |
| `quote` | Rich text | No |
| `callout` | icon + rich text | No |
| `code` | plain text + language | No |
| `divider` | — | No |
| `image` | storage_url + caption | No |
| `file` | storage_url + name | No |
| `embed` | url + preview_url | No |
| `bookmark` | url + OG title/description/image (fetched server-side via oEmbed/OG scrape) | No |
| `equation` | LaTeX string (rendered client-side via KaTeX) | No |
| `table_of_contents` | Auto-generated from `heading_1/2/3` blocks on page save — no stored content, derived at read time | No |
| `breadcrumb` | No stored content — resolved at read time from page ancestor chain | No |
| `column_list` | — | Yes (must contain only `column` children) |
| `column` | fractional width (0.0–1.0) | Yes (any block type) |
| `synced_block` | `source_block_id` — points to the canonical block; `null` if this IS the canonical | Yes |
| `sub_page` | child `page_id` — renders as an inline page link card | No |
| `table` | — | Yes (rows) |
| `table_row` | — | Yes (cells) |
| `database` | schema JSON | Yes (rows) |
| `database_row` | property values JSON | Yes (sub-blocks) |

### Database Property Types

All properties are stored as a typed schema on the `database` block and as typed values on each `database_row`.

| Property type | Value shape | Notes |
|---|---|---|
| `text` | String | |
| `number` | f64 + optional format (integer, percent, currency) | |
| `checkbox` | bool | |
| `url` | String | validated on write |
| `email` | String | validated on write |
| `phone` | String | |
| `select` | option_id (single) | Options list stored in database schema |
| `multi_select` | Vec\<option_id\> | |
| `status` | option_id | Opinionated select: Not Started / In Progress / Done groups |
| `date` | ISO-8601 + optional end date + timezone | |
| `person` | Vec\<UserId\> | Must be workspace members |
| `files` | Vec\<FileId\> | Ties to storage-service |
| `formula` | expression string | Evaluated server-side at query time; typed output |
| `relation` | Vec\<RowId\> from a target database | **Phase 12.5** |
| `rollup` | aggregation config over a relation | **Phase 12.5** |
| `created_time` | auto-populated | Read-only |
| `last_edited_time` | auto-populated | Read-only |
| `created_by` | UserId, auto-populated | Read-only |
| `last_edited_by` | UserId, auto-populated | Read-only |
| `unique_id` | auto-incrementing integer per database | Read-only |

### Additional Block Endpoints
- [ ] `POST /blocks/:id/sync` — create a synced copy of a block in another page (sets `source_block_id`)
- [ ] `GET  /blocks/:id/sync-instances` — list all synced copies of a canonical block
- [ ] `POST /blocks/:id/unsync` — detach a synced copy (copy content, remove `source_block_id`)
- [ ] Synced block write path: writes to canonical block, fans out to all instances via NATS event
- [ ] `GET  /pages/:id/toc` — derive table of contents from heading blocks (no DB read — pure tree traversal)

### Versioning & Diff
- [ ] Block-level change log (who changed what, when)
- [ ] Page snapshot at configurable intervals (for restore)
- [ ] `GET /pages/:id/history` — list snapshots
- [ ] `POST /pages/:id/history/:snapshot_id/restore`
- [ ] `GET /pages/:id/history?before=:timestamp` — **binary search** the snapshot log to find the most recent snapshot before a given time (**binary search**)
- [ ] `GET /pages/:id/diff?from=:snap_a&to=:snap_b` — Myers diff between two snapshots (**DP**)
- [ ] Fractional index rebalancer: when sort keys exceed max length, rebalance with minimum re-keys (**interval DP**)

### Block Navigation
- [ ] `GET /blocks/:id/prev-sibling` and `GET /blocks/:id/next-sibling` — find adjacent sibling at same nesting level without a full parent scan (**monotonic stack** over the flattened block sequence)
- [ ] `GET /pages/:id/blocks/flat` — flattened depth-first ordered list with nesting level attached, used by the frontend virtual list renderer

### In-Page Search
- [ ] `GET /pages/:id/search?q=:phrase` — exact phrase search within a single page's block content (**KMP** for single-pattern; **Rabin-Karp** rolling hash for multi-pattern highlight)
- [ ] Results include block ID + character offset of each match for frontend highlight

### L1 In-Process Cache
- [ ] Per-service `moka` cache for hot page and block reads — implement with **LFU** eviction (stable popular pages stay resident longer than recently-accessed-once pages)
- [ ] Per-service session/permission cache — implement with **LRU** eviction (recency matters more than frequency for session data)
- [ ] Cache invalidation: NATS `BlockUpdated` / `PageUpdated` events evict the relevant key

---

## Phase 4 — Collaboration Service

> **Rust concepts:** `tokio` tasks, channels (`mpsc`, `broadcast`), `Arc<RwLock<T>>`, unsafe + raw pointers for CRDT internals, `Pin`/`Unpin`
> **System design:** CRDTs (YATA / RGA), operational transform trade-offs, WebSocket session management, presence

### Real-Time
- [ ] `WS /collaboration/pages/:id` — WebSocket endpoint per page
- [ ] Session registry (active users per document) in Redis
- [ ] Presence: broadcast cursor positions and selection ranges
- [ ] CRDT-based text merging for rich-text spans (Automerge-rs or custom YATA)
- [ ] Awareness protocol (user name, color, cursor) over WebSocket

### Sync Protocol
- [ ] Client sends `Op` messages (insert, delete, format)
- [ ] Server applies to authoritative state, broadcasts to peers
- [ ] Client reconnect: catchup via ops since `last_seen_seq`
- [ ] Conflict-free merge on reconnect (no user-visible merge conflicts)

---

## Phase 5 — Search Service

> **Rust concepts:** Trait objects, `Box<dyn Trait>`, dynamic dispatch, `tantivy` index internals, thread pools
> **System design:** Inverted index, TF-IDF, incremental indexing via events
> **DSA:** Trie (autocomplete), KMP (phrase search), sliding window (dedup), binary search (term lookup)

- [ ] Full-text search across pages and blocks (Tantivy TF-IDF / BM25)
- [ ] Filter by workspace, page, block type, author, date range
- [ ] `GET /search?q=...&workspace_id=...` — paginated results
- [ ] Indexing worker: consume NATS events from document-service
- [ ] Re-index endpoint (admin): rebuild from document-service snapshot
- [ ] Tantivy-based local index (swap to Meilisearch/OpenSearch in cloud)

### Autocomplete (Trie)
- [ ] In-memory **trie** over all page titles and workspace member display names, rebuilt on `PageCreated` / `PageRenamed` / `MemberJoined` NATS events
- [ ] `GET /search/autocomplete?q=:prefix&workspace_id=...` — returns up to 10 prefix matches in O(k) where k = number of matches (**trie prefix search**)
- [ ] `GET /search/mention?q=:prefix&workspace_id=...` — same trie, filtered to members only (powers `@mention` inline suggestions)
- [ ] Trie nodes store a `Vec<(score, record_id)>` sorted by recency + access frequency so the most relevant result is always first

### Phrase & Exact Match Search
- [ ] `GET /search/phrase?q=:exact_phrase&workspace_id=...` — exact phrase match across all block content using **KMP** (single phrase) or **Aho-Corasick** (multiple phrases simultaneously, e.g. all search terms highlighted at once)
- [ ] Results deduped within a sliding 100ms window — if the same block matches multiple overlapping queries, coalesce them (**sliding window dedup**)

### Query Optimisation
- [ ] Tantivy's term dictionary is a sorted list; use **binary search** to locate terms during query planning — understand this internally before calling `searcher.search()`

---

## Phase 6 — Storage Service

> **Rust concepts:** `async` streams, `tokio::io`, `bytes::Bytes`, multipart parsing, presigned URL generation
> **System design:** Direct-to-storage upload pattern, content-addressed storage, CDN integration

- [ ] `POST /storage/presign` — generate presigned upload URL (S3/MinIO)
- [ ] `GET  /storage/files/:id` — fetch file metadata
- [ ] `DELETE /storage/files/:id` — soft delete, object cleanup job
- [ ] Content-type validation and virus scanning hook
- [ ] Image resizing pipeline (thumbnail generation on upload event)
- [ ] Storage quota enforcement per workspace

---

## Phase 7 — Notification Service

> **Rust concepts:** `tokio::select!`, `broadcast` channels, `futures::stream`, `Pin<Box<dyn Stream>>`
> **System design:** Fan-out writes, pub/sub, at-least-once delivery, idempotency keys
> **DSA:** Sliding window (burst dedup), two pointers (pagination cursor)

- [ ] In-app notification bell (unread count, mark read)
- [ ] `GET /notifications` — paginated notification list
- [ ] `PATCH /notifications/:id/read`
- [ ] `WS /notifications/stream` — real-time push via WebSocket
- [ ] Notification triggers: page shared, comment added, member invited
- [ ] NATS subscriber: consume events from all other services
- [ ] Delivery deduplication via idempotency key (Redis `SETNX`)

### Burst Deduplication (Sliding Window)
- [ ] Suppress duplicate notifications when many events fire in rapid succession: if the same `(user, event_type, source_id)` triple appears more than once within a 30-second **sliding window**, coalesce into a single notification with a count (**sliding window counter** in Redis sorted set, keyed by timestamp)
- [ ] `GET /notifications?cursor=:id&limit=20` — cursor-based pagination using a **two-pointer** scan over the sorted notification log (avoids offset-based O(n) scans)

---

## Phase 8 — API Gateway

> **Rust concepts:** Tower `Service`/`Layer` trait, type-erased middleware, `hyper` internals, `tower-http`
> **System design:** API gateway patterns, rate limiting algorithms, circuit breakers
> **DSA:** Sliding window counter, token bucket, leaky bucket — implement all three, compare

- [ ] Reverse proxy to all upstream services
- [ ] JWT validation middleware (verify signature, extract claims)
- [ ] Per-user rate limiting — implement **all three** algorithms and compare:
  - **Token bucket** — greedy refill at fixed rate; allows short bursts; O(1) per request with two Redis fields (`tokens`, `last_refill`)
  - **Leaky bucket** — greedy drain at fixed rate; smooths bursts; implemented as a Redis queue with a drain worker
  - **Sliding window counter** — most accurate; uses a Redis sorted set of request timestamps; evict entries older than the window with `ZREMRANGEBYSCORE` before counting (**sliding window** over a time-sorted set)
- [ ] Expose `X-RateLimit-Limit`, `X-RateLimit-Remaining`, `X-RateLimit-Reset` headers
- [ ] Request ID injection (`X-Request-Id` header)
- [ ] Distributed tracing context propagation (W3C `traceparent`)
- [ ] CORS and security headers
- [ ] Circuit breaker per upstream (fail fast on repeated errors) — states: Closed → Open → Half-Open
- [ ] Health check aggregation: `GET /health`

**DSA lesson:** Implement all three rate limiters back-to-back. The sliding window is the most interesting — it's literally a **two-pointer** problem on a time-sorted sequence. Read: [Figma's rate limiting blog post](https://www.figma.com/blog/an-alternative-approach-to-rate-limiting/).

---

## Phase 9 — Analytics & ETL Service

> **Rust concepts:** Iterator adapters, `rayon` for data parallelism, custom `Display`/`Debug` impls, `serde` for schema evolution
> **System design:** Lambda / Kappa architecture, event sourcing for analytics, materialized views

### Event Ingestion
- [ ] All services publish `DomainEvent` to NATS topic `events.*`
- [ ] Analytics service subscribes and persists raw events to append-only table

### ETL Pipeline
- [ ] **Extract:** Raw events from SurrealDB append-only table
- [ ] **Transform:** Aggregate into daily active users, page views, edits per user, popular pages
- [ ] **Load:** Materialized summary tables (for dashboards)
- [ ] Scheduled job: run pipeline every hour via `tokio` task with cron

### API
- [ ] `GET /analytics/workspace/:id/summary` — WAU, MAU, page count, block count
- [ ] `GET /analytics/workspace/:id/pages/popular` — top pages by views/edits

### Range Queries (Prefix Sum)
- [ ] `GET /analytics/workspace/:id/events?from=:date&to=:date&type=:event_type` — count events in an arbitrary date range
- [ ] Implementation: during the hourly ETL job, build a **prefix sum array** over daily event counts bucketed by `(workspace, event_type, date)` — store in a materialised table. Any range query `[from, to]` then answers in O(1): `prefix[to] - prefix[from-1]` (**prefix sum / running aggregate**)
- [ ] `GET /analytics/workspace/:id/edits/by-user?from=:date&to=:date` — per-user edit counts in range, same prefix sum approach

### Stream Sampling (Reservoir Sampling)
- [ ] `GET /analytics/workspace/:id/sample-events?n=100&type=:event_type` — return a uniformly random sample of N events from the full event log without loading all events into memory (**reservoir sampling** — single pass, O(N) space for the reservoir regardless of stream length)
- [ ] Used by the analytics dashboard to render a representative scatter-plot of activity without fetching millions of raw events

---

## Phase 10 — Observability, DevOps & Cloud

> **Rust concepts:** `tracing` instrumentation, `metrics` crate, procedural macros
> **System design:** SLIs/SLOs, distributed tracing, GitOps, IaC

- [ ] Structured JSON logging via `tracing-subscriber`
- [ ] Distributed traces via OpenTelemetry → Jaeger
- [ ] Prometheus metrics scrape endpoint per service
- [ ] Grafana dashboards: request rate, error rate, latency (RED method)
- [ ] Dockerfiles: multi-stage builds (builder + distroless runtime)
- [ ] Docker Compose: full local stack
- [ ] Kubernetes manifests: Deployments, Services, ConfigMaps, Secrets
- [ ] Horizontal Pod Autoscaler on `document-service` and `collaboration-service`
- [ ] Pulumi IaC (Rust SDK): VPC, RDS, ElastiCache, ECS/EKS
- [ ] GitHub Actions CI/CD: test → build → push image → deploy

---

## Phase 11 — Comments & Inline Discussions

> **Rust concepts:** Recursive comment trees, `Box<T>`, `serde` self-referential types
> **System design:** Fan-out notifications, threaded discussions, mention resolution
> **DSA:** Tree traversal for comment threads

- [ ] Inline block-level comments (anchor comment to a specific block)
- [ ] Threaded replies (parent/child comment tree)
- [ ] Resolve / re-open discussion threads
- [ ] `@mention` a user in a comment — triggers notification
- [ ] `[[page]]` backlink in comment — adds a link to the referenced page
- [ ] Reaction emoji on comments
- [ ] `GET /blocks/:id/comments` — paginated comment thread
- [ ] `POST /blocks/:id/comments` — create comment
- [ ] `PATCH /comments/:id/resolve`

**Distributed Systems lesson:** Notification fan-out when a user is mentioned — naive loop vs fan-out workers vs pub/sub fan-out.

---

## Phase 12 — Backlinks & Bidirectional References

> **Rust concepts:** Graph algorithms on `SurrealDB` record links, `HashSet` dedup
> **System design:** Reverse index, eventual consistency of backlink graph
> **DSA:** Graph traversal (BFS/DFS), bidirectional adjacency list

- [ ] `[[PageTitle]]` syntax in block text creates a `references` graph edge
- [ ] `GET /pages/:id/backlinks` — list all pages that link to this page
- [ ] Backlink sidebar panel in UI
- [ ] Backlinks update asynchronously via NATS event when a block is saved
- [ ] Orphaned backlinks cleaned up when a page is deleted
- [ ] `GET /pages/:id/graph` — return page + all linked pages as a graph (nodes + edges) for visualisation
- [ ] `GET /pages/:id/distance?to=:other_id` — shortest path (hop count) between two pages (**BFS shortest path**)
- [ ] `GET /pages/:id/reachable?depth=3` — all pages reachable within N hops (**BFS / DFS with depth limit**)
- [ ] Cycle detection: warn when a page links back to an ancestor (**DFS cycle detection**)
- [ ] `GET /workspaces/:id/clusters` — detect strongly connected page clusters (**Tarjan's / Kosaraju's SCC**)

### Page Connectivity (Union-Find)
- [ ] `GET /pages/connected?a=:page_id&b=:page_id` — are two pages connected via any chain of references? (**Union-Find** / Disjoint Set Union — after building the DSU over the backlink graph, any connectivity query is O(α) amortised, far faster than BFS for repeated queries)
- [ ] `GET /workspaces/:id/components` — list all connected components in the workspace page graph; each component is a set of mutually reachable pages (**Union-Find** full partition)
- [ ] DSU is rebuilt from the full `references` edge set on startup and updated incrementally: `UNITE(a, b)` on `BacklinkCreated` event, full rebuild on `BacklinkDeleted` (deletions require rebuild — Union-Find does not support splits)

**DSA lesson:** Union-Find answers "are A and B connected?" faster than BFS when you have many repeated queries against a mostly-stable graph. The path compression + union-by-rank optimisation is worth understanding from first principles. Read: [CP-Algorithms — DSU](https://cp-algorithms.com/data_structures/disjoint_set_union.html).

**Distributed Systems lesson:** Backlink index is eventually consistent — explore read-your-writes consistency trade-offs.

---

## Phase 12.5 — Database Relations & Rollups

> **Rust concepts:** Recursive query resolution, `Arc<dyn Trait>` for multi-database resolvers, `futures::join_all` for parallel row fetches
> **System design:** Bidirectional reference integrity across databases, lazy vs eager relation loading, N+1 query problem
> **DSA:** Graph traversal — relation chains form a DAG; rollup aggregation is a reduce over a subgraph

This is Notion's most powerful feature. A `relation` property on database A stores record IDs from database B. A `rollup` property aggregates values from the related records. The two databases may be in different pages or different workspaces (read-only cross-workspace relations).

### Data Model

```
database_row:A:1  --[relation_property]--> database_row:B:5
database_row:A:1  --[relation_property]--> database_row:B:9
```

SurrealDB models this naturally as a graph edge: `RELATE database_row:A:1->relation_prop_name->database_row:B:5`. The inverse direction (B shows "linked from A") is a `RELATE` in reverse or a `<-` traversal query.

### Relation API
- [ ] `POST /databases/:id/schema/properties` — add a `relation` property; body includes `target_database_id` and `sync_direction` (`one_way` | `bidirectional`)
- [ ] Bidirectional relation: automatically creates a mirrored relation property on the target database
- [ ] `PATCH /databases/:db_id/rows/:row_id/properties/:prop_id` — set relation values (array of target row IDs)
- [ ] `GET  /databases/:db_id/rows/:row_id/relations/:prop_id` — paginated list of related rows with their full property values
- [ ] Relation integrity: when a target row is deleted, remove it from all relation properties pointing to it (via NATS `RowDeleted` event)
- [ ] Cross-database relations within the same workspace only (cross-workspace is read-only in Phase 20+)

### Rollup API
- [ ] `POST /databases/:id/schema/properties` — add a `rollup` property; body includes `relation_property_id`, `target_property_id`, `aggregation`
- [ ] Supported aggregations: `count`, `count_values`, `count_unique`, `count_empty`, `count_not_empty`, `sum`, `average`, `min`, `max`, `range`, `percent_empty`, `percent_not_empty`, `show_original` (list all values)
- [ ] Rollup is computed at query time — not stored — to avoid stale values
- [ ] `GET /databases/:id/views/:view_id/rows` must resolve rollup values inline for table/board/gallery views
- [ ] Rollup on a `date` property supports: `earliest_date`, `latest_date`, `date_range`
- [ ] Rollup on a `checkbox` property supports: `percent_checked`, `percent_unchecked`

### Linked Database Views
- [ ] `POST /pages/:id/blocks` with type `linked_database` — embed a filtered view of any existing database inline in a page
- [ ] `linked_database` block stores: `source_database_id` + a `view_config` (filters, sorts, visible properties)
- [ ] `linked_database` is read-only by default; toggle to allow inline editing
- [ ] Multiple `linked_database` blocks can reference the same source — each with independent view config

### Formula Evaluation
- [ ] Formula expressions reference other property values by name: `prop("Price") * prop("Quantity")`
- [ ] Supported functions: arithmetic, `if`, `concat`, `length`, `format`, `toNumber`, `now`, `dateBetween`, `dateAdd`
- [ ] Formula type is inferred from the expression (string, number, boolean, date)
- [ ] Formula errors surface as `null` with an error message, not a 500

**DSA lesson:** Rollup aggregation over relation chains is a reduce over a subgraph. When relations chain (A → B → C), evaluating a rollup on A over a rollup-property on B triggers recursive resolution — bound the depth to prevent cycles. This is topological sort territory: [DDIA Chapter 5 — replication and derived data](https://dataintensive.net/).

**Distributed Systems lesson:** The N+1 problem — naively fetching each related row individually kills performance. Explore the DataLoader pattern (batch + deduplicate fetches within a single request window). Read: [The DataLoader pattern explained](https://github.com/graphql/dataloader).

---

## Phase 13 — Database Views (Kanban, Calendar, Table, Gallery)

> **Rust concepts:** Trait objects for view renderers, CQRS read models, `HashMap`-based projections
> **System design:** CQRS — separate read model per view type, projection rebuilds
> **DSA:** Sorting algorithms, grouping, filtering on arbitrary property types

Each view is a different *projection* of the same underlying database block rows.

- [ ] **Table view** — rows + columns, sortable, filterable
- [ ] **Board/Kanban view** — group rows by a `select` property
- [ ] **Gallery view** — grid of cards using cover image or first image block
- [ ] **Calendar view** — rows plotted by a `date` property
- [ ] `POST /databases/:id/views` — create a view with filter/sort config
- [ ] `GET /databases/:id/views/:view_id/rows` — filtered + sorted rows
- [ ] View configuration persisted as JSONB/object: filters, sorts, grouping
- [ ] Formula properties (computed from other properties at query time)

### Calendar View — Date Overlap Queries (Interval Tree)
- [ ] **Calendar view** queries: given a visible date range (e.g., the current month), find all database rows whose `date` or `date_range` property overlaps the visible window (**interval overlap query**)
- [ ] Naive implementation (scan all rows, check overlap) first — then replace with an **interval tree** to answer "which rows overlap [start, end]?" in O(log n + k)
- [ ] `GET /databases/:id/views/:view_id/calendar?start=:date&end=:date` — returns only rows that fall within or overlap the window
- [ ] **Sweep line** for conflict detection: given two rows with overlapping date ranges, surface a warning in the Calendar view UI (e.g., two tasks scheduled for the same time slot) by sweeping a line across all date intervals sorted by start time

**DSA lesson:** Build the naive O(n) scan first, then profile it against a 10,000-row database. The interval tree's O(log n + k) becomes necessary at scale. Read: [Introduction to Interval Trees — CP-Algorithms](https://cp-algorithms.com/data_structures/segment_tree.html) and [CLRS Chapter 14 — Augmenting Data Structures](https://mitpress.mit.edu/9780262046305/introduction-to-algorithms/).

**System Design lesson:** CQRS — the view config is a query specification; the DB rows are the write model.

---

## Phase 14 — Page Templates

> **Rust concepts:** Deep clone of recursive tree structures, `Arc` for shared template blocks
> **System design:** Copy-on-write, immutable snapshots as template sources
> **DSA:** Tree deep copy, structural sharing

- [ ] `POST /pages/:id/save-as-template` — snapshot current page tree as template
- [ ] `GET /workspaces/:id/templates` — list available templates
- [ ] `POST /workspaces/:id/pages/from-template/:template_id` — deep clone template
- [ ] Workspace-level and global (public) template library
- [ ] Template preview thumbnail

**DSA lesson:** Implementing an efficient deep-clone of a DAG (handle shared blocks without infinite loops).

---

## Phase 15 — Publish to Web & CDN Integration

> **Rust concepts:** Feature flags for public vs private rendering paths, cache headers
> **System design:** Cache invalidation, CDN, public/private split, static site generation
> **Security:** Public access without auth, preventing over-sharing

- [ ] `PATCH /pages/:id/publish` — toggle public visibility + generate stable public URL
- [ ] `GET /pub/:slug` — unauthenticated read-only page view (served by frontend)
- [ ] Cache-Control headers: `s-maxage` for CDN, `no-store` for authenticated views
- [ ] Invalidate CDN cache on block update (via API or surrogate keys)
- [ ] Password-protected public pages

**System Design lesson:** Cache invalidation strategies — TTL vs event-driven purge vs surrogate keys.

---

## Phase 16 — Webhooks

> **Rust concepts:** Outbox pattern, `tokio` background worker, `reqwest` HTTP client, exponential backoff
> **System design:** Outbox pattern, at-least-once delivery, idempotency keys, retry storms
> **Distributed Systems:** Exactly-once vs at-least-once delivery guarantees

- [ ] `POST /workspaces/:id/webhooks` — register a webhook URL + event filter
- [ ] `GET /workspaces/:id/webhooks` — list webhooks
- [ ] `DELETE /webhooks/:id`
- [ ] `GET /webhooks/:id/deliveries` — delivery history with status + response body
- [ ] Event types: `page.created`, `page.updated`, `page.deleted`, `member.joined`, `comment.created`
- [ ] Outbox table: events written atomically with DB change, polled by delivery worker
- [ ] Delivery worker: HTTP POST with `X-Signature` HMAC-SHA256 header
- [ ] Exponential backoff with jitter on failure (max 3 retries over 24h)
- [ ] Webhook secret rotation

**DSA lesson:** Exponential backoff with full jitter — calculate the math, implement it without floating point drift.

---

## Phase 17 — Audit Log Service

> **Rust concepts:** Append-only data structures, `serde` schema evolution, `Display` for structured log lines
> **System design:** Event sourcing, compliance queries, GDPR right-to-erasure on an immutable log
> **Distributed Systems:** Append-only log as the source of truth (Kafka/NATS as event backbone)

- [ ] All mutating actions across all services emit an `AuditEvent` to NATS
- [ ] Audit service persists events to append-only SurrealDB table (no UPDATE/DELETE)
- [ ] `GET /audit/workspaces/:id` — paginated audit log for a workspace (Admin only)
- [ ] Filter by user, event type, date range
- [ ] GDPR: anonymize user references in audit log on account deletion (pseudonymisation, not deletion)
- [ ] Tamper-evidence: each event includes a hash chaining to the previous event (like a blockchain log)

**Distributed Systems lesson:** Why an append-only log is powerful — event sourcing, CQRS, time travel queries, audit compliance.

---

## Phase 18 — Distributed Session Routing (Collaboration Scaling)

> **Rust concepts:** Consistent hashing with `HashRing`, `Arc<AtomicUsize>` for metrics, unsafe ring buffer
> **System design:** Consistent hashing, stateful service scaling, session affinity
> **Distributed Systems:** Leader election, sticky sessions, distributed coordination

- [ ] API gateway routes WebSocket connections using consistent hashing on `page_id`
- [ ] All connections for the same page land on the same collaboration-service instance
- [ ] Instance registry in Redis (heartbeat + TTL)
- [ ] On instance failure, sessions rehash to surviving instances (minimal disruption)
- [ ] `GET /admin/collaboration/sessions` — cluster-wide session distribution stats

**DSA lesson:** Consistent hashing ring — implement the ring, understand why it minimises rehashing on node changes.

---

## Phase 19 — Saga Pattern: Workspace Deletion

> **Rust concepts:** State machine with typestate, `thiserror` for saga step errors, compensating transactions
> **System design:** Distributed transactions, saga choreography vs orchestration
> **Distributed Systems:** Eventual consistency, compensating transactions, idempotency

Deleting a workspace spans multiple services (user, document, storage, notifications). A simple HTTP chain is not safe under partial failure.

**Saga Steps (Choreography-style):**
1. `user-service`: mark workspace as `deleting`, emit `WorkspaceDeletionStarted`
2. `document-service`: soft-delete all pages + blocks, emit `DocumentsDeleted`
3. `storage-service`: queue file object deletion, emit `FilesQueued`
4. `notification-service`: delete notifications, emit `NotificationsDeleted`
5. `user-service`: remove all members, emit `MembersRemoved`
6. `user-service`: hard-delete workspace, emit `WorkspaceDeleted`

**Compensating transactions** (rollback if step fails):
- If `DocumentsDeleted` fails → undo `workspace.deleting` flag

- [ ] `DeletionSaga` state machine in `user-service`
- [ ] Saga state persisted to DB (survives service restart mid-saga)
- [ ] Saga coordinator with timeout detection
- [ ] Dead-letter queue for stuck sagas

**Distributed Systems lesson:** Why two-phase commit is impractical in microservices — sagas as the alternative.

---

## Phase 20 — API Keys & External Developer Access

> **Rust concepts:** Constant-time comparison for key verification, `Display` for key formatting
> **System design:** OAuth2 scopes, API key lifecycle, rate limiting by key

- [ ] `POST /workspaces/:id/api-keys` — generate a named API key (shown once)
- [ ] API keys stored hashed (SHA-256), prefix stored in plaintext for display
- [ ] `GET /workspaces/:id/api-keys` — list keys (prefix + name + last used)
- [ ] `DELETE /api-keys/:id` — revoke key
- [ ] Scope-based permissions on keys (`pages:read`, `pages:write`, `members:read`)
- [ ] API gateway validates API keys as an alternative auth method to JWT
- [ ] Per-key rate limits tracked in Redis

---

## Phase 21 — Advanced Analytics: HyperLogLog & Approximate Counting

> **Rust concepts:** Unsafe bit manipulation, `std::hash`, custom data structures
> **System design:** Approximate vs exact counting trade-offs, probabilistic data structures
> **DSA:** HyperLogLog, Bloom filter, Count-Min Sketch

- [ ] Unique page visitors per day (HyperLogLog — approximate, memory-efficient)
- [ ] "Has this user seen this page?" check (Bloom filter — false positives OK)
- [ ] Top-K popular pages (Count-Min Sketch)
- [ ] Per-workspace storage usage (exact counter in Redis `INCRBY`)
- [ ] Storage quota enforcement: block upload if quota exceeded

### Stream Sampling (Reservoir Sampling)
- [ ] `GET /analytics/workspace/:id/events/sample?n=:k` — reservoir sample of k events drawn uniformly at random from the full event stream in one pass (**reservoir sampling** — Algorithm R; O(k) space, O(n) time, no pre-knowledge of n required)
- [ ] Expose this to workspace admins as "representative activity sample" for anomaly spotting

**DSA lesson:** When exact counting is too expensive — implement HyperLogLog from scratch before using a library.

---

## Phase 22 — Undo / Redo & Operation History

> **Rust concepts:** Command pattern, `VecDeque` for bounded history, CRDT undo semantics
> **System design:** Client-side vs server-side undo, per-user vs shared undo history
> **DSA:** Stack, ring buffer, operation inversion

- [ ] Per-user, per-session undo/redo stack (stored client-side in Leptos state)
- [ ] Undo an operation = send inverse operation to collaboration service
- [ ] Server validates inverse op is still applicable (may conflict with others' edits)
- [ ] `GET /pages/:id/history` — full operation history (paginated)
- [ ] `POST /pages/:id/history/:seq/restore` — restore to a point in time

### Operation Collapsing (Monotonic Stack)
- [ ] Before pushing a new op to the undo stack, collapse it with the previous op if they are "adjacent and compatible" (e.g., 5 consecutive single-character inserts at the same cursor position become one word-level insert) — this is a **monotonic stack** invariant: only push when the new op breaks the monotone condition; otherwise merge with top
- [ ] Collapsing rules: `Insert(pos, ch)` followed immediately by `Insert(pos+1, ch2)` → merge into `Insert(pos, ch+ch2)`; `Delete` followed by `Delete` at the same position → merge
- [ ] This mirrors how VS Code / Vim collapse undo history into word-granularity chunks rather than character-granularity

**Distributed Systems lesson:** Undo in a collaborative environment — why a simple stack breaks and what CRDT undo means.

---

## Phase 23 — Import / Export Pipeline

> **Rust concepts:** `tokio::io` streaming, custom parser combinators (`nom`), `serde` for schema mapping
> **System design:** ETL pipeline design, idempotent imports, progress streaming
> **DSA:** Parser combinators, tokenizer design

### Import
- [ ] Import from Markdown (`.md` files) → BitTree block tree
- [ ] Import from Notion export (`.zip` of `.html` or `.md` files)
- [ ] Import progress streamed via SSE (`text/event-stream`)
- [ ] Idempotent: re-importing same content updates rather than duplicates — detect duplicate blocks using **Rabin-Karp rolling hash** fingerprinting (hash each block's content; if hash matches an existing block in the target page, skip insert)

### Export
- [ ] Export page to Markdown
- [ ] Export page to PDF (via headless rendering or `printpdf` crate)
- [ ] Export workspace to `.zip` of Markdown files (mirrors Notion export format)

**ETL lesson:** The full extract → transform → load cycle — parsing foreign formats, mapping schemas, handling partial failures.

---

## Phase 24 — Full-Stack Frontend (Leptos)

> **Rust concepts:** Reactive signals, server functions, WASM, shared types between client and server
> **System design:** SSR + hydration, optimistic UI updates, offline-first considerations

- [ ] `libs/shared` crate: shared DTOs, newtype wrappers, validation (wasm32-compatible)
- [ ] Page tree sidebar (recursive component, lazy-load children)
- [ ] Block editor: render + edit each block type
- [ ] Collaborative cursor presence overlay
- [ ] Search modal (`Cmd+K`)
- [ ] Database view switcher (table / board / calendar / gallery)
- [ ] Notifications bell + dropdown
- [ ] Settings pages: workspace, members, billing, API keys
- [ ] Dark mode
- [ ] Drag-and-drop block reordering (fractional index key generation in WASM)

---

## Phase 25 — BitTree Expression Language (BEL)

> **Rust concepts:** Recursive enums + `Box<T>` for AST nodes, pattern matching on AST variants, `thiserror` for parse/type errors with source spans, `Display` for pretty-printing, `From`/`Into` for IR lowering, `wasm32`-compatible (runs in both browser and server)
> **System design:** Language pipeline as a layered service, shared `libs/bel` crate consumed by multiple services
> **DSA:** Recursive descent parsing, Pratt parsing (precedence climbing), AST traversal, type inference, finite automata (lexer)
> **Compiler concepts:** Lexing → parsing → AST → type checking → evaluation / code generation

BEL is a small, safe, statically-typed expression language embedded in BitTree. The **same language** powers four distinct use cases — one parser, four evaluation backends.

### Language Overview

```
-- Database view filter
status = "Done" AND assigned = @me AND due < today()

-- Nested groups
(priority = "High" OR priority = "Critical") AND NOT archived

-- Formula property
if(prop("Price") > 100, floor(prop("Price") * 0.9), prop("Price"))

-- Computed text
concat(prop("First Name"), " ", prop("Last Name"))

-- Date arithmetic
dateAdd(prop("Start"), 7, "days") > now()

-- Search query (structured prefix syntax)
type:page modified:>2024-01-01 author:@me "exact phrase"

-- Automation condition (Phase 25.5)
row.status CHANGED TO "Done" AND row.assignee = @me
```

### Compiler Pipeline

```
Source string (UTF-8)
        │
        ▼
  ┌─────────────┐
  │    Lexer     │  Tokenize: identifiers, literals, operators, keywords
  │  (FSM-based) │  Produces: Vec<Token> with byte-span positions
  └──────┬──────┘
         │
         ▼
  ┌─────────────┐
  │   Parser    │  Recursive descent for statements
  │ (Pratt for  │  Pratt parser for infix expressions (precedence + associativity)
  │ expressions)│  Produces: Expr (recursive Rust enum, Box<Expr> for children)
  └──────┬──────┘
         │
         ▼
  ┌─────────────┐
  │ Type Checker│  Walk AST, infer and propagate types
  │             │  Produces: TypedExpr — every node annotated with BelType
  │             │  Errors: TypeMismatch, UnknownProp, ArityError with span
  └──────┬──────┘
         │
        ─┼─ fan out to one of four backends ─
         │
  ┌──────┴──────────────────────────────────────────┐
  │                                                  │
  ▼                      ▼              ▼            ▼
Interpreter          SurrealQL        WASM        Search
(formula eval)       Transpiler      Evaluator    Query
                  (filter → WHERE)  (client-side  Parser
                                    formula eval)
```

### Phase 25.1 — `libs/bel` Crate: Lexer

> **DSA:** Finite automaton — the lexer is a hand-rolled state machine with explicit states for each token class

- [ ] Token types: `Ident`, `StringLit`, `NumberLit`, `BoolLit`, `DateLit`, `Null`, `At` (for `@me`), `Prop`, `And`, `Or`, `Not`, `If`, `In`, `Changed`, `To`, `Eq`, `Ne`, `Lt`, `Lte`, `Gt`, `Gte`, `Plus`, `Minus`, `Star`, `Slash`, `LParen`, `RParen`, `Comma`, `Colon`, `Dot`, `EOF`
- [ ] Each `Token` carries a `Span { start: usize, end: usize }` for error reporting
- [ ] Lexer errors: `UnexpectedChar`, `UnterminatedString`, `InvalidNumberLiteral`
- [ ] `Lexer::new(src: &str) -> Lexer` — zero-copy, borrows the source string
- [ ] `Lexer` implements `Iterator<Item = Result<Token, LexError>>`

**DSA lesson:** The lexer is a **finite automaton** — each character advances a state. Hand-roll it before reaching for `logos` (the Rust lexer generator) so you understand what the generated code does. Read: [Crafting Interpreters — Scanning](https://craftinginterpreters.com/scanning.html).

### Phase 25.2 — `libs/bel` Crate: Parser & AST

> **DSA:** Recursive descent (statements, function calls), Pratt parser (infix expressions with precedence table), recursive `Box<Expr>` enum

- [ ] AST node: `Expr` — a Rust enum with variants:
  - `Literal(Value)` — string, number, bool, date, null
  - `Prop(String)` — `prop("Name")` — references a database property by name
  - `Ident(String)` — bare identifier or `@me` / `@user:uuid`
  - `BinOp { op: BinOpKind, lhs: Box<Expr>, rhs: Box<Expr> }` — arithmetic + comparison + logical
  - `UnaryOp { op: UnaryOpKind, expr: Box<Expr> }` — `NOT`, unary `-`
  - `Call { name: String, args: Vec<Expr> }` — `if(...)`, `concat(...)`, `today()`, `now()`, `floor()`, `dateAdd(...)`, etc.
  - `In { expr: Box<Expr>, list: Vec<Expr> }` — `status IN ["Todo", "In Progress"]`
  - `Changed { prop: String, to: Box<Expr> }` — automation trigger condition
- [ ] Pratt parser with explicit **binding power table**: `OR` < `AND` < `NOT` < comparison < `+`/`-` < `*`/`/` — correctly handles `a AND b OR c` as `a AND (b OR c)` without parentheses
- [ ] Parser error: `ParseError { kind: ParseErrorKind, span: Span }` — never panics; all errors are values
- [ ] `Parser::parse_filter(src: &str) -> Result<Expr, Vec<ParseError>>` — entry point for filter expressions
- [ ] `Parser::parse_formula(src: &str) -> Result<Expr, Vec<ParseError>>` — entry point for formula properties

**DSA lesson:** Pratt parsing is the most elegant way to handle operator precedence. Once you understand it, recursive descent for expressions feels clunky by comparison. Read: [Pratt Parsers — Made Simple (matklad)](https://matklad.github.io/2020/04/13/simple-but-powerful-pratt-parsing.html) — matklad is the author of rust-analyzer; this is the canonical Rust-flavoured Pratt explainer.

### Phase 25.3 — `libs/bel` Crate: Type Checker

> **DSA:** AST traversal (post-order walk), type inference via constraint propagation

- [ ] `BelType` enum: `Text`, `Number`, `Boolean`, `Date`, `List(Box<BelType>)`, `Null`, `Unknown`
- [ ] Type checker walks the AST bottom-up, inferring the type of each `Expr` node
- [ ] `TypedExpr` — wraps `Expr` with its inferred `BelType` at each node
- [ ] Type errors: `TypeMismatch { expected: BelType, got: BelType, span: Span }`, `UnknownProperty { name: String, span: Span }`, `ArityMismatch { fn_name: String, expected: usize, got: usize, span: Span }`
- [ ] Property type resolution: the checker accepts a `PropertySchema` (map of prop name → `BelType`) injected at check time — the language is schema-aware
- [ ] `if(cond, then, else)` requires `cond: Boolean`, `then` and `else` must unify to the same type
- [ ] Functions are typed via a built-in function registry: `today() → Date`, `concat(...Text) → Text`, `floor(Number) → Number`, `dateAdd(Date, Number, Text) → Date`

### Phase 25.4 — Filter Backend: SurrealQL Transpiler

> **DSA:** AST-to-target compilation — tree transformation via pattern matching

- [ ] `FilterTranspiler::transpile(expr: &TypedExpr, schema: &PropertySchema) -> Result<String, TranspileError>` — walks the typed AST and emits a SurrealQL `WHERE` clause fragment
- [ ] `BinOp(And)` → `(...) AND (...)`, `BinOp(Eq)` → `property_values.<prop_id> = $val`
- [ ] `In` → `property_values.<prop_id> IN [...]`
- [ ] Date comparisons emit SurrealQL `time::` functions: `today()` → `time::floor(time::now(), 1d)`
- [ ] `@me` resolves to the current user's ID, injected as a bound parameter (never interpolated into the query string — **SQL injection prevention**)
- [ ] Output is a parameterised query fragment: `(String, BTreeMap<String, Value>)` — the fragment + its bound parameters

**Security lesson:** The transpiler must never interpolate user values directly into the query string — always bind parameters. The type checker enforces that `@me` and string literals are values, never identifiable as SQL keywords.

### Phase 25.5 — Formula Backend: Tree-Walking Interpreter

> **DSA:** Tree-walking interpreter — recursive evaluation of the typed AST

- [ ] `Interpreter::eval(expr: &TypedExpr, row: &DatabaseRow) -> Result<Value, EvalError>` — evaluates a formula expression against a database row's property values
- [ ] Evaluation is pure: no side effects, no I/O
- [ ] `Value` enum: `Text(String)`, `Number(f64)`, `Boolean(bool)`, `Date(DateTime<Utc>)`, `List(Vec<Value>)`, `Null`
- [ ] Built-in function implementations: `if`, `concat`, `length`, `toNumber`, `floor`, `ceil`, `round`, `today`, `now`, `dateAdd`, `dateBetween`, `format`
- [ ] Division by zero → `Value::Null` (not a panic); all errors are `Value::Null` with an attached `EvalError`
- [ ] Short-circuit evaluation: `false AND <anything>` never evaluates the right side

### Phase 25.6 — WASM Build: Client-Side Formula Evaluation

- [ ] `libs/bel` compiles to `wasm32-unknown-unknown` — no I/O, no threads, no `std::fs`
- [ ] Gate any server-only code with `#[cfg(not(target_arch = "wasm32"))]`
- [ ] Export `bel_eval(formula: &str, row_json: &str) -> String` as a WASM function callable from Leptos
- [ ] Client evaluates formula properties locally as the user types — no round trip needed

### Phase 25.7 — API: BEL Endpoints

- [ ] `POST /bel/validate` — body: `{ "expression": "...", "context": "filter" | "formula", "schema": { ... } }` → returns `{ "valid": true }` or `{ "errors": [{ "message": "...", "span": { "start": 0, "end": 5 } }] }`
- [ ] `POST /bel/explain` — returns a human-readable description of what an expression does (for UI tooltip)
- [ ] `POST /bel/autocomplete` — body: `{ "expression": "...", "cursor": 12, "schema": { ... } }` → returns completions at cursor position (property names, function names, enum option values)
- [ ] Database view filter `POST /databases/:id/views` now accepts `filter_expression: String` (BEL) alongside the legacy JSON filter config — both are supported during migration

### Phase 25.8 — Automation Rules (Trigger-Action)

> **DSA:** Event pattern matching — the trigger condition is a BEL expression evaluated against before/after row snapshots

- [ ] `POST /workspaces/:id/automations` — create an automation rule: `{ "trigger": "row.status CHANGED TO \"Done\"", "action": { "type": "notify", "target": "@assignee", "message": "..." } }`
- [ ] `GET /workspaces/:id/automations` — list rules
- [ ] `DELETE /automations/:id`
- [ ] `GET /automations/:id/runs` — execution history (succeeded / failed / skipped)
- [ ] Trigger evaluation: on every `RowUpdated` NATS event, evaluate each automation's BEL trigger condition against `{ before: row_snapshot, after: row_snapshot }`; execute action if `true`
- [ ] Supported actions: `notify(target, message)`, `set_property(prop, value)`, `create_row(database_id, properties)`, `webhook(url)`
- [ ] Action expressions (the `message` field) are also BEL formula expressions: `"Completed: " + row.name`
- [ ] Automation execution is async via NATS — trigger evaluation is O(automations) per row update; bound to 50 rules per workspace on free plan

**System Design lesson:** The automation evaluator is a rules engine. Naive approach: linear scan of all rules per event. Optimised approach: build a discrimination tree (decision tree) over rule conditions so unrelated rules are skipped without evaluation.

---

## DSA Feature Targets

> These are concrete features whose implementation *requires* you to encounter a specific DSA concept. Cross-referenced with the DSA Concepts Map in `ROADMAP.md`.

### Trees

| Feature | DSA Concept | Service | Phase |
|---|---|---|---|
| Block tree render (DFS iterative) | DFS without recursion — explicit stack | `document-service` | 3 |
| Delete block + all descendants (BFS) | BFS traversal, queue-based | `document-service` | 3 |
| Page sidebar: lazy-load children | BFS level-by-level, pagination | `document-service` | 3 |
| Deep-clone page for templates | Tree copy — handle shared block refs | `template-service` | 14 |
| `@mention` / page title autocomplete | Trie insert + prefix search | `search-service` | 5 |
| Analytics: edits in date range | Segment tree range query (sum) | `analytics-service` | 21 |
| Snapshot diff viewer | Myers diff — DP on tree edit distance | `document-service` | 3 |
| Calendar date-range overlap queries | Interval tree — O(log n + k) overlap query | `document-service` | 13 |

### Graphs

| Feature | DSA Concept | Service | Phase |
|---|---|---|---|
| Backlink index | Bidirectional adjacency via SurrealDB edges | `document-service` | 12 |
| "Pages reachable from X" explorer | Graph BFS/DFS with depth limit | `document-service` | 12 |
| Circular reference detection | Cycle detection (DFS + colour marking) | `document-service` | 12 |
| Knowledge cluster view | Strongly connected components (Tarjan's or Kosaraju's) | `document-service` | 12 |
| Page link distance | BFS shortest path (unweighted graph) | `document-service` | 12 |
| Relation chain rollup resolution | DAG traversal with cycle detection + depth bound | `document-service` | 12.5 |
| Page connectivity queries | Union-Find (DSU) with path compression + union-by-rank | `document-service` | 12 |
| Connected components of page graph | Union-Find full partition | `document-service` | 12 |
| Collaboration session routing | Consistent hashing ring | `api-gateway` | 18 |
| Workspace saga step ordering | Topological sort on dependency graph | `user-service` | 19 |

### Dynamic Programming

| Feature | DSA Concept | Service | Phase |
|---|---|---|---|
| Snapshot diff: "what changed?" | Edit distance / Wagner-Fischer | `document-service` | 3 |
| Code block line diff | Myers diff algorithm | `document-service` | 3 |
| CRDT undo ancestor detection | Longest common subsequence | `collaboration-service` | 22 |
| Fractional key rebalancing | Interval DP — minimum re-key operations | `document-service` | 3 |
| Rollup formula evaluation | Memoised tree reduce over relation DAG | `document-service` | 12.5 |
| ETL batch scheduler | 0-1 Knapsack — maximise events in memory budget | `analytics-service` | 9 |

### Backtracking

| Feature | DSA Concept | Service | Phase |
|---|---|---|---|
| Markdown import parser | `nom` parser combinators — backtrack on failed rules | `template-service` | 23 |
| Wildcard search `page:*rust*` | Recursive wildcard matcher with backtracking | `search-service` | 5 |
| All paths between two pages (depth-limited) | Exhaustive DFS with backtracking + pruning | `document-service` | 12 |
| Regex search on block content | Regex engine backtracking (use `regex` crate, study its NFA internally) | `search-service` | 5 |

### Strings & Searching

| Feature | DSA Concept | Service | Phase |
|---|---|---|---|
| In-page exact phrase search | KMP — O(n+m) single-pattern search | `document-service` | 3 |
| Multi-term highlight in search results | Aho-Corasick — simultaneous multi-pattern search | `search-service` | 5 |
| Duplicate block detection on import | Rabin-Karp rolling hash — fingerprint each block's content | `template-service` | 23 |

### Sliding Window & Two Pointers

| Feature | DSA Concept | Service | Phase |
|---|---|---|---|
| Notification burst deduplication | Sliding window counter (Redis sorted set + ZREMRANGEBYSCORE) | `notification-service` | 7 |
| Sliding window rate limiter | Sliding window over timestamp-sorted request log | `api-gateway` | 8 |
| Cursor-based notification pagination | Two-pointer scan over sorted notification log | `notification-service` | 7 |
| Search result dedup within time window | Sliding window coalesce | `search-service` | 5 |

### Searching & Ordering

| Feature | DSA Concept | Service | Phase |
|---|---|---|---|
| Snapshot lookup by timestamp | Binary search over ordered snapshot log | `document-service` | 3 |
| Block sibling navigation | Monotonic stack over flattened block sequence | `document-service` | 3 |
| Undo op collapsing | Monotonic stack — merge adjacent compatible ops | `collaboration-service` | 22 |
| Calendar conflict detection | Sweep line over date-interval set | `document-service` | 13 |

### Sampling & Approximation

| Feature | DSA Concept | Service | Phase |
|---|---|---|---|
| Representative event sample for dashboard | Reservoir sampling (Algorithm R) — O(k) space, single pass | `analytics-service` | 9, 21 |
| Analytics range queries | Prefix sum over daily bucketed counts | `analytics-service` | 9 |

### Caching

| Feature | DSA Concept | Service | Phase |
|---|---|---|---|
| Hot page + block L1 cache | LFU eviction (`moka`) — frequency-stable popular content | all services | 3+ |
| Session + permission L1 cache | LRU eviction (`moka`) — recency-biased short-lived data | all services | 3+ |

### Compiler / Language

| Feature | DSA Concept | Service | Phase |
|---|---|---|---|
| BEL lexer | Finite automaton — hand-rolled state machine per token class | `libs/bel` | 25.1 |
| BEL filter parser | Recursive descent (statements) + Pratt parser (infix precedence climbing) | `libs/bel` | 25.2 |
| BEL AST | Recursive enum + `Box<Expr>` — self-referential algebraic data type in Rust | `libs/bel` | 25.2 |
| BEL type checker | Post-order AST traversal + type constraint propagation + unification | `libs/bel` | 25.3 |
| SurrealQL transpiler | Tree transformation via structural pattern matching (AST → target IR) | `libs/bel` | 25.4 |
| Formula interpreter | Tree-walking interpreter — recursive evaluation with short-circuit semantics | `libs/bel` | 25.5 |
| WASM formula evaluator | `wasm32` feature gating — same crate compiles to server and browser | `libs/bel` | 25.6 |
| BEL autocomplete | Trie over property/function names + cursor position tracking in the token stream | `bel-service` | 25.7 |
| Automation rules engine | Decision tree discrimination — skip unmatched rules without full evaluation | `bel-service` | 25.8 |

### Greedy

| Feature | DSA Concept | Service | Phase |
|---|---|---|---|
| Fractional index key generation | Greedy midpoint string selection | `document-service` | 3 |
| Presence colour assignment | Greedy graph colouring | `collaboration-service` | 4 |
| Token bucket rate limiter | Greedy refill, O(1) per request | `api-gateway` | 8 |
| Leaky bucket rate limiter | Greedy drain, compare with token bucket | `api-gateway` | 8 |
| Webhook retry scheduling | Greedy next-due selection (min-heap) + jitter | `webhook-service` | 16 |
| ETL job interval scheduling | Activity selection (sort by end time, greedy pick) | `analytics-service` | 9 |

### Heaps & Priority Queues

| Feature | DSA Concept | Service | Phase |
|---|---|---|---|
| Webhook retry queue | Min-heap keyed on `next_attempt_at` | `webhook-service` | 16 |
| Top-N popular pages | Max-heap / partial sort (no full sort needed) | `analytics-service` | 21 |
| Analytics multi-partition merge | K-way merge with min-heap | `analytics-service` | 9 |

### Probabilistic Structures

| Feature | DSA Concept | Service | Phase |
|---|---|---|---|
| Unique daily visitors | HyperLogLog — implement from scratch first | `analytics-service` | 21 |
| "Has user seen page?" | Bloom filter — implement, understand false positive rate | `analytics-service` | 21 |
| Top-K pages (space-efficient) | Count-Min Sketch | `analytics-service` | 21 |
