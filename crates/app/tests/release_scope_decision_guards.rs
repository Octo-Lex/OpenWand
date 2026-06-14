//! Guard tests for v1.0 release-scope decisions (Wave 119A).

#[cfg(test)]
mod release_scope_decision_guards {
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

    // ---- Decision document exists ----

    #[test]
    fn release_decisions_document_exists() {
        let content = read_docs_doc("V100_RELEASE_DECISIONS.md");
        assert!(content.contains("Wave 119A"));
        assert!(content.contains("VJ-1"));
        assert!(content.contains("VJ-2"));
        assert!(content.contains("VJ-3"));
    }

    // ---- VJ-1 classified as deferred ----

    #[test]
    fn vj1_external_review_is_deferred_not_omitted() {
        let content = read_docs_doc("V100_RELEASE_DECISIONS.md");
        // Must contain a DEFERRED classification
        assert!(
            content.contains("DEFERRED") || content.contains("Deferred"),
            "VJ-1 must be classified"
        );
        // Must NOT claim external review was completed
        assert!(
            !content.contains("external review was completed"),
            "Must not claim external review completed"
        );
        // Must have rationale
        assert!(content.contains("Rationale"), "VJ-1 must have rationale");
    }

    // ---- VJ-2 provider decision ----

    #[test]
    fn vj2_provider_expansion_deferred_matrix_preserved() {
        let content = read_docs_doc("V100_RELEASE_DECISIONS.md");
        assert!(content.contains("VJ-2"));
        // Must NOT claim provider completeness
        assert!(
            !content.contains("provider completeness") || content.to_lowercase().contains("not provider complete"),
            "Must not claim provider completeness"
        );
        // Must mention LM Studio or Z.AI
        assert!(
            content.contains("LM Studio") || content.contains("Z.AI"),
            "Must reference validated providers"
        );
    }

    // ---- VJ-3 Linux GUI decision ----

    #[test]
    fn vj3_linux_gui_partial_accepted_as_caveat() {
        let content = read_docs_doc("V100_RELEASE_DECISIONS.md");
        assert!(content.contains("VJ-3"));
        // Check the VJ-3 section specifically for deferred/partial classification
        let vj3_section = content.split("VJ-3").nth(1).unwrap_or("");
        assert!(
            vj3_section.contains("Deferred") || vj3_section.contains("DEFERRED"),
            "VJ-3 must be classified as deferred"
        );
        assert!(
            vj3_section.contains("Partial") || vj3_section.contains("partial"),
            "VJ-3 must reference partial validation"
        );
        // Must mention partial or deferred
        assert!(
            content.contains("Partial") || content.contains("partial") || content.contains("Deferred"),
            "Must classify as partial or deferred"
        );
    }

    // ---- Decisions do not upgrade caveats ----

    #[test]
    fn decisions_do_not_upgrade_caveats() {
        let content = read_docs_doc("V100_RELEASE_DECISIONS.md");
        assert!(
            content.to_lowercase().contains("does not upgrade"),
            "Must state no caveat upgrade"
        );
    }

    // ---- External review packet refreshed ----

    #[test]
    fn external_review_packet_refreshed_to_v090() {
        let content = read_docs_doc("EXTERNAL_REVIEW_PACKET.md");
        assert!(
            content.contains("v0.9.0"),
            "External review packet should reference v0.9.0"
        );
    }

    // ---- Deferred-007 marked resolved ----

    #[test]
    fn deferred_007_publication_marked_resolved() {
        let content = read_docs_doc("DEFERRED_RISKS.md");
        let section = content.split("DEFERRED-007").nth(1).unwrap_or("");
        assert!(
            section.contains("Resolved") || section.contains("CLOSED"),
            "DEFERRED-007 should be marked resolved or closed"
        );
    }

    // ---- No execution authority added ----

    #[test]
    fn decisions_add_no_execution_authority() {
        let content = read_docs_doc("V100_RELEASE_DECISIONS.md");
        assert!(!content.contains("approve this operation"));
        assert!(!content.contains("bypass policy"));
        assert!(!content.contains("execute tool"));
    }
}
