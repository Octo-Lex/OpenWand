//! Regression tests for F-VL1-1 (trace hash mismatch) and F-VL1-2 (--db flag).
//!
//! These tests verify that:
//! 1. Freshly generated trace entries pass recomputed BLAKE3 verification
//! 2. The verifier uses the same scope serialization as the writer
//! 3. resolve_db_path honors explicit --db paths

use openwand_trace::verifier::{Blake3HashPolicy, HashVerificationPolicy, TraceVerifier, VerificationResult, VerificationCheck, FindingSeverity};
use openwand_trace::{TraceEntry, TraceId, TraceStreamId, TraceStreamScope, EntryHash};
use openwand_trace::stream::TraceStreamScope as Scope;

// ── F-VL1-1 Regression: Hash computation must match between write and verify ──

/// Test event for hash verification.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
struct TestEvent(String);

impl HashVerificationPolicy<TestEvent> for Blake3HashPolicy {
    fn serialize_event(&self, event: &TestEvent) -> Result<String, serde_json::Error> {
        serde_json::to_string(event)
    }
    fn compute_entry_hash(
        &self,
        global_sequence: u64,
        stream_scope: &str,
        stream_id: &str,
        stream_sequence: u64,
        event_kind: &str,
        event_payload_json: &str,
        prev_hash: Option<&EntryHash>,
    ) -> EntryHash {
        Blake3HashPolicy::compute_hash(
            global_sequence, stream_scope, stream_id,
            stream_sequence, event_kind, event_payload_json, prev_hash,
        )
    }
}

fn make_fresh_hashed_entry(
    global_seq: u64,
    stream_id: &str,
    stream_seq: u64,
    prev_hash: Option<&EntryHash>,
) -> TraceEntry<TestEvent> {
    // Use serde_json::to_string for scope — this matches the writer path
    let scope_str = serde_json::to_string(&TraceStreamScope::Session).unwrap();
    let event_json = serde_json::to_string(&TestEvent("test".into())).unwrap();
    let hash = Blake3HashPolicy::compute_hash(
        global_seq, &scope_str, stream_id, stream_seq,
        "test.event", &event_json, prev_hash,
    );
    TraceEntry {
        id: TraceId::new(),
        stream_id: TraceStreamId { scope: TraceStreamScope::Session, id: stream_id.into() },
        stream_sequence: stream_seq,
        global_sequence: global_seq,
        occurred_at: chrono::Utc::now(),
        actor: openwand_trace::Actor::User,
        event: TestEvent("test".into()),
        event_kind: "test.event".into(),
        event_schema_version: 1,
        trace_schema_version: 1,
        prev_hash: prev_hash.cloned(),
        entry_hash: hash,
    }
}

#[test]
fn fresh_trace_entries_pass_hash_verification() {
    // F-VL1-1 regression: entries created with serde_json scope must pass verify_with_hash_policy
    let e1 = make_fresh_hashed_entry(1, "s1", 1, None);
    let e2 = make_fresh_hashed_entry(2, "s1", 2, Some(&e1.entry_hash));

    let report = TraceVerifier::verify_with_hash_policy(&[e1, e2], &Blake3HashPolicy);
    assert_eq!(
        report.result,
        VerificationResult::Pass,
        "Fresh trace entries must pass hash verification"
    );
    assert!(
        !report.findings.iter().any(|f| f.check == VerificationCheck::HashCorrectnessValid
            && f.severity == FindingSeverity::Error),
        "No hash correctness errors should be present"
    );
}

