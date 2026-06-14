//! VK-3 Final Claim Re-audit guard tests (Wave 124A).
//!
//! These tests verify that the final claim re-audit document exists,
//! reconciles all key documents, fixes stale references, adds new claims,
//! and confirms zero overclaiming.

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

fn read_root_doc(name: &str) -> String {
    let p = workspace_root().join(name);
    std::fs::read_to_string(&p)
        .unwrap_or_else(|_| panic!("Failed to read {}", p.display()))
}

// ── Document Existence ──────────────────────────────────────────────────────

#[test]
fn reaudit_document_exists() {
    let doc = read_docs_doc("VK3_FINAL_CLAIM_REAUDIT.md");
    assert!(!doc.is_empty(), "Re-audit document must not be empty");
}

#[test]
fn reaudit_references_wave_124a() {
    let doc = read_docs_doc("VK3_FINAL_CLAIM_REAUDIT.md");
    assert!(doc.contains("124A"), "Re-audit must reference Wave 124A");
}

#[test]
fn reaudit_references_vk3_blocker() {
    let doc = read_docs_doc("VK3_FINAL_CLAIM_REAUDIT.md");
    assert!(doc.contains("VK-3"), "Re-audit must reference VK-3 blocker");
}

// ── Document Reconciliation ─────────────────────────────────────────────────

#[test]
fn reaudit_reconciles_state_md() {
    let doc = read_docs_doc("VK3_FINAL_CLAIM_REAUDIT.md");
    assert!(doc.contains("STATE.md"), "Re-audit must reconcile STATE.md");
}

#[test]
fn reaudit_reconciles_release_notes() {
    let doc = read_docs_doc("VK3_FINAL_CLAIM_REAUDIT.md");
    assert!(
        doc.contains("RELEASE_NOTES"),
        "Re-audit must reconcile release notes"
    );
}

#[test]
fn reaudit_reconciles_external_review_packet() {
    let doc = read_docs_doc("VK3_FINAL_CLAIM_REAUDIT.md");
    assert!(
        doc.contains("EXTERNAL_REVIEW_PACKET"),
        "Re-audit must reconcile external review packet"
    );
}

#[test]
fn reaudit_reconciles_known_gaps() {
    let doc = read_docs_doc("VK3_FINAL_CLAIM_REAUDIT.md");
    assert!(
        doc.contains("KNOWN_GAPS"),
        "Re-audit must reconcile known gaps"
    );
}

#[test]
fn reaudit_reconciles_deferred_risks() {
    let doc = read_docs_doc("VK3_FINAL_CLAIM_REAUDIT.md");
    assert!(
        doc.contains("DEFERRED_RISKS"),
        "Re-audit must reconcile deferred risks"
    );
}

#[test]
fn reaudit_reviews_soak_report() {
    let doc = read_docs_doc("VK3_FINAL_CLAIM_REAUDIT.md");
    assert!(
        doc.contains("RC1_SOAK_REPORT"),
        "Re-audit must review rc.1 soak report"
    );
}

#[test]
fn reaudit_reviews_vk1_classification() {
    let doc = read_docs_doc("VK3_FINAL_CLAIM_REAUDIT.md");
    assert!(
        doc.contains("VK1_EXTERNAL_REVIEW_CLASSIFICATION"),
        "Re-audit must review VK-1 classification"
    );
}

// ── Stale Reference Fixes ───────────────────────────────────────────────────

#[test]
fn external_review_packet_version_updated() {
    let doc = read_docs_doc("EXTERNAL_REVIEW_PACKET.md");
    assert!(
        doc.contains("v1.0.0-rc.1"),
        "EXTERNAL_REVIEW_PACKET must reference v1.0.0-rc.1"
    );
    assert!(
        !doc.contains("**Version:** v0.9.0"),
        "EXTERNAL_REVIEW_PACKET must not still say v0.9.0 as version"
    );
}

#[test]
fn external_review_packet_arc_includes_v1() {
    let doc = read_docs_doc("EXTERNAL_REVIEW_PACKET.md");
    assert!(
        doc.contains("v1.0") && doc.contains("Close"),
        "EXTERNAL_REVIEW_PACKET architecture arc must include v1.0 Close"
    );
}

#[test]
fn external_review_packet_reviewer_version_fixed() {
    let doc = read_docs_doc("EXTERNAL_REVIEW_PACKET.md");
    assert!(
        !doc.contains("# Expected: openwand 0.8.0"),
        "Reviewer checklist must not expect openwand 0.8.0"
    );
}

#[test]
fn security_scan_results_version_updated() {
    let doc = read_docs_doc("SECURITY_SCAN_RESULTS.md");
    assert!(
        doc.contains("v1.0.0-rc.1"),
        "SECURITY_SCAN_RESULTS must reference v1.0.0-rc.1"
    );
}

#[test]
fn authority_review_version_updated() {
    let doc = read_docs_doc("AUTHORITY_REVIEW.md");
    assert!(
        doc.contains("v1.0.0-rc.1"),
        "AUTHORITY_REVIEW must reference v1.0.0-rc.1"
    );
}

#[test]
fn known_gaps_has_124a_stamp() {
    let doc = read_docs_doc("KNOWN_GAPS.md");
    assert!(
        doc.contains("124A"),
        "KNOWN_GAPS must reference Wave 124A"
    );
}

// ── New Claims ──────────────────────────────────────────────────────────────

#[test]
fn reaudit_adds_claim_c19_soak() {
    let doc = read_docs_doc("VK3_FINAL_CLAIM_REAUDIT.md");
    assert!(
        doc.contains("C-19"),
        "Re-audit must add claim C-19 (soak validated)"
    );
}

