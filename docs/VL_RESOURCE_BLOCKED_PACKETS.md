# Resource-Blocked Validation Packet — Wave 133A

**Date:** 2026-06-15
**Wave:** 133A
**Commit:** (recorded at lock)
**Tag:** `wave-133a-lock`

---

## Purpose

This document packages the two remaining VL blockers that cannot be
resolved without external resources:

1. **VL-2**: External human review execution
2. **VL-5**: Linux GUI visual validation

For each blocker, this packet defines:
- Required environment
- Exact commands
- Expected artifacts
- Evidence files to collect
- Pass / partial / fail / blocked classification criteria
- Affected caveats
- Claims that may change only if evidence passes

**This document does not claim either blocker resolved. It makes them
ready to execute when the resource becomes available.**

---

## Part 1: VL-2 — External Review Execution Packet

### 1.1 Required Environment

| Item | Requirement |
|------|-------------|
| Reviewer | Independent third party — not the project builder or build agent |
| System | Windows 10+ or Linux x86_64 |
| Rust | 1.96+ (or use pre-built binaries) |
| Network | Internet access to clone the repo |
| LLM Provider | Any OpenAI-compatible endpoint (LM Studio, Z.AI, OpenAI, Ollama) |
| Time | ~30 minutes for full review |

### 1.2 Reviewer Independence Criteria

A valid external reviewer must be:
- **Not** the project developer
- **Not** Craft Agent (the build agent)
- **Not** an LLM API used in the development pipeline

A valid external reviewer may be:
- A colleague or peer developer
- A security consultant
- An open-source contributor
- An academic reviewer
- A community member who clones the repo independently

### 1.3 Exact Commands

```bash
# ── Step 1: Clone and build ──
git clone https://github.com/Octo-Lex/OpenWand.git
cd OpenWand
git checkout v1.0.1

# Build CLI binary
cargo build --release --bin openwand

# Verify version
./target/release/openwand --version
# Expected: openwand 1.0.1

# Verify artifact identity (Windows)
certutil -hashfile target/release/openwand.exe SHA256
# Expected: 5ED051CAFF4534F372B6ABF10D1263422F3CE1357814A121DF4822848857ECF5

# ── Step 2: Generate a test session ──
# (Requires an LLM endpoint — replace URL/model as needed)
./target/release/openwand --base-url http://localhost:1234/v1 "Hello, what is 2+2?"

# ── Step 3: Verify trace integrity ──
./target/release/openwand trace-verify <session-id> --db openwand.db
# Expected: exit 0, Result: Pass

# ── Step 4: Write an external anchor ──
./target/release/openwand anchor-write <session-id> --db openwand.db \
  --anchor-root /tmp/openwand-anchors

# ── Step 5: Verify the anchor ──
./target/release/openwand anchor-verify <session-id> --db openwand.db \
  --anchor /tmp/openwand-anchors/openwand-checkpoint-1.json
# Expected: exit 0, Pass, Current

# ── Step 6: Operation replay ──
echo '{"operations":[]}' > ops.json
./target/release/openwand operation-replay --session <session-id> \
  --db openwand.db --operations ops.json
# Expected: exit 0, Pass

# ── Step 7: Evidence report ──
./target/release/openwand evidence-report <session-id> \
  --db openwand.db \
  --operations ops.json \
  --anchor /tmp/openwand-anchors/openwand-checkpoint-1.json \
  --output review_report.json
# Expected: Complete

# ── Step 8: Guided review (chains all steps) ──
./target/release/openwand review <session-id> \
  --db openwand.db \
  --operations ops.json \
  --anchor /tmp/openwand-anchors/openwand-checkpoint-1.json \
  --output guided_review.json
# Expected: Complete

# ── Step 9: Review documentation ──
cat docs/AUTHORITY_REVIEW.md
cat docs/SECURITY_SCAN_RESULTS.md
cat docs/API_STABILITY_POLICY.md
cat docs/VL4_PROVIDER_EXPANSION_DECISION.md
```

### 1.4 Expected Artifacts

