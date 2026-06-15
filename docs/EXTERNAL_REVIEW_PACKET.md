# OpenWand External Review Packet

**Version:** v1.0.1
**Updated:** Wave 130A (post-v1.0 external review re-classification)
**Date:** 2026-06-15
**Status:** Reviewer-ready

---

## Purpose

This document packages OpenWand's verification, anchor, authority-review,
scan, and evidence-report capabilities into a single artifact that an external
reviewer can use to independently validate claims.

It includes exact commands, expected outputs, exit code semantics, and honest
statements of what each check proves and does not prove.

---

## 1. Prerequisites

### Build

```bash
# Clone and build (CLI binary — works on Windows and Linux)
git clone https://github.com/Octo-Lex/OpenWand.git
cd OpenWand
cargo build --release --bin openwand

# Desktop binary (requires GTK3 + webkit2gtk-4.1 on Linux)
cargo build --release --bin openwand-ui --features desktop
```

### Verify Build

```bash
./target/release/openwand --version
# Expected: openwand 1.0.1
```

### Verify Artifact Identity

```bash
# CLI binary
sha256sum target/release/openwand
# Expected (v1.0.1): 5ED051CAFF4534F372B6ABF10D1263422F3CE1357814A121DF4822848857ECF5

# Desktop binary
sha256sum target/release/openwand-ui
# Expected (v1.0.1): 2C6AB04D42AA6EFE742643CB344456814222786D6BCB67ACA5D8A56E785C7D40
```

### Generate Test Session (if needed)

```bash
# Run a simple session to generate trace data
./target/release/openwand --base-url http://localhost:1234/v1 "Hello"
# This creates openwand.db with trace entries
```

---

## 2. Trace Integrity Verification

### Command

```bash
openwand trace-verify <session-id> [--db <path>]
```

> **v1.0.1 change:** The `--db` flag is now honored. Previously, the command
> silently used `%APPDATA%/openwand/openwand.db` regardless of the flag.

### What It Does

Reads the trace store for the given session and verifies:
1. **Chain continuity** — each entry's `prev_hash` links to the prior entry's `entry_hash`
2. **Hash correctness** — recomputes BLAKE3 hashes for every entry payload and
   compares against stored `entry_hash` values (via `Blake3HashPolicy`)
3. **Ordering** — entries are globally sequenced with no gaps

> **v1.0.1 fix (F-VL1-1):** Hash recomputation now uses the same canonical
> scope serialization (`serde_json::to_string`) as the writer path. In v1.0.0,
> the verifier used `format!("{:?}")` which produced different bytes, causing
> all fresh traces to fail verification.

### Exit Codes

| Code | Meaning |
|------|---------|
| 0 | Pass — chain is continuous and hashes are correct |
| 1 | Operational error (DB not found, IO error) |
| 2 | Fail — integrity violation detected (broken chain or hash mismatch) |
| 3 | Inconclusive — not enough data to verify |
| 4 | Unsupported — backend does not support verification |

### What It Proves

- The trace store is internally consistent at the time of verification
- Stored hash values match recomputed BLAKE3 hashes
- Entry chain has no broken links or gaps

### What It Does NOT Prove

- Physical immutability of the store file
- That the store was not rewritten before verification
- Tamper detection against an attacker who rewrites store + recomputes hashes

---

## 3. Operation Replay Verification

### Command

```bash
openwand operation-replay --session <session-id> --operations <ops.json> [--db <path>]
```

### Input Format (ops.json)

```json
{
  "operations": [
    {
      "type": "workflow_initiation",
      "workflow_execution_id": "wfx-001"
    },
    {
      "type": "approval_resolution",
      "approval_request_id": "arid-001",
      "tool_call_id": "tc-001"
    },
    {
      "type": "evidence_export",
      "workflow_execution_id": "wfx-001",
      "artifact_path": "/path/to/export.json",
      "artifact_hash": "abc123..."
    }
  ]
}
```

### What It Does

Checks whether each desktop operation has corresponding trace evidence:
- **WorkflowInitiation**: looks for `WorkflowEvent::ModStarted` and
  `ModCompleted` in `Session:workflow:{execution_id}` stream
