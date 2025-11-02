---
description: Start bkmr LSP server for testing
allowed-tools: Bash(cargo run:*)
---

Start the bkmr LSP server for testing with editor integration.

## Current Database

Database location: !`echo $BKMR_DB_URL`

## Task

Start the LSP server:

```bash
cd bkmr && RUST_LOG=debug cargo run -- lsp
```

The LSP server will:
- Listen on stdin/stdout for JSON-RPC messages
- Provide snippet completions
- Support language-aware filtering
- Include template interpolation

## Testing LSP

In another terminal, run LSP tests:
```bash
make test-lsp
make test-lsp-client
make test-lsp-filtering
```

Or test manually with Python scripts:
```bash
python3 scripts/lsp/test_lsp_client.py
```

Press Ctrl+C to stop the server when done.
