---
description: Build bkmr release version
allowed-tools: Bash(cargo build:*)
---

Build the optimized release version of bkmr.

## Current Status

Branch: !`git branch --show-current`
Last commit: !`git log -1 --oneline`

## Task

Build the release version:

```bash
cd bkmr && cargo build --release
```

The release binary will be created at:
`bkmr/target/release/bkmr`

After successful build:
1. Report binary size
2. Suggest running tests if not recently run
3. Suggest installation with `make install` if ready
