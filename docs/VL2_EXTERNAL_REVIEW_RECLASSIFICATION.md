# VL-2 External Review Reclassification — Wave 130A

**Date:** 2026-06-15
**Wave:** 130A
**Commit:** (recorded at lock)
**Tag:** `wave-130a-lock`
**Blocker:** VL-2 — External review execution
**Classification:** **Consciously Deferred — Updated Context (v1.0.1)**

---

## 1. Classification Decision

**VL-2 remains Consciously Deferred.**

No external reviewer has executed the v1.0.1 review packet or guided review
flow. No external reviewer is available at this time.

This is the same outcome as VK-1 (Wave 123A), but with updated context:
the review infrastructure has been validated end-to-end by the builder,
a real defect was found and fixed through the process, and the review packet
has been updated to v1.0.1 with accurate commands and expected outputs.

---

## 2. What Has Changed Since VK-1 (Wave 123A)

| Item | VK-1 (123A) | VL-2 (130A) |
|------|-------------|-------------|
| Review packet version | v1.0.0-rc.1 | v1.0.1 |
| `--db` flag | ❌ Broken (silently ignored) | ✅ Fixed (v1.0.1) |
| Trace verification on fresh data | ❌ Broken (F-VL1-1) | ✅ Fixed (v1.0.1) |
| Guided review flow | Untested end-to-end | ✅ Validated end-to-end |
| Evidence report result | CompleteWithCaveats | ✅ Complete |
| First real workflow evidence | Not run | ✅ VL-1 PASS (post-fix) |
| Regression tests for found bugs | N/A | ✅ 3 tests |
| Artifact SHA-256 in packet | Not included | ✅ Both binaries |
| `--db` flag in packet commands | Not shown | ✅ All commands |

### Key insight

The VL-1 validation cycle (127A → 128A → 129A → 129B) demonstrated that the
review infrastructure **works**:

```
real workflow → blocking evidence defect found (F-VL1-1) →
root cause identified → regression tests added →
patch shipped (v1.0.1) → workflow evidence passes
```

This is meaningful evidence that the verification commands, evidence report,
and guided review flow function correctly. However, it was all performed by
the builder (Craft Agent + user), not by an external reviewer.

---

## 3. Why VL-2 Remains Deferred

### 3.1 No External Reviewer Available

| Item | Status |
|------|--------|
| External human reviewer | Not available |
| Third-party security audit firm | Not engaged |
| Penetration test team | Not engaged |
| Formal certification body | Not engaged |
| Academic peer review | Not performed |
| Open-source community review | Not yet (repo is private/published but no external contributor has reviewed) |

### 3.2 Builder Is Not External

Craft Agent (the development agent) and the user (the developer) are the
builders. Running the review packet as the builder does NOT constitute
external review. The VL-1 validation was an **internal self-test**, not
an external review.

### 3.3 API/LLM Tools Are Not External Reviewers

The Z.AI API, the mock LLM server, and other tools used in development are
part of the build tool chain. They are not independent reviewers.

---

## 4. What An External Reviewer Would Need To Do

The v1.0.1 review packet (`docs/EXTERNAL_REVIEW_PACKET.md`) is ready. An
external reviewer would:

1. **Clone and build** from `v1.0.1` tag
2. **Verify artifact identity** (SHA-256 match)
3. **Generate a test session** (agent turn with an LLM)
4. **Run trace-verify** — confirm Pass on fresh data
5. **Write and verify an anchor** — confirm checkpoint works
6. **Run operation-replay** — confirm correspondence
7. **Generate evidence report** — confirm Complete
8. **Run guided review** — confirm all steps Pass
9. **Review authority surfaces** (12 surfaces in AUTHORITY_REVIEW.md)
10. **Review security scan** (SECURITY_SCAN_RESULTS.md)
11. **Record findings** — Pass / Pass with caveats / Fail
12. **Publish review outcome** as evidence

---

## 5. Caveat Classification

| Caveat | Description | Status |
|--------|-------------|--------|
| X-09 | Not externally reviewed — consciously deferred | **Still Deferred** (updated context) |
| X-15 | External review consciously deferred from v1.0.0 final | **Still Deferred** (updated context) |

X-09 and X-15 remain **active**. No external review evidence exists to
resolve them. The updated context strengthens the case for future resolution
(review infrastructure validated), but does not resolve the caveats.

---

## 6. What This Classification Does NOT Claim

- Not certification
- Not a security audit
- Not external review execution
- Not production readiness
- Not formal verification
- Not that the review packet has been independently validated
- Not that the guided review flow is free of defects (only that the builder
  validated it end-to-end)

---

## 7. Path To Resolution

VL-2 can be resolved when:
1. An independent third party (not the builder) clones the v1.0.1 tag
2. Runs the review packet or guided review flow
3. Records findings (Pass / Pass with caveats / Fail)
4. Publishes the outcome

The review packet is ready. The infrastructure works (validated by VL-1).
What is missing is the **reviewer**, not the **review infrastructure**.

---

## 8. Impact on VL-2 Status

```text
VL-2 External Review Execution
  Classification: Consciously Deferred — Updated Context
  Review packet: Ready (v1.0.1)
  Review infrastructure: Validated end-to-end (VL-1)
  External reviewer: Not available
  Caveats X-09, X-15: Still active
  Path to resolution: Engage an external reviewer to run the packet
```
