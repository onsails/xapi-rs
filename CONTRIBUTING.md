# Contributing to X API v2 Client Library

Thank you for your interest in contributing! This document provides guidelines and requirements for contributors.

## Getting Started

### Prerequisites

- **Rust 1.85 or later** (MSRV - Minimum Supported Rust Version, required for Rust 2024 edition)
- Git
- Familiarity with async Rust and Tokio

### Development Setup

1. Clone the repository:
   ```bash
   git clone https://github.com/yourusername/x-api-client.git
   cd x-api-client
   ```

2. Verify your Rust version:
   ```bash
   rustc --version  # Should be 1.85 or later
   ```

3. Build the project:
   ```bash
   cargo build
   ```

4. Run tests:
   ```bash
   cargo test
   ```

5. Check code formatting and linting:
   ```bash
   cargo fmt --check
   cargo clippy -- -D warnings
   ```

## Minimum Supported Rust Version (MSRV)

This project requires **Rust 1.85 or later**.

### MSRV Policy

- The MSRV is specified in `Cargo.toml` via the `rust-version` field
- The MSRV will only be increased in **major version releases**
- All code must compile on the MSRV without warnings
- CI automatically tests against the MSRV to ensure compliance

### Why Rust 1.85?

Rust 1.85 is required for the **Rust 2024 edition**, which provides:
- Improved error handling and diagnostics
- Better async/await ergonomics
- Enhanced pattern matching capabilities
- Modern language features and improvements

## Development Workflow

### 1. Create a Branch

```bash
git checkout -b feature/your-feature-name
```

### 2. Make Changes

- Write clear, idiomatic Rust code
- Follow the existing code style
- Add tests for new functionality
- Update documentation as needed

### 3. Test Your Changes

```bash
# Run all tests
cargo test --all-features

# Check formatting
cargo fmt --check

# Run clippy
cargo clippy --all-features -- -D warnings

# Test with MSRV (if you have it installed)
cargo +1.85 check --all-features
```

### 4. Commit Your Changes

- Write clear, descriptive commit messages
- Follow conventional commit format if possible:
  - `feat:` for new features
  - `fix:` for bug fixes
  - `docs:` for documentation changes
  - `test:` for test additions/changes
  - `refactor:` for code refactoring
  - `chore:` for maintenance tasks

### 5. Push and Create Pull Request

```bash
git push origin feature/your-feature-name
```

Then create a pull request on GitHub with:
- Clear description of changes
- Reference to any related issues
- Test results

## Code Standards

### Formatting

All code must be formatted with `rustfmt`:

```bash
cargo fmt
```

### Linting

Code must pass `clippy` without warnings:

```bash
cargo clippy --all-features -- -D warnings
```

### Documentation

- All public APIs must have rustdoc comments
- Use examples in documentation where appropriate
- Keep documentation up to date with code changes

### Testing

- Write unit tests for new functionality
- Add integration tests for API endpoints
- Ensure tests pass on the MSRV
- Aim for high test coverage

## Feature Flags

When adding new optional functionality:

1. Create a feature flag in `Cargo.toml`
2. Make dependencies optional where appropriate
3. Document the feature in README.md
4. Test with and without the feature enabled

Example:
```toml
[features]
my-feature = ["some-dependency"]

[dependencies]
some-dependency = { version = "1.0", optional = true }
```

## Pull Request Process

1. Ensure your code passes all CI checks
2. Update documentation if needed
3. Add entries to CHANGELOG.md (if applicable)
4. Request review from maintainers
5. Address review feedback promptly
6. Wait for approval before merging

## CI/CD

Our CI pipeline automatically:
- Tests on MSRV (Rust 1.85)
- Tests on stable Rust
- Runs `cargo fmt --check`
- Runs `cargo clippy` with strict warnings
- Tests all feature combinations
- Checks documentation builds

## Questions?

If you have questions about contributing, please:
- Open an issue for discussion
- Check existing issues and discussions
- Reach out to maintainers

Thank you for contributing! 🎉
