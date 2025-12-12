# Embeddings Provider Configuration

`bkmr` supports multiple embeddings providers through a vendor-agnostic configuration system using **both configuration files and environment variables**. While the environment variables use the `OPENAI_` prefix for backward compatibility, you can use any OpenAI-compatible embeddings provider.

## Configuration Methods

### Method 1: Configuration File (Production-Ready)

For production deployments, use the configuration file at `~/.config/bkmr/config.toml`:

```toml
[embeddings_opts]
api_base = "https://api.openai.com/v1"
model = "text-embedding-3-small"
```

**Benefits:**
- Persistent across sessions
- Version-controllable
- No need to set environment variables repeatedly
- Production-ready

Generate the default config:
```bash
bkmr --generate-config > ~/.config/bkmr/config.toml
```

### Method 2: Environment Variables (Quick Testing)

Environment variables override config file settings, useful for testing or temporary changes:

```bash
export OPENAI_API_BASE="http://localhost:11434/v1"
export OPENAI_MODEL="nomic-embed-text"
```

### Configuration Priority

1. Environment variables (`OPENAI_API_BASE`, `OPENAI_MODEL`) - highest priority
2. Configuration file (`~/.config/bkmr/config.toml`)
3. Defaults if neither is set

## Supported Providers

### OpenAI (Default)

OpenAI provides state-of-the-art embeddings models with high accuracy.

```bash
export OPENAI_API_KEY="sk-your-api-key"
# Optional: Customize base URL and model
export OPENAI_API_BASE="https://api.openai.com/v1"  # default
export OPENAI_MODEL="text-embedding-3-small"  # default (1536 dimensions)
```

**Pros:**
- High quality embeddings
- Well-tested and reliable
- Latest models optimized for cost and performance

**Cons:**
- Requires internet connection
- API costs per request
- Data sent to external service

**Popular Models:**
- `text-embedding-3-small` - Recommended, fast and cost-effective (1536 dimensions)
- `text-embedding-3-large` - Higher quality (3072 dimensions)
- `text-embedding-ada-002` - Legacy model (1536 dimensions)

### Ollama (Local)

Ollama allows you to run embedding models locally for complete privacy and no API costs.

```bash
# Start Ollama (if not already running)
ollama serve

# Pull an embedding model
ollama pull nomic-embed-text

# Configure bkmr
export OPENAI_API_BASE="http://localhost:11434/v1"
export OPENAI_MODEL="nomic-embed-text"
# No API key needed for localhost
```

**Pros:**
- Complete privacy - data never leaves your machine
- No API costs
- Works offline
- Fast response times

**Cons:**
- Requires local compute resources
- May have slightly lower accuracy than cloud models

**Popular Models:**
- `nomic-embed-text` - Fast and efficient (768 dimensions)
- `mxbai-embed-large` - Higher quality (1024 dimensions)
- `all-minilm` - Lightweight (384 dimensions)

### HuggingFace

HuggingFace provides access to thousands of open-source embedding models.

```bash
export OPENAI_API_BASE="https://api-inference.huggingface.co/v1"
export OPENAI_MODEL="sentence-transformers/all-MiniLM-L6-v2"
export OPENAI_API_KEY="hf_your-api-key"
```

**Pros:**
- Access to many open-source models
- Free tier available
- Good balance of cost and quality

**Cons:**
- May have rate limits on free tier
- Variable response times

**Popular Models:**
- `sentence-transformers/all-MiniLM-L6-v2` - Fast and efficient (384 dimensions)
- `sentence-transformers/all-mpnet-base-v2` - Higher quality (768 dimensions)
- `BAAI/bge-small-en-v1.5` - Optimized for retrieval (384 dimensions)

### Voyage AI

Voyage AI provides specialized embeddings optimized for retrieval tasks. **Note:** Uses `X-Api-Key` authentication header (automatically detected).

```bash
export OPENAI_API_BASE="https://api.voyageai.com/v1"
export OPENAI_MODEL="voyage-2"
export OPENAI_API_KEY="pa-your-api-key"
```

