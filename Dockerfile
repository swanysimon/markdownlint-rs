# syntax=docker/dockerfile:1

# Build stage
FROM rust:1.91-alpine AS builder

# Docker provides these automatically based on --platform
ARG TARGETARCH

# Install build dependencies
RUN apk add --no-cache musl-dev

# Set Rust target based on architecture
RUN case "${TARGETARCH}" in \
    amd64) echo "x86_64-unknown-linux-musl" > /tmp/rust-target ;; \
    arm64) echo "aarch64-unknown-linux-musl" > /tmp/rust-target ;; \
    *) echo "Unsupported architecture: ${TARGETARCH}" && exit 1 ;; \
    esac

# Add musl target for static linking
RUN rustup target add $(cat /tmp/rust-target)

# Create a new empty shell project
WORKDIR /build

# Copy manifests
COPY Cargo.toml Cargo.lock ./

# Copy source code
COPY src ./src
COPY tests ./tests

# Build for release with static linking (using dynamic target)
RUN cargo build --release --target $(cat /tmp/rust-target)

# Runtime stage
FROM alpine:3.19

# Install ca-certificates for HTTPS support (if needed in future)
RUN apk add --no-cache ca-certificates

# Create non-root user
RUN addgroup -g 1000 markdownlint && \
    adduser -D -u 1000 -G markdownlint markdownlint

# Copy the target file from builder to determine binary path
COPY --from=builder /tmp/rust-target /tmp/rust-target

# Copy the binary from builder (path depends on architecture)
COPY --from=builder /build/target/$(cat /tmp/rust-target)/release/mdlint /usr/local/bin/mdlint

# Switch to non-root user
USER markdownlint

# Set working directory
WORKDIR /workspace

# Set entrypoint
ENTRYPOINT ["/usr/local/bin/mdlint"]

# Default to showing help
CMD ["--help"]
