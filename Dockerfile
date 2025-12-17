# ==============================================================================
# Stage 1: Frontend Builder
# ==============================================================================
FROM node:20-slim as frontend-builder
WORKDIR /frontend

# Copy frontend package files
COPY struktura/static/package.json struktura/static/package-lock.json ./

# Install dependencies
RUN npm ci

# Copy frontend source and build
COPY struktura/static ./
RUN npm run build

# ==============================================================================
# Stage 2: Rust Dependency Cacher
# ==============================================================================
FROM rust:1.90-slim-bookworm as rust-deps
WORKDIR /app

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy Cargo manifests
COPY struktura/Cargo.toml struktura/Cargo.lock ./
COPY struktura/.sqlx ./.sqlx

# Create dummy source files to cache dependencies
# (needed because Cargo.toml likely defines both bin and lib targets)
RUN mkdir -p src && \
    echo "fn main() {}" > src/main.rs && \
    echo "// dummy lib" > src/lib.rs

# Build dependencies (this layer is cached if Cargo.toml doesn't change)
RUN cargo build --release

# Remove dummy artifacts
RUN rm -f target/release/deps/struktura*

# ==============================================================================
# Stage 3: Rust Application Builder
# ==============================================================================
FROM rust-deps as rust-builder

# Copy actual source code
COPY struktura/src ./src
COPY struktura/migrations ./migrations

# Copy built frontend from stage 1 (needed for include_str! at compile time)
COPY --from=frontend-builder /frontend/dist ./static/dist

# Build the application
# Set SQLX_OFFLINE=true if using compile-time checked queries
# Remove this line if you're not using sqlx compile-time checks
ENV SQLX_OFFLINE=true
RUN cargo build --release

# ==============================================================================
# Stage 4: Runtime
# ==============================================================================
FROM debian:bookworm-slim
WORKDIR /app

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# Copy binary from builder
COPY --from=rust-builder /app/target/release/struktura ./server

# Copy static assets from frontend builder
# Runtime needs dist/ for ServeDir paths in main.rs
COPY --from=frontend-builder /frontend/dist ./static/dist

# Environment configuration
ENV PORT=8000
ENV RUST_LOG=info

# Expose port
EXPOSE 8000

# Execute
CMD ["./server"]