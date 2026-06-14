# OpenWand v1.0.0 Final Roadmap

**Theme:** RC validation and final-release decision.

**Status:** Planning (Wave 121A)

---

## Context

v1.0.0-rc.1 closed the architecture arc into a release-candidate artifact.
The governed execution, observation, operation, verification, assurance,
external anchoring, operational-hardening, and evidence/reporting systems are
present, tested, documented, and caveated.

The path from rc.1 to v1.0.0 final requires two core decisions:

1. External review classification (VK-1)
2. rc.1 soak / regression window (VK-2)

Do NOT jump straight from rc.1 to final without explicitly classifying both.

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
v1.0  release-candidate closure             Close (rc.1)
v1.0  final release                         Close (stable)
```

---

## Blockers (VK series)

| Blocker | Description | Priority |
|---------|-------------|----------|
| VK-1 | External review execution | P0 (gate) ✅ Classified |
| VK-2 | rc.1 soak / regression window | P0 (gate) |
| VK-3 | Final v1.0 claim audit | P1 ✅ Resolved |
| VK-4 | Final v1.0 release preparation | P1 |
| VK-5 | v1.0.0 final declaration | P1 |

---

### VK-1: External Review Execution

**Problem:** The review packet and guided flow exist, but no external reviewer
has executed them. rc.1 deferred this explicitly. Final v1.0 must classify it.

**Goal:** Either execute an external review (preferred) or make a conscious
decision to ship v1.0 final without external review as a documented caveat.

**Decision required:** Executed, required-before-final, or consciously-deferred.
No silent omission.

**If deferred:** v1.0.0 final ships with "external review packet exists but
has not been externally executed" as an explicit caveat. This is acceptable
if it is a conscious decision, not a default.

---

### VK-2: rc.1 Soak / Regression Window

**Problem:** rc.1 was declared but has not been exercised in any soak period,
installation test, or regression window. Bugs may surface from real usage.

**Goal:** Either run a soak/regression window and classify findings, or make
a conscious decision to ship without a soak period as a documented caveat.

**Scope:**
- Run `openwand release-check` and record output
- Attempt a clean installation (clone → build → run)
- Exercise CLI commands on a fresh environment
- Run the guided review flow end-to-end
- Classify any bugs found: blocking, non-blocking, cosmetic
- Record regression findings

**If deferred:** v1.0.0 final ships with "rc.1 soak not performed" as an
explicit caveat.

---

### VK-3: Final v1.0 Claim Audit

**Problem:** The Wave 118A audit was comprehensive, but if rc.1 soak or
external review surfaces findings, the claim ledger may need updates.

**Goal:** Re-run the final assurance audit after rc.1 feedback is classified.

**Scope:**
- Re-run the claim/caveat audit from Wave 118A
- Check for any new stale references
- Verify claim ledger still holds
- Update if findings require

---

### VK-4: Final v1.0 Release Preparation

**Problem:** v1.0.0 final needs an artifact with identity, notes, and caveats.

**Goal:** Prepare v1.0.0 release package:
- Artifact identity (binary, SHA-256, size)
- Release checklist output (`openwand release-check`)
- RELEASE_NOTES_v100_STABLE.md
- Updated STATE.md, WAVES.md, V100_ROADMAP.md

---

### VK-5: v1.0.0 Final Declaration

**Problem:** v1.0.0 final declaration after VK-1 and VK-2 are classified.

**Goal:** Tag v1.0.0 with explicit caveats for all deferred items.

---

## Proposed Wave Sequence

| Wave | Title | Blocker | Description |
|------|-------|---------|-------------|
| 121A | Post-rc.1 Roadmap Reset | — | This document; VK blockers proposed |
| 122A | rc.1 Soak / Regression Window | VK-2 | Run release-check, clean install, CLI sanity, classify bugs |
| 123A | External Review Classification | VK-1 | Classify as executed or consciously deferred |
| 124A | Final Claim Re-audit | VK-3 | Re-run 118A audit after rc.1 findings |
| 125A | v1.0.0 Final Preparation | VK-4 | Artifact, notes, caveats |
| 125B | v1.0.0 Final Declaration | VK-5 | Tag v1.0.0 |

**Core:** VK-1 + VK-2 must be classified before final declaration.

---

## Deferred Items (carried from rc.1)

| Item | Origin | Status |
|------|--------|--------|
| External review execution | VJ-1 / VK-1 | Packet ready; no reviewer ran it |
| Linux GUI visual rendering | VJ-3 | Partial — compile + launch validated |
| Direct OpenAI/Anthropic/Ollama | VJ-2 | Deferred — LM Studio + Z.AI validated |
| macOS runtime | Platform caveat | No environment |
| Stable API guarantee | v0.5.0 caveat | v1.0 may begin API stabilization |
| Production readiness | Global caveat | Not claimed for v1.0.0 |

---

## What v1.0.0 Final Is

v1.0.0 final is the first version number that signals **stable release**. It
means:

- The architecture arc is complete
- rc.1 soak findings are classified
- External review is classified (executed or consciously deferred)
- All verification machinery exists and is tested
- Claims are evidence-backed and caveats are explicit

It does NOT mean:
- Production-ready (requires formal external security audit)
- Feature-complete (provider matrix, platform coverage may be limited)
- API-frozen (API stabilization may begin but is not guaranteed)
- Externally reviewed (unless VK-1 is resolved by execution)