**Pros:**
- Optimized for retrieval and search
- Good performance
- Competitive pricing

**Cons:**
- Requires account and API key
- Less well-known than OpenAI

### Custom OpenAI-Compatible Endpoints

Any service that implements the OpenAI embeddings API can be used.

```bash
export OPENAI_API_BASE="https://your-custom-endpoint.com/v1"
export OPENAI_MODEL="your-embedding-model"
export OPENAI_API_KEY="your-api-key"
```

## Configuration Reference

### Environment Variables

| Variable | Required | Default | Description |
|----------|----------|---------|-------------|
| `OPENAI_API_KEY` | Conditional | None | API key (not required for localhost) |
| `OPENAI_API_BASE` | No | `https://api.openai.com/v1` | Base URL for the embeddings API |
| `OPENAI_API_URL` | No | (alias for API_BASE) | Legacy alias for backward compatibility |
| `OPENAI_MODEL` | No | `text-embedding-3-small` | Model name for embeddings |

### Authentication

Authentication is automatically detected based on the URL:
- **Voyage AI** (api.voyageai.com): Uses `X-Api-Key` header
- **Localhost** (127.0.0.1 or localhost): No authentication required
- **All others**: Uses `Authorization: Bearer` header

### API Compatibility

The embeddings provider must support the OpenAI embeddings API format:

**Request:**
```json
POST /v1/embeddings
{
  "input": "text to embed",
  "model": "model-name"
}
```

**Response:**
```json
{
  "data": [
    {
      "embedding": [0.1, 0.2, 0.3, ...]
    }
  ]
}
```

## Testing Your Configuration

To verify your embeddings configuration:

```bash
# Set your environment variables
export OPENAI_API_KEY="your-key"
export OPENAI_API_BASE="your-url"
export OPENAI_MODEL="your-model"

# Try creating an embedding
bkmr --openai add "test bookmark" test --embeddable
bkmr --openai backfill

# If successful, you should see embedding generation logs
```

## Switching Providers

You can easily switch between providers by changing environment variables:

```bash
# Switch to Ollama (local)
export OPENAI_API_BASE="http://localhost:11434/v1"
export OPENAI_MODEL="nomic-embed-text"
unset OPENAI_API_KEY  # Not needed for localhost

# Switch back to OpenAI
unset OPENAI_API_BASE
unset OPENAI_MODEL
export OPENAI_API_KEY="sk-your-openai-key"
```

## Backward Compatibility

The implementation maintains full backward compatibility:

- If only `OPENAI_API_KEY` is set, OpenAI's default endpoint and model are used
- Existing scripts and configurations continue to work without changes
- The `--openai` CLI flag still works as before

## Troubleshooting

### "OPENAI_API_KEY environment variable not set"
Make sure you've exported the API key in your current shell session.

### "Failed to parse OpenAI response"
Check that your API URL is correct and the endpoint is compatible with the OpenAI API format.

### "Connection refused"
For local providers like Ollama, ensure the service is running: `ollama serve`

### Dimension mismatch errors
Different models produce embeddings of different dimensions. If you switch models, you may need to regenerate all embeddings:

```bash
# Clear existing embeddings and regenerate
bkmr --openai backfill
```

## Best Practices

1. **For production use:** Use OpenAI or Voyage AI for consistent quality
2. **For privacy:** Use Ollama to keep data local
3. **For experimentation:** Use HuggingFace to try different models
4. **For cost optimization:** Use Ollama or smaller HuggingFace models
5. **Test before switching:** Always test with a small dataset when switching providers

## Performance Considerations

- **OpenAI:** ~100-500ms per request (network dependent)
- **Ollama (local):** ~50-200ms per request (hardware dependent)
- **HuggingFace:** ~200-1000ms per request (tier dependent)
- **Voyage AI:** ~100-300ms per request (network dependent)

Choose based on your priorities: speed, cost, privacy, or quality.
