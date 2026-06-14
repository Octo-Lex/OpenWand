# OpenWand v1.0.0-rc.1 — Release Candidate

**Date:** 2026-06-14
**Tag:** `v1.0.0-rc.1`
**Theme:** Release-candidate closure
**Classification:** Release candidate — not stable, not production-ready

---

## Overview

v1.0.0-rc.1 is the first release candidate. It closes the architecture arc
(Control → Close) and demonstrates that every claim is evidence-backed, every
caveat is documented, and every release-scope decision has been classified.

This is NOT a declaration that OpenWand is production-ready. It is a declaration
that the project has reached release-candidate maturity for its scoped claims.

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
v1.0  release-candidate closure             Close
```

---

## What This Release Candidate Represents

### VJ Blocker Reconciliation

| Blocker | Decision |
|---------|----------|
| VJ-1: External review execution | Deferred — packet ready, no reviewer available |
| VJ-2: Provider validation decision | Deferred — LM Studio + Z.AI matrix preserved |
| VJ-3: Linux GUI visual decision | Deferred — partial (compile + launch) accepted |
| VJ-4: Final assurance/caveat audit | ✅ Resolved — zero overclaiming, zero contradictions |
| VJ-5: v1.0 release candidate | This artifact |

### Audit Results (Wave 118A)

- **18 evidence-backed claims** verified valid
- **14 caveats** verified consistently disclaimed
- **Zero overclaiming** across all release notes (v0.2–v0.9)
- **Zero contradictions** in binary sizes, test counts, CVE counts, authority docs
- **8 stale references** identified, all informational (understate, not overclaim)
- 5 of 8 stale references refreshed in Wave 119A

### Release-Scope Decisions (Wave 119A)

All three deferred items are explicitly classified with rationale, not silently
omitted. See `docs/V100_RELEASE_DECISIONS.md` for full decision record.

---

## Metrics

| Metric | Value |
|--------|-------|
| Tests | 4,232 total, 0 failures |
| Production crates (clippy -D warnings) | 11 crates, 0 warnings |
| CVEs (cargo audit) | 0 (721 dependencies) |
| Unmaintained/unsound warnings | 15 (all upstream-blocked, transitive) |
| CLI binary | 17,702,912 bytes (~16.9 MB) |
| CLI SHA-256 | `3F678ACD185D8C1A2FD168202C8B76A3140B20A2E1EB684081A591E0A9A4ECF9` |
| Desktop binary | 19,500,032 bytes (~18.6 MB) |
| Desktop SHA-256 | `8ACCF49D4D0EB82056A148F2751C318441A8C3DFFE0342B92339CFEF50B97457` |
| CLI commands | 40+ including review, release-check, trace-verify, operation-replay |
| Authority surfaces | 12 (4 write-capable, 8 read-only/delegated) |

---

## Verification Capabilities Available

| Capability | Command |
|------------|---------|
| Trace integrity verification | `openwand trace-verify <session-id>` |
| Operation correspondence | `openwand operation-replay <session-id> --operations <ops.json>` |
| Checkpoint anchor write | `openwand anchor-write <session-id> --output <file>` |
| Checkpoint anchor verify | `openwand anchor-verify <file> --session <session-id>` |
| Evidence report export | `openwand evidence-report <session-id> --operations <ops.json> [--anchor <file>]` |
| Guided review flow | `openwand review <session-id> --operations <ops.json> [--anchor <file>]` |
| Release readiness check | `openwand release-check` |

---

## Release Lineage

```
v0.1.0-alpha → v0.1.0-beta → v0.2.0 → v0.3.0 → v0.4.0 → v0.5.0 → v0.6.0 → v0.7.0 → v0.8.0 → v0.9.0 → v1.0.0-rc.1
```

---

## Caveats — What v1.0.0-rc.1 Does NOT Claim

1. **Not production-ready** — this is a release candidate
2. **Not formal security certification** — no external audit performed
3. **Not externally reviewed** — review packet exists but no external reviewer
   has executed it (VJ-1 deferred)
4. **Not provider complete** — only LM Studio and Z.AI validated (VJ-2 deferred)
5. **Not full Linux GUI support** — partial: compile + launch validated, visual
   rendering not captured (VJ-3 deferred)
6. **Not full cross-platform runtime validation** — macOS not validated
7. **Not physical immutability** — trace stores are technically mutable files
8. **Not remote attestation** — no hardware or network attestation
9. **Not stable API guarantee** — APIs may change between versions
10. **Not full cryptographic non-repudiation** — fully consistent tamper
    (rewrite store + anchor) passes
11. **Not API-frozen** — stabilization may begin but is not guaranteed

---

## What v1.0.0-rc.1 DOES Claim

1. Architecture arc is complete (Control → Close)
2. All verification machinery exists, is tested, and is documented
3. All claims are evidence-backed (18 verified claims)
4. All caveats are explicitly documented (14 verified caveats)
5. Zero overclaiming found in audit across all versions
6. External review packet is available for reviewer execution
7. Repeatable release process with automated readiness checks
8. Guided evidence flow reduces reviewer friction
9. Release readiness can be checked with a single command
10. Desktop binary builds on both Windows and Linux

---

## Path to v1.0.0 Stable

v1.0.0 stable requires:
1. At least one external reviewer executes the review packet (VJ-1)
2. Any findings from the external review are addressed
3. No new blocking issues identified
4. Caveats are preserved

If no external reviewer is available, v1.0.0 stable may proceed with the
explicit caveat that external review has not been executed — but this should
be a conscious decision, not a default.
