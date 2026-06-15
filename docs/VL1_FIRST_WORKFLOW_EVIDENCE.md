# VL-1 First Real Workflow Evidence — Wave 127A

**Date:** 2026-06-15
**Wave:** 127A
**Blocker:** VL-1 — User adoption / first real workflow evidence
**Classification:** **Partial — completed with findings**

---

## Workflow Description

**Bounded workflow:** A single agent turn through the released v1.0.0 binary.
User sends a message, agent responds, trace is written, verification commands
are exercised.

**Provider:** Local mock OpenAI-compatible server (mock-vl1) on port 18888.
No external LLM provider was directly reachable from the OpenWand binary.
The mock returns deterministic responses — it is not a real LLM, but it
exercises the full HTTP → SSE → response → trace → memory path.

**Why mock:** The Z.AI API key was not extractable from the MCP source for
direct use by the OpenWand binary. LM Studio was not running. The mock
server was chosen to exercise the full agent loop rather than skip it.

---

## Workflow Steps and Evidence

### Step 1: Agent Turn

**Command:**
```bash
openwand run --base-url http://127.0.0.1:18888/v1 --model mock-vl1 "Hello, this is a VL-1 workflow test"
```

**Result:** ✅ Turn completed
- Stop reason: Natural
- Steps: 0
- Tools called: 0
- Memory episodes projected: 2
- Messages: 2 (user + assistant)
- Assistant response received and displayed

**Session ID:** `01KV4B48CESEJDPAF22WAVFHRP`
**Database:** `openwand.db` (relative to CWD)

### Step 2: Trace Verification

**Command:**
```bash
openwand trace-verify 01KV4B48CESEJDPAF22WAVFHRP
```

**Result:** ❌ **FAIL** — 3 hash mismatches
- Entries checked: 3
- Streams: 1
- Findings: 3 errors (HashCorrectnessValid)
- Stored entry_hash values do not match recomputed BLAKE3 hashes

**Exit code:** 2

**This is a blocking finding.** See F-VL1-1 below.

### Step 3: Operation Replay

**Command:**
```bash
openwand operation-replay --session 01KV4B48CESEJDPAF22WAVFHRP --operations vl1_ops.json
```

**Result:** ✅ Pass (zero operations, zero findings)

### Step 4: Anchor Write

**Command:**
```bash
openwand anchor-write 01KV4B48CESEJDPAF22WAVFHRP --anchor-root /tmp/vl1_anchors2
```

**Result:** ✅ Anchor written
- Sequence: 1
- Entries: 3

### Step 5: Anchor Verify

**Command:**
```bash
openwand anchor-verify 01KV4B48CESEJDPAF22WAVFHRP --anchor <anchor-file>
```

**Result:** ✅ Pass — Current
- Root hash matches
- Freshness: Current

### Step 6: Evidence Report

**Command:**
```bash
openwand evidence-report 01KV4B48CESEJDPAF22WAVFHRP --operations vl1_ops.json --anchor <anchor-file> --output vl1_evidence_report.json
```

**Result:** CompleteWithCaveats (4 caveats)
- Trace verification: Fail
- Operation replay: Pass
- Anchor verification: Pass
- Security scan: Available
- Authority review: Available

### Step 7: Guided Review Flow

**Command:**
```bash
openwand review 01KV4B48CESEJDPAF22WAVFHRP --operations vl1_ops.json --anchor <anchor-file> --output vl1_review_report.json
```

**Result:** CompleteWithCaveats
- Full guided flow completed (6 steps)
- All steps executed
- Trace verification step reported Fail
- Report written to `vl1_review_report.json`

### Step 8: Version/Artifact Identity

```
openwand --version: openwand 1.0.0
Release check: 8/8 PASS (4,371 tests)
```

---

## Findings

### F-VL1-1 (BLOCKING): Trace hash verification fails on fresh data

**Severity:** **Blocking** — undermines claims C-02, C-03, C-15
**Category:** Integrity bug

**Description:** When a real agent turn is run through `openwand run`, the
resulting trace entries have stored `entry_hash` values that do NOT match
recomputed BLAKE3 hashes during `trace-verify`. All 3 entries in the test
session fail hash correctness.

**Evidence:**
```
stored:  3f29bdeba8d42c7163e1b3f10d31e0e2e386116c7bf4cd8106992e2646368d1c
recomp:  a7c7dd3d01f9daa39ff24b8ef65b9571d11dedc5b086bd250d5e27c515c52653
```

**Likely cause:** The hash computation in the writer path differs from the
verifier path. Possible causes:
- Serialization format difference between write-time and verify-time
- Field inclusion/exclusion mismatch in the hash payload
- Schema version drift affecting hash computation

**Impact:** This directly undermines claim C-03 (read-only trace integrity
verification). If hash recomputation fails on fresh data, the verifier
cannot reliably detect tampering. The anchor verification passes because
it uses the STORED hashes (not recomputed), so it only proves the stored
hashes are consistent with themselves — not that they match the data.

**Required action:** Fix the hash computation mismatch before any further
release. This is a v1.0.1 patch candidate.

---

### F-VL1-2 (Non-blocking): trace-verify ignores --db CLI flag

