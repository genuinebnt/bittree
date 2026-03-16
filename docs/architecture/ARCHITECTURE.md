# BitTree — System Architecture

---

## 1. High-Level Service Map

```mermaid
graph TD
    subgraph Clients["Client Layer"]
        BROWSER["Browser\n(Leptos WASM)"]
        SSR["Leptos SSR\n:3000"]
    end

    subgraph Gateway["API Gateway  :8000"]
        GW["api-gateway\n• JWT validation\n• Rate limiting\n• Circuit breaker\n• Consistent hash routing"]
    end

    subgraph Services["Microservices"]
        AUTH["auth-service\n:8001\nJWT · OAuth2 · Argon2id"]
        USER["user-service\n:8002\nProfiles · Workspaces · RBAC"]
        DOC["document-service\n:8003\nPages · Blocks · Snapshots"]
        COLLAB["collaboration-service\n:8004\nWebSocket · CRDT · Presence"]
        SEARCH["search-service\n:8005\nTantivy · Full-text"]
        STORAGE["storage-service\n:8006\nFiles · Presign · Quotas"]
        NOTIF["notification-service\n:8007\nIn-app · WebSocket push"]
        ANALYTICS["analytics-service\n:8008\nETL · Aggregates"]
        WEBHOOK["webhook-service\n:8009\nOutbox · Retry · HMAC"]
        AUDIT["audit-service\n:8010\nAppend-only · Hash chain"]
        TEMPLATE["template-service\n:8011\nDeep clone · Templates"]
    end

    subgraph Infra["Infrastructure (Local: Docker Compose / Cloud: AWS)"]
        PG[("PostgreSQL 16\nauth · users · docs\nstorage · notifications\nanalytics · audit")]
        REDIS[("Redis\nSessions · Blocklist\nPresence · Rate limits\nRecents · Dedup")]
        NATS[/"NATS JetStream\nEvent bus"/]
        MINIO[("MinIO / S3\nObject storage")]
        TANTIVY["Tantivy\n(in-process)\nFull-text index"]
    end

    subgraph Observability["Observability"]
        JAEGER["Jaeger\nDistributed traces"]
        PROM["Prometheus\nMetrics"]
        GRAFANA["Grafana\nDashboards"]
    end

    BROWSER -- "HTTPS / WSS" --> GW
    SSR     -- "HTTPS / WSS" --> GW

    GW -- "gRPC (unary)" --> AUTH
    GW -- "HTTP" --> USER
    GW -- "HTTP" --> DOC
    GW -- "WSS (consistent hash on page_id)" --> COLLAB
    DOC -- "gRPC (bidi streaming)" --> COLLAB
    GW -- "HTTP" --> SEARCH
    GW -- "HTTP" --> STORAGE
    GW -- "HTTP" --> NOTIF
    GW -- "HTTP" --> ANALYTICS
    GW -- "HTTP" --> WEBHOOK
    GW -- "HTTP" --> AUDIT
    GW -- "HTTP" --> TEMPLATE

    AUTH      --> PG
    AUTH      --> REDIS
    USER      --> PG
    USER      --> REDIS
    USER      --> NATS
    DOC       --> PG
    DOC       --> REDIS
    DOC       --> NATS
    COLLAB    --> PG
    COLLAB    --> REDIS
    COLLAB    --> NATS
    SEARCH    --> TANTIVY
    SEARCH    --> NATS
    STORAGE   --> PG
    STORAGE   --> MINIO
    STORAGE   --> NATS
    NOTIF     --> PG
    NOTIF     --> REDIS
    NOTIF     --> NATS
    ANALYTICS --> PG
    ANALYTICS --> NATS
    WEBHOOK   --> PG
    WEBHOOK   --> NATS
    AUDIT     --> PG
    AUDIT     --> NATS
    TEMPLATE  --> PG
    TEMPLATE  --> NATS

    AUTH      -. "OTLP traces" .-> JAEGER
    DOC       -. "OTLP traces" .-> JAEGER
    GW        -. "/metrics"    .-> PROM
    PROM      --> GRAFANA
```

---

## 2. NATS Event Bus — Who Publishes & Who Subscribes

> Every service that mutates state publishes domain events to NATS JetStream.
> Downstream services subscribe and react without coupling to the publisher.

