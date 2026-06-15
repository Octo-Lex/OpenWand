//! VK-5 v1.0.0 Final Declaration guard tests (Wave 125B).
//!
//! These tests verify that v1.0.0 is properly declared, tagged, and
//! documented with all claims, caveats, and non-claims preserved.

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

// ── Declaration Document ────────────────────────────────────────────────────

#[test]
fn declaration_document_exists() {
    let doc = read_docs_doc("V100_FINAL_DECLARATION.md");
    assert!(!doc.is_empty(), "Final declaration document must exist");
}

#[test]
fn declaration_references_wave_125b() {
    let doc = read_docs_doc("V100_FINAL_DECLARATION.md");
    assert!(doc.contains("125B"), "Declaration must reference Wave 125B");
}

#[test]
fn declaration_references_vk5() {
    let doc = read_docs_doc("V100_FINAL_DECLARATION.md");
    assert!(doc.contains("VK-5"), "Declaration must reference VK-5");
}

#[test]
fn declaration_states_v100_released() {
    let doc = read_docs_doc("V100_FINAL_DECLARATION.md");
    assert!(
        doc.contains("v1.0.0") && doc.contains("declared"),
        "Declaration must state v1.0.0 is declared"
    );
}

// ── VK Blocker Closure ──────────────────────────────────────────────────────

#[test]
fn declaration_shows_all_vk_resolved() {
    let doc = read_docs_doc("V100_FINAL_DECLARATION.md");
    assert!(
        doc.contains("VK-1") && doc.contains("VK-2") && doc.contains("VK-3")
            && doc.contains("VK-4") && doc.contains("VK-5"),
        "Declaration must show all VK blockers"
    );
    assert!(
        doc.contains("All VK blockers resolved"),
        "Declaration must state all VK blockers are resolved"
    );
}

// ── Artifact Identity ───────────────────────────────────────────────────────

#[test]
fn declaration_records_cli_sha() {
    let doc = read_docs_doc("V100_FINAL_DECLARATION.md");
    assert!(
        doc.contains("AE2DBB1B5D37D4F1833998A5047256CB47BB1D9F0C3CACB493D19C148BC7EA46"),
        "Declaration must record CLI SHA-256"
    );
}

#[test]
fn declaration_records_desktop_sha() {
    let doc = read_docs_doc("V100_FINAL_DECLARATION.md");
    assert!(
        doc.contains("04B696B533602C196808213A2B70DB6FEAD4A61C2A9F64B598208C8A4CFF4DF4"),
        "Declaration must record desktop SHA-256"
    );
}

#[test]
fn declaration_records_test_count() {
    let doc = read_docs_doc("V100_FINAL_DECLARATION.md");
    assert!(
        doc.contains("4,325"),
        "Declaration must record final test count"
    );
}

#[test]
fn declaration_records_version_string() {
    let doc = read_docs_doc("V100_FINAL_DECLARATION.md");
    assert!(
        doc.contains("openwand 1.0.0"),
        "Declaration must record version string 'openwand 1.0.0'"
    );
}

// ── Claims ──────────────────────────────────────────────────────────────────

#[test]
fn declaration_lists_21_claims() {
    let doc = read_docs_doc("V100_FINAL_DECLARATION.md");
    // Must list through C-21
    assert!(
        doc.contains("C-21"),
        "Declaration must list all 21 claims"
    );
}

#[test]
fn declaration_lists_15_caveats() {
    let doc = read_docs_doc("V100_FINAL_DECLARATION.md");
    assert!(
        doc.contains("X-15"),
        "Declaration must list all 15 caveats through X-15"
    );
}

// ── Non-Claims Preserved ────────────────────────────────────────────────────

#[test]
fn declaration_preserves_not_production_ready() {
    let doc = read_docs_doc("V100_FINAL_DECLARATION.md");
    assert!(doc.contains("Not production-ready"), "Must preserve: not production-ready");
}

#[test]
fn declaration_preserves_not_externally_reviewed() {
    let doc = read_docs_doc("V100_FINAL_DECLARATION.md");
    assert!(doc.contains("Not externally reviewed"), "Must preserve: not externally reviewed");
}

#[test]
fn declaration_preserves_not_provider_complete() {
    let doc = read_docs_doc("V100_FINAL_DECLARATION.md");
    assert!(doc.contains("Not provider completeness"), "Must preserve: not provider complete");
}

#[test]
fn declaration_preserves_not_stable_api() {
    let doc = read_docs_doc("V100_FINAL_DECLARATION.md");
    assert!(doc.contains("Not stable API"), "Must preserve: not stable API");
}

#[test]
fn declaration_preserves_not_physical_immutability() {
    let doc = read_docs_doc("V100_FINAL_DECLARATION.md");
    assert!(doc.contains("Not physical immutability"), "Must preserve: not physical immutability");
}