**Severity:** Medium — user-facing friction
**Category:** CLI bug

**Description:** The `trace-verify`, `evidence-report`, and `review` commands
accept a `--db` flag but ignore it. They always use `dirs::data_dir()` to
construct the database path (`%APPDATA%/openwand/openwand.db` on Windows).

The `run` command uses the `--db` flag correctly (or defaults to `./openwand.db`
in the CWD).

This means: if a user runs `openwand run --db my.db "hello"` and then
`openwand trace-verify SESSION --db my.db`, the trace-verify command will
look in a completely different location and find 0 entries.

**Code location:** `crates/app/src/main.rs`, `cmd_trace_verify` at line 1389:
```rust
async fn cmd_trace_verify(_cli: &Cli, session_id: &str) -> Result<()> {
    // Uses dirs::data_dir() instead of cli.db
```

**Impact:** User confusion. Anyone using a custom `--db` path cannot verify
traces without manually copying the database to the expected location.

**Required action:** Make all verification commands respect the `--db` CLI
flag. v1.0.1 patch candidate.

---

### F-VL1-3 (Non-blocking): Evidence report uses --session, review uses positional

**Severity:** Low — CLI inconsistency
**Category:** CLI friction

**Description:** `evidence-report` takes the session ID as a positional
argument, but its `--session` flag is rejected. The `review` command also
takes it positionally. The `operation-replay` command uses `--session`.
This inconsistency means users must check `--help` for each command.

**Impact:** Minor friction.

---

### F-VL1-4 (Non-blocking): No external LLM provider directly reachable

**Severity:** Low — environment constraint
**Category:** Environment-blocked

**Description:** The Z.AI API key was available through the MCP source but
not extractable for direct use by the OpenWand binary. LM Studio was not
running. The workflow used a local mock server instead of a real LLM.

**Impact:** The agent loop was exercised but with a mock response. The
inference path, tool calling, and sandbox refusal paths were not exercised
with a real LLM. This limits the operational evidence.

---

## User-Facing Friction Recorded

| # | Friction | Impact |
|---|----------|--------|
| 1 | `--db` flag ignored by verification commands | High — user cannot verify custom DBs |
| 2 | `--session` vs positional argument inconsistency | Low — requires checking help per command |
| 3 | No guidance on where the default DB is stored | Medium — trace-verify silently uses APPDATA path |
| 4 | No error when trace-verify finds 0 entries | Medium — should warn user about DB path mismatch |
| 5 | Trace hash failure gives no debug info | Medium — hard to diagnose hash mismatch cause |

---

## Workflow Outcome Classification

**Classification: PARTIAL**

The workflow completed end-to-end — agent turn, trace write, anchor write,
anchor verify, evidence report, guided review all executed. However:

- **Trace verification FAILED** on fresh data (F-VL1-1, blocking)
- **CLI `--db` flag is ignored** by verification commands (F-VL1-2)
- **Mock LLM** was used instead of real provider (F-VL1-4)

The agent loop works. The evidence chain exists. But the core integrity
claim (hash verification) has a bug that must be fixed.

---

## What This Proves

1. ✅ The agent loop runs end-to-end (message → LLM → response → trace)
2. ✅ Trace entries are written to the database
3. ✅ The guided review flow works (6 steps, all executed)
4. ✅ Evidence report generation works
5. ✅ Anchor write + verify works
6. ✅ Operation replay works
7. ❌ **Trace hash verification has a bug** — stored hashes don't match recomputed
8. ❌ Verification commands ignore `--db` flag

---

## What This Does NOT Prove

- Does not prove production readiness
- Does not prove the system works with a real LLM (mock used)
- Does not prove trace integrity (hash verification failed)
- Does not prove external review execution
- Does not resolve any v1.0 caveat

---

## Impact on v1.0 Claims

| Claim | Status After VL-1 |
|-------|-------------------|
| C-02 (BLAKE3 hash chaining) | ⚠️ **Questioned** — hashes are stored but may not be correctly computed |
| C-03 (trace integrity verification) | ⚠️ **Failed** — verification reports Fail on fresh data |
| C-15 (hash verification policy) | ⚠️ **Questioned** — BLAKE3 recomputation does not match |

**These claims are not invalidated** — the verification machinery exists and
runs. But the hash computation has a bug that must be fixed before these
claims can be considered reliable.

**No claim is upgraded. No caveat is resolved.**

---

## Recommendation

1. **F-VL1-1 is blocking.** Fix the hash computation mismatch before v1.0.1.
2. **F-VL1-2 is important.** Fix `--db` handling in all verification commands.
3. After fixes, re-run VL-1 with a real LLM provider (VL-4 dependency).
4. VL-1 remains **Partial** until F-VL1-1 is resolved.

---

## VL Blocker Status After Wave 127A

```
VL-1  First real workflow evidence        ⚠️ PARTIAL — F-VL1-1 blocking
VL-2  External review execution           ⬜ Unchanged
VL-3  v1.0.1 maintenance patch criteria   ⬜ Now urgent (F-VL1-1 needs patch)
VL-4  Provider expansion                  ⬜ Unchanged
VL-5  Linux GUI visual validation         ⬜ Unchanged
VL-6  API stability policy                ⬜ Unchanged
```
