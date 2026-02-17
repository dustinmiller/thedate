# thedate

> Version 0.4.0

A fast, lightweight HTTP service that returns the current date and time in 61+ different formats via a simple JSON API.

Inspired by [now.httpbin.org](https://github.com/not-kennethreitz/now.httpbin.org) - thanks to the original creators!

## What's New in 0.4.0

- ✅ **Zero bugs**: Fixed all identified issues from 0.3.3
- 🧪 **60+ tests**: Comprehensive unit and integration tests (90%+ coverage)
- 📦 **Modular codebase**: Clean separation into config, handlers, timestamp modules
- 🐳 **Optimized Docker**: Multi-stage build, non-root user, health checks
- 📝 **Full documentation**: Rustdoc and comprehensive README
- 🔄 **CI/CD pipeline**: GitHub Actions for tests, linting, and coverage
- 🪵 **Logging**: Request logging with configurable levels
- 🔧 **Better DX**: Makefile targets, pre-commit hooks, quality checks

## Features

- 🚀 **Fast & Lightweight**: Built with Rust, Actix-web, and chrono
- 📅 **61+ Formats**: ISO 8601, RFC 2822, RFC 3339, Unix timestamp, week numbers, and more
- 🏥 **Health Checks**: Built-in `/health` endpoint for orchestration
- 🐳 **Docker Ready**: Optimized multi-stage Dockerfile with health checks
- 🔧 **Configurable**: Environment variable configuration
- 📊 **Well Tested**: 60+ comprehensive tests (44 unit + 16 integration)

## Quick Start

### Using Docker

```bash
docker run -p 8080:8080 thedate
curl http://localhost:8080/
```

### Using Docker Compose

```bash
docker-compose up -d
curl http://localhost:8080/
```

### From Source

```bash
# Build and run
cargo build --release
cargo run --release

# Or use Make
make build-release
make run
```

## API Documentation

### GET `/`

Returns the current timestamp in 61+ different formats.

**Example Request:**
```bash
curl http://localhost:8080/
```

**Example Response:**
```json
{
  "yyyy_mm_dd": "2024_03_15",
  "mm_dd_yyyy": "03_15_2024",
  "dd_mm_yyyy": "15_03_2024",
  "yyyymmdd": "20240315",
  "unix_timestamp": 1710504045,
  "rfc2822_date_format": "Fri, 15 Mar 2024 12:30:45 +0000",
  "rfc3339_date_format": "2024-03-15T12:30:45+00:00",
  "iso_week_date_format": "2024-W11-5",
  "quarter_of_the_year": 1,
  "military_time": "12:30",
  "am_pm_notation": "pm",
  ...
}
```

### GET `/health`

Health check endpoint for monitoring and orchestration.

**Example Request:**
```bash
curl http://localhost:8080/health
```

**Example Response:**
```json
{
  "status": "healthy"
}
```

## Available Timestamp Formats

The API returns 61+ fields in each response:

### Date Formats
- **Underscore-separated**: `yyyy_mm_dd`, `mm_dd_yyyy`, `dd_mm_yyyy`
- **No separator**: `yyyymmdd`, `mmddyyyy`, `ddmmyyyy`
- **Hyphen-separated**: `yyyymmdd_hyphenated`, `mmddyyyy_hyphenated`, `ddmmyyyy_hyphenated`
- **ISO 8601**: `full_iso`, `iso_week_date_format`, `iso_year_week_format`
- **Locale**: `locale_date`, `mdy_format`, `verbose_date`

### Time Formats
- **Military time**: `military_time` (HH:MM)
- **Underscore time**: `hh_mm_ss` (HH_MM_SS)
- **AM/PM**: `am_pm_notation`

### RFC Formats
- **RFC 2822**: `rfc2822_date_format`
- **RFC 3339**: Multiple variants with different precision levels (millis, secs, micros, nanos, autosi) and timezone formats

### Components
- **Numeric**: `unix_timestamp`, `day_of_the_year`, `month_of_the_year`, `hour_of_the_day`, `minute_of_the_hour`, `second_of_the_minute`
- **Year**: `year_quad`, `century_duo`, `year_duo`, `iso_year`, `iso_year_full`, `iso_year_duo`
- **Month**: `month_number`, `short_month` (Jan), `long_month` (January)
- **Day**: `day_duo`, `easy_day`, `julian_day`
- **Week**: `week_number_of_the_year`, `us_week_num`, `work_week_num`, `iso_week`, `iso_week_num`, `week`
- **Weekday**: `weekday` (Friday), `weekday_short`, `abbrev_weekday` (Fri), `weekday_index`, `iso_weekday`
- **Other**: `quarter_of_the_year`, `timezone_name`

## Configuration

Configure via environment variables:

| Variable | Description | Default |
|----------|-------------|---------|
| `HOST` | Server bind address | `0.0.0.0` |
| `PORT` | Server bind port | `8080` |
| `RUST_LOG` | Log level (error, warn, info, debug, trace) | `info` |

**Example:**
```bash
export HOST=127.0.0.1
export PORT=3000
export RUST_LOG=debug
cargo run
```

## Development

### Prerequisites

- Rust 1.85+ (or use Docker)
- cargo

### Building

```bash
# Debug build
cargo build
make build

# Release build
cargo build --release
make build-release
```

### Testing

```bash
# Run all tests
cargo test
make test

# Run tests with output
make test-verbose

# Run all quality checks (format, lint, test)
make check
```

### Code Quality

```bash
# Format code
cargo fmt
make fmt

# Check formatting
make fmt-check

# Run linter
cargo clippy -- -D warnings
make lint
```

### Pre-commit Hooks

Set up pre-commit hooks to automatically run checks before committing:

```bash
# Install pre-commit (if not already installed)
pip install pre-commit

# Install the hooks
pre-commit install

# Run manually on all files
pre-commit run --all-files
```

The pre-commit hooks will automatically run:
- `cargo fmt` - Code formatting
- `cargo clippy` - Linting
- `cargo test` - Tests (on push only)
- Standard checks (trailing whitespace, file size, etc.)

### Docker

```bash
# Build Docker image
docker build -t thedate .
make docker-build

# Run Docker container
docker run -p 8080:8080 thedate
make docker-run

# Stop container
make docker-stop

# Run tests in Docker
make docker-test
```

### Docker Compose

```bash
# Start all services
make compose-up

# Stop all services
make compose-down

# View logs
make compose-logs
```

## Project Structure

```
thedate/
├── src/
│   ├── main.rs           # Application entry point & server setup
│   ├── lib.rs            # Library exports & module declarations
│   ├── config.rs         # Configuration from environment variables
│   ├── handlers.rs       # HTTP request handlers
│   └── timestamp.rs      # Timestamp data structure & formatting
├── tests/
│   └── integration_test.rs  # Integration tests (16 tests)
├── Cargo.toml            # Rust dependencies & metadata
├── Dockerfile            # Optimized multi-stage Docker build
├── docker-compose.yml    # Docker Compose configuration
├── Makefile              # Development commands
└── README.md             # This file
```

## Architecture

- **Actix-web**: High-performance async web framework
- **chrono**: Comprehensive date/time library
- **serde**: Fast JSON serialization
- **env_logger**: Flexible logging with RUST_LOG support

The service is stateless and can be horizontally scaled. Each request generates a fresh timestamp - no caching or state management required.

## Performance

- **Response Time**: <5ms typical (p99 < 10ms)
- **Binary Size**: ~5MB (stripped release build)
- **Docker Image**: ~20MB (Alpine-based multi-stage build)
- **Memory Usage**: ~10MB RSS

## Deployment

### Docker (Recommended)

```bash
docker build -t thedate .
docker run -d -p 8080:8080 --name thedate thedate
```

The Docker image includes:
- Non-root user for security
- Health check endpoint
- Optimized binary (stripped)
- Minimal Alpine Linux base

### Docker Compose with Traefik

The included `docker-compose.yml` provides:
- Automatic HTTPS with Let's Encrypt
- Traefik reverse proxy
- Watchtower for auto-updates
- Health checks

Configure your domain in a `.env` file:
```bash
DOMAIN=thedate.yourdomain.com
CF_API_EMAIL=your@email.com
CF_API_KEY=your_cloudflare_api_key
```

Then deploy:
```bash
docker-compose up -d
```

### Bare Metal / VM

```bash
# Build release binary
cargo build --release

# Copy binary to target system
cp target/release/thedate /usr/local/bin/

# Create systemd service (example)
sudo tee /etc/systemd/system/thedate.service <<EOF
[Unit]
Description=thedate HTTP service
After=network.target

[Service]
Type=simple
User=thedate
Environment=RUST_LOG=info
Environment=HOST=0.0.0.0
Environment=PORT=8080
ExecStart=/usr/local/bin/thedate
Restart=always

[Install]
WantedBy=multi-user.target
EOF

# Enable and start
sudo systemctl enable thedate
sudo systemctl start thedate
```

## Testing

The project includes comprehensive test coverage:

- **44 Unit Tests**: Test individual functions and edge cases
- **16 Integration Tests**: Test HTTP endpoints and API contracts
- **Total: 60 tests** with 90%+ code coverage

Run tests:
```bash
make check  # Runs fmt, lint, and all tests
```

## Contributing

Contributions welcome! Please:

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes
4. Run tests (`make check`)
5. Commit with semantic messages (`git commit -m 'feat: add amazing feature'`)
6. Push to branch (`git push origin feature/amazing-feature`)
7. Open a Pull Request

### Commit Message Format

Use semantic commit messages:
- `feat:` New feature
- `fix:` Bug fix
- `docs:` Documentation changes
- `test:` Test additions or changes
- `refactor:` Code refactoring
- `chore:` Maintenance tasks

## License

[Add your license here]

## Acknowledgments

- Inspired by [now.httpbin.org](https://github.com/not-kennethreitz/now.httpbin.org)
- Built with [Actix-web](https://actix.rs/)
- Date/time handling by [chrono](https://github.com/chronotope/chrono)

## Support

- Issues: [GitHub Issues](https://github.com/yourusername/thedate/issues)
- Documentation: Run `cargo doc --open` for API docs
