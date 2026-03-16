# ADR-003 — gRPC for Selected High-Frequency and Streaming Service Pairs

- **Status:** Accepted
- **Date:** 2026-03-16

---

## Context

BitTree's default inter-service transport is HTTP (Axum) for synchronous request/response calls and NATS JetStream for asynchronous domain events. This covers the vast majority of service communication patterns well.

However, three specific service pairs have communication characteristics that do not fit cleanly into plain HTTP:

1. **`api-gateway` → `auth-service`** — JWT validation is called on _every inbound request_ through the gateway. At scale this is the highest-frequency RPC in the system. Plain HTTP introduces framing overhead, header parsing, and connection management cost that compounds at high request rates. A binary protobuf payload over an already-multiplexed HTTP/2 connection reduces per-call overhead significantly.

2. **`document-service` → `collaboration-service`** — When a document operation is applied, `document-service` must deliver that operation to `collaboration-service` and receive acknowledgement (and potentially further ops) in both directions, continuously, for the lifetime of a live editing session. This is a bidirectional streaming problem, not a request/response problem.

3. **`analytics-service` internal ETL batch ingestion** — The ETL pipeline produces large batches of raw events and needs to stream them to an ingestion endpoint efficiently. Client-streaming allows the producer to send all records in a single RPC without waiting for individual acknowledgements, with back-pressure handled by the gRPC flow control layer.

The goal is also pedagogical: the project is a structured learning exercise. Adding gRPC to the workspace introduces `tonic`, `prost`, `tonic-build`, proto3 schema design, and gRPC streaming patterns — all high-value concepts — while keeping scope contained to the three pairs where gRPC is the right tool.

---

## Decision

Use **gRPC (tonic + prost)** for the following three service pairs only:

| Service Pair | RPC Type | Reason |
|---|---|---|
| `api-gateway` → `auth-service` | Unary RPC | High-frequency JWT validation; binary protobuf + HTTP/2 multiplexing reduces per-call overhead |
| `document-service` → `collaboration-service` | Bidirectional streaming RPC | Continuous op delivery in both directions for the lifetime of a live editing session |
| `analytics-service` internal ETL batch ingestion | Client-streaming RPC | Stream large event batches to the ingestion endpoint with gRPC flow control |

**Everything else uses HTTP (Axum) + NATS JetStream.** No other service pair adopts gRPC.

All `.proto` definitions and `prost`/`tonic-build` codegen live in the `libs/proto` crate. Services import the generated types from `libs/proto` — no `.proto` files are duplicated across service crates.

---

## Rationale

### Why gRPC for these three and not HTTP?

- **`api-gateway` → `auth-service` (unary):** JWT validation is on the critical path of every user-facing request. HTTP/1.1 has no request multiplexing — each validation requires a separate TCP round-trip unless the gateway maintains a keep-alive pool. gRPC over HTTP/2 multiplexes many concurrent validation calls on a single connection and uses binary protobuf instead of JSON, removing serialisation overhead.

- **`document-service` → `collaboration-service` (bidi streaming):** The session lifetime is long (minutes to hours), and ops must flow in both directions continuously. Modelling this as a series of independent HTTP requests would require polling or SSE, both of which add latency and complexity. Bidirectional streaming RPC is the idiomatic solution: one long-lived RPC, two independent streams.

- **`analytics-service` ETL (client-streaming):** Batch ingestion sends thousands of events in a single ETL run. Client-streaming lets the producer push records without waiting for individual acks, while gRPC flow control provides back-pressure — precisely the model needed for bulk ingestion.

### Why not gRPC everywhere?

- Most service pairs are simple, low-to-medium frequency request/response calls where HTTP is idiomatic, well-understood, and trivially observable (curl, browser devtools, Axum middleware).
- NATS JetStream already handles async fan-out cleanly. Replacing it with gRPC server-streaming would be a net regression.
- Limiting gRPC to three pairs keeps tooling overhead (proto schema management, build.rs codegen, tonic server boilerplate) proportional to the benefit.

---

## `libs/proto` Crate

All protobuf definitions are centralised in `libs/proto`:

```
libs/proto/
├── build.rs          ← tonic-build codegen; compiles all .proto files
├── proto/
│   ├── auth.proto    ← ValidateToken RPC (api-gateway ↔ auth-service)
│   ├── collab.proto  ← SyncOps RPC (document-service ↔ collaboration-service)
│   └── analytics.proto ← IngestEvents RPC (analytics-service internal)
└── src/
    └── lib.rs        ← re-exports generated modules
```

Services that are gRPC servers or clients add `libs/proto` as a `[path]` dependency in `Cargo.toml`. Services that do not participate in gRPC do not depend on `libs/proto` at all.

---

## Consequences

### Added to workspace

- `tonic` — gRPC runtime (client + server)
- `prost` — protobuf serialisation
- `tonic-build` — build-time codegen from `.proto` files (dev-dependency in `libs/proto`)
- `build.rs` in `libs/proto` — runs `tonic-build::compile_protos` at workspace build time

### Services affected

| Service | Role |
|---|---|
| `auth-service` | gRPC server (implements `AuthService`) |
| `api-gateway` | gRPC client (calls `ValidateToken`) |
| `collaboration-service` | gRPC server (implements `CollabService`) |
| `document-service` | gRPC client (opens `SyncOps` bidi stream) |
| `analytics-service` | gRPC server (implements `AnalyticsIngestion`) + gRPC client (internal ETL batch push) |

### Services unaffected

All other services (`user-service`, `search-service`, `storage-service`, `notification-service`, `webhook-service`, `audit-service`, `template-service`, `bel-service`, `frontend`) continue to use HTTP (Axum) exclusively and do not depend on `libs/proto`.

---

## Resources

- [tonic — gRPC for Rust (GitHub)](https://github.com/hyperium/tonic) — examples, streaming patterns, interceptors
- [Protocol Buffers Language Guide (proto3)](https://protobuf.dev/programming-guides/proto3/) — proto3 syntax, field types, message definitions
- [gRPC Core Concepts](https://grpc.io/docs/what-is-grpc/core-concepts/) — unary, server-streaming, client-streaming, bidirectional streaming RPCs explained
