//! v1.0.1 Patch Preparation guard tests (Wave 129A).

use std::path::PathBuf;

fn workspace_root() -> PathBuf {
    let p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    p.parent().and_then(|p| p.parent()).map(|p| p.to_path_buf()).unwrap_or_else(|| PathBuf::from(".."))
}

fn read_root_doc(name: &str) -> String {
    let p = workspace_root().join(name);
    std::fs::read_to_string(&p).unwrap_or_else(|_| panic!("Failed to read {}", p.display()))
}

// ── Patch Notes ─────────────────────────────────────────────────────────────

#[test]
fn patch_notes_exist() {
    let doc = read_root_doc("RELEASE_NOTES_v101_PATCH.md");
    assert!(!doc.is_empty(), "v1.0.1 patch notes must exist");
}

#[test]
fn patch_notes_reference_wave_129a() {
    let doc = read_root_doc("RELEASE_NOTES_v101_PATCH.md");
    assert!(doc.contains("129A"), "Patch notes must reference Wave 129A");
}

#[test]
fn patch_notes_record_f_vl1_1_fix() {
    let doc = read_root_doc("RELEASE_NOTES_v101_PATCH.md");
    assert!(doc.contains("F-VL1-1"), "Patch notes must record F-VL1-1 fix");
}

#[test]
fn patch_notes_record_cli_sha() {
    let doc = read_root_doc("RELEASE_NOTES_v101_PATCH.md");
    assert!(
        doc.contains("5ED051CAFF4534F372B6ABF10D1263422F3CE1357814A121DF4822848857ECF5"),
        "Patch notes must record CLI SHA-256"
    );
}

#[test]
fn patch_notes_record_version_string() {
    let doc = read_root_doc("RELEASE_NOTES_v101_PATCH.md");
    assert!(
        doc.contains("openwand 1.0.1"),
        "Patch notes must record version string 'openwand 1.0.1'"
    );
}

#[test]
fn patch_notes_show_no_new_features() {
    let doc = read_root_doc("RELEASE_NOTES_v101_PATCH.md");
    assert!(
        doc.contains("No new features"),
        "Patch notes must state no new features"
    );
}

#[test]
fn patch_notes_show_caveats_unchanged() {
    let doc = read_root_doc("RELEASE_NOTES_v101_PATCH.md");
    assert!(
        doc.contains("Unchanged from v1.0.0") || doc.contains("unchanged"),
        "Patch notes must state caveats are unchanged"
    );
}

#[test]
fn patch_notes_show_vl1_postfix_pass() {
    let doc = read_root_doc("RELEASE_NOTES_v101_PATCH.md");
    assert!(
        doc.contains("Post-Fix") || doc.contains("post-fix") || doc.contains("128A re-run"),
        "Patch notes must show post-fix VL-1 pass evidence"
    );
}

// ── Version ─────────────────────────────────────────────────────────────────

#[test]
fn cargo_toml_version_is_101() {
    let doc = read_root_doc("Cargo.toml");
    assert!(
        doc.contains("version = \"1.0.1\""),
        "Cargo.toml must have version = \"1.0.1\""
    );
}

// ── Release Check ───────────────────────────────────────────────────────────

#[test]
fn release_check_json_exists() {
    let p = workspace_root().join("v101_release_check.json");
    assert!(p.exists(), "v101_release_check.json must exist");
}

// ── No Overclaiming ─────────────────────────────────────────────────────────

#[test]
fn patch_notes_do_not_claim_production_ready() {
    let doc = read_root_doc("RELEASE_NOTES_v101_PATCH.md");
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
fn patch_notes_do_not_expand_claims() {
    let doc = read_root_doc("RELEASE_NOTES_v101_PATCH.md");
    assert!(
        doc.contains("No new claims"),
        "Patch notes must state no new claims added"
    );
}
