# Vendor-Agnostic OpenAI-Compatible Embeddings System for BKMR

**Version:** 1.0  
**Date:** 2025-10-31

## Context

The bkmr bookmark manager currently has a hardcoded OpenAI embeddings implementation. This enhancement abstracts the embeddings layer to support any OpenAI-compatible embeddings provider (Ollama, HuggingFace, Voyage AI, OpenAI-compatible endpoints) while maintaining the existing OPENAI_* environment variable naming for backwards compatibility. The goal is flexibility across vendors without implementing a complex abstraction layer for non-compatible providers.

## Requirements

### Must Have
- Abstract embeddings implementation to be vendor-agnostic for OpenAI-compatible APIs
- Search crates.io for existing multi-vendor embeddings library
- If no suitable library exists, implement HTTP REST pattern with `providers.rs` for common endpoints
- Maintain existing OPENAI_* environment variable naming (backwards compatibility)
- Support URL and model configuration (OPENAI_API_KEY, OPENAI_API_BASE, OPENAI_MODEL)
- Support common providers: Ollama, HuggingFace, Voyage AI, OpenAI, and other OpenAI-compatible services
- Add documentation comments (`///` or `//!`) indicating where non-compatible providers would require different architecture
- Zero breaking changes for existing users

### Out of Scope
- Non-OpenAI-compatible embeddings providers (requires different API patterns)
- OAuth authentication flows
- AWS SigV4 authentication
- gRPC-based APIs
- Streaming embeddings
- Batch optimization beyond OpenAI API

## Configuration & Decisions

**Environment Variables:**
- `OPENAI_API_KEY` - Required
- `OPENAI_API_BASE` - Optional, defaults to `https://api.openai.com/v1`
- `OPENAI_MODEL` - Optional, defaults to `text-embedding-3-small`

**Auth Header Detection:**
Auto-detect based on URL: Voyage AI uses `X-Api-Key`, localhost/Ollama has no auth, everything else uses `Authorization: Bearer`.

**Validation:** Lazy (on first embedding request). `OPENAI_API_BASE` must be valid URL format if provided.

**Error Handling:** Clear messages pointing to env vars, no API key exposure in logs, provider endpoint shown in network errors.

### Provider Support Matrix

**OpenAI-Compatible Cloud Providers:**

| Provider | Base URL | Default Model | Dimensions | Auth Header | Auth Prefix | Context | Cost |
|----------|----------|---------------|------------|-------------|-------------|---------|------|
| OpenAI | `https://api.openai.com/v1` | `text-embedding-3-small` | 1536 | `Authorization` | `Bearer ` | 8K | $0.02/1M |
| Voyage AI | `https://api.voyageai.com/v1` | `voyage-3-large` | 1024 | `X-Api-Key` | `` | 32K | $0.06/1M |
| Ollama | `http://localhost:11434/v1` | `nomic-embed-text` | 768 | `` | `` | 8K | Free |
| HuggingFace | Model-specific URL | Model-specific | Varies | `Authorization` | `Bearer ` | Varies | Free tier |
| Cohere | `https://api.cohere.com/v1` | `embed-v4.0` | 1024 | `Authorization` | `Bearer ` | 512 | $0.12/1M |
| Jina AI | `https://api.jina.ai/v1` | `jina-embeddings-v3` | 1024 | `Authorization` | `Bearer ` | 8K | Free tier |
| Mistral AI | `https://api.mistral.ai/v1` | `mistral-embed` | Varies | `Authorization` | `Bearer ` | Varies | Competitive |
| Google Gemini | `https://generativelanguage.googleapis.com/v1beta/openai` | `text-embedding-004` | 768 | `Authorization` | `Bearer ` | 2K-8K | Free tier |

**Note:** Only providers that use OpenAI-compatible `/v1/embeddings` endpoint with standard request/response format are in scope. Providers like AWS Bedrock, Google Vertex AI, and Snowflake Cortex use different APIs and are out of scope.

### Offline Embedding Engines (Rust)

For users requiring complete privacy and no API costs, several high-quality Rust libraries support local embedding generation:

