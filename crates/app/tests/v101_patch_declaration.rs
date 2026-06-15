//! v1.0.1 Patch Declaration guard tests (Wave 129B).

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

// ── Declaration Document ────────────────────────────────────────────────────

#[test]
fn declaration_document_exists() {
    let doc = read_docs_doc("V101_PATCH_DECLARATION.md");
    assert!(!doc.is_empty());
}

#[test]
fn declaration_references_wave_129b() {
    let doc = read_docs_doc("V101_PATCH_DECLARATION.md");
    assert!(doc.contains("129B"));
}

#[test]
fn declaration_states_v101_declared() {
    let doc = read_docs_doc("V101_PATCH_DECLARATION.md");
    assert!(doc.contains("v1.0.1") && doc.contains("declared"));
}

// ── Fixed Defects ───────────────────────────────────────────────────────────

#[test]
fn declaration_records_f_vl1_1_fixed() {
    let doc = read_docs_doc("V101_PATCH_DECLARATION.md");
    assert!(doc.contains("F-VL1-1") && doc.contains("Blocking"));
}

#[test]
fn declaration_records_f_vl1_2_fixed() {
    let doc = read_docs_doc("V101_PATCH_DECLARATION.md");
    assert!(doc.contains("F-VL1-2"));
}

// ── Artifact Identity ───────────────────────────────────────────────────────

#[test]
fn declaration_records_cli_sha() {
    let doc = read_docs_doc("V101_PATCH_DECLARATION.md");
    assert!(doc.contains("5ED051CAFF4534F372B6ABF10D1263422F3CE1357814A121DF4822848857ECF5"));
}

#[test]
fn declaration_records_desktop_sha() {
    let doc = read_docs_doc("V101_PATCH_DECLARATION.md");
    assert!(doc.contains("2C6AB04D42AA6EFE742643CB344456814222786D6BCB67ACA5D8A56E785C7D40"));
}

#[test]
fn declaration_records_test_count() {
    let doc = read_docs_doc("V101_PATCH_DECLARATION.md");
    assert!(doc.contains("4,416"));
}

// ── Caveats ─────────────────────────────────────────────────────────────────

#[test]
fn declaration_preserves_15_caveats() {
    let doc = read_docs_doc("V101_PATCH_DECLARATION.md");
    assert!(doc.contains("15 caveats") && doc.contains("unchanged") || doc.contains("Unchanged"));
}

// ── No Overclaiming ─────────────────────────────────────────────────────────

#[test]
fn declaration_does_not_claim_production_ready() {
    let doc = read_docs_doc("V101_PATCH_DECLARATION.md");
    for line in doc.lines() {
        let lower = line.to_lowercase();
        if lower.contains("production-ready") || lower.contains("production ready") {
            assert!(lower.contains("not ") || lower.contains("does not"));
        }
    }
}

#[test]
fn declaration_does_not_claim_feature_release() {
    let doc = read_docs_doc("V101_PATCH_DECLARATION.md");
    assert!(doc.contains("Not a feature release"));
}

// ── Version ─────────────────────────────────────────────────────────────────

#[test]
fn cargo_toml_version_is_101() {
    let doc = read_root_doc("Cargo.toml");
    assert!(doc.contains("version = \"1.0.1\""));
}

#[test]
fn state_md_references_v101() {
    let doc = read_root_doc("STATE.md");
    assert!(doc.contains("1.0.1"));
}

// ── Release Notes ───────────────────────────────────────────────────────────

#[test]
fn patch_notes_exist() {
    let doc = read_root_doc("RELEASE_NOTES_v101_PATCH.md");
    assert!(!doc.is_empty());
}
