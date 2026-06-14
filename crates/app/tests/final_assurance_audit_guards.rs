//! Guard tests for final assurance/caveat audit (Wave 118A).

#[cfg(test)]
mod final_assurance_audit_guards {
    use std::path::PathBuf;

    fn workspace_root() -> PathBuf {
        let p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        p.parent()
            .and_then(|p| p.parent())
            .map(|p| p.to_path_buf())
            .unwrap_or_else(|| PathBuf::from(".."))
    }

    fn read_doc(name: &str) -> String {
        let p = workspace_root().join(name);
        std::fs::read_to_string(&p)
            .unwrap_or_else(|_| panic!("Failed to read {}", p.display()))
    }

    fn read_docs_doc(name: &str) -> String {
        let p = workspace_root().join("docs").join(name);
        std::fs::read_to_string(&p)
            .unwrap_or_else(|_| panic!("Failed to read {}", p.display()))
    }

    // ---- Audit document exists ----

    #[test]
    fn final_assurance_audit_document_exists() {
        let content = read_docs_doc("FINAL_ASSURANCE_AUDIT.md");
        assert!(content.contains("Final Assurance/Caveat Audit"));
        assert!(content.contains("Wave 118A"));
    }

    // ---- Overclaiming checks: no release note may claim production-ready ----

    #[test]
    fn no_release_note_overclaims_production_ready() {
        for f in [
            "RELEASE_NOTES.md",
            "RELEASE_NOTES_BETA.md",
            "RELEASE_NOTES_v020_BETA.md",
            "RELEASE_NOTES_v020_RC1.md",
            "RELEASE_NOTES_v020_STABLE.md",
            "RELEASE_NOTES_v030_STABLE.md",
            "RELEASE_NOTES_v040_STABLE.md",
            "RELEASE_NOTES_v050_STABLE.md",
            "RELEASE_NOTES_v060_STABLE.md",
            "RELEASE_NOTES_v070_STABLE.md",
            "RELEASE_NOTES_v080_STABLE.md",
            "RELEASE_NOTES_v090_STABLE.md",
        ] {
            let content = read_doc(f);
            // Every release note must contain "not production-ready" or "Not production-ready"
            assert!(
                content.to_lowercase().contains("not production-ready")
                    || content.to_lowercase().contains("not production ready"),
                "{} must disclaim production readiness",
                f
            );
        }
    }

    // ---- Caveat consistency: immutability caveat in all v0.5+ notes ----

    #[test]
    fn immutability_caveat_present_in_v05_plus() {
        for f in [
            "RELEASE_NOTES_v050_STABLE.md",
            "RELEASE_NOTES_v060_STABLE.md",
            "RELEASE_NOTES_v070_STABLE.md",
            "RELEASE_NOTES_v080_STABLE.md",
            "RELEASE_NOTES_v090_STABLE.md",
        ] {
            let content = read_doc(f);
            let lower = content.to_lowercase();
            assert!(
                lower.contains("not physical") || lower.contains("not fully immutable")
                    || lower.contains("not claim full cryptographic immutability")
                    || lower.contains("physical immutability"),
                "{} must mention immutability caveat",
                f
            );
        }
    }

    // ---- Audit does not upgrade caveats into assurances ----

    #[test]
    fn audit_does_not_upgrade_caveats() {
        let content = read_docs_doc("FINAL_ASSURANCE_AUDIT.md");
        assert!(
            content.contains("does not upgrade any caveat into an assurance")
                || content.contains("does not upgrade"),
            "Audit must state it does not upgrade caveats"
        );
    }

    // ---- Claim ledger contains both claims and caveats ----

    #[test]
    fn audit_ledger_has_claims_and_caveats() {
        let content = read_docs_doc("FINAL_ASSURANCE_AUDIT.md");
        // Must have a claims section
        assert!(content.contains("Claims OpenWand MAY Make") || content.contains("Evidence-Backed"));
        // Must have a caveats section
        assert!(content.contains("Claims OpenWand MUST NOT Make") || content.contains("Caveats"));
        // Must have finding classifications
        assert!(content.contains("STALE") || content.contains("stale"));
        assert!(content.contains("Overclaiming"));
        assert!(content.contains("Contradiction"));
    }

    // ---- No execution authority added ----

    #[test]
    fn audit_adds_no_execution_authority() {
        let content = read_docs_doc("FINAL_ASSURANCE_AUDIT.md");
        // Should not contain instructions to execute tools, approve, or bypass policy
        assert!(!content.contains("approve this operation"));
        assert!(!content.contains("bypass policy"));
        assert!(!content.contains("execute tool"));
    }

    // ---- Zero overclaiming conclusion ----

    #[test]
    fn audit_concludes_zero_overclaiming() {
        let content = read_docs_doc("FINAL_ASSURANCE_AUDIT.md");
        assert!(
            content.contains("Zero overclaiming"),
            "Audit must conclude zero overclaiming"
        );
    }

    // ---- External review caveat preserved ----

    #[test]
    fn external_review_caveat_preserved() {
        let content = read_docs_doc("FINAL_ASSURANCE_AUDIT.md");
        // Must list "Not externally reviewed" as a caveat
        assert!(
            content.contains("Not externally reviewed") || content.contains("externally reviewed"),
            "Audit must preserve external review caveat"
        );
    }
}
