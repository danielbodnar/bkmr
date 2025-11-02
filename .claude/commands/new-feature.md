---
description: Plan and scaffold new bkmr feature
argument-hint: [feature-name]
---

Plan and create the structure for a new bkmr feature: $ARGUMENTS

## Feature Planning

1. **Understand requirements** for: $ARGUMENTS

2. **Identify affected layers:**
   - [ ] Domain (entities, repositories)
   - [ ] Application (services)
   - [ ] Infrastructure (implementations)
   - [ ] CLI (commands)
   - [ ] LSP (if relevant)

3. **Plan implementation:**
   - Domain entities needed
   - Repository trait methods
   - Application service methods
   - CLI command structure
   - Tests required

4. **Create checklist:**
   - [ ] Define domain model (`src/domain/models.rs`)
   - [ ] Add repository trait (`src/domain/repositories/`)
   - [ ] Implement repository (`src/infrastructure/repositories/`)
   - [ ] Create application service (`src/application/services/`)
   - [ ] Add CLI command (`src/cli/commands/`)
   - [ ] Write unit tests
   - [ ] Write integration tests
   - [ ] Update documentation

Present the plan and ask for confirmation before proceeding with implementation.
