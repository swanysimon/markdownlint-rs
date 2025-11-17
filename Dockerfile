# syntax=docker/dockerfile:1

# Build stage
FROM rust:1.91-alpine AS builder

# Install build dependencies
RUN apk add --no-cache musl-dev

# Add musl target for static linking
RUN rustup target add x86_64-unknown-linux-musl

# Create a new empty shell project
WORKDIR /build

# Copy manifests
COPY Cargo.toml Cargo.lock ./

# Copy source code
COPY src ./src
COPY tests ./tests

# Build for release with static linking
RUN cargo build --release --target x86_64-unknown-linux-musl

# Runtime stage
FROM alpine:3.19

# Install ca-certificates for HTTPS support (if needed in future)
RUN apk add --no-cache ca-certificates

# Create non-root user
RUN addgroup -g 1000 markdownlint && \
    adduser -D -u 1000 -G markdownlint markdownlint

# Copy the binary from builder
COPY --from=builder /build/target/x86_64-unknown-linux-musl/release/mdlint /usr/local/bin/mdlint

# Switch to non-root user
USER markdownlint

# Set working directory
WORKDIR /workspace

# Set entrypoint
ENTRYPOINT ["/usr/local/bin/mdlint"]

# Default to showing help
CMD ["--help"]
