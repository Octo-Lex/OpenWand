# v1.0.1 Patch Declaration — Wave 129B

**Declaration date:** 2026-06-15
**Wave:** 129B
**Commit:** (recorded at lock)
**Tag:** `v1.0.1`
**Wave tag:** `wave-129b-lock`
**Predecessor:** v1.0.0 (`v1.0.0`, Wave 125B)

---

## Declaration

**OpenWand v1.0.1 is declared as the first maintenance patch.**

This declaration is made after:
- VL-1 (first real workflow): found blocking defect F-VL1-1 (127A)
- VL-3 (patch criteria): resolved — criteria defined (128A)
- F-VL1-1 fixed: trace hash verifier scope serialization aligned (128A)
- F-VL1-2 fixed: verification commands honor --db flag (128A)
- F-VL1-2b fixed: anchor-write store_root canonicalization (128A)
- VL-1 re-run: PASS — all verification steps pass post-fix (128A)
- Patch preparation: artifacts built, release-check 8/8 PASS (129A)

---

## Fixed Defects

| Bug | Severity | Description |
|-----|----------|-------------|
| F-VL1-1 | **Blocking** | Trace hash verifier used `format!("{:?}")` instead of `serde_json::to_string()` for stream scope — caused all fresh traces to fail verification |
| F-VL1-2 | Medium | Six verification commands ignored `--db` CLI flag, silently used `dirs::data_dir()` |
| F-VL1-2b | Medium | `anchor-write` produced empty `store_root` for relative `--db` paths |

---

## Artifact Identity

| Artifact | Size | SHA-256 |
|----------|------|---------|
| CLI (`openwand.exe`) | 17,700,352 bytes (~16.9 MB) | `5ED051CAFF4534F372B6ABF10D1263422F3CE1357814A121DF4822848857ECF5` |
| Desktop (`openwand-ui.exe`) | 19,500,032 bytes (~18.6 MB) | `2C6AB04D42AA6EFE742643CB344456814222786D6BCB67ACA5D8A56E785C7D40` |

**Version:** `openwand 1.0.1`

---

## Metrics

| Metric | Value |
|--------|-------|
| Tests | 4,416 total, 0 failures |
| Production crates (clippy) | 11 crates, 0 warnings |
| CVEs (cargo audit) | 0 (721 dependencies) |
| Release check | 8/8 PASS |

---

## VL-1 Post-Fix Evidence

| Step | v1.0.0 (127A) | v1.0.1 (128A re-run) |
|------|---------------|----------------------|
| Trace verification | ❌ FAIL (3 mismatches) | ✅ **PASS** (0 findings) |
| Evidence report | CompleteWithCaveats | ✅ **Complete** |
| Guided review | CompleteWithCaveats | ✅ **Complete** |
| `--db` flag | ❌ Ignored | ✅ Honored |

---

## Caveats — Unchanged from v1.0.0

All 15 caveats (X-01 through X-15) remain in force. v1.0.1 fixes the
operational validity of trace verification but does NOT resolve the
underlying caveats on those claims (physical immutability, fully
consistent tamper passes, etc.).

---

## What v1.0.1 IS NOT

- Not a feature release
- Not an API change
- Not a new claim
- Not a caveat resolution (beyond restoring broken verification to working state)
- Not production-ready
- Not formally certified
- Not externally reviewed

---

## Release Lineage

```
v0.1.0-alpha → ... → v1.0.0-rc.1 → v1.0.0 → v1.0.1
```
