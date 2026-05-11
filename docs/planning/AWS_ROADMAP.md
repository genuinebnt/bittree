# AWS Deployment Roadmap

Since BitTree is a distributed microservice architecture, deploying it manually is infeasible. To close the **Cloud Platforms** and **CI/CD** skill gaps identified in our job market analysis, we will target Amazon Web Services (AWS) using Infrastructure as Code (IaC).

This roadmap bridges the gap between local `docker-compose` and production AWS. 

---

## 🏗️ Architecture Mapping (Local to AWS)

| BitTree Component | Local (`docker-compose`) | AWS Equivalent | Why this choice? |
|---|---|---|---|
| **Compute / Containers** | Docker Engine | **Amazon EKS** (Elastic Kubernetes Service) | Matches Phase 16 (Kubernetes). High demand for K8s in Rust roles. |
| **Relational Database** | `postgres:16-alpine` | **Amazon RDS** (PostgreSQL) | Managed backups, Multi-AZ high availability. |
| **Cache & Sessions** | `redis:7` | **Amazon ElastiCache** (Redis) | Fully managed, sub-millisecond latency. |
| **Message Broker** | `nats:2` (JetStream) | **Amazon MSK** (Managed Kafka) or **Self-Hosted NATS on EKS** | SQS/SNS don't support replay well. NATS on EKS is common. |
| **Object Storage** | `minio/minio` | **Amazon S3** | MinIO implements the S3 API, so no code changes needed. |
| **Ingress / Gateway** | Exposed Ports | **AWS ALB** (Application Load Balancer) | Managed TLS termination, routes traffic to EKS ingress. |
| **Secrets & Config** | `.env` file | **AWS Secrets Manager / SSM Parameter Store** | Secure injection at runtime. |
| **CI/CD** | Manual `cargo build` | **GitHub Actions + ECR** | Automated pipeline to push images to Elastic Container Registry. |

---

## 🛣️ Step-by-Step Learning Plan

### Step 1: CI/CD Foundation
*Before touching AWS, we need an automated way to test and build container images.*

1.  **GitHub Actions for CI:** Create a `.github/workflows/ci.yml` pipeline.
    *   **Jobs:** `cargo fmt`, `cargo clippy`, `cargo test`.
    *   **Caching:** Use `Swatinem/rust-cache` to speed up builds.
2.  **Container Image Builds:** Create `.github/workflows/cd.yml`.
    *   **Task:** Build multi-stage Dockerfiles for each service.
    *   **Registry:** Push to GitHub Container Registry (GHCR) or AWS ECR.

### Step 2: Infrastructure as Code (IaC) with Pulumi Rust
*We will not use the AWS Console. Everything must be defined in code using Pulumi.*

1.  **VPC & Networking:** Write Pulumi code to provision a VPC, public/private subnets, NAT gateways, and Internet Gateways.
2.  **Managed Data Tier:** Provision RDS PostgreSQL (single instance initially) and ElastiCache Redis inside the private subnets.
3.  **Storage:** Provision an S3 bucket for the Storage Service and configure IAM policies for minimal access.

### Step 3: Kubernetes (Amazon EKS)
*Deploying and managing the compute cluster.*

1.  **Cluster Provisioning:** Use Pulumi to spin up an EKS cluster with managed node groups.
2.  **Kubernetes Manifests:** Write `Deployment`, `Service`, `ConfigMap`, and `Secret` YAMLs (or use Pulumi Kubernetes Provider) for `api-gateway`, `auth-service`, and `document-service`.
3.  **Ingress Controller:** Deploy AWS Load Balancer Controller to automatically provision an ALB when an `Ingress` resource is created.
4.  **Autoscaling (HPA):** Configure Horizontal Pod Autoscaler based on CPU/Memory usage.

### Step 4: Observability in the Cloud
*Replacing local Jaeger/Prometheus with production-grade monitoring.*

1.  **Logs:** Deploy Fluent Bit as a DaemonSet to ship container logs to Amazon CloudWatch.
2.  **Metrics:** Use Amazon Managed Service for Prometheus (AMP) to scrape metrics from the Rust services.
3.  **Traces:** Configure the OpenTelemetry Collector to send traces to AWS X-Ray.

---

## 🛠️ Required Code Changes in BitTree

Because of our **Ports & Adapters (Clean Architecture)** design, the domain layer requires **zero changes**. You only need to implement AWS-specific infrastructure adapters:

1.  **`S3Store` Adapter:** Ensure the `ObjectStore` trait implementation uses `aws-sdk-s3` properly (authenticating via IAM roles/IRSA on EKS rather than static access keys).
2.  **Health Checks:** EKS requires readiness and liveness probes. Ensure every service has a `/health` HTTP endpoint (even background workers might need a lightweight HTTP server for this).
3.  **Graceful Shutdown:** Ensure Tokio intercepts `SIGTERM` (sent by Kubernetes during pod scale-down/rolling updates) and cleanly shuts down active WebSocket connections and database pools.

---

## 📚 What This Adds to Your Resume

By completing this AWS integration, you bridge the exact gaps identified in `SKILLS_ANALYSIS.md`:

*   **Cloud Platform Depth:** VPC, IAM, RDS, ElastiCache, S3.
*   **Kubernetes (K8s):** Pods, Deployments, Services, Ingress, HPA.
*   **IaC:** Pulumi (which translates perfectly to Terraform concepts).
*   **CI/CD:** GitHub Actions, container registries, deployment pipelines.
*   **Cloud Native Rust:** Running Rust in containers with proper signal handling, telemetry, and IAM authentication.