| Artifact | File | Format |
|----------|------|--------|
| Trace verification output | Console stdout + exit code | Text |
| Anchor write output | `openwand-checkpoint-1.json` | JSON |
| Anchor verification output | Console stdout + exit code | Text |
| Evidence report | `review_report.json` | JSON |
| Guided review report | `guided_review.json` | JSON |

### 1.5 Evidence Files to Collect

The reviewer should collect and submit:

1. **Reviewer identity** — name/role/affiliation (or pseudonymous handle)
2. **Environment description** — OS, Rust version, provider used
3. **Console output** for each command (stdout + exit codes)
4. **Generated JSON files** (review_report.json, guided_review.json)
5. **Reviewer findings** — any defects, observations, or concerns
6. **Overall classification** — Pass / Pass with caveats / Fail / Blocked

### 1.6 Classification Criteria

| Result | Criteria |
|--------|----------|
| **Pass** | All commands exit 0. Evidence report = Complete. No defects found. |
| **Pass with caveats** | All verification commands pass. Reviewer notes non-blocking observations. |
| **Partial** | Some commands pass, some fail. Specific failures documented. |
| **Fail** | Trace verification fails, evidence report = Incomplete, or blocking defect found. |
| **Blocked** | Reviewer cannot execute (build fails, no provider, environment incompatible). |

### 1.7 Affected Caveats

| Caveat | Current Status | If Pass | If Partial/Fail |
|--------|---------------|---------|-----------------|
| X-09 | Deferred | → Resolved (external review executed) | → Updated with findings |
| X-15 | Deferred | → Resolved (external review executed) | → Updated with findings |

### 1.8 Claims That May Change

If the reviewer classifies as **Pass** or **Pass with caveats**:
- Claim C-19 (external review packet exists) → strengthened with execution evidence
- Caveat X-09 → may be resolved or narrowed
- Caveat X-15 → may be resolved or narrowed

If the reviewer classifies as **Fail**:
- Any findings are filed as defects
- Relevant claims may be downgraded
- New caveats may be added

**Claims do NOT change without real evidence.**

---

## Part 2: VL-5 — Linux GUI Visual Validation Packet

### 2.1 Required Environment

The reviewer needs a Linux environment with a **real GPU display pipeline**.
The virtualized GPU environments tested in Wave 109A cannot capture WebKit
compositing output.

| Item | Requirement |
|------|-------------|
| Platform | Linux x86_64 (physical or cloud with GPU) |
| Display | Physical monitor OR remote desktop with GPU compositing |
| GPU | Physical GPU (Intel/AMD/NVIDIA) OR GPU passthrough to VM |
| Compositor | GNOME (Mutter), KDE (KWin), or other full compositor |
| GTK3 | 3.24+ |
| webkit2gtk-4.1 | 2.44+ |
| Rust | 1.96+ |
| Screenshot tool | `gnome-screenshot`, `spectacle`, `scrot`, or `import` |

### 2.2 Approved Environment Configurations

| Config | Description | Expected to Work? |
|--------|-------------|-------------------|
| Physical Linux desktop | Native hardware with monitor | ✅ Best option |
| Cloud VM with GPU passthrough | AWS g4dn, GCP T4, Azure NV-series | ✅ Should work |
| VNC with full desktop env | GNOME/KDE via VNC | ✅ Should work |
| Proxmox VM with virtio-gpu | (already tested — 109A) | ❌ WebKit compositing not capturable |
| WSL2 with Xvfb | (already tested — 109A) | ❌ Blank screenshot |

### 2.3 Exact Commands

