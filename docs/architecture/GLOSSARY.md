# BitTree — Domain Glossary

> Ubiquitous language for the BitTree domain. Consistent naming across code, docs, and conversations.

---

## Core Domain Terms

| Term | Definition |
|---|---|
| **Workspace** | The top-level organisational unit. A user can belong to many workspaces. All pages, files, and members are scoped to a workspace. |
| **Member** | A user who belongs to a workspace with a specific role. |
| **Role** | A named permission level within a workspace: `Owner`, `Admin`, `Editor`, `Commenter`, `Viewer`. |
| **Page** | A document within a workspace. Pages form a tree (parent/child). A page is a container for blocks. |
| **Root Page** | A page with no parent — appears at the top level of the workspace sidebar. |
| **Block** | The atomic content unit. Every piece of content in a page is a block (paragraph, heading, image, etc.). Blocks form a tree. |
| **Block Tree** | The recursive structure of blocks within a page. A block can have children (e.g., a toggle block contains child blocks). |
| **Block Type** | The semantic type of a block: `paragraph`, `heading_1`, `code`, `image`, `database`, etc. |
| **Rich Text Span** | A segment of text with optional formatting (bold, italic, color, link). A block's text content is a list of spans. |
| **Database** | A special page type where blocks are structured rows with typed property columns (similar to a Notion database). |
| **Database Row** | A block within a database page, representing one record with property values. |
| **Snapshot** | A point-in-time copy of an entire page's block tree, used for version history and restore. |
| **Sort Key** | A fractional index string used to order sibling blocks without renumbering. |
| **Version** | An integer incremented on every write, used for optimistic locking. |
| **Presence** | Real-time awareness of which users are viewing/editing a page and where their cursors are. |
| **Op / Operation** | A CRDT operation representing a single insert, delete, or format change on a block's content. |
| **Session** | An authenticated user's active connection to the collaboration service for a specific page. |
| **Invite** | A one-time token sent to an email address granting membership to a workspace. |
| **Credential** | An authentication method for a user: local (password) or OAuth2 (GitHub, Google). |
| **Access Token** | A short-lived JWT used to authenticate API requests (15 minutes). |
| **Refresh Token** | A long-lived token used to obtain new access tokens (30 days). Stored hashed; rotated on each use. |
| **Token Family** | A group of refresh tokens from the same original login. If any token in the family is reused after rotation, the entire family is revoked. |
| **Domain Event** | An immutable record of something that happened (e.g., `PageCreated`, `BlockUpdated`, `MemberJoined`). Published to NATS and consumed by other services. |
| **Outbox** | A pattern where domain events are written to a DB table atomically with the state change, then published to the broker by a background worker. Prevents dual-write inconsistencies. |

---

## Infrastructure Terms

| Term | Definition |
|---|---|
| **Port** | A trait (interface) the domain uses to interact with the outside world (e.g., `PageRepo`, `EventPublisher`). |
| **Adapter** | A concrete implementation of a port for a specific technology (e.g., `PostgresPageRepo`, `NatsPublisher`). |
| **Repository** | An adapter that abstracts data persistence behind a trait. |
| **Testcontainer** | A Docker container started programmatically during tests to provide real infrastructure (Postgres, Redis). |
| **JetStream** | NATS's durable, persistent messaging layer (equivalent to Kafka consumer groups). Used for reliable event delivery. |
| **Fractional Index** | A lexicographically sortable string key that allows inserting between two elements. See `block.sort_key`. |
| **LTREE** | A PostgreSQL extension for storing and querying hierarchical label paths. Used for `page.path`. |
| **JSONB** | PostgreSQL's binary JSON column type. Used for `block.content` and `block.properties`. |
| **Optimistic Locking** | A concurrency strategy where a `version` field is checked on write — if it has changed, the write is rejected, prompting a retry. |

---

## CRDT Terms (Phase 4)

| Term | Definition |
|---|---|
| **CRDT** | Conflict-free Replicated Data Type — a data structure that can be merged from multiple concurrent sources without conflicts. |
| **YATA** | Yet Another Transformation Approach — the CRDT algorithm used by Yjs for sequence (text) CRDTs. |
| **RGA** | Replicated Growable Array — another sequence CRDT algorithm. |
| **Awareness** | The Yjs protocol for sharing ephemeral, non-persistent state like cursor positions and user names. |
| **Vector Clock** | A logical timestamp mechanism tracking the causal order of events across distributed nodes. |

---

## Naming Conventions in Code

| Domain Term | Rust Type Name | DB Column/Table Name |
|---|---|---|
| User | `User`, `UserId(Uuid)` | `user`, `user_id` |
| Workspace | `Workspace`, `WorkspaceId(Uuid)` | `workspace`, `workspace_id` |
| Page | `Page`, `PageId(Uuid)` | `page`, `page_id` |
| Block | `Block`, `BlockId(Uuid)` | `block`, `block_id` |
| Block Type | `BlockType` (enum) | `block_type` (varchar) |
| Role | `WorkspaceRole` (enum) | `role` (varchar) |
| Domain Event | `DomainEvent` (enum) | N/A — published to NATS |
| Sort Key | `SortKey(String)` | `sort_key` |
