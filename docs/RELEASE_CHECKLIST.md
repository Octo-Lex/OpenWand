# OpenWand Release Checklist

**Version:** 1.0 (Wave 111A)
**Applies to:** All releases from v0.8.0 onward

---

## Purpose

This document defines the repeatable release process for OpenWand. Every
release — alpha, beta, RC, or stable — must pass every item on this checklist
before the release tag is created.

This checklist is **process evidence**, not a production-readiness claim.
Passing all items means the release process was followed; it does not mean
the software is production-ready.

---

## Pre-Release Checklist

### 1. Test Baseline

```bash
cargo test --workspace --all-targets -q
```

- [ ] All tests pass (0 failures)
- [ ] Record total test count for release notes
- [ ] No flaky tests (re-run if any test fails intermittently)

### 2. Production Clippy

```bash
# All non-app production crates
cargo clippy --release -p openwand-core -p openwand-session -p openwand-llm \
  -p openwand-policy -p openwand-tools -p openwand-store -p openwand-trace \
  -p openwand-memory -p openwand-skills -p openwand-goals -p openwand-workflow \
  -p openwand-content -- -D warnings
```

- [ ] 0 warnings across all production crates
- [ ] No new clippy lints introduced

### 3. Dependency Audit

```bash
cargo audit
```

- [ ] Record CVE count (must be 0 for release)
- [ ] Record total dependency count
- [ ] Record upstream-blocked warning count
- [ ] Compare delta from last release (document in release notes)
- [ ] If any CVE found: BLOCK RELEASE

### 4. CLI Binary Build (Windows)

```bash
cargo build --release --bin openwand --features desktop
```

- [ ] Builds without errors
- [ ] Record binary size
- [ ] Record SHA-256 hash: `python3 -c "import hashlib; print(hashlib.sha256(open('target/release/openwand.exe','rb').read()).hexdigest().upper())"`

### 5. Desktop Binary Build (Windows) — CRITICAL

> **This is the critical correction from Wave 109A.**
> The previous build gate only validated `openwand --features desktop`,
> which compiles the CLI binary with desktop feature flags.
> The actual desktop binary is `openwand-ui` (separate `[[bin]]` target).
> Both must be built and must pass.

```bash
# CLI binary with desktop features
cargo build --release --bin openwand --features desktop
# Desktop UI binary (the actual GUI application)
cargo build --release --bin openwand-ui --features desktop
```

- [ ] `openwand` (CLI) builds without errors
- [ ] `openwand-ui` (desktop) builds without errors ← **was never checked before 109A**
- [ ] Both binaries present in `target/release/`

### 6. Linux Compile Validation

```bash
# On Linux (native or WSL2)
cargo build --release --bin openwand
cargo build --release --bin openwand-ui --features desktop
```

- [ ] `openwand` (CLI) compiles on Linux
- [ ] `openwand-ui` (desktop) compiles on Linux
- [ ] Record Linux distro, kernel, GTK, and webkit2gtk versions
- [ ] If runtime validation attempted: record classification (Pass/Partial/Blocked)

### 7. Artifact Identity

- [ ] Record exact binary file size in bytes
- [ ] Record SHA-256 hash (uppercase, no spaces)
- [ ] Verify hash matches what will be in release notes and STATE.md
- [ ] Binary is under 20 MB (HB-G1 constraint)

### 8. Release Notes Consistency

- [ ] `RELEASE_NOTES_vXXX_STABLE.md` exists
- [ ] Contains: theme, new features, metrics (test count, binary size, SHA-256)
- [ ] Contains: VG/VH blocker resolution status
- [ ] Contains: caveats and non-claims section
- [ ] Version number matches the tag being created
- [ ] No affirmative production-readiness claims
- [ ] No "fully secure" or "formally certified" language

### 9. State File Consistency

- [ ] `STATE.md` version matches release version
- [ ] `STATE.md` binary hash/size matches actual artifact
- [ ] `STATE.md` test count matches actual test run
- [ ] `STATE.md` blocker table is current
- [ ] `WAVES.md` has all wave rows up to current

### 10. Tag Consistency

- [ ] `wave-XXXa-lock` tag exists for the release prep wave
- [ ] `vX.Y.Z` tag will be created on the declaration commit
- [ ] `wave-XXXb-lock` tag will be created on the declaration commit
- [ ] Tags are pushed to GitHub after commit
- [ ] No force-push to protected master branch

### 11. Caveat and Non-Claim Review

Verify the release notes and STATE.md do NOT claim:

- [ ] Production readiness
- [ ] Formal security certification
- [ ] Physical immutability
- [ ] Remote attestation
- [ ] Full immutability (rewrite store + anchor)
- [ ] Cross-platform runtime validation (unless actually validated)
- [ ] Full Linux GUI support (unless actually validated)
- [ ] Provider completeness
- [ ] Stable API guarantee

Verify the release notes DO claim only what is evidence-backed:

- [ ] Trace chain + hash verification (with documented limitations)
- [ ] Operation-to-trace correspondence (for new traces)
- [ ] External checkpoint anchor prefix verification
- [ ] Evidence report aggregation (honest sourcing)
- [ ] Authority boundary enforcement (source-level)
- [ ] Zero CVEs (at time of scan)

### 12. Remote Sync

```bash
git push origin master --tags
```

- [ ] Local and remote master are in sync
- [ ] All tags pushed
- [ ] No uncommitted changes in working tree

---

## Release Declaration Steps

After all checklist items pass:

1. **Commit** the release declaration with updated STATE.md and WAVES.md
2. **Tag** with `vX.Y.Z` and `wave-XXXb-lock`
3. **Push** to GitHub
4. **Verify** remote is in sync

```bash
git add -A
git commit -m "Wave XXXB - vX.Y.Z Declaration

Declares vX.Y.Z stable. [Summary of claims and non-claims.]

Tests: N total, 0 failures.
Binary: N bytes (~X MB).
SHA-256: HASH"

git tag vX.Y.Z
git tag wave-XXXb-lock
git push origin master --tags
```

---

## Post-Release

- [ ] Verify tags on GitHub
- [ ] Create GitHub Release (optional, with release notes)
- [ ] Start post-release roadmap reset wave

---

## Defects Found During Release

If any checklist item fails:

1. **Stop the release** — do not create the tag
2. **Fix the issue** in a new commit
3. **Re-run the entire checklist** from the top
4. **Document the defect** in the wave summary

Example from Wave 109A: The desktop binary `openwand-ui` had never been
compiled as part of the build gate. 9 latent compilation errors were found
and fixed. This checklist item (Section 5) now prevents that class of defect.

---

## Process vs. Product

This checklist is **process evidence**. It proves the release was prepared
methodically. It does not prove the product is:

- Production-ready
- Formally secure
- Free of defects
- Suitable for any particular purpose

The checklist reduces the probability of shipping broken builds, inconsistent
metadata, or overclaimed capabilities. It is a necessary but not sufficient
condition for release quality.
