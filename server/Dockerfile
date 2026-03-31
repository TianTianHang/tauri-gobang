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
WORKDIR /build

# Copy server source
COPY server/ ./server/

# Build the server
RUN cd server && cargo build --release

# Runtime stage
FROM ubuntu:20.04

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Copy binary from builder
COPY --from=builder /build/server/target/release/gobang-server /usr/local/bin/

# Set working directory
WORKDIR /app

# Expose default port
EXPOSE 3001

# Run the server
CMD ["gobang-server"]
