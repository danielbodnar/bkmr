# Embeddings Configuration

This module provides support for OpenAI-compatible embedding providers.

## Configuration

Embeddings are configured via environment variables:

- `OPENAI_API_KEY` (required): Your API key for the provider
- `OPENAI_PROVIDER` (optional): Named provider (openai, voyageai, ollama, huggingface, local)
- `OPENAI_API_BASE` (optional): Custom API endpoint URL
- `OPENAI_MODEL` (optional): Model name to use for embeddings

## Supported Providers

The following providers are pre-configured:

### OpenAI (default)
```bash
export OPENAI_API_KEY="sk-..."
# Or explicitly:
export OPENAI_PROVIDER="openai"
bkmr --openai semsearch "query"
```

### Voyage AI
```bash
export OPENAI_API_KEY="pa-..."
export OPENAI_PROVIDER="voyageai"
bkmr --openai semsearch "query"
```

### Ollama (local)
```bash
export OPENAI_API_KEY="not-needed-for-ollama"
export OPENAI_PROVIDER="ollama"
bkmr --openai semsearch "query"
```

### HuggingFace
```bash
export OPENAI_API_KEY="hf_..."
export OPENAI_PROVIDER="huggingface"
bkmr --openai semsearch "query"
```

### Custom Provider
```bash
export OPENAI_API_KEY="your-key"
export OPENAI_API_BASE="https://custom.example.com/v1"
export OPENAI_MODEL="custom-model"
bkmr --openai semsearch "query"
```

## Adding New Providers

To add a new OpenAI-compatible provider, edit `providers.rs` and add an entry to the `get_known_providers()` function:

```rust
providers.insert(
    "my-provider",
    ProviderConfig {
        name: "my-provider",
        api_base: "https://api.myprovider.com/v1",
        default_model: "my-model",
        auth_header: AuthHeaderType::Bearer,
    },
);
```

## Architecture

- `openai_compatible_provider.rs`: Unified provider that works with any OpenAI-compatible API
- `providers.rs`: Configuration for known providers
- `openai_provider.rs`: Legacy OpenAI-specific provider (kept for backward compatibility)
- `voyageai_provider.rs`: Legacy VoyageAI-specific provider (kept for backward compatibility)

The new `OpenAiCompatibleEmbedding` provider is recommended for all new integrations as it provides better flexibility and maintainability.

## Note on Non-OpenAI-Compatible Providers

This implementation assumes all providers follow the OpenAI embeddings API format. Providers with significantly different authentication schemes, request formats, or response structures may require additional abstraction layers. See code comments in the implementation for details on what would need to be extended.
