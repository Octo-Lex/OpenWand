//! Post-v1.0 Operations Ledger guard tests (Wave 134A).

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

// ── Ledger Exists ────────────────────────────────────────────────────────────

#[test]
fn ledger_document_exists() {
    let doc = read_docs_doc("POST_V100_OPERATIONS_LEDGER.md");
    assert!(!doc.is_empty());
}

#[test]
fn ledger_references_wave_134a() {
    let doc = read_docs_doc("POST_V100_OPERATIONS_LEDGER.md");
    assert!(doc.contains("134A"));
}

// ── Released Versions ────────────────────────────────────────────────────────

#[test]
fn ledger_records_v100_and_v101() {
    let doc = read_docs_doc("POST_V100_OPERATIONS_LEDGER.md");
    assert!(doc.contains("v1.0.0") && doc.contains("v1.0.1"));
}

#[test]
fn ledger_records_current_stable_as_v101() {
    let doc = read_docs_doc("POST_V100_OPERATIONS_LEDGER.md");
    assert!(doc.contains("Current Stable"));
    assert!(doc.contains("v1.0.1"));
}

#[test]
fn ledger_records_artifact_sha256() {
    let doc = read_docs_doc("POST_V100_OPERATIONS_LEDGER.md");
    assert!(doc.contains("5ED051CA"));
    assert!(doc.contains("2C6AB04D"));
}

// ── Shipped Fixes ────────────────────────────────────────────────────────────

#[test]
fn ledger_records_shipped_fixes() {
    let doc = read_docs_doc("POST_V100_OPERATIONS_LEDGER.md");
    for fix in &["F-VL1-1", "F-VL1-2", "F-VL1-2b"] {
        assert!(doc.contains(fix), "Ledger must record fix {}", fix);
    }
}

// ── Open Caveats ─────────────────────────────────────────────────────────────

#[test]
fn ledger_lists_all_15_caveats() {
    let doc = read_docs_doc("POST_V100_OPERATIONS_LEDGER.md");
    for i in 1..=15 {
        let caveat = format!("X-{:02}", i);
        assert!(doc.contains(&caveat), "Ledger must list caveat {}", caveat);
    }
}

#[test]
fn ledger_records_caveat_summary() {
    let doc = read_docs_doc("POST_V100_OPERATIONS_LEDGER.md");
    assert!(doc.contains("Partially narrowed") && doc.contains("Deferred"));
}

#[test]
fn ledger_records_zero_caveats_resolved() {
    let doc = read_docs_doc("POST_V100_OPERATIONS_LEDGER.md");
    assert!(
        doc.contains("0 resolved") || doc.contains("Resolved | 0"),
        "Ledger must state 0 caveats resolved"
    );
}

// ── Claims ───────────────────────────────────────────────────────────────────

#[test]
fn ledger_lists_21_claims() {
    let doc = read_docs_doc("POST_V100_OPERATIONS_LEDGER.md");
    // Must reference claim C-21 (the last claim)
    assert!(doc.contains("C-21"), "Ledger must list claims up to C-21");
}

#[test]
fn ledger_states_all_claims_valid() {
    let doc = read_docs_doc("POST_V100_OPERATIONS_LEDGER.md");
    assert!(
        doc.contains("All 21 claims valid") || doc.contains("21 claims"),
        "Ledger must state all claims valid"
    );
}

// ── VL Blocker Status ────────────────────────────────────────────────────────

#[test]
fn ledger_records_vl_status() {
    let doc = read_docs_doc("POST_V100_OPERATIONS_LEDGER.md");
    for vl in &["VL-1", "VL-2", "VL-3", "VL-4", "VL-5", "VL-6"] {
        assert!(doc.contains(vl), "Ledger must record {}", vl);
    }
}

#[test]
fn ledger_records_4_of_6_resolved() {
    let doc = read_docs_doc("POST_V100_OPERATIONS_LEDGER.md");
    assert!(
        doc.contains("4 of 6") || doc.contains("Resolved: 4"),
        "Ledger must state 4 of 6 VL blockers resolved"
    );
}

// ── Deferred Packets ─────────────────────────────────────────────────────────

#[test]
fn ledger_references_resource_blocked_packets() {
    let doc = read_docs_doc("POST_V100_OPERATIONS_LEDGER.md");
    assert!(
        doc.contains("VL_RESOURCE_BLOCKED_PACKETS"),
        "Ledger must reference the resource-blocked packets document"
    );
}

// ── Test Baseline ────────────────────────────────────────────────────────────

#[test]
fn ledger_records_test_baseline() {
    let doc = read_docs_doc("POST_V100_OPERATIONS_LEDGER.md");
    assert!(doc.contains("4,509") && doc.contains("0"));
}

// ── Release Lineage ──────────────────────────────────────────────────────────

#[test]
fn ledger_includes_release_lineage() {
    let doc = read_docs_doc("POST_V100_OPERATIONS_LEDGER.md");
    assert!(doc.contains("Release Lineage") || doc.contains("release lineage"));
}

// ── Future Work Criteria ─────────────────────────────────────────────────────

#[test]
fn ledger_defines_v102_criteria() {
    let doc = read_docs_doc("POST_V100_OPERATIONS_LEDGER.md");
    assert!(doc.contains("v1.0.2") && doc.contains("blocking defect"));
}

#[test]
fn ledger_defines_v110_criteria() {
    let doc = read_docs_doc("POST_V100_OPERATIONS_LEDGER.md");
    assert!(doc.contains("v1.1.0"));
}

#[test]
fn ledger_defines_external_review_trigger() {
    let doc = read_docs_doc("POST_V100_OPERATIONS_LEDGER.md");
    assert!(
        doc.contains("independent reviewer") && doc.contains("available"),
        "Ledger must define when external review can be opened"
    );
}

#[test]
fn ledger_defines_linux_gui_trigger() {
    let doc = read_docs_doc("POST_V100_OPERATIONS_LEDGER.md");
    assert!(
        doc.contains("physical GPU") || doc.contains("GPU passthrough"),
        "Ledger must define when Linux GUI validation can be opened"
    );
}

#[test]
fn ledger_defines_provider_expansion_trigger() {
    let doc = read_docs_doc("POST_V100_OPERATIONS_LEDGER.md");
    assert!(
        doc.contains("API key") && doc.contains("OpenAI") || doc.contains("Ollama"),
        "Ledger must define when provider expansion can be opened"
    );
}

// ── No Overclaiming ─────────────────────────────────────────────────────────

#[test]
fn ledger_does_not_claim_production_ready() {
    let doc = read_docs_doc("POST_V100_OPERATIONS_LEDGER.md");
    for line in doc.lines() {
        let lower = line.to_lowercase();
        if lower.contains("production ready") || lower.contains("production-ready") {
            assert!(
                lower.contains("not ") || lower.contains("does not"),
                "Must not claim production readiness: {}",
                line
            );
        }
    }
}

#[test]
fn ledger_does_not_claim_external_review_executed() {
    let doc = read_docs_doc("POST_V100_OPERATIONS_LEDGER.md");
    assert!(
        doc.contains("Not external review execution"),
        "Ledger must disclaim external review execution"
    );
}
