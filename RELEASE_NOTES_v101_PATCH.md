# OpenWand v1.0.1 — Maintenance Patch

**Date:** 2026-06-15
**Tag:** `v1.0.1` (pending — prepared by Wave 129A)
**Predecessor:** v1.0.0 (`v1.0.0`, Wave 125B)
**Classification:** Maintenance patch — bug fixes only

---

## Overview

v1.0.1 is the first maintenance patch for OpenWand v1.0.0. It fixes two
defects discovered during the first real workflow evidence test (Wave 127A,
VL-1). No new features, no API changes, no new claims.

---

## Fixes

### F-VL1-1 (Blocking): Trace hash verification fails on fresh data

**Symptom:** `openwand trace-verify` reports Fail on freshly generated
trace entries, even when no tampering occurred.

**Root cause:** The trace verifier used `format!("{:?}", scope)` to
serialize the stream scope (producing `Session`), while the trace writer
used `serde_json::to_string(&scope)` (producing `"Session"` — with JSON
quotes). These different byte inputs produced different BLAKE3 hashes.

**Fix:** Verifier now uses `serde_json::to_string(&entry.stream_id.scope)`
matching the writer's canonical serialization.

**Files changed:**
- `crates/trace/src/verifier.rs` — `verify_with_hash_policy()` scope serialization

**Impact:** Restores trace verification to working state. Claims C-02,
C-03, C-15 are now operationally valid for freshly generated data.

### F-VL1-2: Verification commands ignore --db flag

**Symptom:** `openwand trace-verify --db custom.db` ignores the `--db`
flag and reads from `%APPDATA%/openwand/openwand.db` instead.

**Root cause:** Six verification commands used `dirs::data_dir()` instead
of the `cli.db` parameter.

**Fix:** Added `resolve_db_path()` helper. All verification commands
(trace-verify, operation-replay, anchor-write, anchor-verify,
evidence-report, review) now honor the `--db` flag.

**Files changed:**
- `crates/app/src/main.rs` — 6 command functions updated

### F-VL1-2b: anchor-write empty store_root on relative --db

**Symptom:** `openwand anchor-write` fails with "failed to canonicalize
path" when using a relative `--db` path.

**Root cause:** `db_path.parent()` returns empty string for relative paths.

**Fix:** Canonicalize `db_path` before computing parent directory.

---

## Metrics

| Metric | v1.0.0 | v1.0.1 |
|--------|--------|--------|
| Tests | 4,352 | 4,404 (+52) |
| CLI binary | 17,705,472 bytes | 17,700,352 bytes |
| Desktop binary | 19,501,056 bytes | 19,500,032 bytes |
| CVEs | 0 | 0 |
| Production clippy | 0 warnings | 0 warnings |
| Release check | 8/8 PASS | 8/8 PASS |

### Artifact Identity

| Artifact | Size | SHA-256 |
|----------|------|---------|
| CLI (`openwand.exe`) | 17,700,352 bytes (~16.9 MB) | `5ED051CAFF4534F372B6ABF10D1263422F3CE1357814A121DF4822848857ECF5` |
| Desktop (`openwand-ui.exe`) | 19,500,032 bytes (~18.6 MB) | `2C6AB04D42AA6EFE742643CB344456814222786D6BCB67ACA5D8A56E785C7D40` |
| `openwand --version` | — | `openwand 1.0.1` |

---

## VL-1 Post-Fix Workflow Evidence

| Step | v1.0.0 (127A) | v1.0.1 (128A re-run) |
|------|---------------|----------------------|
| Agent turn | ✅ Pass | ✅ Pass |
| Trace verification | ❌ FAIL (3 mismatches) | ✅ **PASS** (0 findings) |
| Operation replay | ✅ Pass | ✅ Pass |
| Anchor write | ✅ Pass (APPDATA) | ✅ Pass (with --db) |
| Anchor verify | ✅ Pass | ✅ Pass |
| Evidence report | CompleteWithCaveats | ✅ **Complete** |
| Guided review | CompleteWithCaveats | ✅ **Complete** |
| `--db` flag | ❌ Ignored | ✅ Honored |

---

## What v1.0.1 Does NOT Include

- No new features
- No API changes
- No new claims
- No caveat removal
- No policy changes
- No prompt changes

---

## Caveats — Unchanged from v1.0.0

All 15 caveats (X-01 through X-15) from v1.0.0 remain in force.

v1.0.1 does NOT resolve any caveat. The trace verification fix restores
the *operational validity* of claims C-02, C-03, C-15 — but the caveats
on those claims (physical immutability, fully consistent tamper, etc.)
remain unchanged.

---

## Release Lineage

```
v0.1.0-alpha → ... → v1.0.0-rc.1 → v1.0.0 → v1.0.1
```
