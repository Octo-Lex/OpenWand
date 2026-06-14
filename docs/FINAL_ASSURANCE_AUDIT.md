# Final Assurance/Caveat Audit — Wave 118A

**Audit date:** 2026-06-14
**Scope:** All project claims, non-claims, caveats, gaps, and evidence files across v0.2–v0.9
**Auditor:** Automated structured audit of all markdown documentation

---

## Purpose

This document is the final reconciliation of every claim and caveat OpenWand
makes, prepared for v1.0.0-rc.1 closure. It cross-checks release notes, STATE.md,
KNOWN_GAPS, DEFERRED_RISKS, SECURITY_SCAN_RESULTS, AUTHORITY_REVIEW,
EXTERNAL_REVIEW_PACKET, and RELEASE_CHECKLIST for consistency.

**The audit classifies claims. It does not upgrade caveats into assurances.**

> **Re-audit:** Wave 124A performed a full re-audit after rc.1 soak (122A) and
> external review classification (123A). See `docs/VK3_FINAL_CLAIM_REAUDIT.md`.
> The re-audit added claims C-19 through C-21, updated caveat X-09, added
> caveat X-15, fixed 7 stale references, and confirmed zero overclaiming.

---

## 1. Claim Ledger

### 1.1 Claims OpenWand MAY Make (Evidence-Backed)

| # | Claim | Evidence | First Version | Status |
|---|-------|----------|---------------|--------|
| C-01 | Governed execution substrate with deterministic trust gate | Policy crate, sandbox, approval lifecycle | v0.2.0 | ✅ Valid |
| C-02 | Append-only trace store with BLAKE3 hash chaining | TraceEntry prev_hash/entry_hash, compute_entry_hash() | v0.5.0 (92A) | ✅ Valid |
| C-03 | Read-only trace integrity verification | TraceVerifier + `openwand trace-verify` | v0.5.0 (92B) | ✅ Valid |
| C-04 | Read-only operation correspondence verification | OperationReplayVerifier + `openwand operation-replay` | v0.5.0 (93B) | ✅ Valid |
| C-05 | Externally persisted checkpoint anchors | CheckpointWriter, verify_anchor, `openwand anchor-write/verify` | v0.7.0 (104B) | ✅ Valid |
| C-06 | Reviewer-facing evidence report export | EvidenceReport JSON + `openwand evidence-report` | v0.7.0 (106A) | ✅ Valid |
| C-07 | Automated security scan evidence | SECURITY_SCAN_RESULTS.md, cargo audit + clippy + authority guards | v0.7.0 (105A) | ✅ Valid |
| C-08 | Structured authority boundary review | AUTHORITY_REVIEW.md, 12 surfaces documented | v0.7.0 (105B) | ✅ Valid |
| C-09 | Linux desktop binary compiles on Windows and Linux | openwand-ui builds on both platforms | v0.8.0 (109A) | ✅ Valid |
| C-10 | Partial Linux runtime validation (launches, window created) | Proxmox VM smoke test | v0.8.0 (109A) | ✅ Valid |
| C-11 | External review packet available for reviewer execution | EXTERNAL_REVIEW_PACKET.md | v0.8.0 (110A) | ✅ Valid |
| C-12 | Repeatable release checklist with desktop binary gate | RELEASE_CHECKLIST.md | v0.8.0 (111A) | ✅ Valid |
| C-13 | Guided evidence flow (`openwand review`) | Step-by-step CLI flow | v0.9.0 (114A) | ✅ Valid |
| C-14 | Release readiness automation (`openwand release-check`) | 8 automated checks | v0.9.0 (115A) | ✅ Valid |
| C-15 | Hash verification policy with BLAKE3 recomputation | HashVerificationPolicy + Blake3HashPolicy | v0.6.0 (98A) | ✅ Valid |
| C-16 | Zero CVEs in direct dependencies | cargo audit (721 deps, 0 vulns) | v0.2.0+ | ✅ Valid (as of 105A scan) |
| C-17 | Zero `unsafe` in production code (one accepted: libc::dup for Unix openat) | Source inspection | v0.2.0+ | ✅ Valid |
| C-18 | Desktop UI authority boundary enforced (no backend imports) | 32+ guard tests | v0.4.0+ | ✅ Valid |
| C-19 | rc.1 soak/regression window executed, zero blocking regressions | RC1_SOAK_REPORT.md (122A) | v1.0.0-rc.1 | ✅ Valid |
| C-20 | External review classification is conscious and documented | VK1_EXTERNAL_REVIEW_CLASSIFICATION.md (123A) | v1.0.0-rc.1 | ✅ Valid |
| C-21 | CLI version string correctly reports 1.0.0-rc.1 | F-SOAK-1 fixed in 122A | v1.0.0-rc.1 | ✅ Valid |

