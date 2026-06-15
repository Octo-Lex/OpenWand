# v1.0.0 Final Declaration — Wave 125B

**Declaration date:** 2026-06-15
**Wave:** 125B
**Commit:** (recorded at lock)
**Tag:** `v1.0.0`
**Wave tag:** `wave-125b-lock`
**Blocker:** VK-5 — v1.0.0 final declaration

---

## Declaration

**OpenWand v1.0.0 is declared as the first stable release.**

This declaration is made after:
- VK-1 (external review): consciously deferred with rationale (123A)
- VK-2 (rc.1 soak): resolved — zero blocking regressions (122A)
- VK-3 (final claim re-audit): resolved — 21 claims, 15 caveats, zero overclaiming (124A)
- VK-4 (final preparation): resolved — artifacts, notes, identity ready (125A)
- VK-5 (final declaration): this wave (125B)

---

## Final Artifact Identity

| Artifact | Size | SHA-256 |
|----------|------|---------|
| CLI (`openwand.exe`) | 17,705,472 bytes (~16.9 MB) | `AE2DBB1B5D37D4F1833998A5047256CB47BB1D9F0C3CACB493D19C148BC7EA46` |
| Desktop (`openwand-ui.exe`) | 19,501,056 bytes (~18.6 MB) | `04B696B533602C196808213A2B70DB6FEAD4A61C2A9F64B598208C8A4CFF4DF4` |

**Version:** `openwand 1.0.0`

---

## Final Metrics

| Metric | Value |
|--------|-------|
| Tests | 4,325 total, 0 failures |
| Production crates (clippy) | 11 crates, 0 warnings |
| CVEs (cargo audit) | 0 (721 dependencies) |
| Release check | 8/8 PASS |
| CLI commands | 40+ |
| Authority surfaces | 12 (4 write, 8 read-only) |
| Rust toolchain | rustc 1.95.0 |

---

## VK Blocker Closure

| Blocker | Status | Wave |
|---------|--------|------|
| VK-1: External review execution | ✅ Consciously deferred | 123A |
| VK-2: rc.1 soak / regression | ✅ Resolved — zero blocking | 122A |
| VK-3: Final claim re-audit | ✅ Resolved — zero overclaiming | 124A |
| VK-4: Final v1.0 preparation | ✅ Resolved — package ready | 125A |
| VK-5: v1.0.0 final declaration | ✅ This declaration | 125B |

**All VK blockers resolved.**

---

## Architecture Arc (Complete)

```
v0.2  governed execution substrate          Control
v0.3  live observation                      Observe
v0.4  desktop operation requests            Operate
v0.5  read-only verification                Verify
v0.6  evidence-backed assurance hardening   Harden
v0.7  external assurance                    Externally Anchor
v0.8  operational hardening                 Operationally Harden
v0.9  external validation                   Externally Validate
v1.0  release-candidate closure             Close (rc.1)
v1.0  stable release                        Close (stable)
```

---

## 21 Evidence-Backed Claims

| # | Claim | Status |
|---|-------|--------|
| C-01 | Governed execution substrate with deterministic trust gate | ✅ |
| C-02 | Append-only trace store with BLAKE3 hash chaining | ✅ |
| C-03 | Read-only trace integrity verification | ✅ |
| C-04 | Read-only operation correspondence verification | ✅ |
| C-05 | Externally persisted checkpoint anchors | ✅ |
| C-06 | Reviewer-facing evidence report export | ✅ |
| C-07 | Automated security scan evidence | ✅ |
| C-08 | Structured authority boundary review (12 surfaces) | ✅ |
| C-09 | Linux desktop binary compiles on Windows and Linux | ✅ |
| C-10 | Partial Linux runtime validation | ✅ |
| C-11 | External review packet available | ✅ |
| C-12 | Repeatable release checklist | ✅ |
| C-13 | Guided evidence flow | ✅ |
| C-14 | Release readiness automation (8 checks) | ✅ |
| C-15 | Hash verification policy with BLAKE3 | ✅ |
| C-16 | Zero CVEs in dependencies | ✅ |
| C-17 | Zero `unsafe` in production code (one accepted: libc::dup) | ✅ |
| C-18 | Desktop UI authority boundary enforced | ✅ |
| C-19 | rc.1 soak executed, zero blocking regressions | ✅ |
| C-20 | External review classification conscious and documented | ✅ |
| C-21 | CLI version string correctly reports 1.0.0 | ✅ |

---

## 15 Explicit Caveats

| # | Caveat |
|---|--------|
| X-01 | Not production-ready |
| X-02 | Not formal security certification |
| X-03 | Not physical immutability |
| X-04 | Not remote attestation |
| X-05 | Not full cross-platform runtime validation |
| X-06 | Not full Linux GUI support |
| X-07 | Not provider completeness |
| X-08 | Not stable API guarantee |
| X-09 | Not externally reviewed (consciously deferred) |
| X-10 | Not macOS validated |
| X-11 | Fully consistent tamper passes |
| X-12 | Windows final-component TOCTOU residual |
| X-13 | openwand-content is a stub crate |
| X-14 | 15 transitive dependency warnings |
| X-15 | External review consciously deferred from v1.0.0 |

---

## Release Lineage

```
v0.1.0-alpha → v0.1.0-beta → v0.2.0 → v0.3.0 → v0.4.0 → v0.5.0
→ v0.6.0 → v0.7.0 → v0.8.0 → v0.9.0 → v1.0.0-rc.1 → v1.0.0
```

**Total waves:** 125A/B + all prior arc waves
**Total tests:** 4,325

---

## What v1.0.0 IS

- A desktop AI agent with governed execution and evidence-backed verification
- A complete architecture arc from control to stable release
- A stable release with explicitly documented claims and caveats
- A foundation for external review, provider expansion, and platform coverage
- A system where every claim has evidence and every gap is documented

## What v1.0.0 IS NOT

- Not production-ready
- Not formally certified or externally audited
- Not externally reviewed
- Not provider-complete
- Not API-frozen
- Not physically immutable
- Not a substitute for formal security validation

---

## Post-v1.0.0 Roadmap

| Item | Priority | Notes |
|------|----------|-------|
| External review execution | P1 | Packet ready; needs external reviewer |
| Direct provider validation (OpenAI/Anthropic/Ollama) | P2 | Architecture supports it |
| Linux GUI full visual validation | P2 | Needs physical Linux + GPU |
| macOS runtime validation | P3 | Needs macOS environment |
| API stabilization | P3 | May begin post-v1.0 |
| Production hardening | P3 | Requires operational experience |

---

## Declaration Authority

This declaration is made by the project's governed release process. It does
not:
- Claim production readiness
- Claim formal certification
- Claim external review execution
- Upgrade any caveat into assurance
- Add execution authority, policy bypass, or prompt change

v1.0.0 is declared stable with all caveats explicitly in force.
