# ==============================================================================
# Stage 1: Builder
# ==============================================================================
FROM rust:1.82-slim AS builder

WORKDIR /app

# Install system dependencies required for compilation
RUN apt-get update && apt-get install -y \
  pkg-config \
  libssl-dev \
  && rm -rf /var/lib/apt/lists/*

# Copy dependency manifests first for layer caching
COPY Cargo.toml Cargo.lock ./
COPY settings/Cargo.toml settings/
COPY types/Cargo.toml types/
COPY viacep/Cargo.toml viacep/
COPY client/Cargo.toml client/
COPY collaborator/Cargo.toml collaborator/
COPY contact/Cargo.toml contact/
COPY location/Cargo.toml location/
COPY project/Cargo.toml project/

# Copy source code and sqlx offline cache
COPY . .

# Build with offline sqlx (uses pre-generated .sqlx/ cache)
ENV SQLX_OFFLINE=true
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
COPY --from=builder /app/target/release/milions-sys /usr/local/bin/milions-sys

# Copy migrations for automatic startup migration
COPY --from=builder /app/migrations /app/migrations

# Copy config files
COPY --from=builder /app/files /app/files

# Set ownership and permissions
RUN chown appuser:appuser /usr/local/bin/milions-sys && chmod +x /usr/local/bin/milions-sys

USER appuser

EXPOSE 8000

CMD ["milions-sys"]