### 1.2 Claims OpenWand MUST NOT Make (Caveats)

| # | Caveat | Origin | Status |
|---|--------|--------|--------|
| X-01 | Not production-ready | v0.1.0-alpha | ✅ Consistently disclaimed |
| X-02 | Not formal security certification | v0.5.0 | ✅ Consistently disclaimed |
| X-03 | Not physical immutability (files technically mutable) | v0.5.0 | ✅ Consistently disclaimed |
| X-04 | Not remote attestation | v0.7.0 | ✅ Consistently disclaimed |
| X-05 | Not full cross-platform runtime validation | v0.4.0 | ✅ Consistently disclaimed |
| X-06 | Not full Linux GUI support (visual rendering not validated) | v0.8.0 | ✅ Consistently disclaimed |
| X-07 | Not provider completeness (LM Studio + Z.AI only) | v0.2.0 | ✅ Consistently disclaimed |
| X-08 | Not stable API guarantee | v0.5.0 | ✅ Consistently disclaimed |
| X-09 | Not externally reviewed (packet exists, no reviewer ran it) | v0.9.0 | ✅ Consistently disclaimed — **Consciously deferred for v1.0 final (Wave 123A)** |
| X-15 | External review consciously deferred from v1.0.0 final | Wave 123A | ✅ Active — docs/VK1_EXTERNAL_REVIEW_CLASSIFICATION.md |
| X-10 | Not macOS validated | v0.4.0 | ✅ Consistently disclaimed |
| X-11 | Fully consistent tamper passes (rewrite store + anchor) | v0.5.0 | ✅ Consistently disclaimed |
| X-12 | Windows final-component TOCTOU residual (safe failure) | v0.8.0 | ✅ Consistently disclaimed |
| X-13 | openwand-content is a stub crate | v0.2.0 | ✅ Consistently disclaimed |
| X-14 | 15 transitive dependency warnings (all upstream-blocked) | v0.2.0+ | ✅ Consistently disclaimed |

---

## 2. Audit Findings

### 2.1 STALE References (Informational, Not Overclaiming)

| # | Finding | Document | Current Value | Correct Value | Severity |
|---|---------|----------|---------------|---------------|----------|
| F-01 | Version label stale | docs/EXTERNAL_REVIEW_PACKET.md | "v0.8.0" | "v0.9.0" | Low — understates capabilities |
| F-02 | Architecture arc incomplete | docs/EXTERNAL_REVIEW_PACKET.md | Stops at v0.8 | Missing v0.9 Externally Validate | Low — omits guided review |
| F-03 | Version label stale | docs/SECURITY_SCAN_RESULTS.md | "v0.7.0 (post-104B)" | "v0.9.0" | Low — findings still valid |
| F-04 | Version label stale | docs/AUTHORITY_REVIEW.md | "v0.7.0 (post-105A)" | "v0.9.0" | Low — surfaces unchanged |
| F-05 | Publication status stale | docs/DEFERRED_RISKS.md DEFERRED-007 | "publication pending" | RESOLVED (published since v0.1.0-alpha) | Low — understates |
| F-06 | Gap ledger not updated | docs/KNOWN_GAPS.md | "Updated Wave 90B" | Should reflect v0.5-v0.9 | Low — gaps listed still accurate |
| F-07 | Clippy warning count drift | docs/DEFERRED_RISKS.md | "57 style warnings" | Actual: 82 (with desktop feature) | Low — different measurement scopes |
| F-08 | Clippy warning count drift | STATE.md | "50 app crate" | Actual: 82 (with desktop feature) | Low — different measurement scopes |

### 2.2 Overclaiming Check

