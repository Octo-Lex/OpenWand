//! Post-v1.0 Stabilization Roadmap guard tests (Wave 126A).
//!
//! These tests verify that the post-v1.0 roadmap exists, defines the VL
//! blocker series, preserves all v1.0 caveats, and does not overclaim.

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
fn roadmap_document_exists() {
    let doc = read_docs_doc("V100_POST_STABILIZATION_ROADMAP.md");
    assert!(!doc.is_empty(), "Post-v1.0 roadmap must not be empty");
}

#[test]
fn roadmap_references_wave_126a() {
    let doc = read_docs_doc("V100_POST_STABILIZATION_ROADMAP.md");
    assert!(doc.contains("126A"), "Roadmap must reference Wave 126A");
}

#[test]
fn roadmap_has_theme() {
    let doc = read_docs_doc("V100_POST_STABILIZATION_ROADMAP.md");
    assert!(
        doc.contains("Operate") && doc.contains("observe") && doc.contains("harden"),
        "Roadmap must state the stabilization theme"
    );
}

// ── VL Blocker Series ───────────────────────────────────────────────────────

#[test]
fn roadmap_defines_vl1_user_adoption() {
    let doc = read_docs_doc("V100_POST_STABILIZATION_ROADMAP.md");
    assert!(
        doc.contains("VL-1") && doc.contains("User Adoption") || doc.contains("adoption"),
        "Roadmap must define VL-1 (user adoption)"
    );
}

#[test]
fn roadmap_defines_vl2_external_review() {
    let doc = read_docs_doc("V100_POST_STABILIZATION_ROADMAP.md");
    assert!(
        doc.contains("VL-2") && doc.contains("External Review"),
        "Roadmap must define VL-2 (external review)"
    );
}

#[test]
fn roadmap_defines_vl3_maintenance() {
    let doc = read_docs_doc("V100_POST_STABILIZATION_ROADMAP.md");
    assert!(
        doc.contains("VL-3") && doc.contains("maintenance"),
        "Roadmap must define VL-3 (maintenance patch criteria)"
    );
}

#[test]
fn roadmap_defines_vl4_provider() {
    let doc = read_docs_doc("V100_POST_STABILIZATION_ROADMAP.md");
    assert!(
        doc.contains("VL-4") && doc.contains("Provider"),
        "Roadmap must define VL-4 (provider expansion)"
    );
}

#[test]
fn roadmap_defines_vl5_linux_gui() {
    let doc = read_docs_doc("V100_POST_STABILIZATION_ROADMAP.md");
    assert!(
        doc.contains("VL-5") && doc.contains("Linux GUI"),
        "Roadmap must define VL-5 (Linux GUI validation)"
    );
}

#[test]
fn roadmap_defines_vl6_api_stability() {
    let doc = read_docs_doc("V100_POST_STABILIZATION_ROADMAP.md");
    assert!(
        doc.contains("VL-6") && doc.contains("API"),
        "Roadmap must define VL-6 (API stability policy)"
    );
}

// ── VL-1 as Gate ────────────────────────────────────────────────────────────

#[test]
fn roadmap_identifies_vl1_as_gate() {
    let doc = read_docs_doc("V100_POST_STABILIZATION_ROADMAP.md");
    assert!(
        doc.contains("VL-1") && doc.contains("gate"),
        "Roadmap must identify VL-1 as the gating blocker"
    );
}

// ── Strategic Shift ─────────────────────────────────────────────────────────

#[test]
fn roadmap_documents_strategic_shift() {
    let doc = read_docs_doc("V100_POST_STABILIZATION_ROADMAP.md");
    assert!(
        doc.contains("builder loop") || doc.contains("outside the builder"),
        "Roadmap must document the strategic shift from builder to external"
    );
}

// ── Caveat Preservation ─────────────────────────────────────────────────────

#[test]
fn roadmap_preserves_v1_caveats() {
    let doc = read_docs_doc("V100_POST_STABILIZATION_ROADMAP.md");
    assert!(
        doc.contains("15") && (doc.contains("caveat") || doc.contains("Caveat")),
        "Roadmap must acknowledge 15 carried caveats"
    );
}

#[test]
fn roadmap_maps_vl_to_caveats() {
    let doc = read_docs_doc("V100_POST_STABILIZATION_ROADMAP.md");
    assert!(
        doc.contains("X-09") || doc.contains("X-07") || doc.contains("X-08"),
        "Roadmap must map VL blockers to specific caveats"
    );
}

#[test]
fn roadmap_does_not_resolve_caveats_prematurely() {
    let doc = read_docs_doc("V100_POST_STABILIZATION_ROADMAP.md");
    assert!(
        doc.contains("No caveat is resolved") || doc.contains("not resolved until"),
        "Roadmap must state no caveat is resolved until VL evidence exists"
    );
}

// ── No Overclaiming ─────────────────────────────────────────────────────────

#[test]
fn roadmap_does_not_claim_production_ready() {
    let doc = read_docs_doc("V100_POST_STABILIZATION_ROADMAP.md");
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
fn roadmap_does_not_add_authority() {
    let doc = read_docs_doc("V100_POST_STABILIZATION_ROADMAP.md");
    let lower = doc.to_lowercase();
    assert!(
        lower.contains("does not add execution authority"),
        "Roadmap must state it adds no execution authority"
    );
}

#[test]
fn roadmap_does_not_upgrade_caveats() {
    let doc = read_docs_doc("V100_POST_STABILIZATION_ROADMAP.md");
    let lower = doc.to_lowercase();
    assert!(
        lower.contains("does not upgrade"),
        "Roadmap must state it does not upgrade caveats"
    );
}

// ── Wave Sequence ───────────────────────────────────────────────────────────

#[test]
fn roadmap_proposes_wave_sequence() {
    let doc = read_docs_doc("V100_POST_STABILIZATION_ROADMAP.md");
    assert!(
        doc.contains("127A"),
        "Roadmap must propose Wave 127A as next"
    );
}

// ── STATE.md Consistency ────────────────────────────────────────────────────

#[test]
fn state_md_references_v100_released() {
    let doc = read_root_doc("STATE.md");
    assert!(
        doc.contains("v1.0.0") && doc.contains("RELEASED"),
        "STATE.md must show v1.0.0 as released"
    );
}
