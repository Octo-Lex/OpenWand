//! VL-1 First Real Workflow Evidence guard tests (Wave 127A).
//!
//! These tests verify that the first workflow evidence document exists,
//! records the workflow outcome, classifies findings, and preserves caveats.

use std::path::PathBuf;

fn workspace_root() -> PathBuf {
    let p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    p.parent()
        .and_then(|p| p.parent())
        .map(|p| p.to_path_buf())
        .unwrap_or_else(|| PathBuf::from(".."))
}

fn read_docs_doc(name: &str) -> String {
    let p = workspace_root().join("docs").join(name);
    std::fs::read_to_string(&p)
        .unwrap_or_else(|_| panic!("Failed to read {}", p.display()))
}

// ── Document Existence ──────────────────────────────────────────────────────

#[test]
fn evidence_document_exists() {
    let doc = read_docs_doc("VL1_FIRST_WORKFLOW_EVIDENCE.md");
    assert!(!doc.is_empty(), "VL-1 evidence document must not be empty");
}

#[test]
fn evidence_references_wave_127a() {
    let doc = read_docs_doc("VL1_FIRST_WORKFLOW_EVIDENCE.md");
    assert!(doc.contains("127A"), "Evidence must reference Wave 127A");
}

// ── Workflow Classification ─────────────────────────────────────────────────

#[test]
fn evidence_classifies_outcome() {
    let doc = read_docs_doc("VL1_FIRST_WORKFLOW_EVIDENCE.md");
    assert!(
        doc.contains("PARTIAL") || doc.contains("Partial"),
        "Evidence must classify workflow outcome (partial)"
    );
}

// ── Findings Recorded ───────────────────────────────────────────────────────

#[test]
fn evidence_records_f_vl1_1_hash_failure() {
    let doc = read_docs_doc("VL1_FIRST_WORKFLOW_EVIDENCE.md");
    assert!(
        doc.contains("F-VL1-1"),
        "Evidence must record F-VL1-1 (trace hash verification failure)"
    );
}

#[test]
fn evidence_records_f_vl1_1_as_blocking() {
    let doc = read_docs_doc("VL1_FIRST_WORKFLOW_EVIDENCE.md");
    assert!(
        doc.contains("F-VL1-1") && doc.contains("BLOCKING") || doc.contains("Blocking"),
        "Evidence must classify F-VL1-1 as blocking"
    );
}

#[test]
fn evidence_records_f_vl1_2_db_flag_bug() {
    let doc = read_docs_doc("VL1_FIRST_WORKFLOW_EVIDENCE.md");
    assert!(
        doc.contains("F-VL1-2"),
        "Evidence must record F-VL1-2 (--db flag ignored)"
    );
}

#[test]
fn evidence_describes_hash_mismatch() {
    let doc = read_docs_doc("VL1_FIRST_WORKFLOW_EVIDENCE.md");
    assert!(
        doc.contains("entry_hash") && doc.contains("does not match"),
        "Evidence must describe the hash mismatch"
    );
}

// ── Evidence Chain Steps ────────────────────────────────────────────────────

#[test]
fn evidence_records_trace_verification_result() {
    let doc = read_docs_doc("VL1_FIRST_WORKFLOW_EVIDENCE.md");
    assert!(
        doc.contains("Trace Verification") || doc.contains("trace-verify"),
        "Evidence must record trace verification result"
    );
}

#[test]
fn evidence_records_anchor_operations() {
    let doc = read_docs_doc("VL1_FIRST_WORKFLOW_EVIDENCE.md");
    assert!(
        doc.contains("anchor-write") && doc.contains("anchor-verify"),
        "Evidence must record anchor operations"
    );
}

#[test]
fn evidence_records_review_flow() {
    let doc = read_docs_doc("VL1_FIRST_WORKFLOW_EVIDENCE.md");
    assert!(
        doc.contains("Guided Review") || doc.contains("review"),
        "Evidence must record guided review flow execution"
    );
}

#[test]
fn evidence_records_version_identity() {
    let doc = read_docs_doc("VL1_FIRST_WORKFLOW_EVIDENCE.md");
    assert!(
        doc.contains("1.0.0"),
        "Evidence must record version identity"
    );
}

// ── User-Facing Friction ────────────────────────────────────────────────────

#[test]
fn evidence_records_user_friction() {
    let doc = read_docs_doc("VL1_FIRST_WORKFLOW_EVIDENCE.md");
    assert!(
        doc.contains("friction") || doc.contains("Friction"),
        "Evidence must record user-facing friction"
    );
}

// ── Impact on Claims ────────────────────────────────────────────────────────

#[test]
fn evidence_documents_claim_impact() {
    let doc = read_docs_doc("VL1_FIRST_WORKFLOW_EVIDENCE.md");
    assert!(
        doc.contains("C-02") && doc.contains("C-03"),
        "Evidence must document impact on claims C-02 and C-03"
    );
}

// ── No Overclaiming ─────────────────────────────────────────────────────────

#[test]
fn evidence_does_not_claim_production_ready() {
    let doc = read_docs_doc("VL1_FIRST_WORKFLOW_EVIDENCE.md");
    for line in doc.lines() {
        let lower = line.to_lowercase();
        if lower.contains("production-ready") || lower.contains("production ready") {
            assert!(
                lower.contains("not ") || lower.contains("does not"),
                "Must not affirmatively claim production-ready: {}",
                line
            );
        }
    }
}

#[test]
fn evidence_does_not_resolve_caveats() {
    let doc = read_docs_doc("VL1_FIRST_WORKFLOW_EVIDENCE.md");
    assert!(
        doc.contains("No claim is upgraded") || doc.contains("No caveat is resolved")
            || doc.contains("does not resolve"),
        "Evidence must state no caveat is resolved"
    );
}

// ── VL Status ───────────────────────────────────────────────────────────────

#[test]
fn evidence_shows_vl1_partial() {
    let doc = read_docs_doc("VL1_FIRST_WORKFLOW_EVIDENCE.md");
    assert!(
        doc.contains("VL-1") && doc.contains("PARTIAL"),
        "Evidence must show VL-1 as Partial"
    );
}

#[test]
fn evidence_recommends_vl3_urgent() {
    let doc = read_docs_doc("VL1_FIRST_WORKFLOW_EVIDENCE.md");
    assert!(
        doc.contains("VL-3") && (doc.contains("urgent") || doc.contains("Urgent") || doc.contains("patch")),
        "Evidence must recommend VL-3 (patch criteria) as now urgent"
    );
}

// ── Recommendation ──────────────────────────────────────────────────────────

#[test]
fn evidence_recommends_fixing_f_vl1_1() {
    let doc = read_docs_doc("VL1_FIRST_WORKFLOW_EVIDENCE.md");
    assert!(
        doc.contains("Fix") && doc.contains("F-VL1-1"),
        "Evidence must recommend fixing F-VL1-1"
    );
}
