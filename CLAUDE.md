# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

BKMR is a fast, feature-rich CLI knowledge management system written in Rust that handles bookmarks, code snippets, shell commands, markdown documents, and more with AI-powered semantic search capabilities.

## Architecture

The project follows clean architecture with an onion model:

```
Infrastructure → Domain → Application → Presentation (CLI)
```

### Layer Responsibilities

- **Domain** (`src/domain/`): Core business logic, entities, and repository interfaces
- **Application** (`src/application/`): Use cases, services, and orchestration
- **Infrastructure** (`src/infrastructure/`): External systems (SQLite, embeddings, DI)
- **CLI** (`src/cli/`): Command-line interface and user interaction
- **LSP** (`src/lsp/`): Language Server Protocol implementation for editor integration

### Key Components

- **Repository Pattern**: Abstract data access through traits in domain layer
- **Dependency Injection**: Container-based DI in `infrastructure/di/`
- **Template System**: Jinja2-like interpolation via `minijinja`
- **Semantic Search**: OpenAI embeddings integration in `infrastructure/embeddings/`
- **LSP Server**: Built-in Language Server Protocol for snippet completion

## Development Commands

### Building and Testing

```bash
# Build release version
cd bkmr && cargo build --release

# Build debug version (faster compilation)
cd bkmr && cargo build

# Run tests - CRITICAL: Must use single-threaded execution
cd bkmr && cargo test -- --test-threads=1

# Or use Makefile from root
make test           # Runs tests single-threaded
make build          # Release build
make build-fast     # Debug build

# Format code
cd bkmr && cargo fmt

# Lint and auto-fix
cd bkmr && cargo clippy --fix -- -A unused_imports
cd bkmr && cargo fix --lib -p bkmr --tests

# Generate documentation
cd bkmr && cargo doc --open
```

### Installation

```bash
# Install release version
make install        # Installs to ~/bin/bkmr with versioning

# Install debug version (faster)
make install-debug

# Uninstall
make uninstall
```

### Running from Development

```bash
# Create test database
cd bkmr && BKMR_DB_URL=/tmp/bkmr_test.db cargo run -- create-db /tmp/bkmr_test.db

# Add bookmark
cd bkmr && BKMR_DB_URL=/tmp/bkmr_test.db cargo run -- add https://example.com test,demo

# Search
cd bkmr && BKMR_DB_URL=/tmp/bkmr_test.db cargo run -- search test

# Edit bookmark
cd bkmr && BKMR_DB_URL=/tmp/bkmr_test.db cargo run -- edit 1

# Start LSP server
cd bkmr && cargo run -- lsp
```

### Testing Specific Features

```bash
# Test LSP functionality
make test-lsp                # All LSP tests
make test-lsp-client         # LSP protocol communication
make test-lsp-filtering      # LSP filtering behavior
make test-lsp-language       # LSP language-aware filtering

# Test file import
make import-files

# Test URL metadata extraction (verbose)
make test-url-details

# Test database migration
make run-migrate-db

# Test template editing
make test-edit-bookmark-with-template
```

## Error Handling Patterns

The project has strict error handling patterns documented in `ERROR_HANDLING.md`:

### Error Type Hierarchy

1. **Infrastructure**: `SqliteRepositoryError`, `InfrastructureError`, `InterpolationError`
2. **Domain**: `DomainError`, `RepositoryError`
3. **Application**: `ApplicationError`
4. **CLI**: `CliError`

### Key Principles

- All errors implement `.context()` for adding contextual information
- Use `From` traits for explicit error conversions at layer boundaries
- Use `?` operator for propagation with `.map_err()` for layer transitions
- Result type aliases: `DomainResult<T>`, `ApplicationResult<T>`, `CliResult<T>`, `SqliteResult<T>`

### Exit Codes

- `0`: Success
- `64`: Usage error (invalid arguments)
- `65`: Duplicate name conflict (import without --update)
- `130`: User cancellation (Ctrl+C)

## Testing Requirements

**CRITICAL**: All tests MUST be run single-threaded due to shared SQLite database and environment variables:

```bash
cargo test -- --test-threads=1
```

Parallel execution causes race conditions. The Makefile enforces this requirement.

### Test Naming Convention

Follow the `given_X_when_Y_then_Z()` pattern:

```rust
#[test]
fn given_valid_url_when_loading_details_then_returns_correct_metadata() {
    // Arrange
    let url = "https://example.com";

    // Act
    let result = load_url_details(url);

    // Assert
    assert!(result.is_ok());
}
```

## Dependencies and Features

### Core Dependencies

- **diesel**: ORM with SQLite backend, migrations support
- **tower-lsp**: LSP server implementation
- **tokio**: Async runtime for LSP
- **minijinja**: Template interpolation engine
- **reqwest**: HTTP client for URL metadata fetching
- **skim**: Fuzzy finder integration
- **arboard**: Cross-platform clipboard with Wayland support
- **clap**: CLI argument parsing

### Optional Features

- OpenAI embeddings for semantic search
- LSP server always included (no feature flag)

## Database and Migrations

- Uses Diesel ORM with SQLite
- Migrations in `bkmr/migrations/`
- Configuration: `bkmr/diesel.toml`
- Database URL: Set via `BKMR_DB_URL` environment variable

```bash
# Install diesel CLI (if needed)
cargo install diesel_cli --no-default-features --features sqlite

# Run migrations
cd bkmr && diesel migration run
```

## Release Process

```bash
# Version bump commands (from root)
make bump-patch     # x.y.Z
make bump-minor     # x.Y.0
make bump-major     # X.0.0

# Requirements:
# - GITHUB_TOKEN environment variable set
# - Creates git tag and GitHub release automatically
```

Version follows Semantic Versioning and is stored in `VERSION` file.

## Platform Support

- **Linux**: Native support with Wayland clipboard integration
- **macOS**: Full support
- **Windows**: Supported via cross-compilation

## Documentation

- **Wiki**: https://github.com/sysid/bkmr/wiki
- **API Docs**: Generate with `cargo doc --open`
- **Contributing**: See `CONTRIBUTING.md`
- **Error Handling**: See `ERROR_HANDLING.md`

## Common Development Tasks

### Adding New Commands

1. Add command struct in `src/cli/commands/`
2. Implement command handler in `src/application/`
3. Wire up in `src/cli/app.rs`
4. Add tests following naming convention

### Modifying Database Schema

1. Create migration: `diesel migration generate <name>`
2. Edit up.sql and down.sql in `bkmr/migrations/`
3. Test migration with `make run-migrate-db`
4. Update Diesel models in `src/domain/`

### Adding LSP Features

1. Modify LSP handlers in `src/lsp/services/`
2. Test with `make test-lsp`
3. Update editor integration docs

## Code Style

- Use **snake_case** for functions, variables, and file names
- Use **PascalCase** for structs, enums, and traits
- Document all public APIs with examples
- Follow Rust formatting standards (`cargo fmt`)
- Enable all clippy warnings and fix them

## Environment Variables

- `BKMR_DB_URL`: Database file path (required)
- `RUST_LOG`: Logging level (e.g., `skim=info` to reduce noise)
- `OPENAI_API_KEY`: For semantic search features