**Production-Ready Libraries:**
- **`embed_anything`** - Minimalist, high-performance pipeline supporting ONNX, Candle, dense/sparse embeddings. Multi-modal (text, images, audio, PDFs). GPU acceleration via CUDA. Python bindings available. Actively maintained by StarlightSearch.
- **`candle_embed`** - Simple, CUDA/CPU powered embeddings using HuggingFace's Candle framework. Supports any HF model, configurable pooling/normalization. Minimal dependencies, MIT licensed.
- **`candle` (HuggingFace)** - Full ML framework with BERT, T5, JinaBERT embedding models. WASM support for browser deployment. Optimized CPU (MKL/Accelerate) and CUDA backends. Production-ready, serverless-focused.

**Key Features:**
- **No PyTorch Dependency:** Low memory footprint, easy cloud deployment
- **True Multithreading:** Rust's concurrency for parallel processing
- **GPU Acceleration:** CUDA support out-of-the-box via Candle
- **Model Flexibility:** Any HuggingFace model, ONNX runtime support
- **Quantization:** Reduced precision (int8, binary) for storage savings

**Popular Models for Local Deployment:**
- `sentence-transformers/all-MiniLM-L6-v2` (384-dim, lightweight)
- `BAAI/bge-large-en-v1.5` (1024-dim, high quality)
- `nomic-ai/nomic-embed-text-v1` (768-dim, long context)
- `intfloat/e5-large-v2` (1024-dim, Microsoft research)
- `jinaai/jina-embeddings-v2-base-en` (768-dim, optimized)

**Implementation Note:** While offline models solve API cost/privacy concerns, they require different architecture than OpenAI-compatible APIs. Recommend implementing API-based providers first, then consider offline models as separate feature if needed.

**Backwards Compatibility:** Existing users with only `OPENAI_API_KEY` set continue working unchanged. No migration needed.

**Out of Scope:** Non-OpenAI-compatible providers (Vertex AI, Bedrock, Anthropic), OAuth/SigV4 auth, gRPC, streaming, provider-specific optimizations.

## Architecture

### Library Selection Criteria
Search crates.io for libraries matching these criteria:

**Must Have:**
- OpenAI-compatible endpoint support
- Configurable base URLs
- Active maintenance (commit in last 6 months)
- Reasonable dependency footprint (<20 direct dependencies)

**Nice to Have:**
- Multiple provider support built-in
- Good error handling
- Async/await support
- Production usage examples

**Evaluation Process:**
1. Search terms: `openai`, `embeddings`, `llm client`, `inference`
2. Evaluate top 3-5 candidates
3. Check: stars, recent commits, open issues, API design
4. Decision: Use library if it saves >200 lines of code and meets criteria

### Implementation Path A: Using Existing Library

```rust
// Wrap external library in our trait
use external_lib::EmbeddingsClient;

pub struct VendorAgnosticEmbedding {
    client: EmbeddingsClient,
}

impl Embedder for VendorAgnosticEmbedding {
    async fn embed(&self, text: &str) -> Result<Vec<f32>> {
        // NOTE: This assumes OpenAI-compatible API format.
        // Non-compatible providers (Vertex, Bedrock) would need
        // different client initialization and potentially different
        // trait methods to accommodate their request/response formats.
        self.client.embed(text).await.map_err(Into::into)
    }
}
```

### Implementation Path B: Custom HTTP Client

**File Structure:**
```
infrastructure/embeddings/
├── mod.rs
├── providers.rs              # NEW: Provider constants and metadata
├── openai_compatible.rs      # NEW: Generic client (renamed from openai_provider.rs)
├── config.rs                 # NEW: Configuration loading
└── error.rs                  # Existing error types
```

