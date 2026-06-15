# OpenWand Post-v1.0 Stabilization Roadmap

**Theme:** Operate, observe, and harden the released system.
**Created:** Wave 126A
**Status:** Planning

---

## Context

v1.0.0 is released stable. The architecture arc (v0.2 → v1.0) is complete.
The construction phase is over.

The strategic shift is: **stop proving that OpenWand can be built. Start
proving that it can be used, reviewed, maintained, and trusted by people
outside the builder loop.**

This roadmap defines the VL (Validation Lifecycle) blocker series that
governs the post-v1.0 stabilization arc. These blockers are different from
the VK series — they require external actors, real usage, and operational
experience, not internal engineering.

---

## VL Blocker Series

| Blocker | Description | Priority | Category |
|---------|-------------|----------|----------|
| VL-1 | User adoption / first real workflow evidence | P0 (gate) | Operational |
| VL-2 | External review execution | P1 | Assurance |
| VL-3 | v1.0.1 maintenance patch criteria | P1 | Maintenance |
| VL-4 | Provider expansion decision | P2 | Feature |
| VL-5 | Linux GUI visual validation | P2 | Platform |
| VL-6 | API stability policy | P2 | Architecture |

---

### VL-1: User Adoption / First Real Workflow Evidence

**Problem:** OpenWand has been built and tested by its builder. No external
user has performed a real workflow with it — created a session, run an agent
loop, used the desktop UI for a genuine task, and reported the outcome.

**Goal:** Record evidence of at least one real workflow executed by a user
(not the builder) using a real LLM provider. This is the "does it actually
work for someone else" gate.

**Scope:**
- A user clones, builds, and runs OpenWand
- A real session is created with a real provider (LM Studio, Z.AI, or other)
- The agent loop completes at least one genuine task
- The outcome is recorded (success, failure, partial)
- Findings are classified: blocking bug, UX issue, feature gap, or passing

**Resolution:** First real-workflow evidence recorded. Outcome classified.

**If deferred:** v1.0.x ships without external usage evidence. This means
the "stable" label is architecturally validated but not operationally
validated. This is acceptable for a development-stage project but must
not be hidden.

**What this proves:** The system works end-to-end for someone outside the
builder loop. This is the single most important post-v1.0 milestone.

**What this does NOT prove:** Scalability, multi-user safety, production
reliability, or broad usability.

---

### VL-2: External Review Execution

**Problem:** The external review packet exists (EXTERNAL_REVIEW_PACKET.md)
and the guided flow works (`openwand review`), but no external reviewer has
executed either. VK-1 classified this as consciously deferred from v1.0.
Post-v1.0 must resolve it.

**Goal:** An external reviewer executes the review packet or guided flow
and records the outcome.

**Scope:**
- An independent reviewer runs the packet commands
- Trace verification, operation replay, anchor write/verify, evidence report
- Findings are recorded: Pass / Pass with caveats / Fail
- Any security or integrity issues are classified
- The review outcome is published as evidence

**Resolution:** External review executed. Outcome recorded and published.

**If deferred:** The review packet remains available but unexecuted. Each
subsequent release must carry this caveat until resolved.

**What this proves:** An independent party has validated the verification
claims, not just the builder.

**What this does NOT prove:** Formal certification, penetration test, or
comprehensive security audit.

---

### VL-3: v1.0.1 Maintenance Patch Criteria

**Problem:** v1.0.0 may have bugs that surface from real usage. The project
needs criteria for when to cut a patch release and what qualifies.

**Goal:** Define maintenance patch criteria so bug fixes have a clear path
to release without destabilizing the v1.0 arc.

**Scope:**
- Define what constitutes a patch-worthy bug (vs. cosmetic, vs. feature)
- Define the patch release process: branch, fix, test, tag, publish
- Define versioning: v1.0.0 → v1.0.1 for patches, v1.1.0 for features
- Define backward compatibility expectations for patch releases
- Record criteria in a MAINTENANCE_POLICY.md

**Resolution:** Maintenance policy defined. First patch (if needed) follows it.

**If deferred:** Bugs found in usage are fixed on master without a formal
patch release. Acceptable early but creates maintenance debt.

---

### VL-4: Provider Expansion Decision

**Problem:** Only LM Studio (local) and Z.AI (hosted) are validated.
Direct OpenAI, Anthropic, and Ollama paths use the same adapter but are
untested. VJ-2 deferred this from v1.0.

