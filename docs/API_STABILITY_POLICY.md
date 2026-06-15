# API Stability Policy — Wave 131A (VL-6)

**Date:** 2026-06-15
**Wave:** 131A
**Commit:** (recorded at lock)
**Tag:** `wave-131a-lock`
**Blocker:** VL-6 — API stability policy

---

## 1. Purpose

This document defines which OpenWand API surfaces are stable, which are
evolving, which are experimental, which are internal, and which are
explicitly unsupported. It defines compatibility rules per release type
and deprecation procedures.

**This policy classifies stability. It does not automatically make every
surface stable.**

---

## 2. Stability Categories

| Category | Meaning | Compatibility Commitment |
|----------|---------|------------------------|
| **Stable** | Surface is production-quality. Breaking changes only in major releases (x.0.0). | SemVer: patch and minor releases preserve backward compatibility |
| **Supported (Evolving)** | Surface works and is maintained, but may receive additive or minor breaking changes in minor releases. | Breaking changes announced with deprecation window (1 minor cycle). |
| **Experimental** | Surface exists for testing and iteration. May change or be removed without notice. | No compatibility commitment. |
| **Internal** | Surface is for OpenWand's own use. Not intended for external consumers. | No compatibility commitment. May change freely. |
| **Unsupported** | Surface exists but is not maintained, tested, or guaranteed for external use. | No compatibility commitment. |

---

## 3. Surface Classification

### 3.1 CLI Commands

| Command | Category | Rationale |
|---------|----------|-----------|
| `run` (agent turn) | **Stable** | Core function. Users depend on this. |
| `explain` | **Stable** | Core function. |
| `trace-verify` | **Stable** | Evidence path. Reviewers depend on this. |
| `operation-replay` | **Stable** | Evidence path. |
| `anchor-write` | **Stable** | Evidence path. |
| `anchor-verify` | **Stable** | Evidence path. |
| `review` | **Stable** | Evidence path. Guided review flow. |
| `evidence-report` | **Stable** | Evidence path. |
| `release-check` | **Supported (Evolving)** | Internal release tooling. May add checks. |
| `session-rebuild` | **Experimental** | Prototype functionality. |
| `audit-check` | **Internal** | Internal tooling. |
| `task-plan` | **Experimental** | Workflow subsystem, iterating. |
| `workflow-*` (24 commands) | **Experimental** | Workflow subsystem under active design. |
| `audit-packet-*` | **Internal** | Internal audit tooling. |

### 3.2 CLI Flags (Common)

| Flag | Category | Rationale |
|------|----------|-----------|
| `--db <path>` | **Stable** | Explicit database path. Fixed in v1.0.1. |
| `--base-url <url>` | **Stable** | LLM provider endpoint. |
| `--model <name>` | **Stable** | Model selection. |
| `--anchor-root <path>` | **Stable** | Anchor write path. |
| `--anchor <path>` | **Stable** | Anchor verify path. |
| `--operations <path>` | **Stable** | Operation replay input. |
| `--output <path>` | **Stable** | Report output path. |
| `--session <id>` | **Stable** | Session selection. |

### 3.3 JSON Report Schemas

| Schema | Struct | Category | Rationale |
|--------|--------|----------|-----------|
| Trace verification report | `VerificationReport` | **Stable** | Reviewers depend on this. |
| Operation replay report | `ReplayReport` | **Stable** | Reviewers depend on this. |
| Anchor verification report | `AnchorVerificationReport` | **Stable** | Reviewers depend on this. |
| Evidence report | `EvidenceReport` | **Stable** | Reviewers depend on this. |
| Release check report | `ReleaseCheckReport` | **Internal** | Internal tooling output. |

#### Stable JSON Schemas — Field Stability Rules

For **Stable** report schemas:
- Existing fields will not be renamed or removed in patch/minor releases
- New fields may be **added** in minor releases (consumers must tolerate unknown fields)
- Field **types** will not change for existing fields
- The `result` enum values will not be removed (new values may be added)