```bash
# ── Step 1: Clone and build ──
git clone https://github.com/Octo-Lex/OpenWand.git
cd OpenWand
git checkout v1.0.1

# Install Linux dependencies (Ubuntu/Debian)
sudo apt-get install -y libgtk-3-dev libwebkit2gtk-4.1-dev libssl-dev

# Build desktop binary
cargo build --release --bin openwand-ui --features desktop

# ── Step 2: Verify version ──
./target/release/openwand-ui --version
# Expected: openwand 1.0.1

# ── Step 3: Launch desktop UI ──
# (Run in a graphical session — not SSH without X forwarding)
./target/release/openwand-ui &

# Wait for window to appear
sleep 3

# ── Step 4: Capture screenshot ──
# Option A: gnome-screenshot
gnome-screenshot --file=openwand-gui.png

# Option B: spectacle (KDE)
spectacle -b -f -o openwand-gui.png

# Option C: scrot
scrot openwand-gui.png

# Option D: ImageMagick import
import -window root openwand-gui.png

# ── Step 5: Verify window exists ──
xdotool search --name "openwand"
# Expected: window ID returned

# ── Step 6: Capture window geometry ──
xdotool getwindowgeometry $(xdotool search --name "openwand" | head -1)
# Expected: non-trivial size (not 10x10)

# ── Step 7: Interactive UI test (manual) ──
# 1. Click on the text input area
# 2. Type a message
# 3. Verify text appears in the input
# 4. Submit (Enter or click send)
# 5. Verify agent response appears
# 6. Switch tabs (if multiple tabs exist)
# 7. Verify tab content changes

# Record observations for each step.
```

### 2.4 Expected Artifacts

| Artifact | File | Format |
|----------|------|--------|
| Screenshot of GUI | `openwand-gui.png` | PNG image |
| Window geometry output | Console stdout | Text |
| Interactive UI test log | Manual notes | Text/Markdown |

### 2.5 Evidence Files to Collect

1. **Environment description** — distro, kernel, GPU, compositor, display type
2. **Screenshot(s)** — at minimum: initial window, after interaction
3. **Window geometry** — proving non-trivial window size
4. **Interactive UI observations** — click/type/submit/tab results
5. **Stability notes** — any crashes, hangs, or rendering glitches
6. **Overall classification** — Pass / Partial / Fail / Blocked

### 2.6 Classification Criteria

| Result | Criteria |
|--------|----------|
| **Pass** | Screenshot shows rendered GUI content. Interactive UI works (click, type, submit). No crashes. Window size > 200x200. |
| **Partial** | Screenshot shows partial rendering. Some interactive elements work. OR: rendering captured but interaction not tested. |
| **Fail** | Blank/black screenshot. Window not created. Immediate crash. No interactive elements. |
| **Blocked** | Cannot build (missing deps). Cannot launch (no display). Compositor incompatible. |

### 2.7 Affected Caveats

| Caveat | Current Status | If Pass | If Partial/Fail |
|--------|---------------|---------|-----------------|
| X-05 | Not full cross-platform | → Narrowed for Linux | → Unchanged |
| X-06 | Not full Linux GUI | → Resolved for visual rendering | → Updated with findings |

### 2.8 Claims That May Change

If classification is **Pass**:
- Claim C-07 (Linux desktop binary compiles) → strengthened
- Claim C-08 (Linux runtime init) → strengthened with visual evidence
- Caveat X-06 → resolved for Linux visual rendering
- Caveat X-05 → narrowed (Linux fully validated, macOS still not)

If classification is **Partial**:
- Caveat X-06 → updated with partial visual evidence
- New claim may be added for the specific aspects that passed

If classification is **Fail**:
- Defects filed
- Caveats unchanged

**Claims do NOT change without real evidence.**

---

## Part 3: State Summary

### Current VL Posture (after 133A)

```text
VL-1  First real workflow evidence        ✅ pass post-fix
VL-2  External review execution           deferred — packet ready (133A)
VL-3  Maintenance patch criteria          ✅ resolved
VL-4  Provider expansion                  ✅ partially resolved
VL-5  Linux GUI visual validation         deferred — packet ready (133A)
VL-6  API stability policy                ✅ resolved
```

### What This Wave Does

- Makes VL-2 **reviewer-ready**: any third party can execute the review
  by following the packet
- Makes VL-5 **environment-ready**: any tester with a Linux GPU can
  execute the visual validation by following the packet
- Documents exact commands, expected outputs, evidence requirements,
  and classification criteria for both
- Records what caveats and claims may change, and only with evidence

### What This Wave Does NOT Do

- Does not claim VL-2 resolved
- Does not claim VL-5 resolved
- Does not claim external review executed
- Does not claim Linux GUI validated
- Does not claim production readiness
- Does not claim formal certification
- Does not claim provider completeness
- Does not claim global API stability
- Does not claim physical immutability
- Does not claim remote attestation
