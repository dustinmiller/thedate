# Build stage - use official Rust image
FROM rust:1.85-alpine3.20 AS builder

# Install build dependencies
RUN apk add --no-cache musl-dev

WORKDIR /usr/src/thedate

# Copy manifests first for better layer caching
COPY Cargo.toml Cargo.lock ./

# Create dummy source to cache dependencies
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    echo "pub fn dummy() {}" > src/lib.rs && \
    cargo build --release && \
    rm -rf src

# Copy actual source code
COPY src ./src
COPY tests ./tests

# Build with release optimizations
# Touch main.rs to ensure rebuild after dummy cache
RUN touch src/main.rs && \
    cargo build --release && \
    strip target/release/thedate

# Runtime stage - minimal image
FROM alpine:3.20

# Install runtime dependencies only
RUN apk add --no-cache libgcc ca-certificates wget

# Create non-root user
RUN adduser -D -u 1000 thedate

# Copy binary from builder
COPY --from=builder /usr/src/thedate/target/release/thedate /usr/local/bin/thedate

# Switch to non-root user
USER thedate

EXPOSE 8080

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD wget --no-verbose --tries=1 --spider http://localhost:8080/health || exit 1

CMD ["thedate"]
