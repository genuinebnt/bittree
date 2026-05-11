# Distributed Systems Study Guide & Bittree Integration

This guide maps three world-class distributed systems resources—**Designing Data-Intensive Applications (DDIA)**, **MIT 6.824**, and **Fly.io Gossip Glomers**—directly to the features you are building in BitTree. 

By interleaving theory with your practical Rust implementation, you will deeply internalize these concepts.

---

## 📖 1. Designing Data-Intensive Applications (DDIA)

*Read the relevant chapter immediately before or during the corresponding BitTree phase.*

| DDIA Chapter | Core Concept | Maps to BitTree Phase | How to apply it in BitTree |
| :--- | :--- | :--- | :--- |
| **Ch 3: Storage and Retrieval** | B-Trees vs LSM-Trees, Hash Indexes | **Phase 1 (DB) & Phase 7 (Search)** | Compare PostgreSQL's B-Trees (for user queries) against Tantivy's LSM-like inverted index segments. |
| **Ch 5: Replication** | Single-leader, Multi-leader, Leaderless, Replication Lag | **Phase 5 (AWS Roadmap)** | Understand how your AWS RDS PostgreSQL instance handles read-replicas and what happens during failover. |
| **Ch 7: Transactions** | ACID, Isolation Levels (Read Committed, Serializable), Race Conditions | **Phase 1 (Document Service)** | Use `version` columns for optimistic concurrency control to prevent lost updates when two users edit the same page simultaneously. |
| **Ch 9: Consistency & Consensus** | Linearizability, CAP Theorem, Distributed Locks | **Phase 18 (Analytics/ETL)** | Implement a distributed lock using Redis (with a fencing token) to ensure only one ETL worker aggregates daily stats. |
| **Ch 11: Stream Processing** | Message Brokers, Log-based Message Brokers, Event Sourcing | **Phase 13 (Analytics) & Phase 21 (Webhooks)** | This chapter is exactly how **NATS JetStream** works. Apply these concepts when building your event-driven outbox pattern. |

---

## 🛠️ 2. Fly.io Gossip Glomers (in Rust)

*A practical distributed systems challenge using the Maelstrom network simulator. Do these in Rust to master concurrent state management.*

| Challenge | What You Build | Maps to BitTree Phase | BitTree Application |
| :--- | :--- | :--- | :--- |
| **1 & 2: Echo / Unique ID** | Basic RPC, generating globally unique IDs | **Phase 0 & 1** | Replacing standard UUIDs with distributed sortable IDs (like UUIDv7 or Snowflake) for block insertion. |
| **3: Broadcast** | Gossip protocols, eventual consistency, network partition tolerance | **Phase 9 (Notifications)** | Simulates how you might fan out a notification to multiple active WebSocket servers in an unreliable network. |
| **4: Grow-Only Counter** | CRDTs (Conflict-free Replicated Data Types) | **Phase 4 (Collaboration)** | The perfect warm-up for the YATA CRDT you will build for real-time text synchronization. |
| **5: Kafka-style Log** | Append-only logs, offsets, consumer groups | **Phase 13 & 21** | Hands-on practice with the exact primitives underlying NATS JetStream and your Webhook/Audit log architecture. |

---

## 🎓 3. MIT 6.824 (Distributed Systems)

*Focus on the lectures and papers that explain the infrastructure you are relying on.*

| Lecture / Paper | Concept | Maps to BitTree Architecture |
| :--- | :--- | :--- |
| **Lec 3: GFS (Google File System)** | Distributed file storage | **Phase 6 (Storage):** Understands the theory behind how Amazon S3 (and your local MinIO) physically stores files. |
| **Lec 6-8: Raft** | Consensus, Leader Election, Log Replication | **Core Infrastructure:** NATS JetStream uses Raft internally for high availability. You must understand Raft to tune NATS reliably. |
| **Lec 11: Aurora** | Cloud-native databases | **AWS Roadmap:** Amazon Aurora separates compute from storage. Understand why this is vastly superior to standard RDS for scaling BitTree. |
| **Lec 14: Spanner** | TrueTime, Multi-region ACID | **Future Scale:** How BitTree would handle multi-continent active-active PostgreSQL databases without clock-skew data corruption. |

---

## 🗓️ Recommended Chronological Sequence

To avoid burnout, do not read everything at once. Interleave theory with Rust coding:

1. **Foundations (Weeks 1-2)**
   - *Theory:* Read DDIA Chapters 1-3. Watch MIT 6.824 MapReduce & GFS lectures.
   - *Action:* Build BitTree Phase 0 & 1 (Infrastructure & Document CRUD).
2. **Distributed Code Bootcamp (Weeks 3-4)**
   - *Theory:* Read DDIA Chapter 5 (Replication).
   - *Action:* Complete Gossip Glomers Challenges 1, 2, and 3 in Rust.
3. **Concurrency & Real-time (Weeks 5-6)**
   - *Theory:* Read DDIA Chapter 7 (Transactions). Complete Gossip Glomers Challenge 4 (CRDT).
   - *Action:* Build BitTree Phase 4 (Real-time Collaboration & WebSocket CRDT).
4. **Event-Driven Mastery (Weeks 7-8)**
   - *Theory:* Read DDIA Chapter 11 (Stream Processing). Watch MIT 6.824 Raft lectures. Complete Gossip Glomers Challenge 5 (Kafka-style Log).
   - *Action:* Integrate NATS JetStream. Build BitTree Phase 13 (Analytics) and Phase 21 (Webhooks/Outbox).
5. **Cloud Scale (Weeks 9-10)**
   - *Theory:* Read DDIA Chapter 9 (Consensus). Watch MIT 6.824 Aurora & Spanner lectures.
   - *Action:* Execute the `AWS_ROADMAP.md` (Terraform/Pulumi, RDS, EKS).
