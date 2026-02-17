# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

**thedate** is a high-performance HTTP service that returns current date/time in 61+ formats. Built with Rust, Actix-web, and chrono. Version 0.4.0 is production-ready with 90%+ test coverage.

## Essential Commands

### Development Workflow
```bash
# Complete quality check (fmt + clippy + tests)
make check

# Individual checks
cargo fmt                    # Format code
cargo clippy -- -D warnings  # Lint (all warnings are errors)
cargo test                   # Run all tests
cargo test --lib            # Unit tests only
cargo test --test integration_test  # Integration tests only

# Run specific test
cargo test test_quarter_calculation

# Build and run
cargo run                    # Debug build
cargo build --release        # Production build
```

### Pre-commit Hooks
```bash
pre-commit install           # One-time setup
pre-commit run --all-files   # Manual run
```

Hooks automatically run: `cargo fmt`, `cargo clippy`, `cargo test` (on push)

### Docker
```bash
make docker-build            # Build image
make docker-run              # Run locally on :8080
make compose-up              # Start with Traefik/Watchtower
```

## Code Architecture

### Module Structure
```
src/
├── main.rs       - Server initialization, logging setup
├── lib.rs        - Public API exports
├── config.rs     - Environment variable configuration
├── handlers.rs   - HTTP endpoints (home, health_check)
└── timestamp.rs  - Core timestamp generation logic
```

**Critical Design Pattern**: All timestamp formats are generated **once per request** from a single `DateTime<Utc>` in the `from_chrono()` function. This ensures consistency across all 61+ format fields.

### Request Flow
1. HTTP request hits `handlers::home()`
2. `Utc::now()` captures current time
3. `timestamp::from_chrono()` generates all 61 formats synchronously
4. Serde serializes Timestamp struct to JSON
5. Response returned with all formats

**Why this matters**: Any changes to timestamp logic must maintain field consistency. The struct has intentional redundancy (e.g., `week_number_of_the_year` and `week` both use `dt.iso_week().week()`) for API backward compatibility.

### Configuration Pattern
The `Config` struct reads from environment variables:
- `HOST` (default: "0.0.0.0")
- `PORT` (default: "8080")
- `RUST_LOG` (default: "info")

Config is created once in `main()` and consumed to generate bind address. If adding new config, follow the same pattern in `config.rs`.

### Testing Strategy

**Unit tests** (44 tests in `src/timestamp.rs`):
- Test individual format outputs with fixed DateTime values
- Edge cases: leap years, year boundaries, week 53, quarter transitions
- Field consistency checks

**Integration tests** (16 tests in `tests/integration_test.rs`):
- HTTP endpoint behavior
- Complete API contract validation (all 61 fields present)
- JSON serialization roundtrip
- Health check endpoint

**Doc tests** (3 tests):
- Config usage examples in rustdoc comments

When modifying timestamp formats, **both** unit and integration tests must pass. Integration tests validate the full HTTP->JSON pipeline.

## Critical Invariants

### API Backward Compatibility
The JSON response **must always contain exactly 61 fields**. This count is validated in `test_all_61_fields_present()`.

When adding fields:
1. Add to Timestamp struct in `timestamp.rs`
2. Add assignment in `from_chrono()`
3. Update field count in integration test
4. Update README.md format list
5. This is a **breaking change** - bump major version

### Timestamp Field Grouping
Fields are organized in `from_chrono()` with comment sections:
- Date formats (underscore/no-sep/hyphenated)
- Time formats
- RFC formats (2822, 3339 variants)
- ISO week formats
- Numeric components
- Weekday/month names

Maintain this organization when adding formats.

## Docker Build Details

The Dockerfile uses a **multi-stage build** with dependency caching:

1. **Builder stage**: Creates dummy project, builds deps, then rebuilds with real source
2. **Runtime stage**: Copies stripped binary to minimal Alpine image

**Key optimization**: `Cargo.toml` and `Cargo.lock` copied first, then dummy main.rs built to cache dependencies. Real source copied after. This means dependency changes trigger longer builds, but code changes are fast.

**Security**: Runs as non-root user `thedate:1000`

## Common Development Scenarios

### Adding a New Timestamp Format
1. Add field to `Timestamp` struct (alphabetically in appropriate section)
2. Add assignment in `from_chrono()` (in matching section with comment)
3. Add unit test in `timestamp::tests` using `test_dt()` helper
4. Update integration test field count
5. Document in README.md under "Available Timestamp Formats"

### Modifying HTTP Endpoints
Handlers are in `handlers.rs`. Keep them thin - they should only:
- Call timestamp generation
- Return HttpResponse with JSON

Business logic stays in module files, not handlers.

### Updating Dependencies
```bash
cargo update                 # Update within semver
# Edit Cargo.toml for major updates
cargo build                  # Verify build
cargo test                   # Verify tests
docker build -t thedate .    # Verify Docker build
```

Major dependency updates may affect Docker layer caching. Test with `make docker-build`.

## Deployment Context

This service is designed to run behind Traefik reverse proxy (see `docker-compose.yml`):
- Traefik handles HTTPS/Let's Encrypt
- Watchtower handles auto-updates
- Health check at `/health` used by Docker and orchestrators

The service is **stateless** - scales horizontally without coordination.

## When Making Changes

**Always run before committing**:
```bash
make check
```

This runs fmt, clippy, and all tests. Pre-commit hooks will catch issues but running manually is faster for iteration.

**Never commit if**:
- Tests fail
- Clippy warnings exist
- Code isn't formatted

The CI pipeline (`ci.yml`) runs the same checks on push/PR.