```mermaid
graph LR
    subgraph Publishers
        AUTH2["auth-service"]
        USER2["user-service"]
        DOC2["document-service"]
        COLLAB2["collaboration-service"]
        STORAGE2["storage-service"]
    end

    subgraph NATS_Topics["NATS JetStream Topics"]
        T1["auth.user_registered\nauth.user_deleted"]
        T2["users.workspace_created\nusers.member_joined\nusers.member_removed\nusers.workspace_deleting"]
        T3["docs.page_created\ndocs.page_updated\ndocs.page_deleted\ndocs.block_updated\ndocs.block_deleted\ndocs.backlink_created"]
        T4["collab.op_applied"]
        T5["storage.file_uploaded\nstorage.file_deleted"]
    end

    subgraph Subscribers
        SEARCH2["search-service\n(indexes block content)"]
        NOTIF2["notification-service\n(fan-out to users)"]
        ANALYTICS2["analytics-service\n(append raw events)"]
        AUDIT2["audit-service\n(append audit log)"]
        WEBHOOK2["webhook-service\n(deliver to endpoints)"]
        COLLAB3["collaboration-service\n(cross-instance fan-out)"]
        USER3["user-service\n(saga steps)"]
    end

    AUTH2    --> T1
    USER2    --> T2
    DOC2     --> T3
    COLLAB2  --> T4
    STORAGE2 --> T5

    T1 --> ANALYTICS2
    T1 --> AUDIT2
    T2 --> NOTIF2
    T2 --> ANALYTICS2
    T2 --> AUDIT2
    T2 --> WEBHOOK2
    T2 --> USER3
    T3 --> SEARCH2
    T3 --> NOTIF2
    T3 --> ANALYTICS2
    T3 --> AUDIT2
    T3 --> WEBHOOK2
    T4 --> COLLAB3
    T5 --> NOTIF2
    T5 --> ANALYTICS2
    T5 --> AUDIT2
```

---

## 3. Request Flow — Authenticated REST Call

```mermaid
sequenceDiagram
    participant C as Browser
    participant GW as api-gateway
    participant AUTH as auth-service
    participant DOC as document-service
    participant DB as PostgreSQL (docs)
    participant NATS as NATS JetStream

    C->>GW: GET /pages/:id  (Authorization: Bearer <jwt>)
    GW->>GW: Verify JWT signature (RS256 public key, cached)
    GW->>GW: Check rate limit (Redis token bucket)
    GW->>DOC: GET /pages/:id  (X-User-Id, X-Workspace-Id injected)
    DOC->>DB: SELECT page + permission check
    DB-->>DOC: page record
    DOC->>DB: SELECT * FROM blocks WHERE page_id = $1 AND deleted_at IS NULL ORDER BY sort_key
    DB-->>DOC: block tree
    DOC-->>GW: 200 PageResponse
    GW-->>C: 200 PageResponse
```

---

## 4. Request Flow — Real-Time Collaboration (WebSocket)

```mermaid
sequenceDiagram
    participant C1 as Client A
    participant C2 as Client B
    participant GW as api-gateway
    participant COLLAB as collaboration-service
    participant DB as PostgreSQL (docs)
    participant REDIS as Redis (presence)
    participant NATS as NATS (cross-instance)

    C1->>GW: WS /collaboration/pages/:id
    GW->>GW: Consistent hash(page_id) → collab instance 2
    GW->>COLLAB: Proxy WebSocket
    COLLAB->>REDIS: Register session (user, page, instance)
    COLLAB->>DB: LISTEN block_updates_{page_id}

    C1->>COLLAB: Op { insert "hello" at pos 5 }
    COLLAB->>COLLAB: Apply CRDT op, update authoritative state
    COLLAB->>DB: UPDATE docs.blocks SET content = $1 WHERE id = $2 AND version = $3
    COLLAB->>NATS: Publish docs.block_updated
    NATS-->>COLLAB: Fan-out to other instances (if C2 on different instance)
    COLLAB-->>C2: Broadcast Op to all connections on this page
```

---

## 5. Clean Architecture Layers (per service)