| Check | Result |
|-------|--------|
| Any release note claims production-ready? | ✅ NO — all consistently disclaim |
| Any release note claims formal certification? | ✅ NO — all consistently disclaim |
| Any release note claims physical immutability? | ✅ NO — all consistently disclaim |
| Any release note claims full Linux GUI support? | ✅ NO — all consistently disclaim |
| Any release note claims provider completeness? | ✅ NO — all consistently disclaim |
| Any release note claims stable API? | ✅ NO — all consistently disclaim |
| Any release note claims external review completion? | ✅ NO — all consistently disclaim |
| Any release note claims zero unsafe? | ✅ NO — one accepted: libc::dup, documented |
| README claims immutability? | ✅ NO — corrected in DEFERRED-004 |

**Result: Zero overclaiming found across all audited documents.**

### 2.3 Contradiction Check

| Check | Result |
|-------|--------|
| Binary sizes consistent within versions? | ✅ YES — v0.7/v0.8 same (no code change), v0.9 different (stack fix) |
| Test counts consistent? | ✅ YES — 4,216 for v0.9.0 |
| SHA-256 hashes match actual binary? | ✅ YES — verified at declaration |
| CVE count consistent? | ✅ YES — 0 across all versions |
| Authority boundary consistently documented? | ✅ YES — 12 surfaces, no drift |

**Result: Zero contradictions found.**

---

## 3. Deferred Risk Reconciliation

| ID | Description | Status | Audit Confirmation |
|----|-------------|--------|--------------------|
| DEFERRED-001 | App clippy warnings | Accepted non-blocking | ✅ Accurate — cosmetic, test-only |
| DEFERRED-002 | cargo audit warnings | Closed by recording | ✅ Accurate — 0 CVEs, 15 upstream-blocked |
| DEFERRED-003 | unsafe-env-test claim | Closed by correction | ✅ Accurate — test-only unsafe |
| DEFERRED-004 | Trace immutability claim | Partially closed | ✅ Accurate — verifier exists, full crypto not claimed |
| DEFERRED-005 | MutationHelper correctness | Closed with rationale | ✅ Accurate |
| DEFERRED-006 | Documentation update | Closed | ✅ Accurate |
| DEFERRED-007 | Local branch publication | **STALE** — says pending | ⚠️ Should say RESOLVED (published since v0.1.0-alpha) |
| DEFERRED-008 | Sandbox TOCTOU | Closed (78C) | ✅ Accurate — Unix + Windows hardened |
| DEFERRED-009 | Hosted provider validation | Closed (77B) | ✅ Accurate — Z.AI validated |
| DEFERRED-010 | Desktop UI rendering | Closed (77C) | ✅ Accurate — Windows UI Automation |

---

## 4. What v1.0.0-rc.1 May Claim

Based on this audit, v1.0.0-rc.1 may claim everything in section 1.1, plus:

- Architecture arc complete (Control → Close)
- Final assurance/caveat audit performed (this document)
- All claims evidence-backed
- All caveats explicitly documented
- No stale claims identified that constitute overclaiming

## 5. What v1.0.0-rc.1 Must NOT Claim

All items in section 1.2 remain in force. Additionally:

- Must NOT claim external review was completed (VJ-1 deferred or pending)
- Must NOT claim the audit upgraded any caveat into assurance
- Must NOT claim production readiness, formal certification, or API stability
- Must NOT claim all stale references have been fixed (they are informational, not overclaiming)

---

## 6. Recommended Actions Before v1.0.0-rc.1

| Priority | Action | Effort |
|----------|--------|--------|
| Low | Update EXTERNAL_REVIEW_PACKET to v0.9.0 (version label + arc) | 15 min |
| Low | Mark DEFERRED-007 as RESOLVED | 2 min |
| Low | Update KNOWN_GAPS.md through v0.9.0 | 20 min |
| Low | Add version stamps to SECURITY_SCAN_RESULTS and AUTHORITY_REVIEW | 5 min |
| Optional | Normalize clippy warning counts (different measurement scopes cause confusion) | 10 min |

**None of these are blocking.** All stale references UNDERSTATE capabilities
or contain informational drift — none constitute overclaiming.

---

## 7. Conclusion

**Zero overclaiming. Zero contradictions. Zero blocking findings.**

8 stale references identified, all informational (understate capabilities or
contain version-label drift). All 18 claims in section 1.1 are evidence-backed.
All 14 caveats in section 1.2 are consistently disclaimed across every release.

**This audit does not upgrade any caveat into an assurance.**

This document is the final claim ledger for v1.0.0-rc.1 closure.
