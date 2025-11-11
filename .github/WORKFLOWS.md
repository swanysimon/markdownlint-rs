# GitHub Actions Workflows

This document describes the GitHub Actions workflows used in this project.

## Overview

We use GitHub Actions for continuous integration, deployment, and automation. All workflows are defined in `.github/workflows/`.

## Workflows

### 1. CI (`ci.yml`)

**Triggers**: Push to `main`, Pull Requests to `main`

Comprehensive continuous integration pipeline that ensures code quality:

#### Jobs:

**Test Suite** (`test`)
- Runs on: Ubuntu, macOS, Windows
- Rust versions: Stable, Beta
- Executes: Unit tests, integration tests, doc tests
- Uses caching for faster builds

**Clippy** (`clippy`)
- Runs: `cargo clippy --all-targets --all-features -- -D warnings`
- Enforces Rust best practices
- Treats all warnings as errors

**Rustfmt** (`fmt`)
- Runs: `cargo fmt --all -- --check`
- Ensures consistent code formatting
- Fails if code is not formatted

**Build** (`build`)
- Cross-platform builds for:
  - Linux x86_64
  - macOS x86_64
  - macOS aarch64 (Apple Silicon)
  - Windows x86_64
- Verifies binary execution

**Compatibility Tests** (`compatibility`)
- Compares output with markdownlint-cli2
- Uses Docker for reference implementation
- Ensures behavioral compatibility

**Code Coverage** (`coverage`)
- Uses `cargo-tarpaulin`
- Uploads to Codecov
- Tracks test coverage over time

**Security Audit** (`security`)
- Checks for known vulnerabilities
- Uses `cargo-audit` via rustsec

### 2. Release (`release.yml`)

**Triggers**: Tags matching `v*.*.*` (e.g., `v0.1.0`), Manual dispatch

Automated release process for building and publishing binaries:

#### Jobs:

**Create Release** (`create-release`)
- Creates GitHub release
- Extracts version from tag
- Provides upload URL for artifacts

**Build Release Binaries** (`build-release`)
- Builds optimized binaries for:
  - **Linux**: x86_64 (glibc), x86_64 (musl), aarch64
  - **macOS**: x86_64, aarch64 (Apple Silicon)
  - **Windows**: x86_64
- Strips debug symbols for smaller binaries
- Generates SHA256 checksums for each binary
- Creates tarballs (Linux/macOS) and zips (Windows)
- Uploads all artifacts to GitHub release

**Publish to crates.io** (`publish-crates-io`)
- Publishes to crates.io after successful builds
- Requires `CARGO_REGISTRY_TOKEN` secret

#### Release Artifacts

Each release includes:
- Binary archives (`.tar.gz` for Unix, `.zip` for Windows)
- SHA256 checksums (`.sha256` files)
- Multiple platform variants

**Example artifacts**:
```
markdownlint-rs-linux-x86_64.tar.gz
markdownlint-rs-linux-x86_64.tar.gz.sha256
markdownlint-rs-linux-x86_64-musl.tar.gz
markdownlint-rs-linux-x86_64-musl.tar.gz.sha256
markdownlint-rs-linux-aarch64.tar.gz
markdownlint-rs-linux-aarch64.tar.gz.sha256
markdownlint-rs-macos-x86_64.tar.gz
markdownlint-rs-macos-x86_64.tar.gz.sha256
markdownlint-rs-macos-aarch64.tar.gz
markdownlint-rs-macos-aarch64.tar.gz.sha256
markdownlint-rs-windows-x86_64.exe.zip
markdownlint-rs-windows-x86_64.exe.zip.sha256
```

### 3. Dependency Updates (`dependencies.yml`)

**Triggers**: Weekly (Monday 9am UTC), Manual dispatch

Automated dependency maintenance:

#### Jobs:

**Update Dependencies** (`update-dependencies`)
- Runs `cargo update`
- Executes test suite
- Creates PR with updated `Cargo.lock`
- Automated but requires manual review

**Security Audit** (`security-audit`)
- Weekly security vulnerability check
- Reports known issues in dependencies

### 4. Benchmarks (`benchmarks.yml`)

**Triggers**: Push to `main`, Pull Requests

Performance tracking:

#### Jobs:

**Performance Benchmarks** (`benchmark`)
- Tests with different file sizes (1KB, 100KB, 1MB)
- Reports execution times
- Adds results to GitHub Actions summary
- Optional comparison with markdownlint-cli2

## Required Secrets

Configure these secrets in repository settings:

- `GITHUB_TOKEN` - Automatically provided by GitHub
- `CARGO_REGISTRY_TOKEN` - For publishing to crates.io (optional)
- `CODECOV_TOKEN` - For code coverage reporting (optional)

## Badges

Add these badges to your README:

```markdown
[![CI](https://github.com/YOUR_USERNAME/markdownlint-rs/workflows/CI/badge.svg)](https://github.com/YOUR_USERNAME/markdownlint-rs/actions/workflows/ci.yml)
[![Release](https://github.com/YOUR_USERNAME/markdownlint-rs/workflows/Release/badge.svg)](https://github.com/YOUR_USERNAME/markdownlint-rs/actions/workflows/release.yml)
[![codecov](https://codecov.io/gh/YOUR_USERNAME/markdownlint-rs/branch/main/graph/badge.svg)](https://codecov.io/gh/YOUR_USERNAME/markdownlint-rs)
[![Crates.io](https://img.shields.io/crates/v/markdownlint-rs.svg)](https://crates.io/crates/markdownlint-rs)
```

## Triggering Workflows Manually

Some workflows support manual triggering via `workflow_dispatch`:

1. Go to **Actions** tab in GitHub
2. Select the workflow
3. Click **Run workflow**
4. Choose branch and parameters
5. Click **Run workflow**

## Cache Strategy

We use GitHub Actions cache for:
- Cargo registry (`~/.cargo/registry`)
- Cargo git index (`~/.cargo/git`)
- Build artifacts (`target/`)

Cache keys include:
- OS
- Rust version
- `Cargo.lock` hash

This significantly speeds up CI runs.

## Matrix Strategy

Jobs use matrix strategies for testing multiple configurations:

**Test Matrix**:
- OS: Ubuntu, macOS, Windows
- Rust: Stable, Beta

**Build Matrix**:
- Multiple targets and architectures
- Ensures cross-platform compatibility

## Troubleshooting

### CI Failure: "Clippy warnings"
Run locally: `cargo clippy --all-targets --all-features -- -D warnings`

### CI Failure: "Format check"
Run locally: `cargo fmt`

### CI Failure: "Tests failed"
Run locally: `cargo test --all-features`

### Release Failure: "Upload failed"
- Check GitHub token permissions
- Verify release was created
- Check artifact paths

### Crates.io publish failure
- Verify `CARGO_REGISTRY_TOKEN` secret
- Check version number in `Cargo.toml`
- Ensure version not already published

## Local Testing

Before pushing, run the full CI suite locally:

```bash
# Format check
cargo fmt --check

# Linting
cargo clippy --all-targets --all-features -- -D warnings

# Tests
cargo test --all-features

# Build for multiple targets (requires target installed)
cargo build --release --target x86_64-unknown-linux-gnu
```

## Future Improvements

Potential workflow enhancements:
- Nightly Rust testing
- Performance regression tracking
- Automated changelog generation
- Docker image publishing
- Homebrew formula updates
- npm wrapper package
