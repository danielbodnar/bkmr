---
description: Profile bkmr performance
allowed-tools: Bash(cargo:*), Bash(perf:*)
argument-hint: [command to profile]
---

Profile bkmr performance for command: $ARGUMENTS

## Profiling Strategy

1. **CPU profiling with flamegraph:**
```bash
cargo install flamegraph
cd bkmr && cargo flamegraph --bin bkmr -- $ARGUMENTS
```

This will:
- Run the command with profiling
- Generate flamegraph.svg
- Open in browser automatically

2. **Alternative: perf (Linux only):**
```bash
cd bkmr && cargo build --release
perf record --call-graph dwarf ./target/release/bkmr $ARGUMENTS
perf report
```

3. **Memory profiling with valgrind:**
```bash
cd bkmr && cargo build --release
valgrind --tool=massif ./target/release/bkmr $ARGUMENTS
ms_print massif.out.*
```

## Analysis

After profiling:
1. Identify bottlenecks (functions taking >10% of time)
2. Check for unexpected allocations
3. Look for N+1 query patterns
4. Suggest optimizations

Delegate detailed optimization to rust-performance agent if needed.
