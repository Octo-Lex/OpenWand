# OpenWand v0.9.0 — Stable Release

**Date:** 2026-06-14
**Tag:** `v0.9.0`
**Theme:** External validation and adoption readiness

---

## Overview

v0.9.0 moves from internal preparation to external validation. v0.8 made the
review packet and process credible; v0.9 puts that packet in front of a
reviewer and reduces friction around evidence generation.

---

## Architecture Arc

```
v0.2  governed execution substrate          Control
v0.3  live observation                      Observe
v0.4  desktop operation requests            Operate
v0.5  read-only verification                Verify
v0.6  evidence-backed assurance hardening   Harden
v0.7  external assurance                    Externally Anchor
v0.8  operational hardening                 Operationally Harden
v0.9  external validation                   Externally Validate
```

---

## What's New in v0.9.0

### 1. Guided Evidence Flow (Wave 114A)

New CLI command: `openwand review`

Wraps the evidence-report workflow into a step-by-step guided flow:
- Validates inputs upfront (session, operations, anchor)
- Runs trace integrity verification with progress output
- Runs operation correspondence verification
- Runs checkpoint anchor verification (if provided)
- Generates evidence report JSON
- Prints reviewer-ready summary with next steps and non-claims

Does not infer operations silently, mutate trace, create anchors, or
execute tools.

### 2. Release Automation (Wave 115A)

New CLI command: `openwand release-check`

Executes 8 automated release readiness checks:
1. Workspace tests
2. Production clippy (11 crates)
3. Cargo audit (CVEs)
4. CLI binary build (openwand)
5. Desktop binary build (openwand-ui) — 109A gate
6. Artifact identity (size + hash)
7. Documentation presence (5 key docs)
8. STATE.md consistency

4 manual-required items listed explicitly. Does NOT publish, tag, push,
or declare a release.

### 3. Debug Binary Stack Overflow Fix (Wave 115A)

Fixed debug binary stack overflow caused by deep clap derive nesting.
Wrapped main() in spawn_blocking with larger stack.

---

## VI Blocker Resolution

| Blocker | Status |
|---------|--------|
| VI-2: Evidence report UX integration | ✅ Resolved (guided flow) |
| VI-5: Release automation | ✅ Resolved (executable checks) |
| VI-1: External review execution | Deferred (requires external party) |
| VI-3: Provider validation expansion | Deferred (strategic) |
| VI-4: Linux GUI visual validation | Deferred (environment) |

---

## Metrics

| Metric | Value |
|--------|-------|
| Tests | 4,216 total, 0 failures |
| Production crates (clippy) | 11 crates, 0 warnings |
| Binary size | 17,702,912 bytes (~16.9 MB) |
| SHA-256 | `3F678ACD185D8C1A2FD168202C8B76A3140B20A2E1EB684081A591E0A9A4ECF9` |
| CVEs | 0 (last audit: Wave 105A, 721 dependencies) |
| Authority surfaces | 12 (4 write-capable, 3 read-only verifiers) |
| CLI commands | 40+ including review, release-check |

---

## Release Lineage

```
v0.1.0-alpha → v0.1.0-beta → v0.2.0 → v0.3.0 → v0.4.0 → v0.5.0 → v0.6.0 → v0.7.0 → v0.8.0 → v0.9.0
```

---

## Caveats — What v0.9.0 Does NOT Claim

1. **Not production-ready** — this is a development release
2. **Not formal security certification** — no external audit performed
3. **Not physical immutability** — trace stores are technically mutable files
4. **Not remote attestation** — no hardware or network attestation
5. **Not full immutability** — attacker who rewrites store + anchor passes
6. **Not full cross-platform runtime validation** — Linux GUI is Partial
7. **Not full Linux GUI support** — visual rendering not validated
8. **Not provider completeness** — only LM Studio and Z.AI validated
9. **Not stable API guarantee** — APIs may change between versions
10. **Not externally reviewed** — external review packet exists but no external
    reviewer has executed it yet