### 3.4 Trace Event Schemas

| Surface | Category | Rationale |
|---------|----------|-----------|
| `TraceStreamScope` enum | **Stable** | Core to trace identity. |
| `TraceStreamId` struct | **Stable** | Core to trace identity. |
| `TraceEntry` struct (serialized form) | **Stable** | Written to store, read by verifier. |
| `entry_hash` field (BLAKE3) | **Stable** | Hash algorithm pinned. |
| `prev_hash` field (chain linkage) | **Stable** | Chain structure. |
| `global_sequence` field | **Stable** | Ordering. |
| `stream_sequence` field | **Stable** | Per-stream ordering. |
| Event payload formats (serialized JSON) | **Supported (Evolving)** | May add new event types. Existing event types stable. |

#### Trace Compatibility Rules

- BLAKE3 hash algorithm is pinned. Will not change without major version.
- Hash computation inputs (canonical serialization) are pinned for existing
  scopes. v1.0.1 fixed the scope serialization to `serde_json::to_string`.
- New `TraceStreamScope` variants may be added in minor releases. Consumers
  must tolerate unknown scopes.
- New event payload types may be added in minor releases.

### 3.5 Operation Descriptors

| Surface | Category | Rationale |
|---------|----------|-----------|
| `DesktopOperation::WorkflowInitiation` | **Stable** | Operation replay depends on this. |
| `DesktopOperation::ApprovalResolution` | **Stable** | Operation replay depends on this. |
| `DesktopOperation::EvidenceExport` | **Stable** | Operation replay depends on this. |
| Operations JSON file format | **Stable** | Reviewer input format. |

### 3.6 Config Files

| Surface | Category | Rationale |
|---------|----------|-----------|
| `openwand.db` (SQLite trace store) | **Supported (Evolving)** | Schema may receive migrations in minor releases. Backward compatible. |
| Checkpoint anchor JSON files | **Stable** | Must remain readable across versions. |

### 3.7 Desktop Service DTOs

| Surface | Category | Rationale |
|---------|----------|-----------|
| `UiMessage` | **Internal** | Desktop UI internal protocol. |
| `CreateSessionRequest` | **Internal** | Desktop UI internal protocol. |
| `ApprovalResolutionRequest` | **Internal** | Desktop UI internal protocol. |
| `EvidenceExportRequest` | **Internal** | Desktop UI internal protocol. |
| Workflow execution state types | **Experimental** | Workflow subsystem iterating. |

### 3.8 Crate-Level Rust APIs

| Crate | Category | Rationale |
|-------|----------|-----------|
| `openwand-trace` | **Supported (Evolving)** | Trace types and verifier. Stable core, evolving API. |
| `openwand-store` | **Internal** | Storage backend. Not for external use. |
| `openwand-core` | **Internal** | Core domain types. |
| `openwand-session` | **Internal** | Session management. |
| `openwand-policy` | **Internal** | Policy engine. |
| `openwand-llm` | **Internal** | LLM client. |
| `openwand-tools` | **Internal** | Tool execution. |
| `openwand-memory` | **Internal** | Memory store. |
| `openwand-workflow` | **Experimental** | Workflow subsystem under design. |
| `openwand-goals` | **Experimental** | Goal management. |
| `openwand-skills` | **Experimental** | Skills system. |
| `openwand-mcp-pool` | **Internal** | MCP connection pool. |
| `openwand-content` | **Unsupported** | Stub crate. See caveat X-13. |
| `openwand-app` (binaries) | **N/A** | Binary crate, not a library API. |

**No crate is published to crates.io.** All crate-level APIs are internal
to the OpenWand workspace until explicitly published. This classification
documents intent for future publication.

---

## 4. Compatibility Rules

### 4.1 Patch Release (v1.0.x → v1.0.x+1)

