---
description: Create new test file
argument-hint: [test-name]
---

Create a new test file for: $ARGUMENTS

## Test File Structure

Create test file at `bkmr/tests/test_$ARGUMENTS.rs`:

```rust
//! Integration tests for $ARGUMENTS functionality.

use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::TempDir;

#[test]
fn given_valid_input_when_$ARGUMENTS_then_succeeds() {
    // Arrange
    let temp_dir = TempDir::new().unwrap();
    let db_path = temp_dir.path().join("test.db");

    // Create test database
    Command::cargo_bin("bkmr")
        .unwrap()
        .arg("create-db")
        .arg(&db_path)
        .assert()
        .success();

    // Act
    let mut cmd = Command::cargo_bin("bkmr").unwrap();
    cmd.env("BKMR_DB_URL", &db_path);
    cmd.arg("$ARGUMENTS");
    // Add test arguments

    // Assert
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("expected output"));
}

#[test]
fn given_invalid_input_when_$ARGUMENTS_then_fails() {
    // Arrange
    let temp_dir = TempDir::new().unwrap();
    let db_path = temp_dir.path().join("test.db");

    Command::cargo_bin("bkmr")
        .unwrap()
        .arg("create-db")
        .arg(&db_path)
        .assert()
        .success();

    // Act
    let mut cmd = Command::cargo_bin("bkmr").unwrap();
    cmd.env("BKMR_DB_URL", &db_path);
    cmd.arg("$ARGUMENTS");
    cmd.arg("invalid-input");

    // Assert
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("error"));
}
```

## Unit Test Alternative

For unit tests in service modules (`src/application/services/tests/`):

```rust
#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_repository() -> Arc<dyn BookmarkRepository> {
        // Create in-memory or mock repository
        Arc::new(/* test repository */)
    }

    #[test]
    fn given_valid_data_when_$ARGUMENTS_then_returns_expected() {
        // Arrange
        let repository = create_test_repository();
        let service = Service::new(repository);

        // Act
        let result = service.$ARGUMENTS(/* args */);

        // Assert
        assert!(result.is_ok());
    }
}
```

Create the test file?
