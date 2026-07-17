# Stage 1: Build Frontend (Debian-based to support pre-compiled glibc wasm-bindgen)
FROM rust:1.88-slim AS frontend-builder
WORKDIR /app

# Install compilation dependencies and wget
RUN apt-get update && apt-get install -y pkg-config libssl-dev wget ca-certificates && rm -rf /var/lib/apt/lists/*

# Add WASM target
RUN rustup target add wasm32-unknown-unknown

# Download Trunk binary dynamically based on target architecture (glibc variant)
RUN ARCH=$(uname -m) && \
    if [ "$ARCH" = "x86_64" ]; then TRUNK_ARCH="x86_64"; else TRUNK_ARCH="aarch64"; fi && \
    wget -qO- "https://github.com/trunk-rs/trunk/releases/download/v0.21.14/trunk-${TRUNK_ARCH}-unknown-linux-gnu.tar.gz" | tar -xzf- -C /usr/local/bin

COPY Cargo.toml Cargo.lock ./
COPY frontend/Cargo.toml ./frontend/
COPY server/Cargo.toml ./server/

# Cache dependencies
RUN mkdir -p frontend/src && echo "fn main() {}" > frontend/src/lib.rs && \
    mkdir -p server/src && echo "fn main() {}" > server/src/main.rs

# Build Trunk files
COPY frontend/index.html ./frontend/
COPY frontend/Trunk.toml ./frontend/
COPY frontend ./frontend
WORKDIR /app/frontend
RUN trunk build --release

# Stage 2: Build Backend Server (Alpine-based to produce musl binary)
FROM rust:1.88-alpine AS backend-builder
WORKDIR /app

RUN apk add --no-cache pkgconfig openssl-dev musl-dev gcc

COPY Cargo.toml Cargo.lock ./
COPY frontend/Cargo.toml ./frontend/
COPY server/Cargo.toml ./server/

# Dummy builds to cache dependency layers
RUN mkdir -p frontend/src && echo "fn main() {}" > frontend/src/lib.rs && \
    mkdir -p server/src && echo "fn main() {}" > server/src/main.rs && \
    cargo build --bin server --release

# Compile actual server binary
COPY server ./server
RUN touch server/src/main.rs && cargo build --bin server --release

# Stage 3: Production Runner for Backend (using Alpine)
FROM alpine:latest AS runner
WORKDIR /app

RUN apk add --no-cache ca-certificates sqlite

# Copy backend binary
COPY --from=backend-builder /app/target/release/server /app/server

# Copy frontend static files
COPY --from=frontend-builder /app/frontend/dist /app/dist

# Expose backend port
EXPOSE 3000

ENV PORT=3000
CMD ["/app/server"]