**providers.rs:**
```rust
//! OpenAI-compatible embeddings provider configurations
//!
//! This module defines common provider endpoints that follow the OpenAI
//! embeddings API format. All providers here use:
//! - POST /embeddings endpoint
//! - Request: { "model": "...", "input": ["..."] }
//! - Response: { "data": [{ "embedding": [...] }] }
//!
//! # Limitations
//! Non-compatible providers (Google Vertex AI, AWS Bedrock, Anthropic)
//! cannot be added here as they use different API formats, authentication
//! mechanisms, and request/response structures. Those would require:
//! - Separate client implementations
//! - Different trait methods or trait redesign
//! - Provider-specific authentication (OAuth, SigV4, etc.)

pub struct ProviderConfig {
    pub base_url: &'static str,
    pub default_model: &'static str,
    pub auth_header: &'static str,
    pub auth_prefix: &'static str,
}

pub const OPENAI: ProviderConfig = ProviderConfig {
    base_url: "https://api.openai.com/v1",
    default_model: "text-embedding-3-small",
    auth_header: "Authorization",
    auth_prefix: "Bearer ",
};

pub const VOYAGE: ProviderConfig = ProviderConfig {
    base_url: "https://api.voyageai.com/v1",
    default_model: "voyage-3-large",
    auth_header: "X-Api-Key",
    auth_prefix: "",
};

pub const OLLAMA: ProviderConfig = ProviderConfig {
    base_url: "http://localhost:11434/v1",
    default_model: "nomic-embed-text",
    auth_header: "",
    auth_prefix: "",
};

// Note: HuggingFace Inference API has model-specific URLs
// Users should set OPENAI_API_BASE to full model URL
```

**openai_compatible.rs:**
```rust
//! Generic OpenAI-compatible embeddings client
//!
//! This implementation works with any API that follows OpenAI's
//! embeddings endpoint specification. It supports:
//! - Configurable base URLs
//! - Configurable authentication headers
//! - Custom model names
//!
//! # Compatibility Assumptions
//! This assumes all providers:
//! 1. Use JSON POST to /embeddings
//! 2. Accept { "model": "...", "input": [...] }
//! 3. Return { "data": [{ "embedding": [...] }] }
//! 4. Use simple API key authentication in headers
//!
//! # Future Considerations
//! Non-compatible providers would need:
//! - Different request/response serialization
//! - Different authentication (OAuth flows, SigV4 signing)
//! - Different error handling for provider-specific formats
//! - Potentially different HTTP libraries or middleware

use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct EmbeddingRequest {
    model: String,
    input: Vec<String>,
}

#[derive(Deserialize)]
struct EmbeddingResponse {
    data: Vec<EmbeddingData>,
}

#[derive(Deserialize)]
struct EmbeddingData {
    embedding: Vec<f32>,
}

pub struct OpenAiCompatibleClient {
    client: Client,
    base_url: String,
    model: String,
    api_key: String,
    auth_header: String,
    auth_prefix: String,
}

impl OpenAiCompatibleClient {
    pub fn from_env() -> Result<Self> {
        let api_key = std::env::var("OPENAI_API_KEY")
            .map_err(|_| Error::ConfigError("OPENAI_API_KEY not set"))?;
        
        let base_url = std::env::var("OPENAI_API_BASE")
            .unwrap_or_else(|_| "https://api.openai.com/v1".to_string());
        
        let model = std::env::var("OPENAI_MODEL")
            .unwrap_or_else(|_| "text-embedding-3-small".to_string());
        
        // Detect provider from base_url and set auth headers accordingly
        let (auth_header, auth_prefix) = if base_url.contains("voyageai.com") {
            ("X-Api-Key".to_string(), "".to_string())
        } else if base_url.contains("localhost") || base_url.contains("127.0.0.1") {
            ("".to_string(), "".to_string())  // No auth for local Ollama
        } else {
            ("Authorization".to_string(), "Bearer ".to_string())
        };
        
        Ok(Self {
            client: Client::new(),
            base_url,
            model,
            api_key,
            auth_header,
            auth_prefix,
        })
    }
    
    pub async fn embed(&self, text: &str) -> Result<Vec<f32>> {
        let url = format!("{}/embeddings", self.base_url);
        
        let request = EmbeddingRequest {
            model: self.model.clone(),
            input: vec![text.to_string()],
        };
        
        let mut req = self.client.post(&url).json(&request);
        
        // Add authentication header if configured
        if !self.auth_header.is_empty() {
            let auth_value = format!("{}{}", self.auth_prefix, self.api_key);
            req = req.header(&self.auth_header, auth_value);
        }
        
        // NOTE: Error handling here assumes OpenAI-style error responses.
        // Different providers may return errors in different formats.
        let response = req.send().await?;
        
        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await?;
            return Err(Error::ApiError(format!(
                "Provider returned {}: {}",
                status, body
            )));
        }
        
        let embedding_response: EmbeddingResponse = response.json().await?;
        
        embedding_response
            .data
            .into_iter()
            .next()
            .map(|data| data.embedding)
            .ok_or_else(|| Error::ApiError("No embedding in response".to_string()))
    }
}
```