- **ApprovalResolution**: looks for `GateEvent` with matching approval/tool ID
- **EvidenceExport**: looks for `ArtifactEvent::Generated` in
  `Session:export:{workflow_execution_id}` stream

### Exit Codes

| Code | Meaning |
|------|---------|
| 0 | Pass — all operations have corresponding trace evidence |
| 1 | Operational error |
| 2 | Fail — one or more operations have no matching trace evidence |
| 3 | Inconclusive — legacy trace without expected event types |
| 4 | Unsupported — operation type not recognized |

---

## 4. External Checkpoint Anchor

### 4a. Write an Anchor

```bash
openwand anchor-write <session-id> --anchor-root /external/path [--db <path>] [--sequence N]
```

> **v1.0.1 change:** The `--db` flag is now honored. Previously, the command
> silently used `%APPDATA%/openwand/openwand.db`.

### What It Does

Creates a checkpoint anchor file outside the trace store root. The anchor
contains:
- `last_global_sequence` — the highest entry sequence covered
- `root_hash` — BLAKE3 rollup over all `entry_hash` values up to that sequence
- `entry_count` — total entries covered
- `hash_algorithm` — "blake3"
- `created_at` — timestamp

### Path Containment Rules

The anchor root must be:
- Not equal to the store root
- Not inside the store root
- Not containing the store root
- An existing directory

### 4b. Verify an Anchor

```bash
openwand anchor-verify <session-id> --anchor /path/to/anchor.json [--db <path>]
```

### What It Proves

- Entries up to `last_global_sequence` have not been modified, removed, or
  reordered since the anchor was created
- The anchor was written outside the store, so an attacker who only modifies
  the store cannot produce a matching anchor

### What It Does NOT Prove

- Tamper detection against an attacker who rewrites BOTH the store AND the
  anchor file
- Entries appended after the anchor was created (these are simply not covered)
- Remote attestation or cryptographic non-repudiation

---

## 5. Evidence Report

### Command

```bash
openwand evidence-report <session-id> \
  --operations <ops.json> \
  [--anchor /path/to/anchor.json] \
  [--db <path>] \
  --output report.json
```

### Output Result Types

| Result | Meaning |
|--------|---------|
| Complete | All sources available, no caveats |
| CompleteWithCaveats | Missing anchor, stale, inconclusive, or unavailable source |
| Incomplete | Trace loading error or malformed operations |

---

## 6. Guided Review (Single Command)

### Command

```bash
openwand review <session-id> \
  --operations <ops.json> \
  [--anchor /path/to/anchor.json] \
  [--db <path>] \
  --output review.json
```

Chains all verification steps into a single guided flow:
1. Trace verification (chain + hash)
2. Operation replay (correspondence)
3. Anchor verification (if anchor provided)
4. Evidence report aggregation

---

## 7. Security Scan Evidence

### Latest Results (v1.0.1, Wave 129A)

| Check | Result |
|-------|--------|
| Dependencies scanned | 721 |
| CVEs found | 0 |
| Production clippy | 0 warnings across 11 crates |
| Authority boundary guards | All pass |
| Production `unsafe` blocks | 1 (`libc::dup` in sandbox) |

---

## 8. Authority Review

12 authority surfaces identified:

| Surface | Capability | Write Authority |
|---------|-----------|-----------------|
| S1: CLI Parser | Parse commands | No |
| S2: Desktop UI | Display + request | No |
| S3: UiSessionService | Session operations | **Yes** |
| S4: Policy Engine | Evaluate rules | No |
| S5: Tool Executor | Execute tools | **Yes** |
| S6: Session Runner | Run agent loop | **Yes** |
| S7: Memory Store | Read/write memory | Conditional |
| S8: Trace Verifier | Verify integrity | No (read-only) |
| S9: Operation Replay | Verify correspondence | No (read-only) |
| S10: Anchor Writer | Write checkpoint | **Yes** (outside store) |
| S11: Evidence Report | Aggregate evidence | No (read-only) |
| S12: Workflow Gates | Evaluate readiness | No |

4 write-capable surfaces (S3, S5, S6, S10). 3 read-only verifiers (S8, S9, S11).

