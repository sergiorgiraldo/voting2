FROM rust:latest as builder

# Copy the rest
COPY . .

RUN cargo build --release

# Build (install) the actual binaries
RUN cargo install --path .

# Runtime image
FROM debian:bullseye-slim

# Run as "app" user
RUN useradd -ms /bin/bash app

USER app
WORKDIR /app

# Get compiled binaries from builder's cargo install directory
COPY --from=builder /target/release/rust-voting-api /app/rust-voting-api
