//! Guard tests for release-check command (Wave 115A).

#[cfg(test)]
mod release_check_guards {
    use std::path::PathBuf;

    fn main_rs_path() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("src")
            .join("main.rs")
    }

    fn release_check_body(content: &str) -> &str {
        let start = content.find("async fn cmd_release_check").unwrap();
        let search_area = &content[start..];
        let marker = "\n\nfn cmd_eval_task_plan";
        let end_rel = search_area.find(marker).unwrap_or(5000);
        &search_area[..end_rel]
    }

    #[test]
    fn release_check_subcommand_exists() {
        let content = std::fs::read_to_string(main_rs_path()).unwrap();
        assert!(content.contains("name = \"release-check\""));
        assert!(content.contains("Commands::ReleaseCheck"));
    }

    #[test]
    fn release_check_function_exists() {
        let content = std::fs::read_to_string(main_rs_path()).unwrap();
        assert!(content.contains("cmd_release_check"));
    }

    #[test]
    fn does_not_publish_tag_or_push() {
        let content = std::fs::read_to_string(main_rs_path()).unwrap();
        let body = release_check_body(&content);
        let filtered: String = body.lines()
            .filter(|l| !l.contains("does NOT") && !l.contains("does not"))
            .collect::<Vec<_>>()
            .join("\n");
        assert!(!filtered.contains("git push"));
        assert!(!filtered.contains("git tag"));
        assert!(!filtered.contains("publish"));
    }

    #[test]
    fn includes_manual_required() {
        let content = std::fs::read_to_string(main_rs_path()).unwrap();
        let body = release_check_body(&content);
        assert!(body.contains("manual_items") || body.contains("ManualRequired"));
        assert!(body.contains("manual"));
    }

    #[test]
    fn includes_non_claims() {
        let content = std::fs::read_to_string(main_rs_path()).unwrap();
        let body = release_check_body(&content);
        assert!(body.contains("non_claims") || body.contains("does NOT claim"));
        assert!(body.contains("production-ready") || body.contains("production ready"));
    }

    #[test]
    fn includes_desktop_binary_gate() {
        let content = std::fs::read_to_string(main_rs_path()).unwrap();
        let body = release_check_body(&content);
        assert!(body.contains("openwand-ui"));
    }

    #[test]
    fn produces_structured_output() {
        let content = std::fs::read_to_string(main_rs_path()).unwrap();
        let body = release_check_body(&content);
        assert!(body.contains("ReleaseCheckItem"));
        assert!(body.contains("Pass"));
        assert!(body.contains("Fail"));
    }

    #[test]
    fn exits_nonzero_on_failure() {
        let content = std::fs::read_to_string(main_rs_path()).unwrap();
        let body = release_check_body(&content);
        assert!(body.contains("exit(1)"));
    }
}
