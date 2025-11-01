# Embeddings Provider Configuration

`bkmr` supports multiple embeddings providers through a vendor-agnostic configuration system. While the environment variables use the `OPENAI_` prefix for backward compatibility, you can use any OpenAI-compatible embeddings provider.

## Supported Providers

### OpenAI (Default)

OpenAI provides state-of-the-art embeddings models with high accuracy.

```bash
export OPENAI_API_KEY="sk-your-api-key"
# Optional: Customize the model (defaults to text-embedding-ada-002)
export OPENAI_MODEL="text-embedding-ada-002"
```

**Pros:**
- High quality embeddings
- Well-tested and reliable
- 1536-dimensional vectors

**Cons:**
- Requires internet connection
- API costs per request
- Data sent to external service

### Ollama (Local)

Ollama allows you to run embedding models locally for complete privacy and no API costs.

```bash
# Start Ollama (if not already running)
ollama serve

# Pull an embedding model
ollama pull nomic-embed-text

# Configure bkmr
export OPENAI_API_URL="http://localhost:11434"
export OPENAI_MODEL="nomic-embed-text"
export OPENAI_API_KEY="ollama"  # Ollama doesn't require a real key
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
export OPENAI_API_URL="https://api-inference.huggingface.co"
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

Voyage AI provides specialized embeddings optimized for retrieval tasks.

```bash
export OPENAI_API_URL="https://api.voyageai.com"
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
export OPENAI_API_URL="https://your-custom-endpoint.com"
export OPENAI_MODEL="your-embedding-model"
export OPENAI_API_KEY="your-api-key"
```

## Configuration Reference

### Environment Variables

| Variable | Required | Default | Description |
|----------|----------|---------|-------------|
| `OPENAI_API_KEY` | Yes | None | API key for authentication |
| `OPENAI_API_URL` | No | `https://api.openai.com` | Base URL for the embeddings API |
| `OPENAI_MODEL` | No | `text-embedding-ada-002` | Model name for embeddings |

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
export OPENAI_API_URL="your-url"
export OPENAI_MODEL="your-model"

# Try creating an embedding
bkmr --openai add "test bookmark" test --embeddable
bkmr --openai backfill

# If successful, you should see embedding generation logs
```

## Switching Providers

You can easily switch between providers by changing environment variables:

```bash
# Switch to Ollama
export OPENAI_API_URL="http://localhost:11434"
export OPENAI_MODEL="nomic-embed-text"
export OPENAI_API_KEY="ollama"

# Switch back to OpenAI
unset OPENAI_API_URL
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
