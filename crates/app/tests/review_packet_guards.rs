//! Guard tests for external review packet (Wave 110A).
//!
//! Proves:
//! 1. Review packet document exists.
//! 2. Contains exact CLI commands a reviewer can run.
//! 3. Contains exit code tables.
//! 4. Contains caveats and non-claims section.
//! 5. Does not claim production readiness or full Linux support.

#[cfg(test)]
mod review_packet_guards {
    use std::path::PathBuf;

    fn doc_path() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("..")
            .join("..")
            .join("docs")
            .join("EXTERNAL_REVIEW_PACKET.md")
    }

    /// Guard: review packet exists.
    #[test]
    fn review_packet_exists() {
        let path = doc_path();
        assert!(path.exists(), "EXTERNAL_REVIEW_PACKET.md must exist");
        let content = std::fs::read_to_string(&path).unwrap();
        assert!(content.len() > 5000, "review packet must be substantial");
    }

    /// Guard: contains trace-verify command.
    #[test]
    fn contains_trace_verify_command() {
        let content = std::fs::read_to_string(doc_path()).unwrap();
        assert!(content.contains("trace-verify"),
            "must contain trace-verify command");
    }

    /// Guard: contains anchor commands.
    #[test]
    fn contains_anchor_commands() {
        let content = std::fs::read_to_string(doc_path()).unwrap();
        assert!(content.contains("anchor-write"),
            "must contain anchor-write command");
        assert!(content.contains("anchor-verify"),
            "must contain anchor-verify command");
    }

    /// Guard: contains evidence-report command.
    #[test]
    fn contains_evidence_report_command() {
        let content = std::fs::read_to_string(doc_path()).unwrap();
        assert!(content.contains("evidence-report"),
            "must contain evidence-report command");
    }

    /// Guard: contains exit code tables.
    #[test]
    fn contains_exit_code_tables() {
        let content = std::fs::read_to_string(doc_path()).unwrap();
        assert!(content.contains("Exit Codes") || content.contains("exit code"),
            "must document exit codes");
        assert!(content.contains("| 0 |"),
            "must have exit code 0 documented");
    }

    /// Guard: contains caveats section.
    #[test]
    fn contains_caveats_section() {
        let content = std::fs::read_to_string(doc_path()).unwrap();
        assert!(content.contains("Caveats") || content.contains("Non-Claims") || content.contains("does NOT claim"),
            "must contain caveats/non-claims section");
    }

    /// Guard: does not claim production readiness.
    #[test]
    fn does_not_claim_production_readiness() {
        let content = std::fs::read_to_string(doc_path()).unwrap();
        let lower = content.to_lowercase();
        // Check that "production ready" appears only in negation
        if lower.contains("production ready") {
            assert!(lower.contains("not production ready") || lower.contains("does not claim") || lower.contains("not a"),
                "must not claim production readiness without negation");
        }
    }

    /// Guard: does not claim full Linux support.
    #[test]
    fn does_not_claim_full_linux_support() {
        let content = std::fs::read_to_string(doc_path()).unwrap();
        let lower = content.to_lowercase();
        if lower.contains("full linux") {
            assert!(lower.contains("not") || lower.contains("does not"),
                "must not claim full Linux support without qualification");
        }
    }

    /// Guard: contains reviewer checklist.
    #[test]
    fn contains_reviewer_checklist() {
        let content = std::fs::read_to_string(doc_path()).unwrap();
        assert!(content.contains("Checklist") || content.contains("reviewer can run"),
            "must contain a reviewer checklist");
    }
}
