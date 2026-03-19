# ==============================================================================
# Stage 1: Builder
# ==============================================================================
FROM rust:1.77-slim AS builder

WORKDIR /app

# Install system dependencies required for compilation
RUN apt-get update && apt-get install -y \
  pkg-config \
  libssl-dev \
  && rm -rf /var/lib/apt/lists/*

ENV SQLX_OFFLINE=true

RUN sqlx prepare --workspace --  --all-targets

COPY . .

# Build the real application
RUN cargo build --release

# ==============================================================================
# Stage 2: Runtime (minimal image)
# ==============================================================================
FROM debian:bookworm-slim AS runtime

WORKDIR /app

# Install only runtime dependencies
RUN apt-get update && apt-get install -y \
  ca-certificates \
  libssl3 \
  && rm -rf /var/lib/apt/lists/*

# Create a non-root user for security
RUN useradd -ms /bin/bash appuser

# Copy the compiled binary from the builder stage
# Replace "my_app" with your actual binary name (same as [package] name in Cargo.toml)
COPY --from=builder /app/target/release/milions-sys /usr/local/bin/milions-sys

# Set ownership and permissions
RUN chown appuser:appuser /usr/local/bin/milions-sys && chmod +x /usr/local/bin/milions-sys

USER appuser

EXPOSE 8080

CMD ["milions-sys"]
