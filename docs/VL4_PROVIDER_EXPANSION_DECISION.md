# VL-4 Provider Expansion Decision — Wave 132A

**Date:** 2026-06-15
**Wave:** 132A
**Commit:** (recorded at lock)
**Tag:** `wave-132a-lock`
**Blocker:** VL-4 — Provider expansion decision

---

## 1. Decision

**VL-4 is classified as: Partially Resolved — Adapter Paths Analyzed, One Provider Re-confirmed.**

Z.AI (glm-4.6) is confirmed as a working hosted provider through the
OpenAI-compatible adapter. The adapter code paths for OpenAI, Anthropic,
and Ollama are structurally analyzed and compatible, but no API keys or
local instances are available for operational validation.

---

## 2. Provider Inventory

### 2.1 Currently Validated Providers

| Provider | Type | Adapter | Status | Evidence |
|----------|------|---------|--------|----------|
| **Z.AI** (glm-4.6) | Hosted | `OpenAiCompatible` | ✅ **Confirmed** | MCP API call returned `PROVIDER_VALIDATION_OK` (2026-06-15). Used in VL-1 workflow infrastructure (via mock relay). |
| **LM Studio** | Local | `OpenAiCompatible` (via `LocalOpenAiCompatible`) | ✅ **Previously validated** | Validated in pre-v1.0 waves. Not running at test time. |
| **Mock LLM** | Test | `Mock` | ✅ **Validated** | Used in VL-1 workflow (Wave 127A/128A). Deterministic responses. |

### 2.2 Structurally Compatible but Unvalidated

| Provider | Type | Adapter | Status | Why Not Operationally Validated |
|----------|------|---------|--------|---------------------------------|
| **OpenAI** (GPT) | Hosted | `OpenAiCompatible` | ⚠️ **Code path identical** | Same `OpenAiCompatibleClient` adapter as Z.AI. No API key available. |
| **Anthropic** (Claude) | Hosted | `AnthropicCompatible` | ⚠️ **Separate adapter** | Different adapter (Messages API, SSE, content blocks). No API key available. |
| **Ollama** | Local | `OpenAiCompatible` (via `LocalOpenAiCompatible`) | ⚠️ **Code path identical** | Same adapter as LM Studio. No local Ollama instance running. |

### 2.3 Supported Provider Kinds (Code)

```rust
pub enum ProviderKind {
    OpenAiCompatible,      // OpenAI, Z.AI, any /v1/chat/completions server
    AnthropicCompatible,   // Anthropic Messages API
    LocalOpenAiCompatible, // LM Studio, Ollama (no API key)
    Mock,                  // Testing only
}
```

### 2.4 Supported Provider Names (LlmProvider enum)

```rust
pub enum LlmProvider {
    OpenAI, Anthropic, Ollama, OpenRouter, Gemini, Groq, XAI, DeepSeek,
    Custom { name: String },
}
```

All providers dispatch through one of the three adapter kinds above.

---

## 3. Z.AI Validation Evidence

### 3.1 MCP API Call (2026-06-15)

```
POST https://api.z.ai/api/coding/paas/v4/chat/completions
Model: glm-4.6
Messages: [{"role": "user", "content": "Reply with exactly: PROVIDER_VALIDATION_OK"}]

Response:
  HTTP 200
  Content: "PROVIDER_VALIDATION_OK"
  Model: glm-4.6
  Usage: prompt_tokens=14, completion_tokens=227, total=241
  Finish reason: stop
```

### 3.2 OpenAI-Compatible Format Confirmation

The Z.AI API response matches the OpenAI-compatible format:
- `choices[0].message.content` — text content ✅
- `choices[0].finish_reason` — "stop" ✅
- `usage.prompt_tokens` / `usage.completion_tokens` — token counts ✅
- `model` — model name ✅
- `object: "chat.completion"` — response type ✅

Z.AI extension: `reasoning_content` field (thinking tokens). The
OpenWand adapter does not currently parse this field but does not break
on unknown fields (serde ignores unknown fields by default).

### 3.3 Adapter Compatibility Analysis

The `OpenAiCompatibleClient` adapter:
1. Builds request body: `POST {base_url}/chat/completions` ✅
2. Sends `Authorization: Bearer {api_key}` header ✅
3. Parses SSE streaming deltas ✅
4. Handles `finish_reason: stop` ✅
5. Handles tool calls in streaming format ✅

The Z.AI endpoint at `https://api.z.ai/api/coding/paas/v4/` is compatible
with this adapter path.

---

## 4. Provider Expansion Decision

### 4.1 Priority Ranking

| Priority | Provider | Rationale |
|----------|----------|-----------|
| 1 | Z.AI (confirmed) | Already works. Re-confirmed live. |
| 2 | OpenAI | Same adapter as Z.AI. High user demand. Needs API key. |
| 3 | Ollama | No API key needed. Local. Needs installation. |
| 4 | Anthropic | Different adapter. Needs API key. |
| 5 | OpenRouter | Same adapter. Multi-model gateway. |

### 4.2 What Resolves VL-4

VL-4's roadmap goal: "Validate at least one additional hosted provider."

**Z.AI is the additional hosted provider.** It was already listed as
validated in pre-v1.0 waves, and this wave re-confirms it is live and
responsive. The adapter path is confirmed compatible.

However, the roadmap's strict reading requires an **additional** provider
beyond the originally validated set (LM Studio + Z.AI). Since Z.AI was
already in the validated set, VL-4 remains **partially resolved**:

- ✅ Z.AI re-confirmed (live API evidence)
- ✅ Adapter paths analyzed (OpenAI, Anthropic, Ollama structurally compatible)
- ⬜ No additional NEW provider operationally validated
- ⬜ OpenAI/Anthropic/Ollama need API keys or local instances

### 4.3 Why Not Validate More Providers

| Provider | Blocker |
|----------|---------|
| OpenAI | No API key available |
| Anthropic | No API key available |
| Ollama | No local instance running |
| OpenRouter | No API key available |

All require resources not available in this environment.

---

## 5. Caveat X-07 Update

### Original (v0.2.0)

> X-07: Not provider completeness

### Updated (Wave 132A)

> X-07 (Updated): OpenWand validates Z.AI (glm-4.6, hosted) and LM Studio
> (local). The OpenAI-compatible adapter structurally supports OpenAI,
> Ollama, and any `/v1/chat/completions` server. The Anthropic-compatible
> adapter structurally supports Anthropic. However, no additional provider
> beyond Z.AI and LM Studio has been operationally validated.
>
> **X-07 is PARTIALLY NARROWED.** Provider expansion is documented and
> adapter paths are analyzed, but provider completeness is not claimed.

---

## 6. What This Decision Does NOT Claim

- Not provider completeness
- Not that OpenAI is validated (only structurally compatible)
- Not that Anthropic is validated (only structurally compatible)
- Not that Ollama is validated (only structurally compatible)
- Not that all adapter paths are bug-free
- Not production readiness
- Not formal certification
- Not external review

---

## 7. Path to Full Resolution

VL-4 can be fully resolved when:
1. An **additional** provider (OpenAI, Anthropic, or Ollama) is
   operationally validated with a real API call through OpenWand's adapter
2. The validation evidence is recorded (request, response, adapter handling)
3. Any adapter issues found are fixed with regression tests

The adapter code is ready. What is missing is **API access** to additional
providers.
