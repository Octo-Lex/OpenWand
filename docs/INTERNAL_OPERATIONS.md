# Internal Product Operating Mode — Wave 135A

**Date:** 2026-06-16
**Wave:** 135A
**Commit:** (recorded at lock)
**Tag:** `wave-135a-lock`

---

## Declaration

**OpenWand v1.0.1 is the internal stable product baseline.**

The product is finished enough for internal operation. Active
construction is paused. The project is now in Internal Product
Operating Mode.

This is not a claim of external readiness. It is a declaration that
the product has reached sufficient maturity for real internal use, with
standing caveats documented and accepted.

---

## 1. Internal-Use Classification

```text
OpenWand v1.0.1 is internally usable as a finished, caveated product.

It is not externally certified.
It is not formally audited.
It is not provider-complete.
It is not production-ready for broad third-party deployment.
```

The standing caveats (15 total) do NOT prevent internal use. They define
the boundary of what is and is not claimed.

---

## 2. Supported Internal Workflows

### Accepted Workflows

| Workflow | Commands | Status | Caveats |
|----------|----------|--------|---------|
| Agent turn (single query) | `openwand run "query"` | ✅ Supported | Requires LLM provider |
| Agent turn with explanation | `openwand explain` | ✅ Supported | — |
| Trace integrity verification | `openwand trace-verify` | ✅ Supported | C-03 operational post-v1.0.1 |
| Operation correspondence | `openwand operation-replay` | ✅ Supported | — |
| Checkpoint anchor write | `openwand anchor-write` | ✅ Supported | `--db` honored (v1.0.1) |
| Checkpoint anchor verify | `openwand anchor-verify` | ✅ Supported | — |
| Evidence report generation | `openwand evidence-report` | ✅ Supported | — |
| Guided review flow | `openwand review` | ✅ Supported | — |
| Release readiness check | `openwand release-check` | ✅ Supported | Internal tooling |

### Unsupported Internal Workflows

| Workflow | Status | Reason |
|----------|--------|--------|
| macOS deployment | ❌ Unsupported | Caveat X-10: never tested |
| Linux GUI interactive use | ⚠️ Partial | Caveat X-06: no visual validation |
| External/third-party deployment | ❌ Unsupported | Caveat X-01: not production-ready |
| Custom provider without validation | ⚠️ Adapter-inferred | Caveat X-07: only Z.AI + LM Studio validated |

### Experimental Workflows (use at own risk)

| Workflow | Status | Reason |
|----------|--------|--------|
| Workflow subsystem commands | ⚠️ Experimental | Active design, API not stable |
| `session-rebuild` | ⚠️ Experimental | Prototype functionality |
| `task-plan` | ⚠️ Experimental | Iterating |
| `audit-check` / `audit-packet-*` | ⚠️ Internal | Not for general use |

---

## 3. Evidence Capture Expectations

### What to Capture During Internal Use

| Event | What to Record | Where |
|-------|----------------|------|
| Successful workflow | Session ID, command, provider | Operations ledger |
| Bug / unexpected behavior | Command, input, expected vs actual output, session ID | Incident log |
| Trace verification result | Session ID, result (Pass/Fail), findings | Operations ledger |
| Provider issue | Provider name, model, error message, request/response if possible | Provider log |
| UX friction | What was confusing, what took too long, what was missing | Friction log |

### Evidence Integrity

- Run `trace-verify` after any agent turn to confirm trace integrity
- Use `--db` flag to keep test databases separate from production data
- Run `evidence-report` or `review` for any significant workflow
- Keep generated reports for audit trail

---

## 4. Classification Definitions

| Category | Definition | Action |
|----------|------------|--------|
| **Incident** | Trace verification fails on untampered data, agent loop crashes, security boundary violated, data loss | Immediate investigation → v1.0.x patch candidate |
| **Defect** | Command produces wrong output, wrong exit code, or silent failure on valid input | Filed for next patch |
| **Friction** | UX confusion, missing documentation, poor error messages, slow performance | Filed for UX/docs improvement |
| **Enhancement** | New capability request that doesn't exist yet | Filed as v1.1.0 candidate |
| **Deferred caveat** | Known limitation documented in caveat ledger, not yet resolved | Tracked, no action until resource available |
| **Patch blocker** | Defect that breaks a v1.0.1 claim or core Stable CLI command | Triggers v1.0.2 path |

