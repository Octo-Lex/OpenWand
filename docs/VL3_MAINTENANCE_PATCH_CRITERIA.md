# v1.0.1 Maintenance Patch Criteria — Wave 128A (VL-3)

**Date:** 2026-06-15
**Wave:** 128A
**Blocker:** VL-3 — v1.0.1 maintenance patch criteria

---

## Purpose

Defines criteria for v1.0.x patch releases. Establishes what qualifies as a
patch, how it is released, and what constraints apply.

---

## Versioning Policy

| Version Type | Format | Scope |
|-------------|--------|-------|
| Patch | v1.0.x | Bug fixes only — no new features, no API changes |
| Minor | v1.x.0 | New features, backward-compatible API additions |
| Major | vx.0.0 | Breaking API changes, architecture changes |

v1.0.1 is a **patch release** — it fixes bugs found in v1.0.0 without
adding features or changing APIs.

---

## Patch Qualification

A bug qualifies for a patch release if it meets **any** of:

1. **Blocking defect** — breaks a v1.0.0 claim or core functionality
2. **Security-relevant** — affects sandbox, authority, or data integrity
3. **CLI correctness** — command produces wrong output, wrong exit code,
   or silently ignores user input

A bug does NOT qualify if it is:
- Cosmetic (UI styling, log message wording)
- Feature request (new provider, new platform)
- Performance optimization
- Documentation improvement

---

## v1.0.1 Patch Scope (Wave 128A)

| Bug | Severity | Status |
|-----|----------|--------|
| F-VL1-1: Trace hash verification fails on fresh data | Blocking | ✅ Fixed |
| F-VL1-2: Verification commands ignore --db flag | Medium | ✅ Fixed |
| F-VL1-2b: anchor-write store_root empty on relative --db | Medium | ✅ Fixed |

### What v1.0.1 Does NOT Include

- No new features
- No API changes
- No new claims
- No caveat removal
- No policy changes
- No prompt changes
- No runtime behavior changes (except the bug fixes)

---

## Patch Release Process

1. Fix identified bugs on master
2. Add regression tests that reproduce the original bug
3. Re-run full test suite
4. Re-run the workflow that found the bug (VL-1 evidence path)
5. Confirm fixes resolve the findings
6. Tag as v1.0.x
7. Publish

---

## VL-1 Re-run Results (Post-Fix)

### Original Finding vs Post-Fix

| Step | v1.0.0 (Wave 127A) | Post-Fix (Wave 128A) |
|------|--------------------|-----------------------|
| Agent turn | ✅ Pass | ✅ Pass |
| Trace verification | ❌ **FAIL** (3 hash mismatches) | ✅ **PASS** (0 findings) |
| Operation replay | ✅ Pass | ✅ Pass |
| Anchor write | ✅ Pass (APPDATA only) | ✅ Pass (with --db) |
| Anchor verify | ✅ Pass | ✅ Pass |
| Evidence report | CompleteWithCaveats | **Complete** |
| Guided review | CompleteWithCaveats | **Complete** |
| --db flag | ❌ Ignored | ✅ Honored |

### Session: `01KV4DB55123Q146WASN6KFXQ8`
### Result: Trace verification PASS — 3 entries, 1 stream, 0 findings

---

## Constraints

A v1.0.x patch:
- MUST NOT add features
- MUST NOT change APIs
- MUST NOT add claims
- MUST NOT remove caveats without evidence
- MUST NOT claim production readiness
- MUST NOT add execution authority or policy bypass
- MUST include regression tests for all fixed bugs
