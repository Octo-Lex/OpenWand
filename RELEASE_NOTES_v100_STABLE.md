# OpenWand v1.0.0 — Stable Release

**Date:** 2026-06-15
**Tag:** `v1.0.0` (pending — prepared by Wave 125A, declared by Wave 125B)
**Theme:** Architecture arc complete — stable release with explicit caveats
**Classification:** Stable release — NOT production-ready, NOT formally certified

---

## Overview

v1.0.0 is the first stable release of OpenWand. It closes the full architecture
arc (Control → Close) and delivers a desktop AI agent with governed execution,
live observation, read-only verification, externally anchored assurance,
operational hardening, and evidence-backed reporting.

This is NOT a declaration that OpenWand is production-ready. It is a declaration
that the project has reached stable maturity for its scoped claims, with all
caveats explicitly documented.

---

## What "v1.0.0 Stable" Means

v1.0.0 stable means:

- The architecture arc is complete (12 milestones: v0.2 through v1.0)
- rc.1 soak findings are classified — zero blocking regressions (VK-2)
- External review is classified — consciously deferred (VK-1)
- Final claim re-audit is complete — zero overclaiming (VK-3)
- All verification machinery exists, is tested, and is documented
- All 21 claims are evidence-backed
- All 15 caveats are explicitly documented

It does NOT mean:
- Production-ready (requires formal external security audit)
- Externally reviewed (review packet exists, no external reviewer ran it)
- Feature-complete (provider matrix, platform coverage may be limited)
- API-frozen (API stabilization may begin but is not guaranteed)
- Physically immutable (trace stores are technically mutable files)
- Formally certified (no certification body has validated this software)

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

## VK Blocker Reconciliation

| Blocker | Decision | Wave |
|---------|----------|------|
| VK-1: External review execution | Consciously deferred — packet ready, no reviewer | 123A |
| VK-2: rc.1 soak / regression | Resolved — zero blocking regressions | 122A |
| VK-3: Final claim re-audit | Resolved — 21 claims, 15 caveats, zero overclaiming | 124A |
| VK-4: Final v1.0 preparation | This artifact | 125A |
| VK-5: v1.0.0 final declaration | Pending (Wave 125B) | — |

---

## Metrics

| Metric | Value |
|--------|-------|
| Tests | 4,302 total, 0 failures |
| Production crates (clippy -D warnings) | 11 crates, 0 warnings |
| CVEs (cargo audit) | 0 (721 dependencies) |
| Unmaintained/unsound warnings | 15 (all upstream-blocked, transitive) |
| CLI binary | 17,705,472 bytes (~16.9 MB) |
| CLI SHA-256 | `AE2DBB1B5D37D4F1833998A5047256CB47BB1D9F0C3CACB493D19C148BC7EA46` |
| Desktop binary | 19,501,056 bytes (~18.6 MB) |
| Desktop SHA-256 | `04B696B533602C196808213A2B70DB6FEAD4A61C2A9F64B598208C8A4CFF4DF4` |
| CLI commands | 40+ including review, release-check, trace-verify, operation-replay |
| Authority surfaces | 12 (4 write-capable, 8 read-only/delegated) |
| `openwand --version` | `openwand 1.0.0` |
| Release check | 8/8 PASS |

---

## Release Check Results (Wave 125A)

| # | Check | Result |
|---|-------|--------|
| 1 | Workspace tests | ✅ PASS — 4,302 tests |
| 2 | Production clippy | ✅ PASS — 0 warnings on 11 crates |
| 3 | Cargo audit | ✅ PASS — 0 CVEs (721 deps) |
| 4 | CLI binary build | ✅ PASS — openwand builds with desktop features |
| 5 | Desktop binary build | ✅ PASS — openwand-ui builds (109A gate) |
| 6 | Artifact identity | ✅ PASS — 16.9 MB |
| 7 | Documentation presence | ✅ PASS — 5/5 docs present |
| 8 | STATE.md consistency | ✅ PASS — Version, tests, SHA-256 present |

**Overall: Ready** — exit code 0.

---

## Verification Capabilities

| Capability | Command |
|------------|---------|
| Trace integrity verification | `openwand trace-verify <session-id>` |
| Operation correspondence | `openwand operation-replay --session <id> --operations <ops.json>` |
| Checkpoint anchor write | `openwand anchor-write <session-id> --anchor-root <dir>` |
| Checkpoint anchor verify | `openwand anchor-verify <session-id> --anchor <file>` |
| Evidence report export | `openwand evidence-report --session <id> --operations <ops.json> [--anchor <file>] --output <file>` |
| Guided review flow | `openwand review <session-id> --operations <ops.json> [--anchor <file>]` |
| Release readiness check | `openwand release-check` |

---

## Release Lineage

