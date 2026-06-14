# OpenWand — Project State

## Version
0.9.0 (stable) — planning v1.0.0

## Status
**v0.9.0 stable released. v1.0.0 in progress. Wave 118A (final assurance audit) locked.**

Release: v0.9.0 — tag `v0.9.0`

Binary: 17,702,912 bytes (~16.9 MB), SHA-256 `3F678ACD185D8C1A2FD168202C8B76A3140B20A2E1EB684081A591E0A9A4ECF9`

Stable for v0.9.0 milestone scope. Not production-ready. Not formal security review.
No physical immutability. No remote attestation. No stable API guarantee.
No full Linux GUI support. No provider completeness. No external review.

## Workspace Structure
```
crates/
├── core/       Domain IDs, vocabulary, events, snapshots           (lib)
├── trace/      Generic trace substrate (TraceStore<E>)              (lib)
├── store/      Trace+Memory persistence, StoredEvent bridge         (lib)
├── session/    Loro CRDT session + SessionRunner                    (lib)
├── memory/     3-tier memory + ACE Skillbook                        (lib)
├── tools/      ToolExecutor + local tools + composite seam          (lib)
├── mcp-pool/   MCP server pool via rmcp + MockGateway               (lib)
├── policy/     Deterministic trust gate, BuiltinPolicyEngine        (lib)
├── llm/        Provider-normalized LLM boundary, SSE adapter        (lib)
├── skills/     YAML + Markdown skill store                          (lib)
├── goals/      Fitness functions + improvement                      (lib)
├── workflow/   Evidence ladder: 24 capabilities, leaf crate         (lib)
└── app/        CLI binary + desktop UI + evaluation + coordination  (bin)
```

Note: `openwand-content` is a stub crate (add() only). Will be implemented when rich rendering is needed.

## Test Count

**v0.4.0 stable baseline (Wave 90B):** 3,999 tests on Windows, 0 failures.
- 3,939 carried from v0.3.0
- +60 new tests from v0.4.0 arc (88A: +14, 88B: +20, 88C: +16, 89A: +10)

**v0.9.0 stable baseline (Wave 116A):** 4,216 tests on Windows, 0 failures.
- 4,200 carried from v0.8.0
- +16 from v0.9.0 arc (114A: +8, 115A: +8)
- +8 from 118A audit guards

**Clippy posture:** 0 actionable production warnings on 11 non-app crates (HB-G5).
50 app crate pedantic/test-only warnings accepted as cosmetic.

**Desktop feature build:** PASS (0 errors, 0 warnings).

## v1.0.0 Release-Candidate Closure Arc

| Wave | Title | Tag | Deliverable |
|------|-------|-----|-------------|
| 117A | Post-v0.9 Roadmap Reset | `wave-117a-lock` | v1.0.0 roadmap (VJ-1 through VJ-5) |
| 118A | Final Assurance Audit | (this wave) | docs/FINAL_ASSURANCE_AUDIT.md |

## v1.0.0 Blocker Plan

| Blocker | Description | Priority |
|---------|-------------|----------|
| VJ-1: External review execution | Have external reviewer run the packet; classify or defer | P0 (gate) |
| VJ-2: Provider validation decision | Validate or explicitly defer from v1.0 scope | P1 |
| VJ-3: Linux GUI visual decision | Validate or preserve as documented caveat | P1 |
| VJ-4: Final assurance/caveat audit | Reconcile all claims, non-claims, gaps, evidence | P0 (gate) |
| VJ-5: v1.0 release candidate | Create rc.1 with artifact, checklist, caveats | P1 |

## v0.9.0 External Validation Arc

| Wave | Title | Tag | Deliverable |
|------|-------|-----|-------------|
| 113A | Post-v0.8 Roadmap Reset | `wave-113a-lock` | v0.9.0 roadmap (VI-1 through VI-5) |
| 114A | Guided Evidence Flow | `wave-114a-lock` | `openwand review` guided CLI command |
| 115A | Release Automation | `wave-115a-lock` | `openwand release-check` executable readiness checks |
| 116A | v0.9.0 Release Preparation | `wave-116a-lock` | Release artifact, notes, VI reconciliation |
| 116B | v0.9.0 Declaration | `v0.9.0` | Tag v0.9.0, publish release |

## v0.9.0 Blocker Plan

| Blocker | Description | Priority |
|---------|-------------|----------|
| VI-1: External review execution | Deferred — requires external party | P1 (core) |
| VI-2: Evidence report UX integration | ✅ Resolved — `openwand review` guided flow | P1 (core) |
| VI-3: Provider validation expansion | Direct OpenAI/Anthropic/Ollama if strategic | P2 |
| VI-4: Linux GUI visual validation | Move from partial to rendered GUI validation | P2 |
| VI-5: Release automation | ✅ Resolved — `openwand release-check` | P2 |

