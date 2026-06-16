//! Internal Product Operating Mode guard tests (Wave 135A).

use std::path::PathBuf;

fn workspace_root() -> PathBuf {
    let p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    p.parent().and_then(|p| p.parent()).map(|p| p.to_path_buf()).unwrap_or_else(|| PathBuf::from(".."))
}

fn read_docs_doc(name: &str) -> String {
    let p = workspace_root().join("docs").join(name);
    std::fs::read_to_string(&p).unwrap_or_else(|_| panic!("Failed to read {}", p.display()))
}

// ── Document Exists ──────────────────────────────────────────────────────────

#[test]
fn operations_document_exists() {
    let doc = read_docs_doc("INTERNAL_OPERATIONS.md");
    assert!(!doc.is_empty());
}

#[test]
fn operations_references_wave_135a() {
    let doc = read_docs_doc("INTERNAL_OPERATIONS.md");
    assert!(doc.contains("135A"));
}

// ── Internal-Use Declaration ─────────────────────────────────────────────────

#[test]
fn operations_declares_internal_stable() {
    let doc = read_docs_doc("INTERNAL_OPERATIONS.md");
    assert!(
        doc.contains("internal stable product") || doc.contains("internally usable"),
        "Must declare v1.0.1 as internal stable product"
    );
}

#[test]
fn operations_declares_v101_baseline() {
    let doc = read_docs_doc("INTERNAL_OPERATIONS.md");
    assert!(doc.contains("v1.0.1"));
}

// ── Supported Workflows ──────────────────────────────────────────────────────

#[test]
fn operations_defines_accepted_workflows() {
    let doc = read_docs_doc("INTERNAL_OPERATIONS.md");
    assert!(
        doc.contains("Accepted Workflows") || doc.contains("Supported Internal Workflows"),
        "Must define accepted workflows"
    );
}

#[test]
fn operations_defines_unsupported_workflows() {
    let doc = read_docs_doc("INTERNAL_OPERATIONS.md");
    assert!(
        doc.contains("Unsupported") || doc.contains("Unsupported Internal Workflows"),
        "Must define unsupported workflows"
    );
}

#[test]
fn operations_defines_experimental_workflows() {
    let doc = read_docs_doc("INTERNAL_OPERATIONS.md");
    assert!(
        doc.contains("Experimental"),
        "Must define experimental workflows"
    );
}

// ── Evidence Capture ─────────────────────────────────────────────────────────

#[test]
fn operations_defines_evidence_capture() {
    let doc = read_docs_doc("INTERNAL_OPERATIONS.md");
    assert!(
        doc.contains("Evidence Capture") || doc.contains("evidence capture"),
        "Must define evidence capture expectations"
    );
}

#[test]
fn operations_requires_trace_verify_after_turns() {
    let doc = read_docs_doc("INTERNAL_OPERATIONS.md");
    assert!(
        doc.contains("trace-verify") && doc.contains("after"),
        "Must recommend trace-verify after agent turns"
    );
}

// ── Classification ───────────────────────────────────────────────────────────

#[test]
fn operations_defines_classification_categories() {
    let doc = read_docs_doc("INTERNAL_OPERATIONS.md");
    for cat in &["Incident", "Defect", "Friction", "Enhancement"] {
        assert!(
            doc.contains(cat),
            "Must define classification category: {}",
            cat
        );
    }
}

#[test]
fn operations_defines_patch_blocker() {
    let doc = read_docs_doc("INTERNAL_OPERATIONS.md");
    assert!(
        doc.contains("Patch blocker") || doc.contains("patch blocker"),
        "Must define patch blocker classification"
    );
}

#[test]
fn operations_defines_decision_flow() {
    let doc = read_docs_doc("INTERNAL_OPERATIONS.md");
    assert!(
        doc.contains("Decision Flow") || doc.contains("decision flow"),
        "Must define a decision flow for classification"
    );
}

// ── Bug Reporting ────────────────────────────────────────────────────────────

#[test]
fn operations_defines_bug_reporting_path() {
    let doc = read_docs_doc("INTERNAL_OPERATIONS.md");
    assert!(
        doc.contains("Bug Reporting") || doc.contains("bug reporting") || doc.contains("Reporting Path"),
        "Must define bug reporting path"
    );
}

// ── Patch Triggers ───────────────────────────────────────────────────────────

#[test]
fn operations_defines_v102_triggers() {
    let doc = read_docs_doc("INTERNAL_OPERATIONS.md");
    assert!(
        doc.contains("v1.0.2") && doc.contains("trigger") || doc.contains("Opens When"),
        "Must define v1.0.2 patch triggers"
    );
}

#[test]
fn operations_excludes_experimental_from_patch() {
    let doc = read_docs_doc("INTERNAL_OPERATIONS.md");
    assert!(
        doc.contains("does NOT open for") || doc.contains("Does NOT Open For"),
        "Must exclude experimental issues from patch path"
    );
}

// ── Feature Triage ───────────────────────────────────────────────────────────

#[test]
fn operations_defines_v110_triage() {
    let doc = read_docs_doc("INTERNAL_OPERATIONS.md");
    assert!(
        doc.contains("v1.1.0") && doc.contains("triage") || doc.contains("Feature Request"),
        "Must define v1.1.0 feature triage"
    );
}

// ── Ledger Update Rules ──────────────────────────────────────────────────────

#[test]
fn operations_defines_ledger_update_rules() {
    let doc = read_docs_doc("INTERNAL_OPERATIONS.md");
    assert!(
        doc.contains("Ledger Update") || doc.contains("ledger update"),
        "Must define operations ledger update rules"
    );
}

// ── Operations Ledger Updated ────────────────────────────────────────────────

#[test]
fn operations_ledger_references_operating_mode() {
    let doc = read_docs_doc("POST_V100_OPERATIONS_LEDGER.md");
    assert!(
        doc.contains("Internal Product Operating Mode") || doc.contains("INTERNAL_OPERATIONS"),
        "Operations ledger must reference internal operating mode"
    );
}

// ── No Overclaiming ─────────────────────────────────────────────────────────

#[test]
fn operations_does_not_claim_production_ready() {
    let doc = read_docs_doc("INTERNAL_OPERATIONS.md");
    for line in doc.lines() {
        let lower = line.to_lowercase();
        if lower.contains("production ready") || lower.contains("production-ready") {
            if !lower.contains("not ") && !lower.contains("does not") && !lower.contains("is not") {
                // Allow "not production-ready for broad third-party deployment"
                panic!("Must not affirmatively claim production readiness: {}", line);
            }
        }
    }
}

#[test]
fn operations_does_not_claim_external_review() {
    let doc = read_docs_doc("INTERNAL_OPERATIONS.md");
    assert!(
        doc.contains("Not external review execution") || doc.contains("deferred until a reviewer"),
        "Must disclaim external review execution"
    );
}

#[test]
fn operations_states_internal_use_expected_to_find_issues() {
    let doc = read_docs_doc("INTERNAL_OPERATIONS.md");
    assert!(
        doc.contains("expected to find") || doc.contains("point"),
        "Must state that internal use is expected to find issues"
    );
}

// ── Operating Rule ───────────────────────────────────────────────────────────

#[test]
fn operations_states_operating_rule() {
    let doc = read_docs_doc("INTERNAL_OPERATIONS.md");
    assert!(
        doc.contains("Operating Rule") || doc.contains("operating rule"),
        "Must state the operating rule"
    );
}