### Decision Flow

```
Event observed
  → Is trace integrity violated or core command broken?
    YES → Incident → immediate investigation → v1.0.x patch path
    NO → Is the output wrong or exit code incorrect?
      YES → Defect → filed for next patch
      NO → Is it confusing, slow, or missing docs?
        YES → Friction → filed for UX/docs
        NO → Is it a new capability?
          YES → Enhancement → v1.1.0 candidate
          NO → Is it a known caveat?
            YES → Deferred caveat → tracked, no action
            NO → Classify and document
```

---

## 5. Bug Reporting Path

### For Internal Use

1. **Reproduce** — run the command again with the same inputs
2. **Capture** — save console output, exit code, session ID
3. **Verify** — run `trace-verify` on the session
4. **Classify** — incident, defect, friction, or enhancement (see §4)
5. **Record** — add to operations ledger or file as patch candidate

### Information to Include

```
- Command: openwand trace-verify <session> --db <path>
- Version: openwand 1.0.1
- Provider: Z.AI (glm-4.6) / LM Studio / other
- Input: <what was the query or operation>
- Expected: <what should have happened>
- Actual: <what actually happened>
- Exit code: <0/1/2/3/4>
- Session ID: <if applicable>
- Trace-verify result: <Pass/Fail/Inconclusive>
```

---

## 6. Patch Trigger Criteria

### v1.0.2 Opens When

Any of:
- A Stable CLI command (run, explain, trace-verify, operation-replay,
  anchor-write, anchor-verify, review, evidence-report) produces wrong
  output on valid input
- Trace verification fails on untampered fresh data (regression of F-VL1-1)
- `--db` flag is ignored by any Stable command (regression of F-VL1-2)
- A security boundary is violated (sandbox escape, policy bypass)
- A Stable JSON report schema breaks backward compatibility

### v1.0.2 Does NOT Open For

- Experimental command issues (workflow-*, session-rebuild, task-plan)
- Documentation improvements
- New feature requests
- Performance optimizations
- Cosmetic issues

### Patch Process

1. Reproduce the issue
2. Write a regression test that fails
3. Fix the code
4. Confirm regression test passes
5. Run full test suite (0 failures)
6. Bump version to v1.0.2
7. Run release-check (8/8 PASS)
8. Tag and publish

---

## 7. Feature Request Triage

### v1.1.0 Candidate Criteria

A feature request becomes a v1.1.0 candidate when:
- It adds a new capability (not a bug fix)
- It does not break any Stable surface
- It has a clear use case from internal use
- It fits within the API stability policy (additive to Stable, or
  stabilization of an Experimental surface)

### Triage Process

1. **Log** the request with use case and motivation
2. **Classify** as enhancement, friction resolution, or experimental promotion
3. **Assess** API stability impact (does it change a Stable surface?)
4. **Batch** for v1.1.0 scope decision (not opened until intentionally chosen)

### v1.1.0 Does NOT Open Until

The user explicitly chooses to open a v1.1.0 feature scope. Internal use
evidence should accumulate first.

---

## 8. Operations Ledger Update Rules

The operations ledger (`docs/POST_V100_OPERATIONS_LEDGER.md`) is updated:

| Event | Update |
|-------|--------|
| Patch released (v1.0.x) | Add to released versions, add shipped fix, update test baseline |
| Incident resolved | Add to shipped fixes, update caveats if affected |
| Caveat narrowed/resolved | Update caveat status, update claim if affected |
| VL blocker status change | Update VL status table |
| Feature batched for v1.1.0 | Add to v1.1.0 candidate list |
| Test baseline changes | Update test count |

---

## 9. What This Mode Does NOT Claim

- Not external review execution
- Not formal certification
- Not production readiness for third parties
- Not provider completeness
- Not full Linux GUI validation
- Not global API freeze
- Not physical immutability
- Not remote attestation
- Not freedom from defects

Internal use is expected to find issues. That is the point.

---

## 10. Operating Rule

```text
OpenWand v1.0.1 is the internal stable product.
All internal usage goes through the operations ledger.
Bugs become patch candidates.
Friction becomes UX/documentation work.
New capability requests become v1.1 candidates.
External review remains deferred until a reviewer exists.
```
