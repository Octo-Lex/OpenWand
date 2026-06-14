# OpenWand v0.9.0 Roadmap

**Theme:** External validation and adoption readiness.

**Status:** Planning (Wave 113A)

---

## Context

v0.8.0 completed the operational hardening arc: Linux desktop binary compilation
(9 latent bugs fixed), partial Linux GUI runtime validation, external review
packet, and repeatable release checklist. The architecture arc reached:

```
Control → Observe → Operate → Verify → Harden → Externally Anchor → Operationally Harden
```

v0.9.0 moves from internal preparation to external validation. v0.8 made the
review packet and process credible; v0.9 puts that packet in front of a real
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

## Blockers (VI series)

| Blocker | Description | Priority |
|---------|-------------|----------|
| VI-1 | External review execution | P1 (core) |
| VI-2 | Evidence report UX integration | P1 (core) |
| VI-3 | Provider validation expansion | P2 (strategic) |
| VI-4 | Linux GUI visual/interactivity validation | P2 |
| VI-5 | Release automation | P2 |

---

### VI-1: External Review Execution

**Problem:** v0.8.0 created an external review packet
(`docs/EXTERNAL_REVIEW_PACKET.md`) but no reviewer has actually run it.

**Goal:** Have a real reviewer (external party, CI pipeline, or independent
agent) execute the review packet and classify the outcome.

**Scope:**
- A reviewer follows the 10-step checklist in the packet
- Reviewer records: commands run, outputs, exit codes, discrepancies
- Reviewer classifies: Pass (all commands work as documented), Partial
  (some commands fail or output differs), Blocked (environmental)
- Reviewer's findings are recorded as evidence

**Authority boundary:** The reviewer executes read-only verification commands.
No execution authority is granted to the reviewer.

---

### VI-2: Evidence Report UX Integration

**Problem:** Evidence report generation is CLI-only. A reviewer or operator
must construct JSON files and run CLI commands manually.

**Goal:** Reduce friction by providing a guided flow for evidence generation.

**Candidate scope:**
- Guided CLI flow: `openwand review` interactive command that walks through
  evidence-report generation step by step
- Or: a shell script that automates the 10-step reviewer checklist
- Generates ops.json from session trace data automatically
- Presents results in human-readable summary before writing JSON

**Authority boundary:** The guided flow wraps existing CLI commands.
No new execution authority.

---

### VI-3: Provider Validation Expansion

**Problem:** Provider validation is limited to LM Studio and Z.AI.

**Goal:** Validate at least one additional direct provider.

**Scope:** Depends on API key availability. Marked P2.

---

### VI-4: Linux GUI Visual/Interactivity Validation

**Problem:** Linux GUI validation is Partial — binary compiles, window
creates, but visual rendering not captured.

**Goal:** Move from partial runtime validation to rendered and interacted GUI.

**Approach options:**
- Native Linux desktop with physical display
- VNC remote desktop with full compositor
- CI pipeline with virtual display + GPU passthrough

**Scope:** Marked P2. Depends on environment availability.

---

### VI-5: Release Automation

**Problem:** The release checklist (v0.8.0) is a manual document. Items are
checked by hand.

**Goal:** Turn the release checklist into executable checks where practical.

**Candidate scope:**
- `scripts/release-check.sh`: runs tests, clippy, audit, both binary builds,
  records metrics, exits non-zero on any failure
- `scripts/release-verify.sh`: verifies STATE.md hash/size/tests match actual
- Not all checklist items are automatable (caveat review requires judgment)

**Authority boundary:** Scripts run existing commands. No new authority.

---

## Proposed Wave Sequence

| Wave | Title | Blocker | Description |
|------|-------|---------|-------------|
| 113A | Post-v0.8 Roadmap Reset | — | This document; VI blockers proposed |
| 114A | Guided Evidence Flow | VI-2 | Reduce friction for evidence generation |
| 115A | Release Automation | VI-5 | Executable release checks |
| 116A | v0.9.0 Release Preparation | — | Reconcile blockers, release notes |
| 116B | v0.9.0 Declaration | — | Tag v0.9.0 |

**Note:** VI-1 (external review execution) requires an external party and
may not be schedulable. VI-3 (provider expansion) and VI-4 (Linux GUI
visual) are environment-dependent.

---

## Deferred Items (from prior arcs)

| Item | Origin | Status |
|------|--------|--------|
| Linux GUI visual rendering | VH-1 / VI-4 | Partial — compile + launch validated, rendering not captured |
| Direct OpenAI/Anthropic/Ollama | VG-4 / VH-2 / VI-3 | Deferred — LM Studio + Z.AI validated |
| macOS runtime | Platform caveat | No environment |
| Full signature-based anchoring | DEFERRED-004 | External anchor done; signatures may need v1.0+ |
| Stable API guarantee | v0.5.0 caveat | Post-production-readiness |
| Production readiness | Global caveat | Not claimed for any version |

---

## Success Criteria for v0.9.0

1. **Evidence generation friction reduced** — guided flow or automation exists
2. **Release checks are executable** — at least partially automated
3. **No new write authority for verifiers**
4. **Legacy behavior preserved** — no retroactive failures
5. **All caveats preserved** — no overclaiming

---

## What v0.9.0 is NOT

- Not production-ready
- Not a formal external security audit
- Not full cryptographic non-repudiation
- Not full cross-platform runtime validation
- Not full Linux GUI support
- Not stable API guarantee
- Not provider completeness
