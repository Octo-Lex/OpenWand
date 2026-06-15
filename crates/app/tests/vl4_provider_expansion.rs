//! VL-4 Provider Expansion Decision guard tests (Wave 132A).

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
fn decision_document_exists() {
    let doc = read_docs_doc("VL4_PROVIDER_EXPANSION_DECISION.md");
    assert!(!doc.is_empty());
}

#[test]
fn decision_references_wave_132a() {
    let doc = read_docs_doc("VL4_PROVIDER_EXPANSION_DECISION.md");
    assert!(doc.contains("132A"));
}

// ── Provider Classification ──────────────────────────────────────────────────

#[test]
fn decision_records_zai_validated() {
    let doc = read_docs_doc("VL4_PROVIDER_EXPANSION_DECISION.md");
    assert!(doc.contains("Z.AI") && doc.contains("Confirmed"));
}

#[test]
fn decision_records_lm_studio_validated() {
    let doc = read_docs_doc("VL4_PROVIDER_EXPANSION_DECISION.md");
    assert!(doc.contains("LM Studio"));
}

#[test]
fn decision_records_openai_unvalidated() {
    let doc = read_docs_doc("VL4_PROVIDER_EXPANSION_DECISION.md");
    assert!(doc.contains("OpenAI") && doc.contains("No API key"));
}

#[test]
fn decision_records_anthropic_unvalidated() {
    let doc = read_docs_doc("VL4_PROVIDER_EXPANSION_DECISION.md");
    assert!(doc.contains("Anthropic") && doc.contains("No API key"));
}

#[test]
fn decision_records_ollama_unvalidated() {
    let doc = read_docs_doc("VL4_PROVIDER_EXPANSION_DECISION.md");
    assert!(doc.contains("Ollama") && doc.contains("No local"));
}

// ── Adapter Analysis ─────────────────────────────────────────────────────────

#[test]
fn decision_analyzes_openai_compatible_adapter() {
    let doc = read_docs_doc("VL4_PROVIDER_EXPANSION_DECISION.md");
    assert!(doc.contains("OpenAiCompatible"));
}

#[test]
fn decision_analyzes_anthropic_adapter() {
    let doc = read_docs_doc("VL4_PROVIDER_EXPANSION_DECISION.md");
    assert!(doc.contains("AnthropicCompatible"));
}

#[test]
fn decision_records_zai_api_evidence() {
    let doc = read_docs_doc("VL4_PROVIDER_EXPANSION_DECISION.md");
    assert!(
        doc.contains("PROVIDER_VALIDATION_OK"),
        "Decision must record Z.AI API validation evidence"
    );
}

// ── ProviderKind enum in source ──────────────────────────────────────────────

#[test]
fn provider_kind_enum_has_four_variants() {
    let content = read_root_doc("crates/llm/src/provider_config.rs");
    for kind in &["OpenAiCompatible", "AnthropicCompatible", "LocalOpenAiCompatible", "Mock"] {
        assert!(
            content.contains(kind),
            "ProviderKind must include {}",
            kind
        );
    }
}

#[test]
fn llm_provider_enum_has_multiple_providers() {
    let content = read_root_doc("crates/llm/src/request.rs");
    for provider in &["OpenAI", "Anthropic", "Ollama", "OpenRouter"] {
        assert!(
            content.contains(provider),
            "LlmProvider must include {}",
            provider
        );
    }
}

// ── Caveat X-07 ──────────────────────────────────────────────────────────────

#[test]
fn decision_narrows_x07_partially() {
    let doc = read_docs_doc("VL4_PROVIDER_EXPANSION_DECISION.md");
    assert!(
        doc.contains("X-07") && doc.contains("PARTIALLY NARROWED"),
        "Decision must state X-07 is partially narrowed"
    );
}

#[test]
fn decision_does_not_resolve_x07_fully() {
    let doc = read_docs_doc("VL4_PROVIDER_EXPANSION_DECISION.md");
    assert!(
        !doc.contains("X-07 is resolved") && !doc.contains("X-07 is fully resolved"),
        "Decision must not claim X-07 is fully resolved"
    );
}

// ── Classification ───────────────────────────────────────────────────────────

#[test]
fn decision_classifies_as_partially_resolved() {
    let doc = read_docs_doc("VL4_PROVIDER_EXPANSION_DECISION.md");
    assert!(
        doc.contains("Partially Resolved") || doc.contains("partially resolved"),
        "Decision must classify VL-4 as partially resolved"
    );
}

// ── No Overclaiming ─────────────────────────────────────────────────────────

#[test]
fn decision_does_not_claim_provider_completeness() {
    let doc = read_docs_doc("VL4_PROVIDER_EXPANSION_DECISION.md");
    assert!(
        doc.contains("Not provider completeness") || doc.contains("not provider completeness"),
        "Decision must disclaim provider completeness"
    );
}

#[test]
fn decision_does_not_claim_production_ready() {
    let doc = read_docs_doc("VL4_PROVIDER_EXPANSION_DECISION.md");
    for line in doc.lines() {
        let lower = line.to_lowercase();
        if lower.contains("production ready") || lower.contains("production-ready") {
            assert!(
                lower.contains("not ") || lower.contains("does not"),
                "Must not claim production readiness: {}",
                line
            );
        }
    }
}

#[test]
fn decision_provides_resolution_path() {
    let doc = read_docs_doc("VL4_PROVIDER_EXPANSION_DECISION.md");
    assert!(
        doc.contains("Path to Full Resolution") || doc.contains("path to"),
        "Decision must provide a path to full resolution"
    );
}
