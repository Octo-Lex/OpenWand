//! API Stability Policy guard tests (Wave 131A, VL-6).

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

// ── Document Exists ──────────────────────────────────────────────────────────

#[test]
fn policy_document_exists() {
    let doc = read_docs_doc("API_STABILITY_POLICY.md");
    assert!(!doc.is_empty());
}

#[test]
fn policy_references_wave_131a() {
    let doc = read_docs_doc("API_STABILITY_POLICY.md");
    assert!(doc.contains("131A"));
}

// ── Stability Categories Defined ────────────────────────────────────────────

#[test]
fn policy_defines_five_categories() {
    let doc = read_docs_doc("API_STABILITY_POLICY.md");
    assert!(doc.contains("**Stable**"));
    assert!(doc.contains("**Supported (Evolving)**"));
    assert!(doc.contains("**Experimental**"));
    assert!(doc.contains("**Internal**"));
    assert!(doc.contains("**Unsupported**"));
}

// ── CLI Classification ──────────────────────────────────────────────────────

#[test]
fn policy_classifies_core_cli_as_stable() {
    let doc = read_docs_doc("API_STABILITY_POLICY.md");
    // The 8 core commands must be classified Stable
    for cmd in &["run", "explain", "trace-verify", "operation-replay",
                 "anchor-write", "anchor-verify", "review", "evidence-report"] {
        assert!(doc.contains(cmd), "Policy must classify {}", cmd);
    }
}

#[test]
fn policy_classifies_workflow_as_experimental() {
    let doc = read_docs_doc("API_STABILITY_POLICY.md");
    assert!(
        doc.contains("workflow") && doc.contains("Experimental"),
        "Workflow commands must be classified Experimental"
    );
}

// ── Report Schema Classification ────────────────────────────────────────────

#[test]
fn policy_classifies_report_schemas() {
    let doc = read_docs_doc("API_STABILITY_POLICY.md");
    for schema in &["VerificationReport", "ReplayReport", "AnchorVerificationReport", "EvidenceReport"] {
        assert!(doc.contains(schema), "Policy must classify {}", schema);
    }
}

#[test]
fn policy_classifies_release_check_as_internal() {
    let doc = read_docs_doc("API_STABILITY_POLICY.md");
    assert!(
        doc.contains("ReleaseCheckReport") && doc.contains("Internal"),
        "ReleaseCheckReport must be classified Internal"
    );
}

// ── Trace Schema Classification ─────────────────────────────────────────────

#[test]
fn policy_classifies_trace_scopes_as_stable() {
    let doc = read_docs_doc("API_STABILITY_POLICY.md");
    assert!(doc.contains("TraceStreamScope") && doc.contains("Stable"));
}

#[test]
fn policy_pins_blake3() {
    let doc = read_docs_doc("API_STABILITY_POLICY.md");
    assert!(
        doc.contains("BLAKE3") && doc.contains("pinned"),
        "Policy must pin BLAKE3 as the hash algorithm"
    );
}

// ── Operation Descriptors ───────────────────────────────────────────────────

#[test]
fn policy_classifies_operation_descriptors() {
    let doc = read_docs_doc("API_STABILITY_POLICY.md");
    for op in &["WorkflowInitiation", "ApprovalResolution", "EvidenceExport"] {
        assert!(doc.contains(op), "Policy must classify {}", op);
    }
}

// ── Crate Classification ────────────────────────────────────────────────────

#[test]
fn policy_classifies_openwand_content_as_unsupported() {
    let doc = read_docs_doc("API_STABILITY_POLICY.md");
    assert!(
        doc.contains("openwand-content") && doc.contains("Unsupported"),
        "openwand-content must be classified Unsupported"
    );
}

#[test]
fn policy_classifies_workflow_crate_as_experimental() {
    let doc = read_docs_doc("API_STABILITY_POLICY.md");
    assert!(doc.contains("openwand-workflow") && doc.contains("Experimental"));
}

#[test]
fn policy_states_no_crate_published() {
    let doc = read_docs_doc("API_STABILITY_POLICY.md");
    assert!(
        doc.contains("not published") || doc.contains("No crate is published"),
        "Policy must state no crate is published to crates.io"
    );
}

// ── Compatibility Rules ─────────────────────────────────────────────────────

#[test]
fn policy_defines_patch_release_rules() {
    let doc = read_docs_doc("API_STABILITY_POLICY.md");
    assert!(doc.contains("Patch Release") && doc.contains("No breaking changes"));
}

#[test]
fn policy_defines_minor_release_rules() {
    let doc = read_docs_doc("API_STABILITY_POLICY.md");
    assert!(doc.contains("Minor Release") && doc.contains("Additive"));
}

#[test]
fn policy_defines_major_release_rules() {
    let doc = read_docs_doc("API_STABILITY_POLICY.md");
    assert!(doc.contains("Major Release"));
}

#[test]
fn policy_defines_deprecation_window() {
    let doc = read_docs_doc("API_STABILITY_POLICY.md");
    assert!(doc.contains("Deprecation") && doc.contains("one minor"));
}

// ── Caveat X-08 ─────────────────────────────────────────────────────────────

#[test]
fn policy_narrows_x08_partially() {
    let doc = read_docs_doc("API_STABILITY_POLICY.md");
    assert!(
        doc.contains("X-08") && doc.contains("PARTIALLY NARROWED"),
        "Policy must state X-08 is partially narrowed"
    );
}

#[test]
fn policy_does_not_resolve_x08_fully() {
    let doc = read_docs_doc("API_STABILITY_POLICY.md");
    assert!(
        !doc.contains("X-08 is resolved") && !doc.contains("X-08 is fully resolved"),
        "Policy must not claim X-08 is fully resolved"
    );
}

// ── No Overclaiming ─────────────────────────────────────────────────────────

#[test]
fn policy_does_not_claim_global_api_freeze() {
    let doc = read_docs_doc("API_STABILITY_POLICY.md");
    assert!(
        doc.contains("Not a global API freeze"),
        "Policy must disclaim global API freeze"
    );
}

#[test]
fn policy_does_not_claim_production_ready() {
    let doc = read_docs_doc("API_STABILITY_POLICY.md");
    for line in doc.lines() {
        let lower = line.to_lowercase();
        // Check for affirmative production readiness claims
        // (not "production-quality" as a descriptor in a stability category)
        if (lower.contains("production ready") || lower.contains("production-ready"))
            && !lower.contains("not ")
            && !lower.contains("doesn't")
            && !lower.contains("does not")
        {
            panic!("Must not claim production readiness: {}", line);
        }
    }
}
