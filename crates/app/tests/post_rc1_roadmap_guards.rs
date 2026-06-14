//! Guard tests for post-rc.1 roadmap reset (Wave 121A).

#[cfg(test)]
mod post_rc1_roadmap_guards {
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

    // ---- Roadmap document exists ----

    #[test]
    fn final_roadmap_document_exists() {
        let content = read_docs_doc("V100_FINAL_ROADMAP.md");
        assert!(content.contains("v1.0.0 Final Roadmap"));
        assert!(content.contains("Wave 121A"));
    }

    // ---- VK blockers defined ----

    #[test]
    fn vk_blockers_defined() {
        let content = read_docs_doc("V100_FINAL_ROADMAP.md");
        assert!(content.contains("VK-1"));
        assert!(content.contains("VK-2"));
        assert!(content.contains("VK-3"));
        assert!(content.contains("VK-4"));
        assert!(content.contains("VK-5"));
    }

    // ---- VK-1 and VK-2 are P0 gates ----

    #[test]
    fn vk1_and_vk2_are_p0_gates() {
        let content = read_docs_doc("V100_FINAL_ROADMAP.md");
        let vk1_section = content.split("VK-1").nth(1).unwrap_or("");
        let vk2_section = content.split("VK-2").nth(1).unwrap_or("");
        assert!(
            vk1_section.contains("P0") || content.contains("VK-1 | External review execution | P0"),
            "VK-1 must be P0 gate"
        );
        assert!(
            vk2_section.contains("P0") || content.contains("VK-2 | rc.1 soak / regression window | P0"),
            "VK-2 must be P0 gate"
        );
    }

    // ---- Core recommendation present ----

    #[test]
    fn core_recommendation_present() {
        let content = read_docs_doc("V100_FINAL_ROADMAP.md");
        assert!(
            content.contains("VK-1 + VK-2") || content.contains("VK-1 and VK-2"),
            "Roadmap must state VK-1+VK-2 as core"
        );
    }

    // ---- Does not jump from rc.1 to final ----

    #[test]
    fn does_not_skip_classification_before_final() {
        let content = read_docs_doc("V100_FINAL_ROADMAP.md");
        assert!(
            content.contains("Do NOT jump") || content.contains("Do not jump"),
            "Must warn against jumping from rc.1 to final"
        );
    }

    // ---- Caveats preserved ----

    #[test]
    fn caveats_preserved() {
        let content = read_docs_doc("V100_FINAL_ROADMAP.md");
        let lower = content.to_lowercase();
        assert!(
            lower.contains("production") && (lower.contains("not claimed") || lower.contains("requires") || lower.contains("not production")),
            "Must preserve production-readiness caveat"
        );
        assert!(
            lower.contains("api") && (lower.contains("not guaranteed") || lower.contains("may begin") || lower.contains("not stable")),
            "Must preserve API stability caveat"
        );
    }

    // ---- No execution authority added ----

    #[test]
    fn roadmap_adds_no_execution_authority() {
        let content = read_docs_doc("V100_FINAL_ROADMAP.md");
        assert!(!content.contains("approve this operation"));
        assert!(!content.contains("bypass policy"));
    }

    // ---- Deferred items carried ----

    #[test]
    fn deferred_items_carried() {
        let content = read_docs_doc("V100_FINAL_ROADMAP.md");
        assert!(content.contains("External review execution"));
        assert!(content.contains("Linux GUI visual"));
        assert!(content.contains("Direct OpenAI") || content.contains("provider"));
    }
}