## v0.8.0 Operational Hardening Arc

| Wave | Title | Tag | Deliverable |
|------|-------|-----|-------------|
| 108A | Post-v0.7 Roadmap Reset | `wave-108a-lock` | v0.8.0 roadmap (VH-1 through VH-5) |
| 109A | Linux GUI Smoke Test | `wave-109a-lock` | Proxmox VM native Linux: desktop binary compiles (9 bugs fixed), window created, rendering not verified |
| 110A | External Review Packet | `wave-110a-lock` | Reviewer-ready packet: commands, exit codes, caveats |
| 111A | Release Process Hardening | `wave-111a-lock` | Repeatable release checklist with openwand-ui build gate correction |
| 112A | v0.8.0 Release Preparation | `wave-112a-lock` | Release artifact, notes, VH reconciliation |
| 112B | v0.8.0 Declaration | `v0.8.0` | Tag v0.8.0, publish release |

## v0.8.0 Blocker Plan

| Blocker | Description | Priority |
|---------|-------------|----------|
| VH-1: Linux GUI runtime validation | ✅ Partial — Proxmox VM, desktop binary compiles, window created | P1 (core) |
| VH-2: Provider validation expansion | Direct OpenAI/Anthropic/Ollama if strategic | P2 |
| VH-3: External review packet | ✅ Resolved — docs/EXTERNAL_REVIEW_PACKET.md | P1 (core) |
| VH-4: Evidence report UX integration | Surface evidence report in desktop or guided CLI | P2 |
| VH-5: Release/process hardening | ✅ Resolved — docs/RELEASE_CHECKLIST.md | P2 |

## v0.7.0 External Assurance Arc

| Wave | Title | Tag | Deliverable |
|------|-------|-----|-------------|
| 103A | Post-v0.6 Roadmap Reset | `wave-103a-lock` | v0.7.0 roadmap (VG-1 through VG-5) |
| 104A | External Anchor Design | `wave-104a-lock` | Anchor DTOs, root-hash computation, verification semantics |
| 104B | CheckpointWriter + Anchor CLI | `wave-104b-lock` | Writer, path containment, CLI commands, integration tests |
| 105A | Automated Security Scanning | `wave-105a-lock` | cargo audit, clippy, authority guards, SECURITY_SCAN_RESULTS.md |
| 105B | Structured Authority Review | `wave-105b-lock` | AUTHORITY_REVIEW.md: 12 surfaces, write-authority map, residual risks |
| 106A | Evidence Report Export | `wave-106a-lock` | Aggregated JSON report: trace verify + operation replay + anchor + scan + authority review |
| 107A | v0.7.0 Release Preparation | `wave-107a-lock` | Release artifact, notes, blocker reconciliation |
| 107B | v0.7.0 Declaration | `v0.7.0` | Tag v0.7.0, publish release |

## v0.7.0 Blocker Plan

| Blocker | Description | Priority |
|---------|-------------|----------|
| VG-1: External anchor/checkpoint | RESOLVED (104A-104B): CheckpointWriter + verify_anchor + CLI | P1 (core) |
| VG-2: Security review execution | RESOLVED (105A-105B): Automated scanning + structured authority review | P1 (core) |
| VG-3: Linux GUI runtime | DEFERRED: Environment-gated; compile-validated only | P2 |
| VG-4: Provider validation expansion | DEFERRED: Strategic; LM Studio + Z.AI validated | P2 |
| VG-5: Evidence UX hardening | RESOLVED (106A): Evidence report export with sourced summaries | P2 |

## v0.6.0 Evidence-Backed Assurance Arc

| Wave | Title | Tag | Deliverable |
|------|-------|-----|-------------|
| 97A | Post-v0.5 Roadmap Reset | `wave-97a-lock` | v0.6.0 roadmap (VF-1 through VF-5) |
| 98A | Hash Verification Policy | `wave-98a-lock` | HashVerificationPolicy trait + Blake3HashPolicy |
| 98B | Hash Recomputation CLI | `wave-98b-lock` | trace-verify with hash correctness checking |
| 99A | Trace-backed Workflow Initiation | `wave-99a-lock` | ModStarted/ModCompleted trace emission |
| 99B | Trace-backed Evidence Export | `wave-99b-lock` | ArtifactGenerated trace emission |
| 101A | TD-93B-1 Module Naming | `wave-101a-lock` | Renamed operation_audit.rs to operation_replay.rs |
| 102A | v0.6.0 Release Preparation | `wave-102a-lock` | Release artifact, notes, blocker reconciliation |
| 102B | v0.6.0 Declaration | `v0.6.0` | Tag v0.6.0, publish release |

