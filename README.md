# X API v2 Client Library

A type-safe, async-first Rust library for the X (Twitter) API v2, designed for bot development and production use.

[![Crates.io](https://img.shields.io/crates/v/x-api-client.svg)](https://crates.io/crates/x-api-client)
[![Documentation](https://docs.rs/x-api-client/badge.svg)](https://docs.rs/x-api-client)
[![License](https://img.shields.io/crates/l/x-api-client.svg)](https://github.com/yourusername/x-api-client#license)
[![Rust Version](https://img.shields.io/badge/rust-1.85%2B-blue.svg)](https://www.rust-lang.org)

## Features

- **Complete API Coverage**: Full implementation of all X API v2 endpoints
- **Multi-Authentication Support**: OAuth 1.0a, OAuth 2.0, and Bearer tokens
- **Intelligent Rate Limiting**: Automatic rate limit tracking and request queuing
- **Robust Streaming**: Production-ready streaming with automatic reconnection and backfill
- **Type-Safe**: Strongly-typed models with builder patterns
- **Automatic Retry Logic**: Configurable retry strategies with exponential backoff
- **Bot-Optimized**: High-level abstractions for common bot patterns

## Minimum Supported Rust Version (MSRV)

This crate requires **Rust 1.85 or later** (required for Rust 2024 edition).

The MSRV is enforced via the `rust-version` field in `Cargo.toml` and will be checked during compilation. We follow a conservative MSRV policy and will only increase the MSRV in major version releases.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
x-api-client = "0.1"
```

### Optional Features

This library uses Cargo features to make certain functionality optional, keeping the default build lightweight.

```toml
[dependencies]
# Default: minimal dependencies
x-api-client = "0.1"

# With scheduling support
x-api-client = { version = "0.1", features = ["scheduling"] }

# With all features
x-api-client = { version = "0.1", features = ["scheduling"] }
```

#### Available Features

- **`scheduling`**: Enables scheduled tweet posting with cron syntax via `tokio-cron-scheduler`
  - Adds the ability to schedule tweets at specific times or recurring intervals
  - Example: `client.schedule_tweet(content, "0 0 9 * * *")` to post daily at 9 AM

- **`real_api_tests`**: Marker feature for running integration tests against the real X API
  - Used in development to enable tests that make actual API calls
  - Requires valid API credentials in environment variables
  - Not needed for normal library usage

#### Default Features

By default, no optional features are enabled to minimize dependencies. The core functionality (tweets, users, authentication, rate limiting, streaming, etc.) is always available.

## Quick Start

```rust
// Example usage will be added as implementation progresses
```

## Development Status

This library is currently under active development. Check the [project roadmap](docs/ROADMAP.md) for implementation progress.

## Documentation

- [API Documentation](https://docs.rs/x-api-client)
- [Examples](examples/)
- [Contributing Guide](CONTRIBUTING.md)

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contributing

Contributions are welcome! Please read [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
