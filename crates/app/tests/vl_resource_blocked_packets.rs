//! Resource-Blocked Validation Packet guard tests (Wave 133A).

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
fn packet_document_exists() {
    let doc = read_docs_doc("VL_RESOURCE_BLOCKED_PACKETS.md");
    assert!(!doc.is_empty());
}

#[test]
fn packet_references_wave_133a() {
    let doc = read_docs_doc("VL_RESOURCE_BLOCKED_PACKETS.md");
    assert!(doc.contains("133A"));
}

// ── VL-2 External Review Packet ──────────────────────────────────────────────

#[test]
fn vl2_packet_defines_environment() {
    let doc = read_docs_doc("VL_RESOURCE_BLOCKED_PACKETS.md");
    assert!(
        doc.contains("Required Environment") && doc.contains("Reviewer"),
        "VL-2 packet must define required environment"
    );
}

#[test]
fn vl2_packet_defines_commands() {
    let doc = read_docs_doc("VL_RESOURCE_BLOCKED_PACKETS.md");
    assert!(
        doc.contains("git clone") && doc.contains("trace-verify") && doc.contains("evidence-report"),
        "VL-2 packet must include exact commands"
    );
}

#[test]
fn vl2_packet_defines_expected_artifacts() {
    let doc = read_docs_doc("VL_RESOURCE_BLOCKED_PACKETS.md");
    assert!(
        doc.contains("Expected Artifacts") && doc.contains("review_report.json"),
        "VL-2 packet must define expected artifacts"
    );
}

#[test]
fn vl2_packet_defines_classification_criteria() {
    let doc = read_docs_doc("VL_RESOURCE_BLOCKED_PACKETS.md");
    for criterion in &["Pass", "Partial", "Fail", "Blocked"] {
        assert!(
            doc.contains(criterion),
            "VL-2 packet must define {} classification",
            criterion
        );
    }
}

#[test]
fn vl2_packet_defines_evidence_files() {
    let doc = read_docs_doc("VL_RESOURCE_BLOCKED_PACKETS.md");
    assert!(
        doc.contains("Evidence Files to Collect") || doc.contains("Evidence files to collect"),
        "VL-2 packet must define evidence files to collect"
    );
}

#[test]
fn vl2_packet_defines_reviewer_independence() {
    let doc = read_docs_doc("VL_RESOURCE_BLOCKED_PACKETS.md");
    assert!(
        doc.contains("Reviewer Independence") || doc.contains("independent"),
        "VL-2 packet must define reviewer independence criteria"
    );
}

#[test]
fn vl2_packet_states_builder_not_external() {
    let doc = read_docs_doc("VL_RESOURCE_BLOCKED_PACKETS.md");
    assert!(
        doc.contains("Not the project developer") || doc.contains("Not Craft Agent") || doc.contains("not the project builder"),
        "VL-2 packet must state builder is not a valid external reviewer"
    );
}

#[test]
fn vl2_packet_lists_affected_caveats() {
    let doc = read_docs_doc("VL_RESOURCE_BLOCKED_PACKETS.md");
    assert!(
        doc.contains("X-09") && doc.contains("X-15"),
        "VL-2 packet must list affected caveats"
    );
}

// ── VL-5 Linux GUI Packet ────────────────────────────────────────────────────

#[test]
fn vl5_packet_defines_environment() {
    let doc = read_docs_doc("VL_RESOURCE_BLOCKED_PACKETS.md");
    assert!(
        doc.contains("Physical GPU") || doc.contains("physical GPU") || doc.contains("GPU"),
        "VL-5 packet must define GPU requirement"
    );
}

#[test]
fn vl5_packet_defines_commands() {
    let doc = read_docs_doc("VL_RESOURCE_BLOCKED_PACKETS.md");
    assert!(
        doc.contains("openwand-ui") && doc.contains("screenshot"),
        "VL-5 packet must include desktop launch and screenshot commands"
    );
}

#[test]
fn vl5_packet_defines_approved_environments() {
    let doc = read_docs_doc("VL_RESOURCE_BLOCKED_PACKETS.md");
    assert!(
        doc.contains("Approved Environment") || doc.contains("Approved"),
        "VL-5 packet must define approved environment configurations"
    );
}

#[test]
fn vl5_packet_records_what_failed_before() {
    let doc = read_docs_doc("VL_RESOURCE_BLOCKED_PACKETS.md");
    assert!(
        doc.contains("virtio-gpu") || doc.contains("WebKit compositing"),
        "VL-5 packet must document what failed in previous testing"
    );
}

#[test]
fn vl5_packet_defines_interactive_test() {
    let doc = read_docs_doc("VL_RESOURCE_BLOCKED_PACKETS.md");
    assert!(
        doc.contains("Interactive UI") || doc.contains("interactive"),
        "VL-5 packet must define interactive UI test steps"
    );
}

#[test]
fn vl5_packet_lists_affected_caveats() {
    let doc = read_docs_doc("VL_RESOURCE_BLOCKED_PACKETS.md");
    assert!(
        doc.contains("X-05") && doc.contains("X-06"),
        "VL-5 packet must list affected caveats X-05 and X-06"
    );
}

#[test]
fn vl5_packet_defines_classification_criteria() {
    let doc = read_docs_doc("VL_RESOURCE_BLOCKED_PACKETS.md");
    // Must have specific pass criteria for visual rendering
    assert!(
        doc.contains("rendered GUI content") || doc.contains("rendered"),
        "VL-5 packet must define visual rendering pass criteria"
    );
}

// ── State Summary ────────────────────────────────────────────────────────────

#[test]
fn packet_includes_vl_status_summary() {
    let doc = read_docs_doc("VL_RESOURCE_BLOCKED_PACKETS.md");
    assert!(
        doc.contains("VL-1") && doc.contains("VL-6"),
        "Packet must include full VL status summary"
    );
}

#[test]
fn packet_marks_vl2_and_vl5_as_deferred() {
    let doc = read_docs_doc("VL_RESOURCE_BLOCKED_PACKETS.md");
    assert!(
        doc.contains("deferred") && doc.contains("packet ready"),
        "Packet must mark VL-2 and VL-5 as deferred with packets ready"
    );
}

// ── No Overclaiming ─────────────────────────────────────────────────────────

#[test]
fn packet_does_not_claim_vl2_resolved() {
    let doc = read_docs_doc("VL_RESOURCE_BLOCKED_PACKETS.md");
    assert!(
        doc.contains("Does not claim VL-2 resolved"),
        "Packet must disclaim VL-2 resolution"
    );
}

#[test]
fn packet_does_not_claim_vl5_resolved() {
    let doc = read_docs_doc("VL_RESOURCE_BLOCKED_PACKETS.md");
    assert!(
        doc.contains("Does not claim VL-5 resolved"),
        "Packet must disclaim VL-5 resolution"
    );
}

#[test]
fn packet_does_not_claim_production_ready() {
    let doc = read_docs_doc("VL_RESOURCE_BLOCKED_PACKETS.md");
    assert!(
        doc.contains("Does not claim production readiness") || doc.contains("Not production"),
        "Packet must disclaim production readiness"
    );
}

#[test]
fn packet_states_claims_need_evidence() {
    let doc = read_docs_doc("VL_RESOURCE_BLOCKED_PACKETS.md");
    assert!(
        doc.contains("Claims do NOT change without real evidence"),
        "Packet must state claims do not change without evidence"
    );
}
