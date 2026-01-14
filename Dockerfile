# Multi-stage build for minimal final image size
# Stage 1: Build the application
FROM rustlang/rust:nightly-alpine AS builder

# Install build dependencies
RUN apk add --no-cache \
    musl-dev \
    sqlite-dev \
    openssl-dev \
    pkgconfig

# Create a new empty shell project
WORKDIR /app

# Copy manifests
COPY Cargo.toml ./

# Copy source code
COPY src ./src
COPY static ./static

# Build for release with static linking
ENV RUSTFLAGS="-C target-feature=-crt-static"
RUN cargo build --release --bin jobweaver

# Stage 2: Create minimal runtime image
FROM alpine:3.19

# Install runtime dependencies only
RUN apk add --no-cache \
    libgcc \
    sqlite-libs \
    ca-certificates

# Create non-root user for security
RUN addgroup -g 1000 jobweaver && \
    adduser -D -u 1000 -G jobweaver jobweaver

# Set working directory
WORKDIR /app

# Copy the binary from builder
COPY --from=builder /app/target/release/jobweaver /usr/local/bin/jobweaver

# Copy static files
COPY --from=builder /app/static /app/static

# Create directory for database
RUN mkdir -p /app/data && \
    chown -R jobweaver:jobweaver /app

# Switch to non-root user
USER jobweaver

# Expose port
EXPOSE 8080

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD wget --no-verbose --tries=1 --spider http://localhost:8080/ || exit 1

# Set environment variables
ENV RUST_LOG=info
ENV DATABASE_PATH=/app/data/controlm.db

# Run the application
CMD ["jobweaver", "serve", "-d", "/app/data/controlm.db", "-p", "8080", "--host", "0.0.0.0"]