## Embeddings Ecosystem Research

### Available Rust Crates

**Multi-Provider Libraries:**
- **`ai` (crates.io/crates/ai)** - Actively maintained, supports OpenAI-compatible providers including Ollama, Gemini, Azure OpenAI. Has `Client::from_url()` for custom endpoints and `EmbeddingsRequestBuilder`. Best candidate for library approach.
- **`openai_dive` (v1.2)** - Async OpenAI library with support for OpenAI-compatible APIs via custom base URLs. Good documentation, active maintenance.
- **`openagent`** - Comprehensive OpenAI client with embeddings support, but focused on OpenAI specifically.
- **`async-openai`** - Fully async, low-level and high-level APIs, good for OpenAI but less multi-provider focused.

**OpenAI-Only Libraries:**
- `openai` - Basic unofficial Rust library
- `openai_api_rust` - Alternative community-maintained
- `openai-rust` - Matches official docs closely

**Specialized:**
- `embed_anything` - High-performance with ONNX/local models, not API-focused
- `ai-gateway` - Proxy/gateway rather than client library

**Recommendation:** Evaluate `ai` crate first - it explicitly supports configurable base URLs and multiple OpenAI-compatible providers. If it doesn't fit, build custom implementation.

### Cloud Embedding Providers & Models

**OpenAI**
- API: `https://api.openai.com/v1/embeddings`
- Models: `text-embedding-3-small` (1536-dim, $0.02/1M tokens), `text-embedding-3-large` (3072-dim, $0.13/1M tokens), `text-embedding-ada-002` (1536-dim, legacy)
- Auth: `Authorization: Bearer {key}`
- Context: 8K tokens

**Voyage AI**
- API: `https://api.voyageai.com/v1/embeddings`
- Models:
  - `voyage-3-large`: 1024-dim default (supports 256, 512, 2048), 32K context, $0.06/1M tokens
  - `voyage-3.5`: 1024-dim default (2048, 512, 256), 32K context, best quality
  - `voyage-3.5-lite`: 512-dim default, 32K context, most cost-effective
  - `voyage-3`: 1024-dim, 32K context
  - `voyage-code-3`: 1536-dim, optimized for code
  - `voyage-law-2`, `voyage-finance-2`: 1024-dim, domain-specific
- Auth: `X-Api-Key: {key}` (non-standard header)
- Features: Matryoshka embeddings, quantization (int8, binary)

**Ollama (Local)**
- API: `http://localhost:11434/v1/embeddings` (OpenAI-compatible endpoint)
- Models:
  - `nomic-embed-text`: 768-dim, 8192 context, high-performance, surpasses OpenAI ada-002
  - `mxbai-embed-large`: 1024-dim, 512 context, state-of-the-art
  - `all-minilm`: 384-dim, 256 context, lightweight and fast
  - `snowflake-arctic-embed`: 1024-dim, multilingual
- Auth: None (local server)
- Features: Completely free, offline, privacy-focused

**Cohere**
- API: `https://api.cohere.com/v1/embeddings`
- Models: `embed-v4.0` (multimodal), `embed-multilingual-v3.0` (100+ languages, 1024-dim), `embed-english-v3.0`, light versions (384-dim)
- Auth: `Authorization: Bearer {key}`
- Context: 512 tokens
- Cost: $0.12/1M tokens

**Jina AI**
- API: `https://api.jina.ai/v1/embeddings`
- Models: `jina-embeddings-v3` (1024-dim, 89+ languages), `jina-embeddings-v4` (2048-dim multimodal), task-specific adapters
- Auth: `Authorization: Bearer {key}`
- Context: 8192 tokens
- Cost: 10M free tokens, then paid
- Features: Open source models available

**Mistral AI**
- API: `https://api.mistral.ai/v1/embeddings`
- Models: `mistral-embed` (general purpose), specialized models
- Auth: `Authorization: Bearer {key}`
- Features: European-developed, efficient processing, competitive pricing

