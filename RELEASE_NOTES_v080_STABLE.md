# OpenWand v0.8.0 — Stable Release

**Date:** 2026-06-14
**Tag:** `v0.8.0`
**Theme:** Operational hardening and platform closure

---

## Overview

v0.8.0 closes practical operation gaps around platform runtime validation,
reviewer handoff, and repeatable release process.

v0.7 made evidence externally anchorable. v0.8 makes the platform/runtime and
reviewer handoff credible.

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
```

---

## What's New in v0.8.0

### 1. Linux Desktop Binary Compilation (Wave 109A)

The `openwand-ui` desktop binary had never been compiled as part of the build
gate. 9 latent compilation errors (from Waves 88A–88C) were found and fixed:

- Missing `service` parameter in `render_inspector_pane()`
- Format string placeholder count mismatch
- `&*` deref on owned values (4 occurrences)
- Borrowed data escaping function via closure
- Moved values in `FnMut` closures (`arid`, `tool_name`)
- `workflow_execution_id` reference lifetime

Both `openwand` and `openwand-ui` now compile on Windows and Linux.

### 2. Linux GUI Runtime Partial Validation (Wave 109A)

Native Linux VM validation on Proxmox (Ubuntu 24.04, virtio-gpu, Xorg):

- ✅ Desktop binary launches and creates a window (xdotool confirmed)
- ✅ GTK/WebKit/Dioxus stack initializes without crash
- ✅ Direct rendering confirmed (glxinfo)
- ✅ Application stable for 6+ seconds
- ❌ Visual rendering not captured (WebKit compositing limitation in virtualized GPU)

Classification: **Partial** — not full Linux GUI support.

### 3. External Review Packet (Wave 110A)

A single reviewer-ready artifact (`docs/EXTERNAL_REVIEW_PACKET.md`) packaging:
- Trace verification, operation replay, checkpoint anchor commands
- Evidence report, security scan, authority review summaries
- Exact CLI commands, exit code tables, expected outputs
- 10-step quick reviewer checklist
- Complete caveats and non-claims ledger

### 4. Repeatable Release Checklist (Wave 111A)

A 12-section release process checklist (`docs/RELEASE_CHECKLIST.md`) covering:
- Tests, production clippy, cargo audit
- **Both** CLI and desktop binary build gates (109A correction)
- Linux compile/runtime validation status
- Artifact identity, release notes consistency, tag consistency
- Caveat/non-claim review

Explicitly states: process evidence, not production readiness.

---

## VH Blocker Resolution

| Blocker | Status |
|---------|--------|
| VH-1: Linux GUI runtime validation | ✅ Partial — resolved for v0.8 |
| VH-3: External review packet | ✅ Resolved |
| VH-5: Release/process hardening | ✅ Resolved |
| VH-2: Provider validation expansion | Deferred (strategic) |
| VH-4: Evidence report UX integration | Deferred (P2) |

---

## Metrics

| Metric | Value |
|--------|-------|
| Tests | 4,200 total, 0 failures |
| Production crates (clippy) | 11 crates, 0 warnings |
| Binary size | 18,344,960 bytes (~17.5 MB) |
| SHA-256 | `3CBBB103BC386D579801F2F50EB4E3A27DCB031D015E147C0324EA9B4A02BD3C` |
| CVEs | 0 (last audit: Wave 105A, 721 dependencies) |
| Upstream-blocked warnings | 15 (13 GTK3 desktop-only, 1 atomic-polyfill, 1 rand 0.7) |
| Authority surfaces | 12 (4 write-capable, 3 read-only verifiers) |

---

## Release Lineage

```
v0.1.0-alpha → v0.1.0-beta → v0.2.0 → v0.3.0 → v0.4.0 → v0.5.0 → v0.6.0 → v0.7.0 → v0.8.0
```

---

## Caveats — What v0.8.0 Does NOT Claim

1. **Not production-ready** — this is a development release
2. **Not formal security certification** — no external audit performed
3. **Not physical immutability** — trace stores are technically mutable files
4. **Not remote attestation** — no hardware or network attestation
5. **Not full immutability** — attacker who rewrites store + anchor passes
6. **Not full cross-platform runtime validation** — Linux GUI is Partial
7. **Not full Linux GUI support** — visual rendering not validated
8. **Not provider completeness** — only LM Studio and Z.AI validated
9. **Not stable API guarantee** — APIs may change between versions
10. **Not evidence report UX integration** — CLI-only, no desktop surface
