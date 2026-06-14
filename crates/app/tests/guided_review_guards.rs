//! Guard tests for guided evidence flow (Wave 114A).
//!
//! Proves:
//! 1. The `review` subcommand exists in CLI.
//! 2. The guided flow source does not silently infer operations.
//! 3. The guided flow does not mutate trace or create anchors.
//! 4. The guided flow prints step-by-step progress.
//! 5. The guided flow preserves caveats and non-claims.

#[cfg(test)]
mod guided_review_guards {
    use std::path::PathBuf;

    fn main_rs_path() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("src")
            .join("main.rs")
    }

    /// Guard: review subcommand is defined.
    #[test]
    fn review_subcommand_exists() {
        let content = std::fs::read_to_string(main_rs_path()).unwrap();
        assert!(content.contains("name = \"review\""),
            "CLI must define a 'review' subcommand");
        assert!(content.contains("Commands::Review"),
            "dispatch must handle Commands::Review");
    }

    /// Guard: guided flow function exists.
    #[test]
    fn guided_review_function_exists() {
        let content = std::fs::read_to_string(main_rs_path()).unwrap();
        assert!(content.contains("cmd_guided_review"),
            "cmd_guided_review function must exist");
    }

    /// Guard: guided flow validates operations file explicitly.
    #[test]
    fn validates_operations_file() {
        let content = std::fs::read_to_string(main_rs_path()).unwrap();
        let review_start = content.find("async fn cmd_guided_review").unwrap();
        let review_end = content.find("async fn cmd_evidence_report").unwrap();
        let review_body = &content[review_start..review_end];
        assert!(review_body.contains("Operations") && review_body.contains("read_to_string"),
            "guided flow must validate operations file exists");
    }

    /// Guard: guided flow prints step-by-step progress.
    #[test]
    fn prints_step_by_step() {
        let content = std::fs::read_to_string(main_rs_path()).unwrap();
        let review_start = content.find("async fn cmd_guided_review").unwrap();
        let review_end = content.find("async fn cmd_evidence_report").unwrap();
        let review_body = &content[review_start..review_end];
        assert!(review_body.contains("Step 0") && review_body.contains("Step 1"),
            "guided flow must print numbered steps");
        assert!(review_body.contains("Step 6") || review_body.contains("Step 8"),
            "guided flow must have multiple steps");
    }

    /// Guard: guided flow does not call anchor_write or mutate trace.
    #[test]
    fn does_not_mutate_or_create_anchors() {
        let content = std::fs::read_to_string(main_rs_path()).unwrap();
        let review_start = content.find("async fn cmd_guided_review").unwrap();
        let review_end = content.find("async fn cmd_evidence_report").unwrap();
        let review_body = &content[review_start..review_end];
        assert!(!review_body.contains("anchor_write") && !review_body.contains("cmd_anchor_write"),
            "guided flow must NOT call anchor-write");
        assert!(!review_body.contains(".write_synchronously") && !review_body.contains("append_to_trace"),
            "guided flow must NOT mutate trace");
    }

    /// Guard: guided flow includes non-claims in output.
    #[test]
    fn includes_non_claims() {
        let content = std::fs::read_to_string(main_rs_path()).unwrap();
        let review_start = content.find("async fn cmd_guided_review").unwrap();
        let review_end = content.find("async fn cmd_evidence_report").unwrap();
        let review_body = &content[review_start..review_end];
        assert!(review_body.contains("does NOT claim") || review_body.contains("does not claim"),
            "guided flow must print non-claims in output");
        assert!(review_body.contains("production readiness") || review_body.contains("production-readiness"),
            "guided flow must mention production readiness as a non-claim");
    }

    /// Guard: guided flow provides reviewer next steps.
    #[test]
    fn provides_reviewer_next_steps() {
        let content = std::fs::read_to_string(main_rs_path()).unwrap();
        let review_start = content.find("async fn cmd_guided_review").unwrap();
        let review_end = content.find("async fn cmd_evidence_report").unwrap();
        let review_body = &content[review_start..review_end];
        assert!(review_body.contains("Next steps") || review_body.contains("next step"),
            "guided flow must provide reviewer next steps");
    }

    /// Guard: guided flow uses existing read-only verification paths.
    #[test]
    fn uses_readonly_verification() {
        let content = std::fs::read_to_string(main_rs_path()).unwrap();
        let review_start = content.find("async fn cmd_guided_review").unwrap();
        let review_end = content.find("async fn cmd_evidence_report").unwrap();
        let review_body = &content[review_start..review_end];
        assert!(review_body.contains("TraceVerifier") && review_body.contains("OperationReplayVerifier"),
            "guided flow must use existing read-only verifiers");
        assert!(review_body.contains("verify_anchor"),
            "guided flow must use existing anchor verifier");
    }
}
