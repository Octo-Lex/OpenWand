//! VK-4 v1.0.0 Final Preparation guard tests (Wave 125A).
//!
//! These tests verify that the v1.0.0 final release package is prepared:
//! release notes exist, artifact identities are recorded, version string
//! is correct, VK blockers are reconciled, and all caveats are preserved.
//!
//! Critically: these tests verify preparation. They do NOT verify declaration
//! (that's Wave 125B / VK-5).

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

// ── Release Notes Existence ─────────────────────────────────────────────────

#[test]
fn stable_release_notes_exist() {
    let doc = read_root_doc("RELEASE_NOTES_v100_STABLE.md");
    assert!(!doc.is_empty(), "v1.0.0 stable release notes must exist");
}

#[test]
fn stable_release_notes_reference_wave_125a() {
    let doc = read_root_doc("RELEASE_NOTES_v100_STABLE.md");
    assert!(
        doc.contains("125A"),
        "Release notes must reference Wave 125A"
    );
}

#[test]
fn stable_release_notes_do_not_declare_v100() {
    let doc = read_root_doc("RELEASE_NOTES_v100_STABLE.md");
    // The release notes say "pending declaration" — they don't declare
    assert!(
        doc.contains("pending") || doc.contains("Pending"),
        "Release notes must not declare v1.0.0 — it must be pending"
    );
}

// ── Artifact Identity ───────────────────────────────────────────────────────

#[test]
fn stable_release_notes_record_cli_sha() {
    let doc = read_root_doc("RELEASE_NOTES_v100_STABLE.md");
    assert!(
        doc.contains("AE2DBB1B5D37D4F1833998A5047256CB47BB1D9F0C3CACB493D19C148BC7EA46"),
        "Release notes must record CLI SHA-256"
    );
}

#[test]
fn stable_release_notes_record_desktop_sha() {
    let doc = read_root_doc("RELEASE_NOTES_v100_STABLE.md");
    assert!(
        doc.contains("04B696B533602C196808213A2B70DB6FEAD4A61C2A9F64B598208C8A4CFF4DF4"),
        "Release notes must record desktop SHA-256"
    );
}

#[test]
fn stable_release_notes_record_cli_size() {
    let doc = read_root_doc("RELEASE_NOTES_v100_STABLE.md");
    assert!(
        doc.contains("17,705,472"),
        "Release notes must record CLI binary size"
    );
}

#[test]
fn stable_release_notes_record_test_count() {
    let doc = read_root_doc("RELEASE_NOTES_v100_STABLE.md");
    assert!(
        doc.contains("4,302"),
        "Release notes must record test count"
    );
}

// ── Version String ──────────────────────────────────────────────────────────

#[test]
fn stable_release_notes_show_version_100() {
    let doc = read_root_doc("RELEASE_NOTES_v100_STABLE.md");
    assert!(
        doc.contains("openwand 1.0.0"),
        "Release notes must show version string as 'openwand 1.0.0'"
    );
}

// ── Release Check ───────────────────────────────────────────────────────────

#[test]
fn stable_release_notes_record_release_check_pass() {
    let doc = read_root_doc("RELEASE_NOTES_v100_STABLE.md");
    assert!(
        doc.contains("8/8") || doc.contains("8 of 8"),
        "Release notes must record release check 8/8 pass"
    );
}

#[test]
fn release_check_json_exists() {
    let p = workspace_root().join("v100_final_release_check.json");
    assert!(
        p.exists(),
        "v100_final_release_check.json must exist"
    );
}

// ── VK Blocker Reconciliation ───────────────────────────────────────────────

#[test]
fn stable_release_notes_reconcile_vk_blockers() {
    let doc = read_root_doc("RELEASE_NOTES_v100_STABLE.md");
    assert!(
        doc.contains("VK-1") && doc.contains("VK-2") && doc.contains("VK-3")
            && doc.contains("VK-4") && doc.contains("VK-5"),
        "Release notes must reconcile all VK blockers"
    );
}

#[test]
fn stable_release_notes_show_vk5_pending() {
    let doc = read_root_doc("RELEASE_NOTES_v100_STABLE.md");
    assert!(
        doc.contains("VK-5") && (doc.contains("Pending") || doc.contains("pending")),
        "Release notes must show VK-5 as pending"
    );
}

// ── Claims and Caveats ──────────────────────────────────────────────────────

#[test]
fn stable_release_notes_list_21_claims() {
    let doc = read_root_doc("RELEASE_NOTES_v100_STABLE.md");
    assert!(
        doc.contains("C-21"),
        "Release notes must list all 21 claims including C-21"
    );
}

