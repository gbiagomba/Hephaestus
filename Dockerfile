# Multi-stage build for a small, secure final image

# Build stage
FROM rust:1.83-slim AS builder
WORKDIR /app

# Install build dependencies
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy manifests and create dummy main for dependency caching
COPY Cargo.toml Cargo.lock ./
RUN mkdir -p src && \
    echo "fn main(){}" > src/main.rs && \
    cargo build --release || true && \
    rm -rf src

# Copy actual source code
COPY src ./src

# Build the application
RUN cargo build --release && \
    strip target/release/hephaestus

# Runtime stage
FROM debian:bookworm-slim AS runtime

# Install runtime dependencies (git is required for git operations)
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    git \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Create non-root user
RUN useradd -m -u 10001 -s /bin/bash appuser

# Set working directory
WORKDIR /home/appuser

# Copy binary from build stage
COPY --from=builder /app/target/release/hephaestus /usr/local/bin/hephaestus

# Ensure binary is executable
RUN chmod +x /usr/local/bin/hephaestus

# Switch to non-root user
USER appuser

# Set environment variables
ENV USER=appuser \
    HOME=/home/appuser

# Health check (optional)
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD hephaestus --version || exit 1

# Set entrypoint
ENTRYPOINT ["/usr/local/bin/hephaestus"]
CMD ["--help"]

# Labels for metadata
LABEL org.opencontainers.image.title="Hephaestus" \
      org.opencontainers.image.description="Secure, cross-platform git helper CLI" \
      org.opencontainers.image.version="3.1.0" \
      org.opencontainers.image.authors="gilles.infosec@gmail.com" \
      org.opencontainers.image.licenses="GPL-3.0" \
      org.opencontainers.image.source="https://github.com/yourusername/hephaestus"
