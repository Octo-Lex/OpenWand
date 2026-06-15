//! VL-3 Maintenance Patch Criteria + Trace Fix guard tests (Wave 128A).

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

// ── Maintenance Patch Criteria ───────────────────────────────────────────────

#[test]
fn patch_criteria_document_exists() {
    let doc = read_docs_doc("VL3_MAINTENANCE_PATCH_CRITERIA.md");
    assert!(!doc.is_empty());
}

#[test]
fn patch_criteria_references_wave_128a() {
    let doc = read_docs_doc("VL3_MAINTENANCE_PATCH_CRITERIA.md");
    assert!(doc.contains("128A"));
}

#[test]
fn patch_criteria_defines_versioning() {
    let doc = read_docs_doc("VL3_MAINTENANCE_PATCH_CRITERIA.md");
    assert!(doc.contains("v1.0.x") && doc.contains("Patch"));
}

#[test]
fn patch_criteria_records_f_vl1_1_fixed() {
    let doc = read_docs_doc("VL3_MAINTENANCE_PATCH_CRITERIA.md");
    assert!(doc.contains("F-VL1-1") && doc.contains("Fixed"));
}

#[test]
fn patch_criteria_records_f_vl1_2_fixed() {
    let doc = read_docs_doc("VL3_MAINTENANCE_PATCH_CRITERIA.md");
    assert!(doc.contains("F-VL1-2") && doc.contains("Fixed"));
}

#[test]
fn patch_criteria_shows_vl1_rerun_pass() {
    let doc = read_docs_doc("VL3_MAINTENANCE_PATCH_CRITERIA.md");
    assert!(doc.contains("Post-Fix") && doc.contains("PASS"));
}

#[test]
fn patch_criteria_does_not_add_features() {
    let doc = read_docs_doc("VL3_MAINTENANCE_PATCH_CRITERIA.md");
    assert!(doc.contains("No new features") || doc.contains("MUST NOT add features"));
}

#[test]
fn patch_criteria_does_not_remove_caveats() {
    let doc = read_docs_doc("VL3_MAINTENANCE_PATCH_CRITERIA.md");
    assert!(doc.contains("MUST NOT remove caveats"));
}

// ── resolve_db_path exists in main.rs ────────────────────────────────────────

#[test]
fn resolve_db_path_function_exists() {
    let content = read_root_doc("crates/app/src/main.rs");
    assert!(
        content.contains("fn resolve_db_path"),
        "resolve_db_path function must exist in main.rs"
    );
}

#[test]
fn verification_commands_no_longer_use_underscore_cli() {
    let content = read_root_doc("crates/app/src/main.rs");
    // cmd_trace_verify should now take cli: &Cli, not _cli: &Cli
    assert!(
        !content.contains("async fn cmd_trace_verify(_cli"),
        "cmd_trace_verify must not use _cli (must honor --db)"
    );
    assert!(
        !content.contains("async fn cmd_anchor_write(\n    _cli"),
        "cmd_anchor_write must not use _cli"
    );
    assert!(
        !content.contains("async fn cmd_anchor_verify(\n    _cli"),
        "cmd_anchor_verify must not use _cli"
    );
}

// ── Verifier uses serde_json scope ───────────────────────────────────────────

#[test]
fn verifier_uses_serde_json_scope() {
    let content = read_root_doc("crates/trace/src/verifier.rs");
    // The fix replaces format!("{:?}", ...) with serde_json::to_string(...)
    assert!(
        content.contains("serde_json::to_string(&entry.stream_id.scope)"),
        "Verifier must use serde_json::to_string for scope serialization"
    );
}

#[test]
fn verifier_no_longer_uses_debug_format_for_scope() {
    let content = read_root_doc("crates/trace/src/verifier.rs");
    // The old line used format!("{:?}", entry.stream_id.scope) directly
    // in the compute_entry_hash call. Make sure that specific pattern is gone.
    let lines: Vec<&str> = content.lines().collect();
    for (i, line) in lines.iter().enumerate() {
        // Look for the compute_entry_hash call in verify_with_hash_policy
        if line.contains("&format!(\"{:?}\", entry.stream_id.scope)") {
            panic!("Verifier must not use Debug format for scope in hash computation (line {})", i);
        }
    }
}
