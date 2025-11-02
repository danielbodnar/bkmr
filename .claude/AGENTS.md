# BKMR Sub-Agents Documentation

This document describes the specialized sub-agents available for the bkmr project, their purposes, and when Claude Code will invoke them.

## Overview

The bkmr project uses specialized sub-agents to maintain code quality, architectural consistency, and development velocity. Each agent has a focused responsibility and specialized expertise.

## Agent Architecture

```
Main Claude
    ├── bkmr (Core Agent) - General development, architecture, coordination
    ├── rust-architect - Clean architecture, Rust patterns, error handling
    ├── rust-performance - Performance optimization, profiling, benchmarking
    ├── lsp-specialist - LSP server development, tower-lsp integration
    ├── mcp-builder - Model Context Protocol server implementation
    ├── api-designer - REST API design, HTTP server implementation
    └── ui-architect - Desktop and Web UI design and implementation
```

## Agent Catalog

### 1. bkmr (Core Agent)

**File**: `.claude/agents/bkmr.md`

**Purpose**: Primary development agent for general bkmr features, architecture decisions, and workflow coordination.

**Expertise**:
- Clean architecture patterns (Domain → Application → Infrastructure → CLI)
- Repository pattern implementation
- Diesel ORM and SQLite integration
- Error handling hierarchy
- Testing strategies with single-threaded execution
- Feature development coordination

**Invocation triggers**: General feature development, architecture questions, integration between layers, cross-cutting concerns

### 2. rust-architect

**File**: `.claude/agents/rust-architect.md`

**Purpose**: Clean architecture, Rust design patterns, and error handling hierarchy specialist.

**Expertise**:
- Clean architecture onion model
- Repository pattern and trait design
- Error type hierarchies with `thiserror`
- From trait implementations for error conversion
- Result type aliases and error propagation
- Dependency injection patterns

**Invocation triggers**: Refactoring for clean architecture, error handling improvements, repository interface design, layer boundary definitions

### 3. rust-performance

**File**: `.claude/agents/rust-performance.md`

**Purpose**: Performance optimization, profiling, and benchmarking expert.

**Expertise**:
- Profiling with `cargo flamegraph`, `perf`, `valgrind`
- Benchmarking with `criterion`
- Memory optimization and allocation patterns
- SQLite query optimization
- Async runtime performance (tokio)
- Binary size reduction

**Invocation triggers**: Performance issues, memory usage concerns, query optimization, benchmark creation, profiling sessions

### 4. lsp-specialist

**File**: `.claude/agents/lsp-specialist.md`

**Purpose**: Language Server Protocol implementation expert using tower-lsp.

**Expertise**:
- LSP protocol specification
- tower-lsp framework
- Completion provider implementation
- Language-aware filtering
- Template interpolation in LSP context
- Async LSP handlers with tokio

**Invocation triggers**: LSP server enhancements, completion provider modifications, protocol debugging, editor integration issues

### 5. mcp-builder

**File**: `.claude/agents/mcp-builder.md`

**Purpose**: Model Context Protocol server implementation for bkmr functionality.

**Expertise**:
- MCP protocol specification
- Tool and resource definitions
- Server configuration with stdio/HTTP transport
- Integration with bkmr repository layer
- Schema definitions for MCP tools

**Invocation triggers**: Creating MCP server, adding MCP tools, MCP resource providers, AI assistant integration

### 6. api-designer

**File**: `.claude/agents/api-designer.md`

**Purpose**: REST API and HTTP server design for bkmr web interface.

**Expertise**:
- HTTP server frameworks (axum, actix-web)
- RESTful API design
- OpenAPI specification
- Authentication and authorization
- Request validation
- API versioning strategies

**Invocation triggers**: Creating REST API endpoints, API authentication, HTTP server configuration, API documentation

### 7. ui-architect

**File**: `.claude/agents/ui-architect.md`

**Purpose**: Desktop and web UI implementation for bkmr.

**Expertise**:
- Desktop UI with Tauri
- Web UI with Astro + Vue
- TypeScript integration
- State management patterns
- Component architecture
- Accessibility and responsive design

**Invocation triggers**: Creating desktop application, web interface development, UI/UX design, frontend-backend integration

## Slash Commands

The project includes 26 slash commands for common development tasks:

### Testing & Quality
- `/test` - Run tests single-threaded
- `/test-feature` - Test specific feature
- `/test-import` - Test file import
- `/semantic-test` - Test semantic search
- `/add-test` - Create new test file
- `/coverage` - Generate coverage report
- `/fix-clippy` - Fix clippy warnings
- `/check-arch` - Validate architecture

### Building & Performance
- `/build` - Build release version
- `/benchmark` - Run performance benchmarks
- `/profile` - Profile performance
- `/security-audit` - Run security audit
- `/deps-update` - Update dependencies

### Development
- `/new-feature` - Plan and scaffold feature
- `/quick-feature` - Rapid prototype
- `/add-command` - Create CLI command
- `/add-migration` - Create database migration
- `/refactor` - Refactor code

### Operations
- `/run-lsp` - Start LSP server
- `/release` - Prepare for release
- `/db-inspect` - Inspect database
- `/analyze-error` - Analyze errors

### Documentation
- `/update-docs` - Update documentation
- `/explore-api` - Explore API surface
- `/gen-example` - Generate examples
- `/compare-tools` - Compare with alternatives

## Usage Examples

### Feature Development Workflow

```bash
# 1. Plan feature
/new-feature export-json

# 2. Create command structure
/add-command export

# 3. Implement (agents handle this)

# 4. Test
/test-feature export

# 5. Check architecture
/check-arch

# 6. Fix any issues
/fix-clippy
```

### Performance Optimization Workflow

```bash
# 1. Profile
/profile search "test query"

# 2. Benchmark
/benchmark

# 3. Optimize (rust-performance agent handles)

# 4. Verify improvement
/benchmark
```

### Release Workflow

```bash
# 1. Update docs
/update-docs

# 2. Run full test suite
/test

# 3. Check coverage
/coverage

# 4. Security audit
/security-audit

# 5. Release
/release patch
```

## Agent Selection

Claude Code automatically selects agents based on:
- **Task context**: Nature of work being performed
- **File paths**: Files being modified
- **Keywords**: Terms in user requests
- **Current agent**: Agents can delegate to specialists

## Best Practices

1. **Trust automatic selection** - Let Claude choose the right agent
2. **Provide context** - Be specific about what you need
3. **Use slash commands** - For routine tasks
4. **Let agents delegate** - Complex tasks may involve multiple agents

## Testing Requirements

**CRITICAL**: All tests must run single-threaded:
```bash
cargo test -- --test-threads=1
```

This is enforced in all agent prompts and testing commands.

## Version History

- **1.0** (2025-10-31) - Initial agent system with 7 specialized agents and 26 slash commands