#[test]
fn stable_release_notes_list_caveats() {
    let doc = read_root_doc("RELEASE_NOTES_v100_STABLE.md");
    // Must preserve all key non-claims
    assert!(doc.contains("Not production-ready"), "Must preserve: not production-ready");
    assert!(doc.contains("Not formal security certification") || doc.contains("Not formal"), "Must preserve: not formal cert");
    assert!(doc.contains("Not externally reviewed"), "Must preserve: not externally reviewed");
    assert!(doc.contains("Not provider complete"), "Must preserve: not provider complete");
    assert!(doc.contains("Not stable API"), "Must preserve: not stable API");
    assert!(doc.contains("Not physical immutability"), "Must preserve: not physical immutability");
    assert!(doc.contains("Not remote attestation"), "Must preserve: not remote attestation");
}

#[test]
fn stable_release_notes_list_15_caveats() {
    let doc = read_root_doc("RELEASE_NOTES_v100_STABLE.md");
    assert!(
        doc.contains("15 caveats") || doc.contains("15 Caveats") || doc.contains("15 explicit"),
        "Release notes must reference 15 caveats"
    );
}

// ── No Overclaiming ─────────────────────────────────────────────────────────

#[test]
fn stable_release_notes_do_not_claim_production_ready() {
    let doc = read_root_doc("RELEASE_NOTES_v100_STABLE.md");
    // Must appear only in negation context or in caveat list (numbered with **Not...)
    for line in doc.lines() {
        let lower = line.to_lowercase();
        if lower.contains("production-ready") || lower.contains("production ready") {
            assert!(
                lower.contains("not ") || lower.contains("does not") || lower.contains("this is a stable release, not") || lower.contains("requires"),
                "Must not affirmatively claim production-ready: {}",
                line
            );
        }
    }
}

#[test]
fn stable_release_notes_do_not_claim_external_review() {
    let doc = read_root_doc("RELEASE_NOTES_v100_STABLE.md");
    for line in doc.lines() {
        let lower = line.to_lowercase();
        if lower.contains("external review was completed")
            || lower.contains("external review was executed")
            || lower.contains("externally reviewed")
        {
            assert!(
                lower.contains("not ") || lower.contains("does not") || lower.contains("no external reviewer"),
                "Must not affirmatively claim external review: {}",
                line
            );
        }
    }
}

#[test]
fn stable_release_notes_do_not_claim_certification() {
    let doc = read_root_doc("RELEASE_NOTES_v100_STABLE.md");
    for line in doc.lines() {
        let lower = line.to_lowercase();
        if lower.contains("formal certification") || lower.contains("formally certified") {
            assert!(
                lower.contains("not ") || lower.contains("does not") || lower.contains("no certification body"),
                "Must not affirmatively claim formal certification: {}",
                line
            );
        }
    }
}

// ── Architecture Arc ────────────────────────────────────────────────────────

#[test]
fn stable_release_notes_show_complete_arc() {
    let doc = read_root_doc("RELEASE_NOTES_v100_STABLE.md");
    assert!(
        doc.contains("Control") && doc.contains("Close"),
        "Release notes must show architecture arc from Control to Close"
    );
}

// ── STATE.md Consistency ────────────────────────────────────────────────────

#[test]
fn state_md_shows_version_100() {
    let doc = read_root_doc("STATE.md");
    assert!(
        doc.contains("1.0.0"),
        "STATE.md must reference version 1.0.0"
    );
}

#[test]
fn state_md_shows_wave_125a() {
    let doc = read_root_doc("STATE.md");
    assert!(
        doc.contains("125A"),
        "STATE.md must reference Wave 125A"
    );
}

// ── Cargo.toml Version ──────────────────────────────────────────────────────

#[test]
fn cargo_toml_version_at_least_100() {
    let doc = read_root_doc("Cargo.toml");
    assert!(
        doc.contains("version = \"1.0.0\"") || doc.contains("version = \"1.0.1\""),
        "Cargo.toml must have version 1.0.0 or later"
    );
}

// ── Post-v1.0.0 Roadmap ────────────────────────────────────────────────────

#[test]
fn stable_release_notes_have_post_v100_roadmap() {
    let doc = read_root_doc("RELEASE_NOTES_v100_STABLE.md");
    assert!(
        doc.contains("Post-v1.0.0") || doc.contains("Post-v100") || doc.contains("Roadmap"),
        "Release notes must include post-v1.0.0 roadmap"
    );
}
