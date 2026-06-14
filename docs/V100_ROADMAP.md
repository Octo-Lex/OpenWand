# OpenWand v1.0.0 Roadmap

**Theme:** Release-candidate closure.

**Status:** In progress (Wave 119A)

---

## Context

v0.9.0 completed the external validation and adoption readiness arc: guided
evidence flow (`openwand review`), release automation (`openwand release-check`),
and a release package with preserved caveats. The architecture arc reached:

```
Control → Observe → Operate → Verify → Harden → Externally Anchor →
Operationally Harden → Externally Validate
```

v1.0.0 is the release-candidate closure milestone. It does NOT need every
deferred platform/provider item closed — but it does need:

1. An externally exercised review path (VJ-1)
2. A final claim/caveat audit (VJ-4)
3. A release candidate artifact with explicit caveats (VJ-5)

Deferred items (Linux visual validation, provider expansion) may remain as
documented caveats in v1.0.0 if they cannot be resolved.

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
v1.0  release-candidate closure             Close
```

---

## Blockers (VJ series)

| Blocker | Description | Priority |
|---------|-------------|----------|
| VJ-1 | External review execution — Deferred (no reviewer available) | P0 (gate) |
| VJ-2 | Provider validation decision — Deferred (matrix preserved) | P1 |
| VJ-3 | Linux GUI visual decision — Deferred (partial accepted) | P1 |
| VJ-4 | Final assurance/caveat audit — Resolved | P0 (gate) |
| VJ-5 | v1.0 release candidate | P1 |

---

### VJ-1: External Review Execution

**Problem:** The external review packet exists (`docs/EXTERNAL_REVIEW_PACKET.md`)
and the guided flow exists (`openwand review`), but no external reviewer has
actually executed them.

**Goal:** Have a real external party run the review packet and classify the
outcome: Pass, Partial, or Blocked.

**Scope:**
- A reviewer (independent agent, CI pipeline, or external contributor) follows
  the review packet or guided flow
- Reviewer records: commands run, outputs, exit codes, discrepancies
- Reviewer classifies the outcome
- Reviewer's findings are recorded as evidence

**If no reviewer is available:** Document the deferral explicitly. v1.0.0 may
ship with "review packet exists but has not been externally exercised" as a
caveat, but VJ-1 must be classified (not silently ignored).

---

### VJ-2: Provider Validation Decision

**Problem:** Provider validation is limited to LM Studio and Z.AI. Direct
OpenAI, Anthropic, and Ollama paths remain unvalidated.

**Goal:** Either validate at least one direct provider path, or explicitly
defer provider expansion from v1.0.0 scope with a documented caveat.

**Decision required:** Validate or defer. No silent omission.

---

### VJ-3: Linux GUI Visual/Interactivity Decision

**Problem:** Linux GUI validation is Partial — desktop binary compiles and
launches, but visual rendering and interactive UI are not validated.

**Goal:** Either validate with a real display/compositor, or preserve as a
documented caveat with clear scope statement.

**Decision required:** Validate or defer. No overclaiming.

---

### VJ-4: Final Assurance/Caveat Audit

**Problem:** Over 9 release cycles (v0.1.0-alpha through v0.9.0), claims,
caveats, gaps, and release notes have accumulated. A final reconciliation is
needed to ensure consistency.

**Goal:** Audit all claims, non-claims, gaps, release notes, and evidence files
for consistency.

**Scope:**
- Cross-check all release notes for claim consistency
- Verify STATE.md matches actual artifacts
- Verify all caveats in release notes match actual limitations
- Verify evidence files (SECURITY_SCAN_RESULTS, AUTHORITY_REVIEW, etc.) are current
- Verify no stale claims remain from earlier versions
- Produce audit report documenting findings

---

### VJ-5: v1.0 Release Candidate

**Problem:** v1.0.0 needs a release candidate artifact with explicit caveats.

**Goal:** Create v1.0.0-rc.1 with:
- Artifact identity (binary, SHA-256, size)
- Release checklist output (`openwand release-check`)
- Explicit caveats for all deferred items
- Final claim/non-claim ledger

**Scope:** After VJ-1 and VJ-4 are classified/resolved.

---

## Proposed Wave Sequence

| Wave | Title | Blocker | Description |
|------|-------|---------|-------------|
| 117A | Post-v0.9 Roadmap Reset | — | This document; VJ blockers proposed |
| 118A | Final Assurance Audit | VJ-4 | Reconcile all claims, caveats, gaps |
| 119A | Provider/Linux Decision | VJ-2, VJ-3 | Classify as validated or explicitly deferred |
| 120A | v1.0.0-rc.1 Preparation | VJ-5 | Release candidate artifact |
| 120B | v1.0.0-rc.1 Declaration | — | Tag v1.0.0-rc.1 |

**Note:** VJ-1 (external review execution) requires an external party and may
need to be classified as "deferred with rationale" rather than resolved.

---

## Deferred Items (from prior arcs)

| Item | Origin | Status |
|------|--------|--------|
| External review execution | VI-1 / VJ-1 | Packet exists; no external reviewer yet |
| Linux GUI visual rendering | VH-1 / VI-4 / VJ-3 | Partial — compile + launch validated |
| Direct OpenAI/Anthropic/Ollama | VG-4 / VH-2 / VI-3 / VJ-2 | Deferred — LM Studio + Z.AI validated |
| macOS runtime | Platform caveat | No environment |
| Full signature-based anchoring | DEFERRED-004 | External anchor done; signatures post-v1.0 |
| Stable API guarantee | v0.5.0 caveat | v1.0 may begin API stabilization |
| Production readiness | Global caveat | Not claimed for v1.0.0-rc.1 |

---

## What v1.0.0 Is

v1.0.0 is the first version number that signals **release-candidate maturity**.
It means:

- The architecture arc is complete
- All verification machinery exists and is tested
- The review packet is available for external execution
- Claims are evidence-backed and caveats are explicit
- The release process is repeatable and automated

It does NOT mean:
- Production-ready (that requires external security audit, load testing, etc.)
- Feature-complete (provider matrix, platform coverage may be limited)
- API-frozen (API stabilization may begin but is not guaranteed)

---

## Success Criteria for v1.0.0-rc.1

1. **VJ-4 resolved** — final assurance audit complete, findings documented
2. **VJ-1 classified** — external review either executed or explicitly deferred
3. **VJ-2/VJ-3 classified** — provider and Linux GUI validated or explicitly deferred
4. **Release candidate artifact exists** with identity, checklist, caveats
5. **No new write authority for verifiers**
6. **No overclaiming** — all claims evidence-backed

---

## What v1.0.0 is NOT

- Not production-ready
- Not a formal external security audit
- Not full cryptographic non-repudiation
- Not full cross-platform runtime validation
- Not full Linux GUI support (unless VJ-3 resolved)
- Not provider complete (unless VJ-2 resolved)
- Not API-frozen (stabilization may begin but not guaranteed)
