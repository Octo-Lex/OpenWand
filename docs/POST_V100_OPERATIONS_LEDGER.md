# Post-v1.0 Operations Ledger — Wave 134A

**Date:** 2026-06-15
**Wave:** 134A
**Commit:** (recorded at lock)
**Tag:** `wave-134a-lock`

---

## Purpose

A standing operations ledger for OpenWand post-release status. Tracks
released versions, shipped fixes, open caveats, deferred validations,
external-resource blockers, and criteria for opening future work.

This is a living document. It is updated at each wave boundary.

---

## 1. Released Versions

| Version | Tag | Date | Type | Commit |
|---------|-----|------|------|--------|
| v1.0.0 | `v1.0.0` | 2026-06-14 | Stable release | (Wave 125B) |
| v1.0.1 | `v1.0.1` | 2026-06-15 | Maintenance patch | `0a5b443` |

### Current Stable

**v1.0.1** — `openwand 1.0.1`

### Operating Mode

**Internal Product Operating Mode** (135A, 2026-06-16)

The product is finished for internal use. Active construction is paused.
See `docs/INTERNAL_OPERATIONS.md` for operating rules, supported workflows,
evidence capture expectations, bug classification, and patch triggers.
External review (VL-2) and Linux GUI validation (VL-5) remain deferred.

### Artifact Identity (v1.0.1)

| Artifact | Size | SHA-256 |
|----------|------|---------|
| CLI (`openwand.exe`) | 17,700,352 bytes | `5ED051CAFF4534F372B6ABF10D1263422F3CE1357814A121DF4822848857ECF5` |
| Desktop (`openwand-ui.exe`) | 19,500,032 bytes | `2C6AB04D42AA6EFE742643CB344456814222786D6BCB67ACA5D8A56E785C7D40` |

---

## 2. Shipped Fixes

| Bug | Version Fixed | Severity | Root Cause |
|-----|--------------|----------|------------|
| F-VL1-1 | v1.0.1 (128A) | **Blocking** | Trace hash verifier used `format!("{:?}")` instead of `serde_json::to_string()` for scope serialization — caused all fresh traces to fail verification |
| F-VL1-2 | v1.0.1 (128A) | Medium | Six verification commands ignored `--db` CLI flag, silently used `dirs::data_dir()` |
| F-VL1-2b | v1.0.1 (128A) | Medium | `anchor-write` produced empty `store_root` for relative `--db` paths |

---

## 3. Open Caveats (15 total)

| ID | Description | Since | Status | Last Updated |
|----|-------------|-------|--------|--------------|
| X-01 | Not production-ready | v0.1.0-alpha | Disclaimed | — |
| X-02 | Not formal security certification | v0.5.0 | Disclaimed | — |
| X-03 | Not physical immutability | v0.5.0 | Disclaimed | — |
| X-04 | Not remote attestation | v0.7.0 | Disclaimed | — |
| X-05 | Not full cross-platform runtime validation | v0.4.0 | Disclaimed | May narrow with VL-5 |
| X-06 | Not full Linux GUI support | v0.8.0 | Disclaimed | May resolve with VL-5 |
| X-07 | Not provider completeness | v0.2.0 | **Partially narrowed** (132A) | Z.AI + LM Studio validated |
| X-08 | Not stable API guarantee | v0.5.0 | **Partially narrowed** (131A) | 8 CLI commands + 4 schemas Stable |
| X-09 | Not externally reviewed | v0.9.0 | **Deferred** | Updated context (130A) |
| X-10 | Not macOS validated | v0.4.0 | Disclaimed | — |
| X-11 | Fully consistent tamper passes | v0.5.0 | Disclaimed | — |
| X-12 | Windows final-component TOCTOU residual | v0.8.0 | Disclaimed | — |
| X-13 | openwand-content is a stub crate | v0.2.0 | Disclaimed | — |
| X-14 | 15 transitive dependency warnings | v0.2.0+ | Disclaimed | — |
| X-15 | External review consciously deferred | 123A | **Deferred** | Updated context (130A) |

### Caveat Summary

| Category | Count | Caveats |
|----------|-------|---------|
| Disclaimed (permanent) | 8 | X-01, X-02, X-03, X-04, X-10, X-11, X-12, X-13, X-14 |
| Partially narrowed | 2 | X-07 (132A), X-08 (131A) |
| Deferred (external resource) | 2 | X-09 (130A), X-15 (130A) |
| May narrow with VL-5 | 2 | X-05, X-06 |
| Resolved | 0 | — |

**Total: 15 caveats. 0 resolved. 2 partially narrowed. 2 deferred.**

---

## 4. Claims (21 total)

