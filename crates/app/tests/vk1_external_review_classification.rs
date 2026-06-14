//! VK-1 External Review Classification guard tests (Wave 123A).
//!
//! These tests verify that the external review classification document
//! exists, is correctly classified as "Consciously Deferred", and does
//! not claim external review completion.

use std::path::PathBuf;

fn workspace_root() -> PathBuf {
    let p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    p.parent()
        .and_then(|p| p.parent())
        .map(|p| p.to_path_buf())
        .unwrap_or_else(|| PathBuf::from(".."))
}

fn load_doc(name: &str) -> String {
    let p = workspace_root().join("docs").join(name);
    std::fs::read_to_string(&p)
        .unwrap_or_else(|_| panic!("Failed to read {}", p.display()))
}

const CLASSIFICATION_DOC: &str = "VK1_EXTERNAL_REVIEW_CLASSIFICATION.md";
const REVIEW_PACKET_DOC: &str = "EXTERNAL_REVIEW_PACKET.md";

// ── Document Existence ──────────────────────────────────────────────────────

#[test]
fn classification_document_exists() {
    let doc = load_doc(CLASSIFICATION_DOC);
    assert!(!doc.is_empty(), "VK-1 classification document must not be empty");
}

#[test]
fn review_packet_still_exists() {
    // The review packet must still exist — deferral does not mean removal.
    let doc = load_doc(REVIEW_PACKET_DOC);
    assert!(!doc.is_empty(), "External review packet must still exist");
}

#[test]
fn classification_document_has_wave_123a_stamp() {
    let doc = load_doc(CLASSIFICATION_DOC);
    assert!(
        doc.contains("123A"),
        "Classification document must reference Wave 123A"
    );
}

// ── Classification Correctness ──────────────────────────────────────────────

#[test]
fn classification_is_consciously_deferred() {
    let doc = load_doc(CLASSIFICATION_DOC);
    assert!(
        doc.contains("Consciously Deferred"),
        "VK-1 must be classified as 'Consciously Deferred'"
    );
}

#[test]
fn classification_does_not_say_executed() {
    let doc = load_doc(CLASSIFICATION_DOC);
    // The classification matrix must mark Executed as ❌
    assert!(
        doc.contains("❌ No — no external reviewer ran the packet"),
        "VK-1 must explicitly NOT be classified as Executed"
    );
}

#[test]
fn classification_does_not_say_partial() {
    let doc = load_doc(CLASSIFICATION_DOC);
    assert!(
        doc.contains("❌ No — no external review was performed at all"),
        "VK-1 must explicitly NOT be classified as Partial"
    );
}

#[test]
fn classification_does_not_say_blocked() {
    let doc = load_doc(CLASSIFICATION_DOC);
    // Blocked would mean something prevents future review — it doesn't.
    assert!(
        doc.contains("nothing blocks future external review"),
        "VK-1 must explicitly NOT be classified as Blocked"
    );
}

// ── Rationale Requirements ──────────────────────────────────────────────────

#[test]
fn classification_has_deferral_rationale() {
    let doc = load_doc(CLASSIFICATION_DOC);
    assert!(
        doc.contains("Deferral Rationale"),
        "Classification must include deferral rationale"
    );
}

#[test]
fn classification_documents_what_exists() {
    let doc = load_doc(CLASSIFICATION_DOC);
    assert!(
        doc.contains("External Review Packet") && doc.contains("Guided Review Flow"),
        "Classification must document what review infrastructure exists"
    );
}

#[test]
fn classification_documents_what_does_not_exist() {
    let doc = load_doc(CLASSIFICATION_DOC);
    assert!(
        doc.contains("Not available") || doc.contains("Not performed"),
        "Classification must document what does not exist"
    );
}

#[test]
fn classification_acknowledges_self_review_is_not_external() {
    let doc = load_doc(CLASSIFICATION_DOC);
    assert!(
        doc.contains("does not constitute") && doc.contains("external review"),
        "Classification must acknowledge that self-review is not external review"
    );
}

// ── Claim/Caveat Ledger ─────────────────────────────────────────────────────

#[test]
fn classification_updates_caveat_x09() {
    let doc = load_doc(CLASSIFICATION_DOC);
    assert!(
        doc.contains("X-09"),
        "Classification must update caveat X-09 (not externally reviewed)"
    );
}

#[test]
fn classification_adds_caveat_x15() {
    let doc = load_doc(CLASSIFICATION_DOC);
    assert!(
        doc.contains("X-15"),
        "Classification must add caveat X-15 (consciously deferred)"
    );
}

#[test]
fn claim_c11_remains_valid() {
    let doc = load_doc(CLASSIFICATION_DOC);
    assert!(
        doc.contains("C-11"),
        "Classification must reference claim C-11 (review packet available)"
    );
}

// ── No Overclaiming ─────────────────────────────────────────────────────────

#[test]
fn classification_does_not_claim_external_review_completion() {
    let doc = load_doc(CLASSIFICATION_DOC);
    let lower = doc.to_lowercase();
    // Check that "external review completion" only appears in negation context
    if lower.contains("external review completion") {
        // Must be in a "must not claim" or "does not claim" context
        assert!(
            lower.contains("does not claim external review completion")
                || lower.contains("not claim external review completion"),
            "External review completion must only appear in negation context"
        );
    }
}

#[test]
fn classification_does_not_add_authority() {
    let doc = load_doc(CLASSIFICATION_DOC);
    assert!(
        doc.contains("Adds no execution authority"),
        "Classification must state it adds no execution authority"
    );
}

#[test]
fn classification_does_not_upgrade_caveats() {
    let doc = load_doc(CLASSIFICATION_DOC);
    assert!(
        doc.contains("Upgrades no caveat into assurance"),
        "Classification must state it upgrades no caveat into assurance"
    );
}

// ── Release Impact ──────────────────────────────────────────────────────────

#[test]
fn classification_specifies_release_note_requirement() {
    let doc = load_doc(CLASSIFICATION_DOC);
    assert!(
        doc.contains("release notes") || doc.contains("release-notes"),
        "Classification must specify release note requirements for v1.0.0 final"
    );
}

#[test]
fn classification_specifies_final_release_impact() {
    let doc = load_doc(CLASSIFICATION_DOC);
    assert!(
        doc.contains("v1.0.0 final"),
        "Classification must specify impact on v1.0.0 final release"
    );
}

// ── VK Status Tracking ──────────────────────────────────────────────────────

#[test]
fn classification_shows_vk1_and_vk2_resolved() {
    let doc = load_doc(CLASSIFICATION_DOC);
    assert!(
        doc.contains("VK-1") && doc.contains("VK-2"),
        "Classification must show VK-1 and VK-2 status"
    );
}

#[test]
fn classification_opens_path_to_vk3() {
    let doc = load_doc(CLASSIFICATION_DOC);
    assert!(
        doc.contains("VK-3") && doc.contains("124A"),
        "Classification must open the path to VK-3 (Wave 124A)"
    );
}
