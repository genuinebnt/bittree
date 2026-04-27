# ADR-004 — Full Microservices from Day One (vs Modular Monolith)

**Date:** 2026-04-27
**Status:** Accepted
**Supersedes:** N/A (replaces the informal "modular monolith" description in ROADMAP.md and FEATURE_LIST.md)
**Deciders:** @genuinebasilnt

---

## Context

The original project description positioned BitTree as a **Modular Monolith** — a single binary with strict internal module boundaries that could be "extracted to microservices later." The `ARCHITECTURE.md` and `CLOUD_PORTABILITY.md`, however, were already written in terms of independent services (`auth-service`, `document-service`, etc.) each with their own port, schema, and deployment unit.

This created an internal inconsistency: the planning docs said one thing; the architecture docs assumed another. The question was whether to commit to microservices from day one or defer that complexity.

The primary project goal is **maximum learning ROI** across: distributed systems, service communication patterns, independent deployability, network boundaries, and inter-service consistency challenges. These concepts are only encountered authentically when services are genuinely separate processes.

---

## Decision

Build BitTree as a **Full Microservices** system from the first phase:

- One Rust binary per service (`services/<name>/src/main.rs`)
- One PostgreSQL schema per service (`auth`, `users`, `docs`, `storage`, `notifications`, `analytics`, `audit`)
- Services communicate via **NATS JetStream** (async events) and **gRPC** (selected high-frequency pairs — see ADR-003)
- No shared in-process state between services
- Each service has its own `config.yaml` and is independently deployable

---

## Rationale

### The learning value only exists if the boundary is real

A modular monolith lets you define module boundaries in code, but:
- All modules share a single process — no network failure, no partial availability, no serialisation boundary
- All modules share one database connection pool — no schema isolation, no independent migration cadence
- "Extracting to a microservice later" means rewriting the network layer, serialisation, auth propagation, and distributed transaction handling all at once

The distributed systems concepts on the learning roadmap (at-least-once delivery, idempotency, saga, leader election, consistent hashing, CRDT cross-instance fan-out) require real network boundaries to be encountered authentically.

### The workspace layout already assumed microservices

`ROADMAP.md` Phase 0 already defined `services/…` as "one binary crate per microservice." `ARCHITECTURE.md` already mapped out `auth-service :8001`, `document-service :8003`, etc. `CLOUD_PORTABILITY.md` already defined per-service schemas. The monolith description was an inconsistency, not a design decision.

### Building microservices from Phase 0 teaches more

| Concept | Modular Monolith | Full Microservices |
|---|---|---|
| Network failure handling | Not encountered | Required from Phase 1 |
| Serialisation boundaries | Optional | Every inter-service call |
| Saga / distributed transactions | Simulated | Real |
| Auth propagation (X-User-Id header) | In-process | Explicit contract |
| Independent deployment | Not possible | Phase 5+ |
| Schema migrations per service | Coupled | Independent `sqlx migrate` |
| NATS JetStream event delivery | Optional add-on | Core from Phase 1 |

---

## Consequences

### Workspace layout

```
services/
  api-gateway/          :8000
  auth-service/         :8001
  user-service/         :8002
  document-service/     :8003
  collaboration-service/:8004
  search-service/       :8005
  storage-service/      :8006
  notification-service/ :8007
  analytics-service/    :8008
  webhook-service/      :8009
  audit-service/        :8010
  template-service/     :8011
  frontend/             :3000
libs/
  infra/        shared config, telemetry, error types, define_id! macro
  domain/       domain primitives — wasm32-compatible, zero external deps
  bel/          BitTree Expression Language
  proto/        protobuf definitions (tonic + prost)
  test-utils/   Testcontainers wrappers, TestContext
```

### Auth propagation

The `api-gateway` validates the JWT (via gRPC `ValidateToken` RPC on `auth-service`) and injects `X-User-Id` and `X-Workspace-Id` headers on every proxied request. Downstream services trust these headers — they never re-validate the JWT.

### Database isolation

Each service runs its own `sqlx migrate` against its own PostgreSQL schema. Locally, one Postgres instance with multiple schemas. In production, one RDS instance with schema-level isolation (or separate RDS instances per service at scale).

No cross-schema joins. Data needed across services is propagated via NATS domain events and materialised locally.

### Additional complexity accepted

- Local dev requires `docker compose up` to start all services before any code runs
- Phase 0 must wire the full service skeleton (health endpoint + config + tracing) before Phase 1 domain logic
- Debugging spans multiple service logs — OpenTelemetry distributed tracing is required from Phase 0, not optional

### What is unchanged

- Clean Architecture layers within each service (Presentation → Domain ← Infrastructure)
- Ports & adapters: every external dependency behind a trait
- One `config.yaml` per service; secrets via env vars
- Integration tests hit real local services via `#[sqlx::test]` and Testcontainers

---

## Trade-offs Accepted

- Higher Phase 0 scaffolding cost — each service needs health endpoint, config loading, and tracing wired before feature work begins
- `docker compose up` becomes the mandatory first step for any local development
- Debugging a single feature may require reading logs from two or three service containers

---

## Resources

| Resource | What to Learn |
|---|---|
| [Microservices Patterns — Chris Richardson](https://microservices.io/patterns/) | Saga, CQRS, event sourcing, API gateway, service mesh |
| [DDIA Chapter 8 — The Trouble with Distributed Systems](https://dataintensive.net/) | Why network boundaries change everything |
| [Zero To Production In Rust](https://www.zero2prod.com/) | Production-grade Rust service skeleton (health endpoint, config, tracing) |
| [tokio tutorial](https://tokio.rs/tokio/tutorial) | Async runtime required for each service binary |
