# OpenWand External Review Packet

**Version:** v0.8.0 (candidate)
**Date:** 2026-06-14
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
# Expected: openwand 0.8.0
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
openwand trace-verify <session-id>
```

### What It Does

Reads the trace store for the given session and verifies:
1. **Chain continuity** — each entry's `prev_hash` links to the prior entry's `entry_hash`
2. **Hash correctness** — recomputes BLAKE3 hashes for every entry payload and
   compares against stored `entry_hash` values (via `Blake3HashPolicy`)
3. **Ordering** — entries are globally sequenced with no gaps

### Exit Codes

| Code | Meaning |
|------|---------|
| 0 | Pass — chain is continuous and hashes are correct |
| 1 | Operational error (DB not found, IO error) |
| 2 | Fail — integrity violation detected (broken chain or hash mismatch) |
| 3 | Inconclusive — not enough data to verify |
| 4 | Unsupported — backend does not support verification |

### Expected Output (Pass)

```
Trace Verification Report
=========================
Session:    <session-id>
Result:     Pass
Entries:    <N>
Hash Policy: Blake3HashPolicy

Checks performed:
  - Chain continuity (prev_hash → entry_hash linkage)
  - Hash recomputation (BLAKE3 over payload bytes)
  - Global sequence ordering

Note: Pass proves that the trace store is internally consistent and
that stored hashes match recomputed hashes. It does NOT prove physical
immutability — an attacker who rewrites the store AND recomputes all
hashes can produce a self-consistent trace. Full immutability requires
an external trust anchor (see Section 4).
```

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
openwand operation-replay --session <session-id> --operations <ops.json>
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

### What It Proves

- Desktop-initiated operations (workflow runs, approvals, exports) have
  corresponding trace entries in the session's trace store
- The "replay" is correspondence checking, NOT execution replay — the verifier
  does not instantiate runners, tools, exporters, gates, or policies

### What It Does NOT Prove

- That the operations were executed correctly
- That the trace entries themselves are authentic (see Section 2)
- Workflow operations on legacy traces (pre-v0.6) — reported as Inconclusive

---

## 4. External Checkpoint Anchor

### 4a. Write an Anchor

```bash
openwand anchor-write <session-id> --anchor-root /external/path [--sequence N]
```

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

### Collision Protection

Writing a checkpoint with a sequence number that already exists for that
session is rejected.

### 4b. Verify an Anchor

```bash
openwand anchor-verify <session-id> --anchor /path/to/anchor.json
```

### What It Does

1. Reads the anchor file
2. Recomputes the root hash from the trace store (entries up to
   `last_global_sequence`)
3. Compares against the anchor's stored `root_hash`
4. Reports freshness status (Current / Stale)

### Exit Codes

| Code | Meaning |
|------|---------|
| 0 | Pass — anchor hash matches store hash |
| 1 | Operational error |
| 2 | Fail — hash mismatch (tampering detected) |
| 3 | Missing — no anchor file found |
| 4 | Unsupported |

### Freshness

- **Current**: store has no entries beyond `last_global_sequence`
- **Stale**: store has additional entries (append-only growth). This is NOT
  a failure — it means the anchor is simply outdated, not tampered.

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
openwand evidence-report \
  --session <session-id> \
  --operations <ops.json> \
  [--anchor /path/to/anchor.json] \
  --output report.json
```

### What It Does

Aggregates all verification results into a single JSON report:
1. **Trace verification** — chain + hash results
2. **Operation replay** — correspondence results
3. **Anchor verification** — if anchor file provided
4. **Security scan summary** — sourced from `docs/SECURITY_SCAN_RESULTS.md`
5. **Authority review summary** — sourced from `docs/AUTHORITY_REVIEW.md`

### Output Result Types

| Result | Meaning |
|--------|---------|
| Complete | All sources available, no caveats |
| CompleteWithCaveats | Missing anchor, stale, inconclusive, or unavailable source |
| Incomplete | Trace loading error or malformed operations |

### What It Proves

- Aggregates existing evidence into a reviewer-readable format
- Each source is honestly reported (missing sources → "unavailable", not faked)

### What It Does NOT Prove

- Creates no new assurance facts beyond what individual checks prove
- Does not replace running individual verification commands

---

## 6. Security Scan Evidence

### Source Document

`docs/SECURITY_SCAN_RESULTS.md` (generated by `cargo audit` + production clippy)

### Latest Results (Wave 105A)

| Check | Result |
|-------|--------|
| Dependencies scanned | 721 |
| CVEs found | 0 |
| Upstream-blocked warnings | 15 (13 GTK3 desktop-only, 1 atomic-polyfill via Loro, 1 rand 0.7 via wry) |
| Production clippy | 0 warnings across 12 crates |
| Authority boundary guards | All pass |
| Production `unsafe` blocks | 1 (`libc::dup` in sandbox) |

