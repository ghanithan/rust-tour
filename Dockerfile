# Multi-stage Dockerfile for Rust Tour
FROM node:18-alpine AS web-builder

# Install build dependencies for native modules like node-pty
RUN apk add --no-cache \
    python3 \
    make \
    g++ \
    musl-dev

# Set working directory for web build
WORKDIR /app/web

# Copy web package files
COPY web/package*.json ./
RUN npm ci --only=production

# Copy web source and build
COPY web/ ./
RUN npm run build

# Rust build stage
FROM rust:1.75-alpine AS rust-builder

# Install system dependencies for building
RUN apk add --no-cache \
    musl-dev \
    pkgconfig \
    openssl-dev \
    libc6-compat

# Set working directory
WORKDIR /app

# Copy Cargo files first for better caching
COPY Cargo.toml Cargo.lock ./
COPY exercise-framework/Cargo.toml ./exercise-framework/
COPY web-server/Cargo.toml ./web-server/

# Create temporary workspace config without exercises for dependency caching
RUN cp Cargo.toml Cargo.toml.original && \
    sed 's/"exercises\/ch\*\/ex\[0-9\]\[0-9\]\*",//g' Cargo.toml > Cargo.toml.temp && \
    mv Cargo.toml.temp Cargo.toml

# Create dummy source files for dependency caching
RUN mkdir -p exercise-framework/src web-server/src && \
    echo "fn main() {}" > exercise-framework/src/lib.rs && \
    echo "fn main() {}" > web-server/src/main.rs

# Build dependencies (without exercises)
RUN cargo build --release --package rust-tour

# Restore original workspace config
RUN mv Cargo.toml.original Cargo.toml

# Copy actual source code
COPY exercise-framework/ ./exercise-framework/
COPY web-server/ ./web-server/

# Copy exercises for final build
COPY exercises/ ./exercises/

# Copy built web assets
COPY --from=web-builder /app/web/dist ./web/dist

# Copy scripts
COPY scripts/ ./scripts/

# Build the actual application
RUN touch exercise-framework/src/lib.rs web-server/src/main.rs && \
    cargo build --release --package rust-tour

# Copy solutions if they exist (optional)
RUN mkdir -p ./solutions/

# Final runtime stage
FROM alpine:3.18

# Install runtime dependencies
RUN apk add --no-cache \
    ca-certificates \
    git \
    bash \
    wget

# Create non-root user
RUN addgroup -g 1001 -S rustuser && \
    adduser -S -D -H -u 1001 -h /app -s /sbin/nologin -G rustuser rustuser

# Set working directory
WORKDIR /app

# Copy the built binary
COPY --from=rust-builder /app/target/release/rust-tour /usr/local/bin/rust-tour

# Copy exercise content and scripts
COPY --from=rust-builder --chown=rustuser:rustuser /app/exercises ./exercises
COPY --from=rust-builder --chown=rustuser:rustuser /app/scripts ./scripts

# Create progress directory
RUN mkdir -p /app/progress && chown rustuser:rustuser /app/progress

# Switch to non-root user
USER rustuser

# Expose the port
EXPOSE 3000

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD wget --no-verbose --tries=1 --spider http://localhost:3000/health || exit 1

# Set environment variables
ENV RUST_LOG=info
ENV RUST_TOUR_HOST=0.0.0.0
ENV RUST_TOUR_PORT=3000

# Start command
CMD ["rust-tour"]