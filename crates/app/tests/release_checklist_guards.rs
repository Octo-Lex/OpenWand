//! Guard tests for release checklist (Wave 111A).
//!
//! Proves:
//! 1. Release checklist document exists.
//! 2. Contains all required sections.
//! 3. Includes the critical openwand-ui desktop binary build gate.
//! 4. Distinguishes process evidence from production readiness.
//! 5. Contains caveat/non-claim review section.

#[cfg(test)]
mod release_checklist_guards {
    use std::path::PathBuf;

    fn doc_path() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("..")
            .join("..")
            .join("docs")
            .join("RELEASE_CHECKLIST.md")
    }

    /// Guard: checklist exists and is substantial.
    #[test]
    fn checklist_exists() {
        let path = doc_path();
        assert!(path.exists(), "RELEASE_CHECKLIST.md must exist");
        let content = std::fs::read_to_string(&path).unwrap();
        assert!(content.len() > 3000, "checklist must be substantial");
    }

    /// Guard: contains test baseline section.
    #[test]
    fn contains_test_baseline() {
        let content = std::fs::read_to_string(doc_path()).unwrap();
        assert!(content.contains("Test Baseline") || content.contains("cargo test"),
            "must contain test baseline section");
    }

    /// Guard: contains production clippy section.
    #[test]
    fn contains_production_clippy() {
        let content = std::fs::read_to_string(doc_path()).unwrap();
        assert!(content.contains("Production Clippy") || content.contains("cargo clippy"),
            "must contain production clippy section");
    }

    /// Guard: contains cargo audit section.
    #[test]
    fn contains_cargo_audit() {
        let content = std::fs::read_to_string(doc_path()).unwrap();
        assert!(content.contains("cargo audit") || content.contains("Dependency Audit"),
            "must contain cargo audit section");
    }

    /// Guard: contains the CRITICAL desktop binary build gate.
    /// This is the correction from Wave 109A — openwand-ui must be built,
    /// not just openwand --features desktop.
    #[test]
    fn contains_desktop_ui_binary_build_gate() {
        let content = std::fs::read_to_string(doc_path()).unwrap();
        assert!(content.contains("openwand-ui"),
            "must contain openwand-ui desktop binary build gate (109A correction)");
        assert!(content.contains("CRITICAL") || content.contains("critical"),
            "desktop binary gate must be marked as critical");
    }

    /// Guard: contains Linux compile validation section.
    #[test]
    fn contains_linux_validation() {
        let content = std::fs::read_to_string(doc_path()).unwrap();
        assert!(content.contains("Linux"),
            "must contain Linux validation section");
    }

    /// Guard: contains artifact identity section (SHA-256, size).
    #[test]
    fn contains_artifact_identity() {
        let content = std::fs::read_to_string(doc_path()).unwrap();
        assert!(content.contains("SHA-256") || content.contains("sha256") || content.contains("SHA"),
            "must contain artifact SHA-256 recording");
    }

    /// Guard: contains caveat/non-claim review section.
    #[test]
    fn contains_caveat_review() {
        let content = std::fs::read_to_string(doc_path()).unwrap();
        assert!(content.contains("Non-Claim") || content.contains("non-claim") || content.contains("does NOT claim"),
            "must contain caveat/non-claim review section");
    }

    /// Guard: distinguishes process evidence from production readiness.
    #[test]
    fn distinguishes_process_from_production() {
        let content = std::fs::read_to_string(doc_path()).unwrap();
        assert!(content.contains("process evidence") || content.contains("Process vs"),
            "must distinguish process evidence from production readiness");
    }

    /// Guard: contains tag consistency section.
    #[test]
    fn contains_tag_consistency() {
        let content = std::fs::read_to_string(doc_path()).unwrap();
        assert!(content.contains("Tag Consistency") || content.contains("tag"),
            "must contain tag consistency section");
    }
}
