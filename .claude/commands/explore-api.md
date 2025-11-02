---
description: Explore and understand bkmr API surface
allowed-tools: Read, Grep, Bash(cargo doc:*)
---

Explore the bkmr API surface and public interfaces.

## API Discovery

### 1. Generate documentation
```bash
cd bkmr && cargo doc --open
```

### 2. List public types
```bash
cd bkmr && rg "^pub struct|^pub enum|^pub trait" src/ --no-heading
```

### 3. Repository traits
```bash
cd bkmr && rg "pub trait.*Repository" src/domain/repositories/ -A 10
```

### 4. Application services
```bash
cd bkmr && rg "pub struct.*Service" src/application/services/ -A 5
```

### 5. CLI commands
```bash
cd bkmr && rg "pub struct.*Command" src/cli/commands/ -A 5
```

## API Surface Report

Generate a report covering:

1. **Public Traits**
   - Repository interfaces
   - Service interfaces

2. **Public Structs**
   - Domain models
   - Command definitions
   - Configuration types

3. **Public Functions**
   - Utility functions
   - Factory functions

4. **Error Types**
   - Error hierarchies
   - Error variants

Present the API surface in a structured format for documentation or external integration planning.