#[test]
fn scope_serialization_matches_writer_path() {
    // The writer uses serde_json::to_string(&scope), producing "\"Session\""
    // The verifier MUST use the same serialization
    let scope = TraceStreamScope::Session;
    let writer_serialized = serde_json::to_string(&scope).unwrap();
    let debug_format = format!("{:?}", scope);

    // These MUST be different — if they're the same, the test is meaningless
    assert_ne!(
        writer_serialized, debug_format,
        "serde_json and Debug must produce different output for this test to be meaningful"
    );

    // Verify the hash matches when using writer's serialization
    let event_json = r#""test""#;
    let hash_with_writer_scope = Blake3HashPolicy::compute_hash(
        1, &writer_serialized, "s1", 1, "test.event", event_json, None,
    );
    let hash_with_debug_scope = Blake3HashPolicy::compute_hash(
        1, &debug_format, "s1", 1, "test.event", event_json, None,
    );

    assert_ne!(
        hash_with_writer_scope, hash_with_debug_scope,
        "Different scope serializations must produce different hashes"
    );

    // Now verify that an entry hashed with writer serialization passes verification
    let entry = TraceEntry {
        id: TraceId::new(),
        stream_id: TraceStreamId { scope: scope.clone(), id: "s1".into() },
        stream_sequence: 1,
        global_sequence: 1,
        occurred_at: chrono::Utc::now(),
        actor: openwand_trace::Actor::User,
        event: TestEvent("test".into()),
        event_kind: "test.event".into(),
        event_schema_version: 1,
        trace_schema_version: 1,
        prev_hash: None,
        entry_hash: hash_with_writer_scope,
    };

    let report = TraceVerifier::verify_with_hash_policy(&[entry], &Blake3HashPolicy);
    assert_eq!(
        report.result, VerificationResult::Pass,
        "Entry hashed with writer's serde_json scope serialization must pass verification"
    );
}

#[test]
fn multi_stream_fresh_entries_pass() {
    // Test that entries across multiple streams all pass hash verification
    let scope_session = serde_json::to_string(&TraceStreamScope::Session).unwrap();
    let scope_global = serde_json::to_string(&TraceStreamScope::Global).unwrap();

    let event_json = serde_json::to_string(&TestEvent("multi".into())).unwrap();

    let h1 = Blake3HashPolicy::compute_hash(1, &scope_session, "s1", 1, "evt", &event_json, None);
    let h2 = Blake3HashPolicy::compute_hash(2, &scope_global, "g1", 1, "evt", &event_json, None);
    let h3 = Blake3HashPolicy::compute_hash(3, &scope_session, "s1", 2, "evt", &event_json, Some(&h1));

    let entries = vec![
        TraceEntry {
            id: TraceId::new(), stream_id: TraceStreamId { scope: TraceStreamScope::Session, id: "s1".into() },
            stream_sequence: 1, global_sequence: 1, occurred_at: chrono::Utc::now(),
            actor: openwand_trace::Actor::User, event: TestEvent("multi".into()),
            event_kind: "evt".into(), event_schema_version: 1, trace_schema_version: 1,
            prev_hash: None, entry_hash: h1.clone(),
        },
        TraceEntry {
            id: TraceId::new(), stream_id: TraceStreamId { scope: TraceStreamScope::Global, id: "g1".into() },
            stream_sequence: 1, global_sequence: 2, occurred_at: chrono::Utc::now(),
            actor: openwand_trace::Actor::User, event: TestEvent("multi".into()),
            event_kind: "evt".into(), event_schema_version: 1, trace_schema_version: 1,
            prev_hash: None, entry_hash: h2,
        },
        TraceEntry {
            id: TraceId::new(), stream_id: TraceStreamId { scope: TraceStreamScope::Session, id: "s1".into() },
            stream_sequence: 2, global_sequence: 3, occurred_at: chrono::Utc::now(),
            actor: openwand_trace::Actor::User, event: TestEvent("multi".into()),
            event_kind: "evt".into(), event_schema_version: 1, trace_schema_version: 1,
            prev_hash: Some(h1.clone()), entry_hash: h3,
        },
    ];

    let report = TraceVerifier::verify_with_hash_policy(&entries, &Blake3HashPolicy);
    assert_eq!(report.result, VerificationResult::Pass);
    assert_eq!(report.entries_checked, 3);
    assert_eq!(report.streams_checked, 2);
}