**Goal:** Decide which providers to validate next and execute validation.

**Scope:**
- Prioritize providers based on user demand (informed by VL-1 adoption)
- Validate at least one additional hosted provider (OpenAI or Anthropic)
- Validate direct Ollama path (local, no API key needed)
- Record validation evidence per provider

**Resolution:** At least one additional provider validated and documented.

**If deferred:** LM Studio + Z.AI matrix preserved. Provider caveat
remains in force.

---

### VL-5: Linux GUI Visual Validation

**Problem:** Linux desktop binary compiles and launches, but visual
rendering has never been captured (WebKit compositing limitation in
virtualized GPU). VJ-3 deferred this.

**Goal:** Validate Linux GUI visual rendering on real hardware or a
properly configured virtual display.

**Scope:**
- Test on physical Linux with GPU compositing, or
- Test on cloud Linux with virtual display + compositor (Mutter, KWin)
- Capture screenshot evidence
- Test interactive UI (click, type, tab switch)
- Record findings

**Resolution:** Linux GUI visual rendering validated with screenshot evidence.

**If deferred:** Partial Linux GUI validation (compile + launch) remains.
Visual rendering caveat stays in force.

---

### VL-6: API Stability Policy

**Problem:** v1.0.0 carries "not stable API" as caveat X-08. The project
needs a policy for when and how APIs will be stabilized.

**Goal:** Define API stability tiers and a roadmap for stabilization.

**Scope:**
- Identify public API surface (CLI commands, crate interfaces, config format)
- Define stability tiers: experimental, stable, frozen
- Define deprecation policy
- Document in API_STABILITY_POLICY.md
- Begin stabilizing the most mature surfaces

**Resolution:** API stability policy defined. First surfaces marked stable.

**If deferred:** "Not stable API" caveat remains. All interfaces may change
without notice.

---

## Proposed Wave Sequence

| Wave | Title | Blocker | Description |
|------|-------|---------|-------------|
| 126A | Post-v1.0 Stabilization Roadmap | — | This document; VL blockers proposed |
| 127A | First Real Workflow Evidence | VL-1 | Record or defer first external usage evidence |
| 128A | Maintenance Patch Criteria | VL-3 | Define v1.0.x patch policy |
| 129A | API Stability Policy | VL-6 | Define API tiers and stabilization roadmap |
| 130A+ | Provider Expansion | VL-4 | Validate additional providers (demand-driven) |
| 131A+ | Linux GUI Validation | VL-5 | Physical/cloud Linux visual test |
| 132A+ | External Review Execution | VL-2 | Independent reviewer runs the packet |

**Note:** VL-1 is the gating blocker. VL-2 through VL-6 can proceed in
parallel or in different orders depending on what real usage surfaces.

---

## What Changed from v1.0 Arc

| v1.0 Arc (VK) | Post-v1.0 Arc (VL) |
|---------------|---------------------|
| Internal engineering | External validation |
| "Can we build it?" | "Can others use it?" |
| Architecture completeness | Operational completeness |
| Evidence-backed claims | Evidence-backed usage |
| Builder-verified | Externally-verified |
| Code + tests | Real workflows + users |

---

## What This Roadmap Does NOT Claim

- Does not claim v1.0.0 is production-ready
- Does not claim external usage has occurred
- Does not claim API stability
- Does not set a timeline for v1.1.0 or v2.0.0
- Does not claim the VL blockers are exhaustive
- Does not add execution authority, policy bypass, or prompt change
- Does not upgrade any v1.0 caveat into assurance

---

## Carried Caveats from v1.0.0

All 15 v1.0.0 caveats (X-01 through X-15) remain in force during the
post-v1.0 stabilization arc. VL blockers do not modify, weaken, or remove
any existing caveat. Each VL resolution may allow specific caveats to be
updated — but only with explicit evidence and documentation.

| VL Blocker | Caveat(s) it could resolve |
|------------|---------------------------|
| VL-1 | (No direct caveat — operational evidence) |
| VL-2 | X-09, X-15 (external review deferred) |
| VL-3 | (No direct caveat — maintenance policy) |
| VL-4 | X-07 (provider completeness) |
| VL-5 | X-06 (Linux GUI support) |
| VL-6 | X-08 (stable API guarantee) |

**No caveat is resolved until the corresponding VL blocker is resolved
with real evidence.**
