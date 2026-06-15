//! VL-2 External Review Reclassification guard tests (Wave 130A).

use std::path::PathBuf;

fn workspace_root() -> PathBuf {
    let p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    p.parent().and_then(|p| p.parent()).map(|p| p.to_path_buf()).unwrap_or_else(|| PathBuf::from(".."))
}

fn read_docs_doc(name: &str) -> String {
    let p = workspace_root().join("docs").join(name);
    std::fs::read_to_string(&p).unwrap_or_else(|_| panic!("Failed to read {}", p.display()))
}

fn read_root_doc(name: &str) -> String {
    let p = workspace_root().join(name);
    std::fs::read_to_string(&p).unwrap_or_else(|_| panic!("Failed to read {}", p.display()))
}

// ── Reclassification Document ────────────────────────────────────────────────

#[test]
fn reclassification_document_exists() {
    let doc = read_docs_doc("VL2_EXTERNAL_REVIEW_RECLASSIFICATION.md");
    assert!(!doc.is_empty());
}

#[test]
fn reclassification_references_wave_130a() {
    let doc = read_docs_doc("VL2_EXTERNAL_REVIEW_RECLASSIFICATION.md");
    assert!(doc.contains("130A"));
}

#[test]
fn reclassification_classifies_as_deferred() {
    let doc = read_docs_doc("VL2_EXTERNAL_REVIEW_RECLASSIFICATION.md");
    assert!(doc.contains("Consciously Deferred"));
}

#[test]
fn reclassification_records_no_external_reviewer() {
    let doc = read_docs_doc("VL2_EXTERNAL_REVIEW_RECLASSIFICATION.md");
    assert!(doc.contains("Not available") || doc.contains("not available"));
}

#[test]
fn reclassification_records_what_changed_since_vk1() {
    let doc = read_docs_doc("VL2_EXTERNAL_REVIEW_RECLASSIFICATION.md");
    // Must acknowledge the VL-1 validation improved the context
    assert!(doc.contains("VL-1") && doc.contains("123A"));
}

#[test]
fn reclassification_records_builder_is_not_external() {
    let doc = read_docs_doc("VL2_EXTERNAL_REVIEW_RECLASSIFICATION.md");
    assert!(
        doc.contains("Builder Is Not External") || doc.contains("not external") || doc.contains("Builder is not"),
        "Must explicitly state builder is not external"
    );
}

// ── Review Packet Updated ────────────────────────────────────────────────────

#[test]
fn review_packet_references_v101() {
    let doc = read_docs_doc("EXTERNAL_REVIEW_PACKET.md");
    assert!(doc.contains("v1.0.1") || doc.contains("1.0.1"));
}

#[test]
fn review_packet_includes_sha256() {
    let doc = read_docs_doc("EXTERNAL_REVIEW_PACKET.md");
    assert!(doc.contains("5ED051CAFF"));
    assert!(doc.contains("2C6AB04D42"));
}

#[test]
fn review_packet_includes_db_flag() {
    let doc = read_docs_doc("EXTERNAL_REVIEW_PACKET.md");
    assert!(doc.contains("--db"));
}

#[test]
fn review_packet_records_f_vl1_1_fix() {
    let doc = read_docs_doc("EXTERNAL_REVIEW_PACKET.md");
    assert!(doc.contains("F-VL1-1") || doc.contains("serde_json"));
}

#[test]
fn review_packet_has_reviewer_checklist() {
    let doc = read_docs_doc("EXTERNAL_REVIEW_PACKET.md");
    assert!(doc.contains("Quick Reviewer Checklist") || doc.contains("Reviewer Checklist"));
}

// ── Caveat Preservation ──────────────────────────────────────────────────────

#[test]
fn x09_remains_active() {
    let doc = read_docs_doc("VL2_EXTERNAL_REVIEW_RECLASSIFICATION.md");
    assert!(doc.contains("X-09") && doc.contains("Still Deferred") || doc.contains("active"));
}

#[test]
fn x15_remains_active() {
    let doc = read_docs_doc("VL2_EXTERNAL_REVIEW_RECLASSIFICATION.md");
    assert!(doc.contains("X-15"));
}

// ── No Overclaiming ─────────────────────────────────────────────────────────

#[test]
fn reclassification_does_not_claim_certification() {
    let doc = read_docs_doc("VL2_EXTERNAL_REVIEW_RECLASSIFICATION.md");
    for line in doc.lines() {
        let lower = line.to_lowercase();
        if (lower.contains("certification") || lower.contains("security audit"))
            && !lower.contains("not ")
            && !lower.contains("does not")
            && !lower.contains("doesn't")
        {
            // Allow in "What This Classification Does NOT Claim" section
            if !line.contains("NOT") && !line.contains("Does NOT") {
                panic!("Must not affirmatively claim certification or audit: {}", line);
            }
        }
    }
}

#[test]
fn reclassification_does_not_claim_external_review_executed() {
    let doc = read_docs_doc("VL2_EXTERNAL_REVIEW_RECLASSIFICATION.md");
    assert!(
        doc.contains("No external reviewer has executed"),
        "Must state no external reviewer has executed"
    );
}

#[test]
fn reclassification_provides_resolution_path() {
    let doc = read_docs_doc("VL2_EXTERNAL_REVIEW_RECLASSIFICATION.md");
    assert!(
        doc.contains("Path To Resolution") || doc.contains("path to resolution"),
        "Must provide a path to resolution"
    );
}

// ── VL-1 evidence referenced ────────────────────────────────────────────────

#[test]
fn reclassification_acknowledges_vl1_postfix() {
    let doc = read_docs_doc("VL2_EXTERNAL_REVIEW_RECLASSIFICATION.md");
    assert!(
        doc.contains("PASS") || doc.contains("pass"),
        "Must acknowledge VL-1 post-fix pass"
    );
}