## v0.6.0 Blocker Resolution

| Blocker | Status | Resolution |
|---------|--------|------------|
| VF-1: Backend hash-correctness | RESOLVED | 98A-98B: HashVerificationPolicy + CLI integration |
| VF-2: Trace-backed operation coverage | RESOLVED | 99A-99B: Workflow initiation + evidence export trace |
| VF-3: Security review execution | DEFERRED | Post-v0.6 |
| VF-4: Linux GUI runtime | DEFERRED | Environment-gated; compile-validated only |
| VF-5: TD-93B-1 naming | RESOLVED | 101A: Renamed to operation_replay.rs |

## v0.5.0 Runtime Integrity Arc

| Wave | Title | Tag | Deliverable |
|------|-------|-----|-------------|
| 91A | Post-v0.4 Roadmap Reset | `wave-91a-lock` | v0.5.0 roadmap (VE-1 through VE-4) |
| 92A | Trace Verifier Core | `wave-92a-lock` | `TraceVerifier::verify()` - chain continuity, ordering, duplicates |
| 92B | Trace Verifier CLI | `wave-92b-lock` | `openwand trace-verify` with distinct exit codes |
| 93A | Operation Replay | `wave-93a-lock` | `OperationReplayVerifier` - desktop operations to trace correspondence |
| 93B | Operation Replay CLI | `wave-93b-lock` | `openwand operation-replay` with JSON operation descriptors |
| 94A | Security Review Preparation | `wave-94a-lock` | Threat model, authority-boundary checklist, caveat ledger |
| 96A | v0.5.0 Release Preparation | `wave-96a-lock` | Release artifact, notes, blocker reconciliation |
| 96B | v0.5.0 Declaration | `v0.5.0` | Tag v0.5.0, publish release |
| 97A | Post-v0.5 Roadmap Reset | (this wave) | v0.6.0 roadmap, VF-1 through VF-5 proposed |

## v0.5.0 Blocker Resolution

| Blocker | Status | Resolution |
|---------|--------|------------|
| VE-1: Trace verifier | RESOLVED | 92A-92B: TraceVerifier + CLI with tamper detection |
| VE-2: Operation replay | RESOLVED | 93A-93B: OperationReplayVerifier + CLI with correspondence checking |
| VE-3: Linux GUI runtime | DEFERRED | Environment-gated; compile-validated (85A), no display server |
| VE-4: Security review prep | RESOLVED | 94A: Threat model, authority-boundary checklist, caveat ledger |

## v0.4.0 Operation Arc

| Wave | Title | Tag | Deliverable |
|------|-------|-----|-------------|
| 87A | Post-v0.3 Roadmap Reset | `wave-87a-lock` | v0.4.0 roadmap, VD-1/2/3 proposed |
| 88A | Workflow Run Initiation from Desktop | `wave-88a-lock` | Desktop → UiSessionService → execution gate → run saved |
| 88B | Approval Resolution from Desktop | `wave-88b-lock` | Desktop → explicit ARID + decision → SessionRunner → governed path |
| 88C | Evidence Export from Desktop | `wave-88c-lock` | Desktop → UiSessionService → export_audit_packet → validated output |
| 89A | Real-Time Inspector Refresh | `wave-89a-lock` | Auto-refresh after operations + manual refresh button |
| 90B | v0.4.0 Release Preparation | (this wave) | Release artifact, notes, blocker reconciliation |

## v0.4.0 Blocker Resolution

| Blocker | Status | Resolution |
|---------|--------|------------|
| VD-1: Live workflow execution depth | ✅ RESOLVED | 88A-89A: initiate, approve/reject, export, refresh |
| VD-2: Linux GUI runtime validation | DEFERRED | Compile-validated (85A), no display server for runtime |
| VD-3: Trace verifier implementation | DEFERRED to v0.5 | Belongs in runtime integrity hardening theme |

## Release Lineage

```
v0.1.0-alpha -> v0.1.0-beta -> v0.2.0-beta -> v0.2.0-rc.1 -> v0.2.0 -> v0.3.0 -> v0.4.0 -> v0.5.0 -> v0.6.0 -> v0.7.0 -> v0.8.0 -> v0.9.0
```

## Hard Boundaries (Global)
- HB-G1: Binary < 20MB
- HB-G2: Zero telemetry, zero cloud storage dependencies
- HB-G3: All data in `~/.openwand/`
- HB-G4: Zero `unsafe` in OpenWand production code (test-only env var manipulation
  excepted; dependencies may use it; Unix libc openat封装在 WorkspaceWriteHandle)
- HB-G5: `cargo clippy` zero warnings on 11 non-app production crates.
  `openwand-app` test-module style warnings accepted as cosmetic.