```
v0.1.0-alpha → v0.1.0-beta → v0.2.0 → v0.3.0 → v0.4.0 → v0.5.0 → v0.6.0 → v0.7.0 → v0.8.0 → v0.9.0 → v1.0.0-rc.1 → v1.0.0
```

---

## Claims (21 Evidence-Backed)

| # | Claim | Evidence |
|---|-------|----------|
| C-01 | Governed execution substrate with deterministic trust gate | Policy crate, sandbox, approval lifecycle |
| C-02 | Append-only trace store with BLAKE3 hash chaining | TraceEntry prev_hash/entry_hash |
| C-03 | Read-only trace integrity verification | TraceVerifier + `openwand trace-verify` |
| C-04 | Read-only operation correspondence verification | OperationReplayVerifier + `openwand operation-replay` |
| C-05 | Externally persisted checkpoint anchors | CheckpointWriter + `openwand anchor-write/verify` |
| C-06 | Reviewer-facing evidence report export | EvidenceReport JSON + `openwand evidence-report` |
| C-07 | Automated security scan evidence | cargo audit (0 CVEs, 721 deps) + clippy + authority guards |
| C-08 | Structured authority boundary review | AUTHORITY_REVIEW.md, 12 surfaces documented |
| C-09 | Linux desktop binary compiles on Windows and Linux | openwand-ui builds on both platforms |
| C-10 | Partial Linux runtime validation | Proxmox VM smoke test |
| C-11 | External review packet available | EXTERNAL_REVIEW_PACKET.md |
| C-12 | Repeatable release checklist | RELEASE_CHECKLIST.md + `openwand release-check` |
| C-13 | Guided evidence flow | `openwand review` |
| C-14 | Release readiness automation | `openwand release-check` (8 checks) |
| C-15 | Hash verification policy with BLAKE3 | HashVerificationPolicy + Blake3HashPolicy |
| C-16 | Zero CVEs in dependencies | cargo audit (721 deps, 0 vulns) |
| C-17 | Zero `unsafe` in production code | Source inspection (one accepted: libc::dup) |
| C-18 | Desktop UI authority boundary enforced | 32+ guard tests |
| C-19 | rc.1 soak executed, zero blocking regressions | RC1_SOAK_REPORT.md |
| C-20 | External review classification is conscious | VK1_EXTERNAL_REVIEW_CLASSIFICATION.md |
| C-21 | CLI version string correctly reports version | F-SOAK-1 fixed in 122A |

---

## Caveats — What v1.0.0 Does NOT Claim

1. **Not production-ready** — this is a stable release, not a production system
2. **Not formal security certification** — no external audit has been performed
3. **Not externally reviewed** — review packet exists but no external reviewer
   has executed it (VK-1 consciously deferred)
4. **Not provider complete** — only LM Studio and Z.AI validated (VK-1/VJ-2
   deferred)
5. **Not full Linux GUI support** — partial: compile + launch validated, visual
   rendering not captured (VJ-3 deferred)
6. **Not full cross-platform runtime validation** — macOS not validated
7. **Not physical immutability** — trace stores are technically mutable files
8. **Not remote attestation** — no hardware or network attestation
9. **Not stable API guarantee** — APIs may change between versions
10. **Not full cryptographic non-repudiation** — fully consistent tamper
    (rewrite store + anchor) passes
11. **Not API-frozen** — stabilization may begin but is not guaranteed
12. **Not zero unsafe** — one accepted `unsafe` block (libc::dup for Unix openat)
13. **Not clean-install validated** — soak exercised existing workspace, not
    fresh clone (F-SOAK-2 environment-blocked)
14. **Not Linux GUI visual/interactive validated** — compile + launch only
15. **Not externally security-audited** — no penetration test, no formal cert

---

## What v1.0.0 DOES Claim

1. Architecture arc is complete (Control → Close → Stable)
2. All verification machinery exists, is tested, and is documented
3. All 21 claims are evidence-backed (final re-audit VK-3)
4. All 15 caveats are explicitly documented
5. Zero overclaiming found in audit across all versions
6. External review packet is available for reviewer execution
7. Repeatable release process with automated readiness checks
8. Guided evidence flow reduces reviewer friction
9. Release readiness can be checked with a single command
10. Desktop binary builds on both Windows and Linux
11. rc.1 soak/regression window passed with zero blocking regressions
12. CLI version string correctly reports `1.0.0`

---

## Post-v1.0.0 Roadmap

| Item | Priority | Notes |
|------|----------|-------|
| External review execution | P1 | Packet ready; needs external reviewer |
| Direct OpenAI/Anthropic/Ollama validation | P2 | Architecture supports it |
| Linux GUI full visual validation | P2 | Needs physical Linux + GPU |
| macOS runtime validation | P3 | Needs macOS environment |
| API stabilization | P3 | May begin post-v1.0 |
| Clean install validation | P2 | Exercise fresh clone → build → run |
| Production hardening | P3 | Requires operational experience |
