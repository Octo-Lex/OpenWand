//! Guard tests for rc.1 soak/regression report (Wave 122A).

#[cfg(test)]
mod rc1_soak_guards {
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

    // ---- Soak report exists ----

    #[test]
    fn soak_report_exists() {
        let content = read_docs_doc("RC1_SOAK_REPORT.md");
        assert!(content.contains("Soak / Regression Report"));
        assert!(content.contains("Wave 122A"));
    }

    // ---- Release check results recorded ----

    #[test]
    fn release_check_results_recorded() {
        let content = read_docs_doc("RC1_SOAK_REPORT.md");
        assert!(content.contains("4,240 tests") || content.contains("4240"));
        assert!(content.contains("PASS"));
    }

    // ---- Blocking regressions classified ----

    #[test]
    fn blocking_regressions_classified() {
        let content = read_docs_doc("RC1_SOAK_REPORT.md");
        assert!(
            content.contains("Blocking regressions") && content.contains("0"),
            "Must classify blocking regressions"
        );
    }

    // ---- Findings classified by category ----

    #[test]
    fn findings_categorized() {
        let content = read_docs_doc("RC1_SOAK_REPORT.md");
        assert!(content.contains("Non-blocking") || content.contains("non-blocking"));
        assert!(content.contains("Environment-blocked") || content.contains("environment-blocked"));
        assert!(content.contains("Deferred") || content.contains("deferred"));
    }

    // ---- Version string finding recorded ----

    #[test]
    fn version_string_finding_recorded() {
        let content = read_docs_doc("RC1_SOAK_REPORT.md");
        assert!(
            content.contains("version string") || content.contains("F-SOAK-1"),
            "Must record version string finding"
        );
    }

    // ---- Desktop launch validated ----

    #[test]
    fn desktop_launch_validated() {
        let content = read_docs_doc("RC1_SOAK_REPORT.md");
        assert!(
            content.contains("launches") || content.contains("Launch"),
            "Must record desktop launch result"
        );
    }

    // ---- Does not declare final readiness ----

    #[test]
    fn does_not_declare_final_readiness() {
        let content = read_docs_doc("RC1_SOAK_REPORT.md");
        assert!(
            content.contains("does not declare") || content.contains("Does NOT Claim"),
            "Must state it does not declare final v1.0"
        );
    }

    // ---- No execution authority added ----

    #[test]
    fn soak_adds_no_execution_authority() {
        let content = read_docs_doc("RC1_SOAK_REPORT.md");
        assert!(!content.contains("approve this operation"));
        assert!(!content.contains("bypass policy"));
    }
}
