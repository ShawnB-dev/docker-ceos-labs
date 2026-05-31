# Stage 1: Build the Rust application
FROM rust:1.77-slim-bookworm AS builder

WORKDIR /app

# Copy Cargo files first to leverage Docker layer caching
COPY Cargo.toml Cargo.lock ./

# Copy source code
COPY src ./src

# Build the application in release mode
RUN cargo build --release

# Stage 2: Create the final minimal image
FROM debian:bookworm-slim

COPY --from=builder /app/target/release/lab07-automation /usr/local/bin/lab07-automation

ENTRYPOINT ["/usr/local/bin/lab07-automation"]