# BitTree — Cloud Portability (Ports & Adapters)

> **Core rule:** The domain layer must never know about cloud providers or local infrastructure. Every external dependency is hidden behind a Rust trait. Configuration determines which concrete implementation is wired in at startup.

---

## Local vs Cloud Stack

| Role | Local (Docker Compose) | Cloud (AWS / SurrealDB Cloud) |
|---|---|---|
| Primary Database | `surrealdb/surrealdb:latest` | SurrealDB Cloud or self-hosted on ECS |
| Cache / Session | `redis:7` | Amazon ElastiCache (Redis) |
| Message Broker | `nats:2` (JetStream) | Amazon SQS + SNS or managed NATS |
| Object Storage | `minio/minio` | Amazon S3 |
| Search Index | Tantivy (in-process) | Amazon OpenSearch (optional swap) |
| Distributed Traces | `jaegertracing/all-in-one` | AWS X-Ray or Honeycomb |
| Metrics | `prom/prometheus` + `grafana/grafana` | Amazon CloudWatch or Grafana Cloud |
| Secrets | `.env` file (git-ignored) | AWS Secrets Manager / SSM Parameter Store |

> **Note on SurrealDB Cloud:** SurrealDB offers a managed cloud offering. For self-hosted cloud deployments, run SurrealDB on ECS/EKS with persistent EBS volumes and regular snapshots to S3.

---

## Trait → Implementation Mapping

### Database Repositories

```
trait UserRepo               → SurrealUserRepo (surrealdb crate)
trait WorkspaceRepo          → SurrealWorkspaceRepo
trait PageRepo               → SurrealPageRepo
trait BlockRepo              → SurrealBlockRepo
trait FileMetadataRepo       → SurrealFileMetadataRepo
trait NotificationRepo       → SurrealNotificationRepo
```

**Swappable:** The trait abstraction means swapping to PostgreSQL (via `sqlx`) only requires a new concrete impl — domain code is unchanged.

### Cache

```
trait CacheStore             → RedisCache (redis-rs / fred)
                             → DragonflyDbCache (same Redis protocol)
                             → InMemoryCache (HashMap, for unit tests)
```

Used for: refresh token blocklist, rate limit counters, session presence, notification dedup.

### Message Broker

```
trait EventPublisher         → NatsPublisher (async-nats, JetStream)
                             → SqsPublisher (aws-sdk-rust)
                             → InMemoryPublisher (tokio::broadcast, for tests)

trait EventSubscriber        → NatsSubscriber
                             → SqsSubscriber
                             → InMemorySubscriber
```

### Object Storage

```
trait ObjectStore            → MinioStore (aws-sdk-s3 with custom endpoint)
                             → S3Store (aws-sdk-s3)
                             → LocalFsStore (tokio::fs, for unit tests)
```

### Search Backend

```
trait SearchIndex            → TantivyIndex (tantivy crate)
                             → OpenSearchIndex (opensearch-rs)
                             → InMemoryIndex (Vec<Document>, for tests)
```

---

## Configuration Strategy

### Rule: One `config.yaml` per service

Safe defaults for local development. No environment-specific config files.

```yaml
# services/auth-service/config.yaml
server:
  host: "0.0.0.0"
  port: 8001

database:
  host: "localhost"
  port: 5432
  name: "bittree_auth"
  max_connections: 10

redis:
  url: "redis://localhost:6379"

jwt:
  access_token_ttl_seconds: 900
  refresh_token_ttl_days: 30

# NO secrets here — loaded from env or .env
```

### Environment Variable Override Convention

All config fields are overridable via env vars using double-underscore nesting:

```
APP__DATABASE__HOST=mydb.cluster.us-east-1.rds.amazonaws.com
APP__DATABASE__PORT=5432
APP__JWT__PRIVATE_KEY_PEM=...
```

The Rust `Settings` struct uses `config` crate with `Environment` source — required fields fail fast at startup if missing.

### Secrets

| Secret | Local | Cloud |
|---|---|---|
| `DATABASE_URL` | `.env` | AWS SSM / Secrets Manager |
| `JWT_PRIVATE_KEY_PEM` | `.env` | AWS Secrets Manager |
| `JWT_PUBLIC_KEY_PEM` | `.env` | AWS Secrets Manager |
| `OAUTH_GITHUB_SECRET` | `.env` | AWS Secrets Manager |
| `OAUTH_GOOGLE_SECRET` | `.env` | AWS Secrets Manager |

---

## Integration Test Strategy

### Rule: Tests hit real local equivalents — never mocks for infrastructure

```
[test] → SurrealDB embedded (Mem) or Testcontainers (SurrealDB + Redis + NATS) → Service under test
```

- `libs/test-utils` exposes `TestContext` that wires concrete impls
- **Unit/integration tests:** Use `Surreal::new::<Mem>(())` — no Docker needed, fast startup, in-process
- **Full integration tests:** Use `Testcontainers` (`surrealdb/surrealdb` container) for parity with prod
- Each test gets an isolated SurrealDB namespace (random UUID) to allow parallel test runs
- Container startup is cached within a test run via `once_cell::sync::Lazy`

### Example pattern (TypeScript-style pseudocode for illustration):

```typescript
// What the Rust version should look like conceptually
const ctx = await TestContext.start();
const repo = PostgresUserRepo::new(ctx.pg_pool);
const result = repo.create(new_user).await;
assert!(result.is_ok());
ctx.cleanup().await;
```

The **Rust implementation** is yours to write — see `libs/test-utils`.

---

## Docker Compose — Local Stack

File: `docker-compose.yml` at workspace root.

Services to define:
- `postgres` — one instance, multiple databases (one per service)
- `redis` — single instance for all services locally
- `nats` — JetStream enabled
- `minio` — S3-compatible object storage
- `jaeger` — all-in-one trace collector (OTLP receiver)
- `prometheus` — scrapes `/metrics` from all services
- `grafana` — pre-provisioned with Prometheus datasource

All services connect to these via `localhost` defaults in `config.yaml`.

---

## Cloud Deployment — AWS

```
                    ┌──────────────────────────────────────┐
                    │              AWS VPC                 │
  Internet ──▶ ALB ─▶ ECS (api-gateway) ──▶ ECS (services) │
                    │                       │              │
                    │              ┌────────▼────────┐     │
                    │              │  RDS Postgres   │     │
                    │              │  ElastiCache    │     │
                    │              │  SQS/SNS        │     │
                    │              │  S3             │     │
                    │              └─────────────────┘     │
                    └──────────────────────────────────────┘
```

IaC: `infra/` directory using Pulumi Rust SDK.
