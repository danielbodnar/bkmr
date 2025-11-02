---
description: Generate usage examples for documentation
argument-hint: [command-name]
---

Generate comprehensive usage examples for bkmr command: $ARGUMENTS

## Example Generation

### 1. Find command definition
```bash
cd bkmr && rg "struct $ARGUMENTSCommand" src/cli/commands/ -A 20
```

### 2. Generate examples

Create examples covering:

**Basic usage:**
```bash
bkmr $ARGUMENTS <basic-args>
```

**With options:**
```bash
bkmr $ARGUMENTS --option value
```

**Common use cases:**
```bash
# Use case 1: [Description]
bkmr $ARGUMENTS [example]

# Use case 2: [Description]
bkmr $ARGUMENTS [example]
```

**Error cases:**
```bash
# What happens when...
bkmr $ARGUMENTS invalid-input
# Expected: Error message
```

### 3. Integration examples

Show how this command fits into workflows:
```bash
# Workflow example
bkmr add "content" tags
bkmr $ARGUMENTS <args>
bkmr search "query"
```

## Output Format

Generate examples in markdown format suitable for:
- README.md
- Wiki pages (bkmr.wiki/)
- Command --help text
- Tutorial documentation

Present examples for review and approval for documentation.
