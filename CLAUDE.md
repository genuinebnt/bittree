# BitTree

Collaborative block-based note-taking app (Notion-inspired). Full-stack Rust **microservices** — one binary per service, independent deployment, event-driven via NATS JetStream.

## Rules

**Read `.agents/agents.md` before every response.** All mentor behavior, strict code review rules, TDD guidance, and documentation requirements live there. This file is fast context only.

---

## Stack

| Layer | Technology |
|---|---|
| HTTP | Axum + Tower + Tokio |
| Frontend | Leptos (SSR + WASM) + `cargo-leptos` |
| Database | PostgreSQL 20 + sqlx (JSONB, LTREE, UUIDv7) |
| Cache / Sessions | Redis |
| Messaging | NATS JetStream |
| Object Storage | MinIO (local) / S3 (cloud) |
| Search | Tantivy (in-process) |
| IaC | Pulumi Rust SDK |
| gRPC | tonic + prost |
| Observability | OpenTelemetry → Jaeger + Prometheus + Grafana |

---

## Crate Layout

```
libs/infra/       telemetry, config, AppError/ApiError, define_id! macro
libs/domain/      domain primitives — wasm32-compatible newtypes, DTOs, events
libs/bel/         BitTree Expression Language (lexer → parser → type checker → backends)
libs/proto/       protobuf definitions (tonic + prost)
libs/test-utils/  Testcontainers wrappers, TestContext
services/         one binary crate per microservice
```

---

## Current Phase: Phase 0 — Foundation

See `docs/planning/ROADMAP.md` for the full phase-by-phase plan and DSA concept map.

---

## Key Docs

| Doc | Purpose |
|---|---|
| `.agents/agents.md` | Mentor rules — governs every response |
| `docs/planning/ROADMAP.md` | Phase-by-phase plan + DSA map |
| `docs/planning/FEATURE_LIST.md` | All features mapped to phases |
| `docs/architecture/DATA_MODEL.md` | PostgreSQL schema (Mermaid ER diagram) |
| `docs/architecture/CLOUD_PORTABILITY.md` | Ports & adapters, cloud vs local equivalents |
| `docs/architecture/adr/` | Architecture Decision Records |
| `docs/architecture/rfc/` | Request for Comments |

---

## Skill Workflow

| When | Run |
|---|---|
| After implementing a feature | `/project:simplify` — Rust idioms, quality, no premature abstractions |
| Before merging any PR | `/project:review` — full code review per agents.md strict mode |
| Any auth / permission / data boundary touched | `/project:security-review` — timing attacks, injection, RBAC |

---

## Architecture Rules (summary)

- Domain layer (`libs/domain`) has **zero** external dependencies — pure Rust types and traits only
- Infrastructure layer implements domain traits; domain never depends on infrastructure
- Every external dependency (DB, cache, broker, storage) is behind a **trait**
- One `config.yaml` per service; safe local defaults; secrets via env vars only — never committed
- Integration tests hit real local services via Testcontainers — **no mocking of infrastructure**
- Cloud services (RDS, S3, ElastiCache) each have a local Docker equivalent implementing the same trait

Full rules: `docs/architecture/CLOUD_PORTABILITY.md` and `.agents/agents.md`.

---

## Documentation Rules (summary)

Before writing any code, check whether these need updating:

1. `docs/planning/FEATURE_LIST.md` — new feature added?
2. `docs/architecture/DATA_MODEL.md` — schema changed?
3. `docs/api/` — new or modified endpoint?
4. `docs/architecture/adr/` — major architectural decision?

Full rules: `.agents/agents.md` § Continuous Documentation.
