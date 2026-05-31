# Stage 1: Build the Rust application
FROM rust:1.77-slim-bookworm AS builder

WORKDIR /app

# Copy Cargo.toml and Cargo.lock first to leverage Docker cache
COPY Cargo.toml .
COPY Cargo.lock .

# Copy source code
COPY src ./src

# Build the application in release mode
RUN cargo build --release

# Stage 2: Create the final minimal image
FROM debian:bookworm-slim

COPY --from=builder /app/target/release/lab08-telemetry /usr/local/bin/lab08-telemetry

ENTRYPOINT ["/usr/local/bin/lab08-telemetry"]