#[test]
fn reaudit_adds_claim_c20_vk1() {
    let doc = read_docs_doc("VK3_FINAL_CLAIM_REAUDIT.md");
    assert!(
        doc.contains("C-20"),
        "Re-audit must add claim C-20 (VK-1 classified)"
    );
}

#[test]
fn reaudit_adds_claim_c21_version() {
    let doc = read_docs_doc("VK3_FINAL_CLAIM_REAUDIT.md");
    assert!(
        doc.contains("C-21"),
        "Re-audit must add claim C-21 (version string fixed)"
    );
}

#[test]
fn final_audit_has_new_claims() {
    let doc = read_docs_doc("FINAL_ASSURANCE_AUDIT.md");
    assert!(
        doc.contains("C-19") && doc.contains("C-20") && doc.contains("C-21"),
        "FINAL_ASSURANCE_AUDIT must include claims C-19 through C-21"
    );
}

// ── Caveat Updates ──────────────────────────────────────────────────────────

#[test]
fn reaudit_documents_caveat_x15() {
    let doc = read_docs_doc("VK3_FINAL_CLAIM_REAUDIT.md");
    assert!(
        doc.contains("X-15"),
        "Re-audit must document caveat X-15 (external review deferred)"
    );
}

#[test]
fn reaudit_confirms_15_caveats() {
    let doc = read_docs_doc("VK3_FINAL_CLAIM_REAUDIT.md");
    assert!(
        doc.contains("15 caveats"),
        "Re-audit must confirm 15 total caveats"
    );
}

// ── Overclaiming Check ──────────────────────────────────────────────────────

#[test]
fn reaudit_confirms_zero_overclaiming() {
    let doc = read_docs_doc("VK3_FINAL_CLAIM_REAUDIT.md");
    assert!(
        doc.contains("Zero overclaiming"),
        "Re-audit must confirm zero overclaiming"
    );
}

#[test]
fn reaudit_does_not_add_authority() {
    let doc = read_docs_doc("VK3_FINAL_CLAIM_REAUDIT.md");
    assert!(
        !doc.contains("production-ready") || doc.contains("Not production-ready"),
        "Re-audit must not claim production readiness"
    );
}

#[test]
fn reaudit_does_not_claim_external_review_completion() {
    let doc = read_docs_doc("VK3_FINAL_CLAIM_REAUDIT.md");
    // The phrase may appear in negation ("Must NOT claim external review was executed").
    // Check that it never appears in an affirmative context.
    for line in doc.lines() {
        let lower = line.to_lowercase();
        if lower.contains("external review was completed")
            || lower.contains("external review was executed")
        {
            assert!(
                lower.contains("must not") || lower.contains("does not") || lower.contains("not claim"),
                "Re-audit must not affirmatively claim external review was completed/executed: {}",
                line
            );
        }
    }
}

// ── Contradiction Check ─────────────────────────────────────────────────────

#[test]
fn reaudit_documents_test_count_growth() {
    let doc = read_docs_doc("VK3_FINAL_CLAIM_REAUDIT.md");
    assert!(
        doc.contains("4,232") && doc.contains("4,269"),
        "Re-audit must document test count growth from 4,232 to 4,269"
    );
}

#[test]
fn reaudit_documents_sha_drift() {
    let doc = read_docs_doc("VK3_FINAL_CLAIM_REAUDIT.md");
    assert!(
        doc.contains("3F678ACD") && doc.contains("0603647A"),
        "Re-audit must document SHA drift between rc.1 declaration and post-fix"
    );
}

#[test]
fn reaudit_confirms_no_contradictions() {
    let doc = read_docs_doc("VK3_FINAL_CLAIM_REAUDIT.md");
    assert!(
        doc.contains("Zero contradictions"),
        "Re-audit must confirm zero contradictions"
    );
}

// ── VK Status Tracking ──────────────────────────────────────────────────────

#[test]
fn reaudit_shows_vk1_vk2_vk3_resolved() {
    let doc = read_docs_doc("VK3_FINAL_CLAIM_REAUDIT.md");
    assert!(
        doc.contains("VK-1") && doc.contains("VK-2") && doc.contains("VK-3"),
        "Re-audit must show VK-1, VK-2, and VK-3 status"
    );
}

#[test]
fn reaudit_opens_path_to_vk4() {
    let doc = read_docs_doc("VK3_FINAL_CLAIM_REAUDIT.md");
    assert!(
        doc.contains("VK-4") && doc.contains("125A"),
        "Re-audit must open the path to VK-4 (Wave 125A)"
    );
}

#[test]
fn reaudit_does_not_upgrade_caveats() {
    let doc = read_docs_doc("VK3_FINAL_CLAIM_REAUDIT.md");
    assert!(
        doc.contains("does not upgrade any caveat"),
        "Re-audit must state it does not upgrade any caveat into assurance"
    );
}

// ── Release Notes Preservation ──────────────────────────────────────────────

#[test]
fn rc1_release_notes_preserve_historical_test_count() {
    let doc = read_root_doc("RELEASE_NOTES_v100_RC1.md");
    assert!(
        doc.contains("4,232"),
        "rc.1 release notes must preserve historical test count (4,232)"
    );
}

#[test]
fn rc1_release_notes_preserve_historical_sha() {
    let doc = read_root_doc("RELEASE_NOTES_v100_RC1.md");
    assert!(
        doc.contains("3F678ACD"),
        "rc.1 release notes must preserve historical SHA-256"
    );
}
