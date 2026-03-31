# Build stage
FROM ubuntu:20.04 AS builder

# Prevent interactive prompts during package installation
ENV DEBIAN_FRONTEND=noninteractive

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    curl \
    build-essential \
    && rm -rf /var/lib/apt/lists/*

# Install Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

# Set working directory
WORKDIR /build/server

# Copy dependency manifests first (for layer caching)
COPY server/Cargo.toml server/Cargo.lock ./

# Create dummy source to cache dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs && cargo build --release 2>/dev/null || true
RUN rm -rf src

# Copy actual source code
COPY server/src ./src/
COPY server/migrations ./migrations/

# Build the server
RUN touch src/main.rs && cargo build --release && \
    mv target/release/gobang-server . && \
    rm -rf target/ src/

# Runtime stage
FROM ubuntu:20.04

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Copy binary from builder
COPY --from=builder /build/server/gobang-server /usr/local/bin/

# Set working directory
WORKDIR /app

# Expose default port
EXPOSE 3001

# Run the server
CMD ["gobang-server"]