#[test]
fn declaration_preserves_not_remote_attestation() {
    let doc = read_docs_doc("V100_FINAL_DECLARATION.md");
    assert!(doc.contains("Not remote attestation"), "Must preserve: not remote attestation");
}

// ── No Overclaiming ─────────────────────────────────────────────────────────

#[test]
fn declaration_does_not_claim_production_ready() {
    let doc = read_docs_doc("V100_FINAL_DECLARATION.md");
    for line in doc.lines() {
        let lower = line.to_lowercase();
        if lower.contains("production-ready") || lower.contains("production ready") {
            assert!(
                lower.contains("not ") || lower.contains("does not") || lower.contains("requires"),
                "Must not affirmatively claim production-ready: {}",
                line
            );
        }
    }
}

#[test]
fn declaration_does_not_claim_external_review_completion() {
    let doc = read_docs_doc("V100_FINAL_DECLARATION.md");
    for line in doc.lines() {
        let lower = line.to_lowercase();
        if lower.contains("external review was completed")
            || lower.contains("external review was executed")
        {
            assert!(
                lower.contains("does not") || lower.contains("not claim"),
                "Must not affirmatively claim external review: {}",
                line
            );
        }
    }
}

#[test]
fn declaration_does_not_claim_formal_certification() {
    let doc = read_docs_doc("V100_FINAL_DECLARATION.md");
    // Check in context: if a "does not:" or "does\nnot:" precedes a list,
    // each bulleted item is negated by the block context.
    let lines: Vec<&str> = doc.lines().collect();
    let mut in_negation_block = false;
    for line in lines {
        let lower = line.to_lowercase();
        // Detect negation block headers ("does not:", "does\nnot:", or "does not claim")
        if lower.contains("does not")
            || lower.contains("does") && lower.trim() == "does"
            || lower.trim() == "not:"
            || lower.contains("must not")
        {
            in_negation_block = true;
        }
        if lower.contains("formal certification") || lower.contains("formally certified") {
            assert!(
                lower.contains("not ") || lower.contains("does not")
                    || in_negation_block,
                "Must not affirmatively claim formal certification: {}",
                line
            );
        }
        // Reset negation block when we hit a non-list line (not starting with - or *)
        if !lower.trim_start().starts_with('-')
            && !lower.trim_start().starts_with('*')
            && !lower.trim().is_empty()
            && !lower.contains("does")
            && !lower.contains("not:")
        {
            in_negation_block = false;
        }
    }
}

// ── Architecture Arc ────────────────────────────────────────────────────────

#[test]
fn declaration_shows_complete_arc() {
    let doc = read_docs_doc("V100_FINAL_DECLARATION.md");
    assert!(
        doc.contains("Control") && doc.contains("Close"),
        "Declaration must show architecture arc from Control to Close"
    );
}

// ── STATE.md Final State ────────────────────────────────────────────────────

#[test]
fn state_md_shows_v100_released() {
    let doc = read_root_doc("STATE.md");
    assert!(
        doc.contains("1.0.0") && (doc.contains("RELEASED") || doc.contains("released")),
        "STATE.md must show v1.0.0 as released"
    );
}

#[test]
fn state_md_shows_wave_125b() {
    let doc = read_root_doc("STATE.md");
    assert!(
        doc.contains("125B"),
        "STATE.md must reference Wave 125B"
    );
}

#[test]
fn state_md_shows_vk5_resolved() {
    let doc = read_root_doc("STATE.md");
    // VK-5 should now show resolved
    assert!(
        doc.contains("VK-5") && doc.contains("Resolved"),
        "STATE.md must show VK-5 as resolved"
    );
}

// ── Cargo.toml ──────────────────────────────────────────────────────────────

#[test]
fn cargo_toml_version_at_least_100() {
    let doc = read_root_doc("Cargo.toml");
    assert!(
        doc.contains("version = \"1.0.0\"") || doc.contains("version = \"1.0.1\""),
        "Cargo.toml must have version 1.0.0 or later"
    );
}

// ── Release Notes ───────────────────────────────────────────────────────────

#[test]
fn stable_release_notes_exist() {
    let doc = read_root_doc("RELEASE_NOTES_v100_STABLE.md");
    assert!(!doc.is_empty(), "v1.0.0 stable release notes must exist");
}

// ── Post-v1.0.0 Roadmap ─────────────────────────────────────────────────────

#[test]
fn declaration_has_post_v100_roadmap() {
    let doc = read_docs_doc("V100_FINAL_DECLARATION.md");
    assert!(
        doc.contains("Post-v1.0.0"),
        "Declaration must include post-v1.0.0 roadmap"
    );
}
