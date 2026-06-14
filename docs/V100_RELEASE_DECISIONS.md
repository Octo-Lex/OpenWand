# v1.0 Release-Scope Decisions — Wave 119A

**Date:** 2026-06-14
**Scope:** Classification of VJ-1, VJ-2, and VJ-3 for v1.0.0-rc.1

---

## Purpose

This document records the decision for each remaining v1.0 release-scope
blocker: whether it is executed, required-before-rc.1, or deferred with
documented rationale.

---

## VJ-1: External Review Execution

**Decision: DEFERRED from rc.1**

**Rationale:**
- No external reviewer is immediately available
- The external review packet (`docs/EXTERNAL_REVIEW_PACKET.md`) exists and is
  reviewer-ready with exact commands, expected outputs, exit codes, and caveats
- The guided review flow (`openwand review`) exists to reduce reviewer friction
- VJ-1 requires an external actor (independent agent, CI pipeline, or
  contributor) — it cannot be resolved internally without misrepresenting
  self-review as external review

**Classification:** Deferred with rationale. Not silently omitted.

**What rc.1 may claim:**
- External review packet is available for reviewer execution
- Guided evidence flow exists to reduce reviewer friction
- All verification capabilities have automated tests

**What rc.1 must NOT claim:**
- External review was completed
- Any external party has validated the claims
- The review packet was executed by an independent actor

**Resolution path:** An external reviewer runs the packet and classifies
the outcome (Pass / Partial / Blocked). The result is recorded as evidence.

---

## VJ-2: Provider Validation Decision

**Decision: DEFERRED — existing validated matrix preserved**

**Rationale:**
- Provider validation covers LM Studio (local) and Z.AI (hosted) — two provider
  families validated across simple turns, trace attribution, tool calling, and
  sandbox refusal
- Direct OpenAI, Anthropic, and Ollama paths use the same OpenAI-compatible
  adapter (`openai_compatible.rs`) and are architecturally equivalent
- v1.0 rc.1 is a release-candidate closure milestone, not a provider expansion
  milestone
- Expanding the provider matrix requires API keys and network access to
  external endpoints — introducing external dependencies for a local-first tool

**Classification:** Deferred. Existing validated matrix preserved as a caveat.

**What rc.1 may claim:**
- LM Studio and Z.AI validated across 4 test categories
- OpenAI-compatible adapter supports any OpenAI-compatible endpoint
- Provider configuration is user-driven

**What rc.1 must NOT claim:**
- Direct OpenAI validation
- Direct Anthropic validation
- Direct Ollama validation
- Provider completeness

**Resolution path:** When strategic priority aligns, validate direct paths
for at least one additional hosted provider (OpenAI or Anthropic).

---

## VJ-3: Linux GUI Visual Decision

**Decision: DEFERRED — partial validation accepted as documented caveat**

**Rationale:**
- Linux desktop binary (`openwand-ui`) compiles on both Windows and Linux
- Proxmox VM smoke test (Wave 109A) confirmed: binary launches, creates window,
  initializes GTK/WebKit/Dioxus, direct rendering confirmed (glxinfo)
- Visual rendering NOT captured due to WebKit compositing limitation in
  virtualized GPU environments — this is NOT an OpenWand defect
- Full Linux visual validation requires a physical Linux machine with GPU
  compositing or a cloud-hosted Linux with virtual display
- macOS runtime is also not validated (no environment available)

**Classification:** Partial validation accepted. Deferred from full validation.

**What rc.1 may claim:**
- Linux desktop binary compiles (both platforms)
- Linux runtime: launches, creates window, initializes GTK/WebKit/Dioxus
- Direct rendering confirmed on Proxmox VM

**What rc.1 must NOT claim:**
- Full Linux GUI support
- Linux visual rendering validated
- Linux interactive UI validated
- macOS runtime validated

**Resolution path:** Test on physical Linux with GPU, or cloud Linux with
virtual display + compositor (e.g., Mutter, KWin).

---

## Summary

| Blocker | Decision | Rationale |
|---------|----------|-----------|
| VJ-1 | Deferred from rc.1 | No external reviewer available; packet ready |
| VJ-2 | Deferred; matrix preserved | rc.1 is closure, not expansion |
| VJ-3 | Deferred; partial accepted | Requires physical Linux with GPU |

**All three are classified, not silently omitted.**

---

## Updated Claim/Caveat Ledger

### Claims Added for rc.1

None. The decisions do not add claims. They preserve existing claims and
formalize deferrals.

### Caveats Preserved for rc.1

| Caveat | Source Decision |
|--------|----------------|
| Not externally reviewed (VJ-1) | External review packet exists but not executed |
| Not provider complete (VJ-2) | Only LM Studio + Z.AI validated |
| Not full Linux GUI support (VJ-3) | Partial — compile + launch validated |
| Not macOS validated (carried) | No macOS environment |
| Not production-ready (global) | All versions |
| Not formal certification (global) | All versions |
| Not stable API (global) | All versions |

---

## What This Wave Does NOT Do

- Does not execute new provider validation
- Does not run new Linux GUI tests
- Does not claim external review completion
- Does not upgrade any caveat into an assurance
- Does not add execution authority, policy bypass, or prompt changes