---

## 9. Caveats and Non-Claims

### OpenWand v1.0.1 does NOT claim:

1. **Production readiness**
2. **Formal security certification**
3. **Physical immutability** — trace stores are mutable files
4. **Remote attestation** — no hardware/network attestation mechanism
5. **Full immutability** — attacker who rewrites store + anchor passes verification
6. **Full cross-platform runtime validation** — Linux GUI is Partial, macOS not tested
7. **Provider completeness** — only LM Studio and Z.AI validated
8. **Stable API guarantee** — APIs may change between versions
9. **Full Linux GUI support** — visual rendering not validated
10. **External review** — no external reviewer has executed this packet
11. **macOS validated** — not tested
12. **Fully consistent tamper passes** — attacker can forge self-consistent state
13. **Windows final-component TOCTOU** — residual timing window
14. **openwand-content stub crate** — not a real implementation
15. **Dependency warnings** — 15 transitive warnings (desktop-only paths)

### OpenWand v1.0.1 DOES claim:

1. **Trace chain + hash verification** — internally consistent under BLAKE3 recomputation
2. **Operation-to-trace correspondence** — desktop operations have matching trace entries
3. **External checkpoint anchors** — prefix verification with path containment
4. **Evidence report aggregation** — honest, sourced evidence in reviewer-readable format
5. **Authority boundary enforcement** — source-level guards on all 12 surfaces
6. **Zero CVEs** — across 721 dependencies at time of scan
7. **Linux desktop binary compilation** — both CLI and desktop binaries compile
8. **Linux runtime initialization** — GTK/WebKit/Dioxus stack initializes without crash
9. **CLI and desktop artifact identity** — SHA-256 recorded and verifiable
10. **4,430 tests pass** — full workspace test suite, 0 failures
11. **First real workflow evidence** — VL-1 passes post-fix (agent turn → trace verify → evidence report)
12. **Maintenance patch discipline** — v1.0.1 fixes found bugs with regression tests

---

## 11. Architecture Arc

```
v0.2  governed execution substrate          Control
v0.3  live observation                      Observe
v0.4  desktop operation requests            Operate
v0.5  read-only verification                Verify
v0.6  evidence-backed assurance hardening   Harden
v0.7  external assurance                    Externally Anchor
v0.8  operational hardening                 Operationally Harden
v0.9  external validation                   Externally Validate
v1.0  release-candidate closure             Close
```

---

## 12. Quick Reviewer Checklist

```bash
# 1. Build
cargo build --release --bin openwand
cargo build --release --bin openwand-ui --features desktop

# 2. Verify version
./target/release/openwand --version
# Expected: openwand 1.0.1

# 3. Create a test session
./target/release/openwand --base-url http://localhost:1234/v1 "Hello"

# 4. Verify trace integrity (with --db flag, v1.0.1)
./target/release/openwand trace-verify <session-id> --db openwand.db
# Expected: exit 0, Result: Pass

# 5. Write an external anchor
./target/release/openwand anchor-write <session-id> --db openwand.db --anchor-root /tmp/anchors

# 6. Verify the anchor
./target/release/openwand anchor-verify <session-id> --db openwand.db --anchor /tmp/anchors/openwand-checkpoint-1.json
# Expected: exit 0, Pass, Current

# 7. Run operation replay
echo '{"operations":[]}' > ops.json
./target/release/openwand operation-replay --session <session-id> --db openwand.db --operations ops.json

# 8. Generate evidence report
./target/release/openwand evidence-report <session-id> \
  --db openwand.db \
  --operations ops.json \
  --anchor /tmp/anchors/openwand-checkpoint-1.json \
  --output review_report.json
# Expected: Complete

# 9. Run guided review (chains all steps)
./target/release/openwand review <session-id> \
  --db openwand.db \
  --operations ops.json \
  --anchor /tmp/anchors/openwand-checkpoint-1.json \
  --output guided_review.json
# Expected: Complete

# 10. Review authority surfaces and scan results
cat docs/AUTHORITY_REVIEW.md
cat docs/SECURITY_SCAN_RESULTS.md
```
