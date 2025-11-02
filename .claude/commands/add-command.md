---
description: Create new CLI command
argument-hint: [command-name]
---

Create a new CLI command for bkmr: $ARGUMENTS

## Command Scaffolding

Create the following files:

### 1. Command struct (`src/cli/commands/$ARGUMENTS.rs`)

```rust
use clap::Args;

#[derive(Args, Debug)]
pub struct $ARGUMENTSCommand {
    /// Description of first argument
    pub arg1: String,

    /// Optional argument
    #[arg(short, long)]
    pub optional_arg: Option<String>,
}
```

### 2. Application service (`src/application/services/$ARGUMENTS_service.rs`)

```rust
use crate::domain::repositories::BookmarkRepository;
use crate::application::errors::{ApplicationError, ApplicationResult};
use std::sync::Arc;

pub struct $ARGUMENTSService {
    repository: Arc<dyn BookmarkRepository>,
}

impl $ARGUMENTSService {
    pub fn new(repository: Arc<dyn BookmarkRepository>) -> Self {
        Self { repository }
    }

    pub fn execute(&self, /* args */) -> ApplicationResult<()> {
        // Implementation
        Ok(())
    }
}
```

### 3. CLI handler (`src/cli/app.rs`)

Add to Command enum and handler:
```rust
// In Command enum
$ARGUMENTS($ARGUMENTSCommand),

// In match statement
Command::$ARGUMENTS(cmd) => handle_$ARGUMENTS_command(cmd, &container),
```

### 4. Tests (`src/application/services/tests/$ARGUMENTS_service_test.rs`)

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_valid_input_when_executing_then_succeeds() {
        // Arrange
        let service = create_test_service();

        // Act
        let result = service.execute(/* args */);

        // Assert
        assert!(result.is_ok());
    }
}
```

## Integration Checklist

- [ ] Command struct created
- [ ] Service implemented
- [ ] CLI wired up
- [ ] Tests written
- [ ] Documentation added
- [ ] Compiled successfully
- [ ] Tests pass (single-threaded)

Proceed with creating these files?
