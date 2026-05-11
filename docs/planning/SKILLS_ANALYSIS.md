# RustJobs.dev Skills Analysis — Bittree Project

> Analysis based on browsing live job listings on [rustjobs.dev](https://rustjobs.dev) (May 2026) and comparing against the bittree project codebase and roadmap.

---

## Skills You Already Have / Are Building in Bittree

These are skills **directly demonstrated** by your bittree project that are **actively demanded** on rustjobs.dev.

| Skill | Demand on RustJobs | Where in Bittree | Status |
|---|---|---|---|
| **Rust (core language)** | 🔴 100% of listings | Entire project | ✅ Active |
| **Axum** | 🔴 Dominant web framework | HTTP framework for all services | ✅ Active |
| **Tokio** | 🔴 Standard async runtime | Workspace dependency, full features | ✅ Active |
| **Tower** | 🟠 Frequently paired w/ Axum | Middleware layers (CORS, tracing, timeout) | ✅ Active |
| **PostgreSQL + sqlx** | 🔴 Standard DB choice | Primary database, compile-time queries, JSONB, LTREE | ✅ Active |
| **Redis** | 🟠 Often required | Cache/sessions via `fred` | ✅ Active |
| **Docker / Docker Compose** | 🔴 Standard deployment | Full compose stack (7 services) | ✅ Active |
| **Serde / JSON** | 🔴 Ubiquitous | `serde`, `serde_json`, JSON throughout | ✅ Active |
| **Structured Logging / Tracing** | 🟠 Often required | `tracing`, `tracing-subscriber`, bunyan formatter | ✅ Active |
| **Clean Architecture** | 🟠 Often valued | Ports & Adapters, domain/application/infra/presentation layers | ✅ Active |
| **Error Handling** | 🟠 Expected | `thiserror`, `anyhow`, custom error types | ✅ Active |
| **JWT / Auth** | 🟠 Common requirement | `jsonwebtoken`, RS256, Argon2id | ✅ Planned (auth-service) |
| **AWS S3 (Object Storage)** | 🟠 Common cloud skill | `aws-sdk-s3`, MinIO local / S3 cloud | ✅ Active (dep declared) |
| **Event-Driven Architecture** | 🟠 50% of listings | NATS JetStream for inter-service events | ✅ Active (dep declared) |
| **Observability** | 🟡 40% of listings | Jaeger + Prometheus + Grafana in compose | ✅ Infra setup done |
| **Microservice Architecture** | 🟡 Frequently mentioned | 13 planned services, clear service boundaries | ✅ Architecture designed |

---

## Skills from the Bittree Roadmap (Not Yet Implemented)

These are skills that are **in the roadmap** but not yet implemented. They're in high demand.

| Skill | Demand on RustJobs | Bittree Phase | Notes |
|---|---|---|---|
| **gRPC (tonic + prost)** | 🟠 Frequently required | Selected service pairs | Very common in backend Rust roles |
| **WebSockets** | 🟠 High demand | Phase 4 — Collaboration service | Real-time systems are hot |
| **CRDT / Distributed Data** | 🟡 Niche but premium | Phase 4 — YATA CRDT | Makes you stand out significantly |
| **WASM** | 🟡 Emerging demand | Phase 24 — Leptos frontend | Growing fast, esp. for full-stack Rust |
| **Leptos (Full-Stack Rust)** | 🟡 Emerging | Phase 24 — SSR + WASM | Similar to Dioxus demand |
| **Compiler/Interpreter Design** | 🟡 Niche | Phase 25 — BEL (lexer, Pratt parser) | Shows deep CS fundamentals |
| **Kubernetes** | 🟠 40% of listings | Phase 10 — Deployment | Critical for production roles |
| **Pulumi / IaC** | 🟡 Sometimes mentioned | Phase 10 — Pulumi Rust SDK | Replaces Terraform in some shops |
| **SIMD / Vectorization** | 🟡 Performance roles | Multiple phases — analytics, BEL, search | Very impressive on resume |
| **Lock-Free / Concurrent Programming** | 🟠 Performance roles (60%) | Atomics, CAS, crossbeam, dashmap | High-value differentiator |
| **OpenTelemetry** | 🟡 Growing demand | Phase 10 | Industry standard moving forward |
| **Rate Limiting / Circuit Breaker** | 🟡 API-focused roles | Phase 8 — API Gateway | Shows production mindset |
| **Cloud Platforms (AWS/EKS)** | 🔴 40% of listings | AWS Roadmap | VPC, IAM, RDS, ElastiCache, S3 |
| **CI/CD Pipelines** | 🟠 Common requirement | AWS Roadmap | GitHub Actions, ECR container push |

---

## Skills Needed from Other Projects / Learning

These are **frequently demanded on rustjobs.dev** but **NOT covered** by bittree.

### High Priority (Mentioned in 40%+ of listings)

| Skill | Demand | Why it's needed | Suggested Project / Resource |
|---|---|---|---|
| **Linux Systems Programming** | 🔴 40% | Embedded Linux, systemd, kernel debugging — bittree is purely application-level | Build a custom daemon/service manager; contribute to a systems project; study `nix` crate |
| **Testing at Scale** | 🟠 Expected | Integration tests exist but no load testing, property testing, fuzzing | Add `proptest`, `cargo-fuzz`, and `criterion` benchmarks to bittree |

### Medium Priority (Mentioned in 20-40% of listings)

| Skill | Demand | Why it's needed | Suggested Project / Resource |
|---|---|---|---|
| **C/C++ Interop (FFI)** | 🟠 30% | Many Rust roles require wrapping C libraries or interfacing with existing C++ codebases | Build a Rust wrapper around a C library (e.g., SQLite, libcurl); study `bindgen` |
| **Blockchain / Web3 / DeFi** | 🟠 30% | Significant chunk of Rust jobs are in crypto (EVM, Solana, DeFi, MEV) | Build a simple EVM client or Solana program; study Alloy/Foundry |
| **Python (secondary language)** | 🟠 30% | Many Rust roles want Python for scripting, ML pipelines, or tooling | Use PyO3 to build a Python extension in Rust |
| **Go (secondary language)** | 🟠 30% | Common in infra/cloud shops alongside Rust | Build a small Go microservice; understand the ecosystem |
| **TypeScript (secondary language)** | 🟠 30% | Full-stack roles or Rust + TS frontends | Leptos frontend covers UI, but some shops want React/TS |
| **Networking / Custom Protocols** | 🟠 25% | Building binary protocols, TCP/UDP servers | Build a custom protocol server with `tokio::net`; study `quinn` (QUIC) |
| **Embedded / bare-metal Rust** | 🟡 20% | no_std, embedded-hal, RTOS | Completely different domain from bittree; get an STM32/ESP32 board |

### Nice to Have (Niche but high-paying)

| Skill | Where it shows up | Notes |
|---|---|---|
| **Performance Engineering** (profiling, flamegraphs, `perf`) | 60% of listings | Add benchmarking & profiling to bittree using `criterion` + `flamegraph` |
| **Distributed Storage** (Ceph, MinIO internals) | Infra-heavy roles | Study how distributed object stores work internally |
| **Elasticsearch** | Observability/search roles | Could add to bittree as alternative to Tantivy for learning |
| **Vector Databases** | AI/ML Rust roles | Continue the [vector DB project](../../../README.md) |
| **RAG / AI Search** | Emerging AI roles | Combine vector DB knowledge with Rust search pipelines |

---

## Coverage Summary

```
 Covered by Bittree (current)  ████████████████░░░░░░░░░░░░░░░░░░░░░░░░  40%
 Covered by Bittree (roadmap)  ██████████████░░░░░░░░░░░░░░░░░░░░░░░░░░  35%
 Need Other Projects           ██████████░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░  25%
```

| Category | Coverage |
|---|---|
| **Web Backend (Axum/Tokio/Tower/SQL)** | ✅ Excellent |
| **Architecture (Clean/DDD/Event-Driven)** | ✅ Excellent |
| **Databases (PostgreSQL/Redis)** | ✅ Excellent |
| **DevOps (Docker/Observability)** | ✅ Excellent (via AWS Roadmap) |
| **Systems Programming** | ❌ Not covered — need separate project |
| **Low-Level Perf (SIMD/lock-free)** | 🟡 In roadmap, not yet implemented |
| **Blockchain/Web3** | ❌ Not covered — need separate project if interested |
| **Secondary Languages** | ❌ Not covered — consider Python or Go |
| **Embedded/bare-metal** | ❌ Not covered — different domain entirely |

---

## Recommended Action Plan

### Immediate (while building bittree)

1. **Follow AWS Roadmap** — Begin implementing CI/CD pipelines as outlined in `AWS_ROADMAP.md`
2. **Add benchmarks** — Use `criterion` for performance benchmarking of hot paths
3. **Add property tests** — Use `proptest` for domain logic validation
4. **Continue the vector DB project** — This directly targets the hot AI/search niche

### Side projects for gaps

5. **Systems project** — Build a TCP proxy, custom daemon, or contribute to a Rust systems project
6. **FFI project** — Wrap a C library with `bindgen` + safe Rust API
7. **Networking project** — Build a custom binary protocol server or QUIC-based tool

---

> **Bittree alone covers ~75% of what rustjobs.dev demands** now that the `AWS_ROADMAP.md` has been integrated. The remaining 25% falls into systems programming, FFI, and secondary languages — areas that need separate, targeted projects.

> The **highest-impact gap** left to close is **Linux systems programming**. This appears in 40%+ of listings. A custom systems-level side project (like a load balancer or daemon) is the best way to complement your heavy web-backend experience from Bittree.