### What This Proves

- No known vulnerabilities in dependencies at time of scan
- Production code passes clippy with zero warnings
- Authority boundaries are enforced at source level

### What This Does NOT Prove

- Absence of zero-day vulnerabilities
- Formal security certification
- Runtime memory safety (only source-level analysis)

---

## 7. Authority Review

### Source Document

`docs/AUTHORITY_REVIEW.md`

### Summary (Wave 105B)

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

### What This Proves

- Clear separation between read-only verification and write-capable execution
- No verifier has mutation authority over trace, memory, or approval records

### What This Does NOT Prove

- Absence of privilege escalation paths (not a formal penetration test)
- Runtime enforcement (source-level analysis only)

---

## 8. Linux GUI Partial Validation

### Source Document

`docs/LINUX_GUI_SMOKE_TEST.md`

### Result: Partial

| Check | Result |
|-------|--------|
| CLI binary compiles on Linux | ✅ Pass |
| Desktop binary (`openwand-ui`) compiles on Linux | ✅ Pass (9 latent bugs fixed in Wave 109A) |
| Desktop binary launches | ✅ Pass |
| GTK/WebKit/Dioxus initializes | ✅ Pass |
| Window created | ✅ Pass (xdotool confirmed) |
| Direct rendering | ✅ Pass (glxinfo confirmed) |
| Application stability | ✅ 6+ seconds, no crash |
| Visual rendering | ❌ Not captured (WebKit compositing limitation in virtualized GPU) |
| Interactive UI | ❌ Not tested |

### What This Proves

- The Linux desktop binary compiles and initializes correctly
- The runtime stack (GTK + WebKit + Dioxus) is functional at the initialization level
- Previous build gate gap (CLI-only, not desktop binary) is now closed

### What This Does NOT Prove

- Full Linux GUI support
- Cross-platform runtime validation complete
- Visual rendering correctness
- Interactive UI behavior

---

## 9. Caveats and Non-Claims

### OpenWand v0.8.0 does NOT claim:

1. **Production readiness** — this is a development release
2. **Formal security certification** — no external audit has been performed
3. **Physical immutability** — trace stores are technically mutable files
4. **Remote attestation** — no hardware or network attestation mechanism
5. **Full immutability** — attacker who rewrites store + anchor passes verification
6. **Cross-platform runtime validation** — Linux GUI is Partial, macOS not tested
7. **Provider completeness** — only LM Studio and Z.AI validated
8. **Stable API guarantee** — APIs may change between versions
9. **Full Linux GUI support** — visual rendering not validated
10. **Interactive UI validation on Linux** — not tested

### OpenWand v0.8.0 DOES claim:

1. **Trace chain + hash verification** — internally consistent under BLAKE3 recomputation
2. **Operation-to-trace correspondence** — desktop operations have matching trace entries
3. **External checkpoint anchors** — prefix verification with path containment
4. **Evidence report aggregation** — honest, sourced evidence in reviewer-readable format
5. **Authority boundary enforcement** — source-level guards on all 12 surfaces
6. **Zero CVEs** — across 721 dependencies at time of scan
7. **Linux desktop binary compilation** — both CLI and desktop binaries compile
8. **Linux runtime initialization** — GTK/WebKit/Dioxus stack initializes without crash

---

## 10. Quick Reviewer Checklist

A reviewer can run these commands in sequence:

```bash
# 1. Build
cargo build --release --bin openwand
cargo build --release --bin openwand-ui --features desktop

# 2. Create a test session
./target/release/openwand --base-url http://localhost:1234/v1 "Hello"

# 3. Verify trace integrity
./target/release/openwand trace-verify <session-id>
# Expected: exit 0, Result: Pass

# 4. Write an external anchor
./target/release/openwand anchor-write <session-id> --anchor-root /tmp/anchors

# 5. Verify the anchor
./target/release/openwand anchor-verify <session-id> --anchor /tmp/anchors/<session-id>_*.json
# Expected: exit 0, Pass, Current

# 6. Create operations file (if desktop operations were performed)
echo '{"operations":[]}' > ops.json

# 7. Run operation replay
./target/release/openwand operation-replay --session <session-id> --operations ops.json

# 8. Generate evidence report
./target/release/openwand evidence-report \
  --session <session-id> \
  --operations ops.json \
  --anchor /tmp/anchors/<session-id>_*.json \
  --output review_report.json
# Expected: CompleteWithCaveats (or Complete if all sources available)

# 9. Review authority surfaces
cat docs/AUTHORITY_REVIEW.md

# 10. Review scan results
cat docs/SECURITY_SCAN_RESULTS.md
```

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
```