| ID | Claim | Status |
|----|-------|--------|
| C-01 | Governed execution substrate with deterministic trust gate | ✅ Valid |
| C-02 | Append-only trace store with BLAKE3 hash chaining | ✅ Valid (operational post-v1.0.1) |
| C-03 | Read-only trace integrity verification | ✅ Valid (operational post-v1.0.1) |
| C-04 | Read-only operation correspondence verification | ✅ Valid |
| C-05 | Externally persisted checkpoint anchors | ✅ Valid |
| C-06 | Reviewer-facing evidence report export | ✅ Valid |
| C-07 | Automated security scan evidence | ✅ Valid |
| C-08 | Structured authority boundary review (12 surfaces) | ✅ Valid |
| C-09 | Linux desktop binary compiles on Windows and Linux | ✅ Valid |
| C-10 | Partial Linux runtime validation | ✅ Valid |
| C-11 | External review packet available | ✅ Valid |
| C-12 | Repeatable release checklist | ✅ Valid |
| C-13 | Guided evidence flow | ✅ Valid |
| C-14 | Release readiness automation (8 checks) | ✅ Valid |
| C-15 | Hash verification policy with BLAKE3 | ✅ Valid (operational post-v1.0.1) |
| C-16 | Zero CVEs in dependencies | ✅ Valid |
| C-17 | Zero `unsafe` in production code (one accepted: libc::dup) | ✅ Valid |
| C-18 | Desktop UI authority boundary enforced | ✅ Valid |
| C-19 | rc.1 soak/regression window executed | ✅ Valid |
| C-20 | External review classification is conscious and documented | ✅ Valid |
| C-21 | CLI version string correctly reports version | ✅ Valid |

**All 21 claims valid.** C-02, C-03, C-15 are operational post-v1.0.1
(the F-VL1-1 fix restored their operational validity).

---

## 5. VL Blocker Status

| Blocker | Description | Status | Wave | Next Action |
|---------|-------------|--------|------|-------------|
| VL-1 | First real workflow evidence | ✅ PASS | 127A/128A | Complete |
| VL-2 | External review execution | ⬜ Deferred — packet ready | 130A/133A | Engage external reviewer |
| VL-3 | Maintenance patch criteria | ✅ RESOLVED | 128A | Complete |
| VL-4 | Provider expansion | ✅ PARTIALLY RESOLVED | 132A | Validate additional provider when API key available |
| VL-5 | Linux GUI visual validation | ⬜ Deferred — packet ready | 133A | Test on Linux with physical GPU |
| VL-6 | API stability policy | ✅ RESOLVED | 131A | Complete |

### Resolved: 4 of 6
### Deferred (packet-ready): 2 of 6

---

## 6. Deferred Validation Packets

| Packet | Document | Resource Needed | Affected Caveats |
|--------|----------|-----------------|------------------|
| External review | `docs/VL_RESOURCE_BLOCKED_PACKETS.md` §1 | Independent human reviewer | X-09, X-15 |
| Linux GUI visual | `docs/VL_RESOURCE_BLOCKED_PACKETS.md` §2 | Linux with physical GPU | X-05, X-06 |

Both packets include exact commands, expected outputs, classification
criteria, and evidence requirements. They are ready to execute.

---

## 7. Test Baseline

| Metric | Value |
|--------|-------|
| Total tests | 4,509 |
| Failures | 0 |
| Production crates | 11 |
| CVEs | 0 (721 dependencies) |
| Production clippy | 0 warnings |

---

## 8. Release Lineage

```
v0.1.0-alpha → v0.1.0-beta → v0.2.0-beta → v0.2.0-rc.1 → v0.2.0
→ v0.3.0 → v0.4.0 → v0.5.0 → v0.6.0 → v0.7.0 → v0.8.0 → v0.9.0
→ v1.0.0-rc.1 → v1.0.0 → v1.0.1 (current)
```

---

## 9. Criteria for Opening Future Work

### v1.0.2 (Patch)

A v1.0.2 patch is opened when:
- A blocking defect is found in v1.0.1
- A security-relevant issue is identified
- A CLI correctness bug is confirmed

**Process:** Fix → regression test → re-run workflow → tag → publish.
Patch scope: bug fixes only. No features, no API changes, no new claims.

### v1.1.0 (Minor)

A v1.1.0 minor release is opened when:
- VL-5 is resolved (Linux GUI visual validation passes)
- A new provider is operationally validated
- VL-2 is resolved (external review passes)
- Feature additions are ready (subject to API stability policy)

**Process:** Minor release may add features to Experimental surfaces,
add new Stable commands, and narrow caveats with evidence.
Deprecated surfaces get one minor cycle notice.

### External Review Execution (VL-2)

Open when:
- An independent reviewer is available
- The reviewer is not the builder or build agent
- The reviewer commits to executing the packet

**Resolution:** Reviewer runs `docs/VL_RESOURCE_BLOCKED_PACKETS.md` §1.
If Pass: X-09 and X-15 resolved. If Fail: defects filed.

### Linux GUI Visual Validation (VL-5)

Open when:
- A Linux environment with physical GPU is available
- OR a cloud VM with GPU passthrough is available
- A tester commits to executing the packet

**Resolution:** Tester runs `docs/VL_RESOURCE_BLOCKED_PACKETS.md` §2.
If Pass: X-05 narrowed, X-06 resolved. If Fail: defects filed.

### Provider Expansion (VL-4 full resolution)

Open when:
- An API key for OpenAI, Anthropic, or OpenRouter is available
- OR a local Ollama instance is installed

**Resolution:** Validate one additional provider through OpenWand's
adapter. Record request/response evidence. If Pass: X-07 further narrowed.

---

## 10. What This Ledger Does NOT Claim

- Not external review execution
- Not full Linux GUI validation
- Not provider completeness
- Not production readiness
- Not formal certification
- Not global API stability
- Not physical immutability
- Not remote attestation