```
┌─────────────────────────────────────────────┐
│  Presentation (axum handlers, extractors)    │  ← HTTP in, HTTP out
│  - Routes, State, Request/Response types     │
├─────────────────────────────────────────────┤
│  Domain (pure Rust — zero external deps)     │  ← Business logic lives here
│  - Entities: Page, Block, User, Workspace    │
│  - Repository traits: PageRepo, BlockRepo    │
│  - Domain errors: thiserror enums            │
│  - Use cases / service structs               │
├─────────────────────────────────────────────┤
│  Infrastructure (implements domain traits)   │  ← Swappable
│  - PostgresPageRepo (impl PageRepo)          │
│  - RedisCache       (impl CacheStore)        │
│  - NatsPublisher    (impl EventPublisher)    │
│  - MinioStore       (impl ObjectStore)       │
└─────────────────────────────────────────────┘
         ↑ dependency arrows point inward only
         Domain never imports Infrastructure
```

**Dependency rule:** Infrastructure depends on Domain. Domain depends on nothing outside `std` and `libs/shared`. This is what makes swapping PostgreSQL (or Redis → DragonflyDB) a single-layer change.

---

## 6. Local Dev Stack (Docker Compose)

| Container | Image | Port | Used by |
|---|---|---|---|
| `postgres` | `postgres:16-alpine` | 5432 | all services |
| `redis` | `redis:7-alpine` | 6379 | auth, user, doc, collab, notif |
| `nats` | `nats:2-alpine` (JetStream) | 4222 | all services |
| `minio` | `minio/minio` | 9000 / 9001 (console) | storage |
| `jaeger` | `jaegertracing/all-in-one` | 16686 (UI) / 4317 (OTLP) | all services |
| `prometheus` | `prom/prometheus` | 9090 | scrapes `:service_port/metrics` |
| `grafana` | `grafana/grafana` | 3001 | dashboard UI |

---

## 7. Cloud Deployment (AWS)

```
Internet
    │
    ▼
┌─────────────────────────────────────────────────────────┐
│                        AWS VPC                           │
│                                                          │
│  Route 53 ──▶ CloudFront ──▶ ALB                        │
│                               │                          │
│              ┌────────────────▼────────────────┐        │
│              │       ECS / EKS Cluster          │        │
│              │                                  │        │
│              │  api-gateway  (public subnet)    │        │
│              │       │                          │        │
│              │  ┌────┴─────────────────────┐    │        │
│              │  │  Services (private subnet) │   │        │
│              │  │  auth · user · doc        │   │        │
│              │  │  collab · search · storage │  │        │
│              │  │  notif · analytics · etc  │   │        │
│              │  └────────────┬──────────────┘   │        │
│              └───────────────┼──────────────────┘        │
│                              │                           │
│         ┌────────────────────▼──────────────────┐       │
│         │           Managed Services              │       │
│         │  RDS PostgreSQL                        │       │
│         │  ElastiCache (Redis)                   │       │
│         │  Amazon MQ / managed NATS              │       │
│         │  S3 (object storage)                   │       │
│         │  CloudWatch / X-Ray (observability)    │       │
│         │  Secrets Manager (JWT keys, DB URLs)   │       │
│         └────────────────────────────────────────┘       │
└─────────────────────────────────────────────────────────┘
```

IaC: `infra/` directory — Pulumi Rust SDK provisions all resources above.

---

## 8. gRPC Transport — Service Pairs

The default inter-service transport is HTTP (Axum) for synchronous calls and NATS JetStream for async events. gRPC (tonic + prost) is used only for the three pairs below, each with a concrete technical justification.

| Service Pair | RPC Type | Proto file | Reason |
|---|---|---|---|
| `api-gateway` → `auth-service` | Unary RPC | `libs/proto/proto/auth.proto` | JWT validation on every request — high frequency; binary protobuf + HTTP/2 multiplexing reduces per-call overhead |
| `document-service` → `collaboration-service` | Bidirectional streaming RPC | `libs/proto/proto/collab.proto` | Continuous op delivery in both directions for the lifetime of a live editing session |
| `analytics-service` internal ETL batch ingestion | Client-streaming RPC | `libs/proto/proto/analytics.proto` | Stream large event batches with gRPC flow control; no per-record round-trip overhead |

All `.proto` definitions and `prost`/`tonic-build` codegen live in the `libs/proto` crate. Services that do not participate in any of these three pairs are unaffected and do not depend on `libs/proto`.

See [`docs/architecture/adr/ADR-003-grpc-selective-transport.md`](adr/ADR-003-grpc-selective-transport.md) for the full decision record.