**Google Gemini**
- API: `https://generativelanguage.googleapis.com/v1beta/openai/embeddings`
- Models: `gemini-embedding-001` (3072-dim adjustable to 768), `text-embedding-004`, `text-multilingual-embedding-002`
- Auth: `Authorization: Bearer {key}` (requires http1_title_case_headers for compatibility)
- Context: 2048-8192 tokens
- Cost: Free tier with generous limits

**HuggingFace Inference API**
- API: `https://api-inference.huggingface.co/pipeline/feature-extraction` or model-specific URLs
- Models: Thousands available (sentence-transformers, e5-large, etc.)
- Auth: `Authorization: Bearer {key}`
- Note: Varies by model, some use different endpoints

### Offline Embedding Engines (Rust-Native)

**Production-Grade Frameworks:**

**`embed_anything` (StarlightSearch)**
- **Architecture:** Rust-native with Python bindings, supports Candle and ONNX backends
- **Features:** Dense, sparse, late-interaction, ColBERT embeddings. Multi-modal (text, images, audio, PDFs). Memory-efficient streaming to vector databases.
- **Performance:** No PyTorch dependency, true multithreading, CUDA acceleration, minimal memory footprint
- **Models:** Any HuggingFace model via Candle, ONNX models (ModernBERT, ColPali, Jina)
- **Use Case:** Production embedding pipeline with document ingestion

**`candle_embed`**
- **Architecture:** Simple wrapper around HuggingFace Candle framework
- **Features:** CUDA/CPU support, configurable pooling/normalization, preset popular models
- **Models:** Any HuggingFace model, includes presets for UAE, GIST, sentence-transformers
- **Use Case:** Lightweight embedding generation with minimal setup

**HuggingFace `candle`**
- **Architecture:** Full ML framework, PyTorch-like API in Rust
- **Features:** BERT, T5, JinaBERT models built-in. WASM support for browser. Optimized CPU (MKL/Accelerate) and CUDA backends.
- **Models:** Pre-implemented: BERT, T5, Jina, DINOv2 (vision)
- **Use Case:** Serverless inference, lightweight deployments, edge computing

**Popular Open-Source Models:**
- `sentence-transformers/all-MiniLM-L6-v2` - 384-dim, 22M params, fast
- `BAAI/bge-large-en-v1.5` - 1024-dim, SOTA open-source
- `BAAI/bge-m3` - 1024-dim, multilingual with Chinese excellence
- `nomic-ai/nomic-embed-text-v1` - 768-dim, 137M params, 8K context
- `intfloat/e5-large-v2` - 1024-dim, Microsoft research quality
- `intfloat/e5-mistral-7b-instruct` - 4096-dim, large enterprise model
- `jinaai/jina-embeddings-v2-base-en` - 768-dim, 137M params
- `jinaai/jina-colbert-v2` - Late-interaction model for reranking

**Quantized Models (ONNX):**
- ModernBERT variants with Q4F16 quantization
- Reduced storage, faster inference, minimal quality loss

### Implementation Decision Matrix

**Use `ai` crate if:**
- ✅ Supports configurable base URLs (confirmed: `Client::from_url()`)
- ✅ Handles authentication headers properly
- ✅ Active maintenance (check recent commits)
- ✅ Reasonable dependencies (<30 direct deps)
- ✅ Saves >200 lines of code vs custom implementation

**Build custom if:**
- ❌ Library doesn't support auth header customization (Voyage needs X-Api-Key)
- ❌ No way to configure per-provider settings
- ❌ Adds unnecessary dependencies or complexity
- ❌ Unclear maintenance status

**Add offline support (separate phase) if:**
- ✅ Users need complete privacy (no cloud APIs)
- ✅ Cost sensitivity for high-volume usage
- ✅ Air-gapped environments required
- ✅ GPU resources available for local inference

## Implementation Approach

### Step 0: Run Test Suite
A comprehensive test suite is provided in `embeddings_tests.rs`. Run with:
```bash
cargo test --test embeddings_tests
```

