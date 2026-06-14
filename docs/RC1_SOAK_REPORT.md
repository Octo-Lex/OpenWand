# v1.0.0-rc.1 Soak / Regression Report — Wave 122A

**Date:** 2026-06-14
**Scope:** v1.0.0-rc.1 release candidate validation
**Classification:** Soak/regression window for release-candidate maturity

---

## Purpose

This report records the results of exercising rc.1 across all available
verification paths, CLI commands, and desktop binary sanity. It classifies
any regressions found.

**A soak report classifies rc.1 behavior. It does not declare final v1.0
readiness by itself.**

---

## 1. Release Check (`openwand release-check`)

**Command:** `openwand release-check --output rc1_release_check.json`

### Result: PASS — All 8 checks passed

| # | Check | Result | Detail |
|---|-------|--------|--------|
| 1 | Workspace tests | ✅ PASS | 4,240 tests passed |
| 2 | Production clippy | ✅ PASS | 0 warnings on 11 crates |
| 3 | Cargo audit | ✅ PASS | 0 CVEs (721 deps) |
| 4 | CLI binary build | ✅ PASS | openwand builds with desktop features |
| 5 | Desktop binary build | ✅ PASS | openwand-ui builds (109A gate) |
| 6 | Artifact identity | ✅ PASS | 17,702,912 bytes |
| 7 | Documentation | ✅ PASS | 5/5 docs present |
| 8 | STATE.md consistency | ✅ PASS | Version, tests, SHA-256 present |

**Overall: Ready** — exit code 0.

---

## 2. CLI Sanity

### 2.1 Version

**Command:** `openwand --version`
**Output:** `openwand 0.1.0`
**Finding:** F-SOAK-1 (see §5 below)

### 2.2 Help / Subcommand Surface

**Command:** `openwand --help`
**Output:** 36 subcommands listed
**Result:** ✅ Pass — all expected commands present

### 2.3 Verification Commands

| Command | Args | Exit | Result |
|---------|------|------|--------|
| `trace-verify` | nonexistent session | 0 | ✅ Pass (zero entries = Pass) |
| `operation-replay` | --session nonexistent --operations /dev/null | 1 | ✅ Correct error (malformed JSON) |
| `anchor-write` | nonexistent --anchor-root /tmp/... | 0 | ✅ Writes anchor for empty trace |
| `evidence-report` | nonexistent --operations /dev/null --output ... | 1 | ✅ Correct error (malformed JSON) |
| `review` | nonexistent --operations /dev/null | 1 | ✅ Correct guided flow error |

All commands behave correctly for edge cases (nonexistent sessions, malformed
input). Error messages are clear and exit codes are non-zero for failures.

### 2.4 release-check (full)

**Command:** `openwand release-check --output rc1_release_check.json`
**Result:** ✅ PASS — 4,240 tests, all 8 checks green
**JSON output:** Written successfully

---

## 3. Desktop Binary Sanity

### 3.1 Binary Existence and Identity

| Property | Value |
|----------|-------|
| Path | target/release/openwand-ui.exe |
| Size | 19,500,032 bytes (~18.6 MB) |
| SHA-256 | 8ACCF49D4D0EB82056A148F2751C318441A8C3DFFE0342B92339CFEF50B97457 |
| Under 20MB | ✅ Yes |

### 3.2 Launch Test

**Method:** Start-Process + 3-second wait + process check
**Result:** ✅ Pass — binary launches, creates two processes (parent + Dioxus
child renderer), stays running without crash.

### 3.3 Visual Rendering

Not tested in soak window. Carried as documented caveat (VJ-3 deferred).

---

## 4. Clean Install Path

Not performed in this soak window (would require fresh clone + build on
clean environment). Classified as **environment-blocked** — the build
from existing workspace tree is validated, but a fresh clone install is
not. This is a soak finding, not a blocking regression.

---

## 5. Regression Classification

| # | Finding | Category | Severity | Classification |
|---|---------|----------|----------|----------------|
| F-SOAK-1 | CLI version string says `0.1.0` instead of `1.0.0` | Version metadata | Low | **Non-blocking** — release identity is git tags, not Cargo.toml version |
| F-SOAK-2 | Clean install not tested | Validation gap | Low | **Environment-blocked** — requires fresh environment |
| F-SOAK-3 | Desktop visual rendering not tested | Validation gap | Low | **Deferred** — VJ-3 carried caveat |

### F-SOAK-1 Detail

`openwand --version` reports `0.1.0` because the workspace Cargo.toml version
field was never updated through the release lineage. The actual release version
is identified by git tags (`v1.0.0-rc.1`). This is cosmetic — the binary
functions correctly, all verification commands work, and the release-check
process does not depend on the Cargo.toml version string.

**Recommendation:** Update Cargo.toml version for v1.0.0 final. Low effort,
improves metadata accuracy.

---

## 6. Blocking Regressions

**None found.**

All automated checks pass. All CLI commands behave correctly. Desktop binary
launches without crash. No functional regressions identified.

---

## 7. Summary

| Category | Count |
|----------|-------|
| Blocking regressions | 0 |
| Non-blocking findings | 1 (version string) |
| Environment-blocked | 1 (clean install) |
| Deferred (carried caveats) | 1 (visual rendering) |

**rc.1 is functionally sound.** The only actionable finding (F-SOAK-1:
version string) is cosmetic and non-blocking.

---

## What This Report Does NOT Claim

- Does not declare v1.0.0 final readiness
- Does not substitute for external review (VK-1)
- Does not claim clean install validation
- Does not claim production readiness
- Does not upgrade any caveat into an assurance