- **Stable surfaces**: No breaking changes. Bug fixes only.
- **Supported (Evolving)**: No breaking changes. Bug fixes only.
- **Experimental**: May change freely.
- **Internal**: May change freely.
- **Unsupported**: May change freely.

### 4.2 Minor Release (v1.x.0 → v1.x+1.0)

- **Stable surfaces**: Additive changes only (new fields, new enum variants,
  new commands). No removals, no renames, no type changes.
- **Supported (Evolving)**: Breaking changes allowed with deprecation window.
  Deprecated surfaces must be marked for one minor cycle before removal.
- **Experimental**: May change freely.
- **Internal**: May change freely.
- **Unsupported**: May change freely.

### 4.3 Major Release (vx.0.0 → vx+1.0.0)

- All surfaces may receive breaking changes.
- Migration guide required.

### 4.4 Deprecation Window

When a **Stable** or **Supported (Evolving)** surface is deprecated:
1. The surface is marked `#[deprecated]` in Rust code
2. Documentation is updated with deprecation notice
3. The surface remains functional for one minor release cycle
4. The surface is removed in the next minor release after the deprecation cycle

**Experimental**, **Internal**, and **Unsupported** surfaces may be removed
without a deprecation window.

---

## 5. Caveat X-08 Update

### Original (v0.5.0)

> X-08: Not stable API guarantee — APIs may change between versions

### Updated (Wave 131A)

> X-08 (Updated): OpenWand now has an API stability policy (see
> `docs/API_STABILITY_POLICY.md`). The following surfaces are classified
> **Stable**:
> - CLI commands: `run`, `explain`, `trace-verify`, `operation-replay`,
>   `anchor-write`, `anchor-verify`, `review`, `evidence-report`
> - CLI flags: `--db`, `--base-url`, `--model`, `--anchor-root`, `--anchor`,
>   `--operations`, `--output`, `--session`
> - JSON report schemas: `VerificationReport`, `ReplayReport`,
>   `AnchorVerificationReport`, `EvidenceReport`
> - Trace schemas: `TraceStreamScope`, `TraceStreamId`, `TraceEntry` fields
> - Operation descriptors: all three `DesktopOperation` variants
> - Anchor JSON files
>
> All other surfaces (crate-level Rust APIs, desktop DTOs, workflow commands,
> session-rebuild, release-check) remain **not stable** and may change.
> **X-08 is partially narrowed — Stable surfaces are committed, but the
> majority of the API surface remains evolving or experimental.**

### Classification

**X-08 is PARTIALLY NARROWED.** It is not resolved. The global "not stable
API guarantee" caveat is narrowed to apply only to non-Stable surfaces.
The explicitly classified Stable surfaces now have a compatibility commitment.

---

## 6. What This Policy Does NOT Claim

- Not a global API freeze
- Not a guarantee that all surfaces are stable
- Not a commitment to never change Experimental/Internal surfaces
- Not a crates.io publication commitment
- Not production readiness
- Not formal certification
- Not external review
- Not provider completeness
- Not full Linux GUI validation
- Not physical immutability
- Not remote attestation

---

## 7. Surface Inventory Summary

| Category | Count | Examples |
|----------|-------|----------|
| Stable | 8 CLI commands, 8 CLI flags, 4 JSON schemas, 3 trace types, 3 operation types, anchor files, 1 crate (`openwand-trace` core) | Core verification + agent commands |
| Supported (Evolving) | `release-check`, trace store, event payloads | Maintained but may change |
| Experimental | `session-rebuild`, `task-plan`, `workflow-*`, `openwand-workflow`, `openwand-goals`, `openwand-skills`, workflow DTOs | Active design, no commitment |
| Internal | `audit-check`, `audit-packet-*`, desktop DTOs, `openwand-store`, `openwand-core`, `openwand-session`, `openwand-policy`, `openwand-llm`, `openwand-tools`, `openwand-memory`, `openwand-mcp-pool`, `ReleaseCheckReport` | Not for external consumers |
| Unsupported | `openwand-content` (stub) | Not maintained |