Initially most tests will fail (types don't exist). Implement code to make tests pass. See `TESTS_README.md` for details.

### Step 1: Evaluate `ai` Crate
Test `ai` crate with all target providers. If it works well, use it. If limitations found, proceed to custom implementation.

### Step 2: Custom Implementation (if needed)
Create modular architecture: `providers.rs` (constants), `config.rs` (env loading), `openai_compatible.rs` (generic client). Provider-specific auth header detection. Comprehensive testing with mocks.

### Step 3: Offline Models (future phase)
If demand exists, add `candle_embed` or `embed_anything` integration as alternative to API-based providers. Requires separate trait implementation due to different architecture.

## Quality Criteria

### Functional Requirements
- ✅ Works with OpenAI official API
- ✅ Works with Ollama (OpenAI-compatible mode)
- ✅ Works with Voyage AI
- ✅ Works with HuggingFace Inference API
- ✅ Supports custom OpenAI-compatible endpoints
- ✅ Backwards compatible with existing OPENAI_API_KEY configs
- ✅ Clear error messages for configuration issues
- ✅ No API key exposure in logs or error messages

### Code Quality

**Rust Comment Conventions:**
- **Documentation comments** (`///` or `//!`) for public APIs explaining limitations
- **Inline comments** (`// NOTE:`) for specific implementation constraints
- **Avoid** made-up prefixes like "FUTURE:" - not idiomatic Rust
- **Avoid** `todo!()` macro for documentation - it's for unimplemented code paths

**Code Standards:**
- Documentation comments (`///`, `//!`) explain architectural limitations
- No unnecessary abstraction (YAGNI for non-compatible providers)
- Follows Rust idioms and existing project patterns
- Comprehensive error handling with descriptive messages
- Test coverage >80% for new code
- No clippy warnings
- Formatted with rustfmt

### Configuration
- Environment variables validated on first use
- Clear error messages for missing/invalid config
- Sensible defaults (OpenAI endpoint, reasonable model)
- Configuration errors caught early with helpful messages

### Documentation
- Each provider has working example
- Environment variable reference table
- Common troubleshooting section
- Explicit note about OpenAI-compatible requirement
- Documentation comments (`///`, `//!`) explain architectural constraints

## Examples

### Example 1: OpenAI (Default, Backwards Compatible)
```bash
export OPENAI_API_KEY="sk-proj-..."
# Uses defaults: https://api.openai.com/v1 and text-embedding-3-small
bkmr search "rust async patterns"

# Output:
# Using OpenAI embeddings (text-embedding-3-small)
# Found 5 bookmarks matching your query...
```

### Example 2: Voyage AI
```bash
export OPENAI_API_KEY="pa-..."
export OPENAI_API_BASE="https://api.voyageai.com/v1"
export OPENAI_MODEL="voyage-3-large"
bkmr search "rust async patterns"

# Output:
# Using Voyage AI embeddings (voyage-3-large)
# Found 5 bookmarks matching your query...
```

### Example 3: Ollama (Local)
```bash
# Start Ollama server first: ollama serve
# Pull embedding model: ollama pull nomic-embed-text

export OPENAI_API_BASE="http://localhost:11434/v1"
export OPENAI_MODEL="nomic-embed-text"
# No API key needed for local Ollama
bkmr search "rust async patterns"

# Output:
# Using Ollama embeddings (nomic-embed-text)
# Found 5 bookmarks matching your query...
```

### Example 4: HuggingFace Inference API
```bash
export OPENAI_API_KEY="hf_..."
export OPENAI_API_BASE="https://api-inference.huggingface.co/models/sentence-transformers/all-MiniLM-L6-v2"
export OPENAI_MODEL="sentence-transformers/all-MiniLM-L6-v2"
bkmr search "rust async patterns"

# Output:
# Using HuggingFace embeddings (sentence-transformers/all-MiniLM-L6-v2)
# Found 5 bookmarks matching your query...
```

### Example 5: Custom OpenAI-Compatible Endpoint
```bash
export OPENAI_API_KEY="custom-key-123"
export OPENAI_API_BASE="https://embeddings.mycompany.internal/v1"
export OPENAI_MODEL="company-embeddings-v2"
bkmr search "rust async patterns"

# Output:
# Using custom embeddings (company-embeddings-v2)
# Found 5 bookmarks matching your query...
```

### Example 6: Error Handling
```bash
# Missing API key
$ export OPENAI_API_BASE="https://api.voyageai.com/v1"
$ bkmr search "test"
Error: Configuration error: OPENAI_API_KEY environment variable not set
Hint: Set your API key with: export OPENAI_API_KEY="your-key-here"

# Invalid URL format
$ export OPENAI_API_KEY="sk-..."
$ export OPENAI_API_BASE="not-a-url"
$ bkmr search "test"
Error: Configuration error: OPENAI_API_BASE is not a valid URL
Provided: not-a-url
Hint: Use format like: https://api.openai.com/v1

# Authentication failure
$ export OPENAI_API_KEY="invalid-key"
$ bkmr search "test"
Error: Authentication failed (401 Unauthorized)
Hint: Verify your OPENAI_API_KEY is correct for the provider at:
      https://api.openai.com/v1

# Model not available
$ export OPENAI_MODEL="nonexistent-model"
$ bkmr search "test"
Error: Provider returned 404: Model 'nonexistent-model' not found
Hint: Check provider documentation for available models
```

## Common Pitfalls

### Implementation Pitfalls
❌ **Don't:**
- Create complex abstractions for non-compatible providers yet (YAGNI)
- Hardcode provider URLs in multiple places - use providers.rs constants
- Assume all providers use "Authorization: Bearer" - Voyage uses "X-Api-Key"
- Silently fail on configuration errors - validate early with clear messages
- Log or expose API keys in error messages
- Make HTTP requests at startup for validation

✅ **Do:**
- Add doc comments (`///`, `//!`) explaining architectural limitations
- Keep the abstraction simple and focused on OpenAI-compatible APIs
- Use provider detection for auth header selection
- Fail fast with descriptive errors on first embedding request
- Mask API keys in any user-visible output
- Support both HTTP and HTTPS for local development

### Configuration Pitfalls
❌ **Don't:**
- Break existing users who only have OPENAI_API_KEY set
- Require users to set all three variables - use sensible defaults
- Validate API key format (provider-specific, often opaque strings)
- Make assumptions about model names across providers

✅ **Do:**
- Validate OPENAI_API_BASE is a valid URL format if provided
- Provide clear error messages when provider endpoints return unexpected responses
- Use environment variables consistently with OpenAI convention
- Document provider-specific quirks (like Voyage's X-Api-Key header)

### Testing Pitfalls
❌ **Don't:**
- Require actual API keys for CI/CD
- Hardcode test data that assumes only OpenAI response format
- Skip error path testing
- Test only happy path scenarios

✅ **Do:**
- Mock multiple provider response formats
- Test configuration loading thoroughly
- Test error paths (invalid URLs, auth failures, network errors)
- Test backwards compatibility explicitly
- Use feature flags for integration tests that need real API keys

### Documentation Pitfalls
❌ **Don't:**
- Claim support for non-OpenAI-compatible providers
- Assume users know about OpenAI-compatible APIs
- Copy/paste examples without testing them
- Forget to document provider-specific requirements

✅ **Do:**
- Explain what "OpenAI-compatible" means clearly
- Provide working examples for each major provider
- Document known limitations explicitly
- Include troubleshooting section with common errors
- Link to provider-specific documentation

### Security Pitfalls
❌ **Don't:**
- Log API keys or tokens in any form
- Commit example API keys to version control (even fake ones)
- Display full API keys in error messages
- Store API keys in configuration files

✅ **Do:**
- Document environment variable approach
- Consider adding API key masking in error messages (show first/last 4 chars)
- Warn about using HTTP (non-HTTPS) endpoints
- Recommend using secret management tools for production
- Clear API keys from memory when possible

## Scope Boundaries

### In Scope for This PR
- OpenAI and OpenAI-compatible APIs
- Providers with `/embeddings` endpoint matching OpenAI format
- Configurable base URLs and models
- Authentication via API keys in headers
- HTTP/HTTPS endpoints
- Environment variable configuration
- Backwards compatibility for existing users
- Documentation for major providers

### Out of Scope (Future Work)
**Non-Compatible Providers:**
- Google Vertex AI (OAuth, different request format)
- AWS Bedrock (SigV4 signing, different API)
- Anthropic embeddings (if they add it - different format)
- Azure OpenAI (different auth, might work but untested)

**Advanced Features:**
- OAuth authentication flows
- AWS SigV4 authentication
- gRPC-based APIs
- Binary response formats
- Streaming embeddings
- Batch optimization beyond what OpenAI API provides
- Provider-specific features (e.g., Voyage's domain specialization)
- Configuration files (environment variables only for now)
- Web UI for provider selection

**Where to Add Documentation Comments:**

Use idiomatic Rust doc comments to explain architectural constraints:

1. **Trait definition** - Use `///` doc comment explaining OpenAI-compatible assumptions
```rust
/// Embedder trait for generating text embeddings.
///
/// # Compatibility
/// Current implementations assume OpenAI-compatible API format.
pub trait Embedder { }
```

2. **Request/response structs** - Document expected JSON structure with `///`
```rust
/// Request format for OpenAI-compatible embeddings endpoints.
/// Expects: { "model": "...", "input": [...] }
#[derive(Serialize)]
struct EmbeddingRequest { }
```

3. **Authentication** - Use `// NOTE:` for inline constraints
```rust
// NOTE: Assumes simple API key in headers (Authorization or X-Api-Key).
// OAuth/SigV4 providers need different authentication.
```

4. **Error handling** - Use `// NOTE:` for format assumptions
```rust
// NOTE: Assumes OpenAI error response format.
// Non-compatible providers may use different error structures.
```

5. **Module (providers.rs)** - Use `//!` module-level doc for scope limitations
```rust
//! # Limitations
//! Non-compatible providers (Vertex AI, Bedrock) require different architecture.
```

## Implementation Checklist

- [ ] Run provided test suite: `cargo test --test embeddings_tests`
- [ ] Evaluate `ai` crate vs custom implementation
- [ ] Implement provider abstraction with auto-detect auth headers
- [ ] Add configuration loading from env vars
- [ ] All unit tests pass (25 tests)
- [ ] Error handling tests pass
- [ ] Backwards compatibility verification
- [ ] Add HTTP mocking library for integration tests
- [ ] Update README with provider examples
- [ ] Document limitations (OpenAI-compatible only)
- [ ] Add doc comments (`///`) explaining architectural limitations at extension points

## References

- **OpenAI Embeddings API:** https://platform.openai.com/docs/guides/embeddings
- **Voyage AI API:** https://docs.voyageai.com/docs/embeddings
- **Ollama OpenAI Compatibility:** https://docs.ollama.com/openai
- **Original PR:** https://github.com/danielbodnar/bkmr/pull/1
- **Abstraction Request:** https://github.com/danielbodnar/bkmr/pull/1#issuecomment-3475303676

## Deliverables

**Test Suite:**
- `embeddings_tests.rs` - 25 comprehensive unit tests covering configuration, provider detection, serialization, and edge cases
- `TESTS_README.md` - Documentation for running and extending tests

**Expected Artifacts:**
- Provider abstraction implementation (library or custom)
- Configuration loading from environment variables
- Request/response serialization
- Provider-specific auth header detection
- Error handling
- Documentation with provider examples https://github.com/danielbodnar/bkmr/pull/1#issuecomment-3475303676

---

## Implementation Checklist

Use this checklist during implementation:

### Phase 1: Research
- [ ] Search crates.io for multi-vendor embeddings libraries
- [ ] Evaluate top 3 candidates against criteria
- [ ] Make build vs buy decision
- [ ] Document decision rationale

### Phase 2: Implementation
- [ ] Create/update file structure
- [ ] Implement provider abstraction
- [ ] Add configuration loading
- [ ] Implement auth header detection
- [ ] Add error handling
- [ ] Add doc comments (`///`) at architecture extension points

### Phase 3: Testing
- [ ] Unit tests for configuration
- [ ] Unit tests for providers
- [ ] Mocked HTTP tests
- [ ] Error handling tests
- [ ] Backwards compatibility tests
- [ ] Integration tests

### Phase 4: Documentation
- [ ] Update README
- [ ] Add provider setup guides
- [ ] Document environment variables
- [ ] Add troubleshooting section
- [ ] Add examples for each provider
- [ ] Document limitations

### Phase 5: Review
- [ ] All tests passing
- [ ] No clippy warnings
- [ ] Code formatted with rustfmt
- [ ] Documentation complete
- [ ] Examples tested
- [ ] Ready for code review

---

**Document Status:** Ready for Implementation  
**Estimated Effort:** 13-21 hours  
**Priority:** High  
**Dependencies:** None
