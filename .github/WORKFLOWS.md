# GitHub Actions Workflows

This document describes the GitHub Actions workflows used in this project.

## Overview

We use GitHub Actions for continuous integration, deployment, and automation. All workflows are defined in `.github/workflows/`.

## CI Strategy

Our CI pipeline is optimized for fast feedback:

1. **Fast checks first** (test, clippy, fmt) - Run in parallel on Linux
2. **Slow checks second** (build, compatibility) - Only run if fast checks pass
3. **Platform testing strategy**:
   - Unit tests run only on Linux (tests should pass regardless of platform)
   - Cross-compilation verified on all target platforms (Linux x86/ARM, macOS x86/ARM, Windows x86)
   - This provides fast feedback while ensuring platform compatibility

This approach gives developers quick feedback on the most common issues while still validating that code compiles and works across all supported platforms.

## Workflows

### 1. CI (`ci.yml`)

**Triggers**: Push to `main`, Pull Requests to `main`, Called by other workflows

Comprehensive continuous integration pipeline that ensures code quality. This workflow is also reusable and can be called by other workflows (like `tag.yml`):

#### Jobs:

**Test Suite** (`test`)
- Runs on: Ubuntu with Rust stable
- Executes: Unit tests, integration tests, doc tests
- Uses caching for faster builds
- Note: Tests run only on Linux for speed; cross-platform compilation verified in build job

**Clippy** (`clippy`)
- Runs: `cargo clippy --all-targets --all-features -- -D warnings`
- Enforces Rust best practices
- Treats all warnings as errors

**Rustfmt** (`fmt`)
- Runs: `cargo fmt --all -- --check`
- Ensures consistent code formatting
- Fails if code is not formatted

**Build** (`build`)
- Cross-platform compilation verification for:
  - Linux x86_64 (Intel/AMD)
  - Linux aarch64 (ARM64)
  - macOS x86_64 (Intel)
  - macOS aarch64 (Apple Silicon ARM64)
  - Windows x86_64 (Intel/AMD)
- Uses `cross` for ARM Linux cross-compilation
- Verifies binary execution on native platforms
- Runs only after fast checks (test, clippy, fmt) pass

**Compatibility Tests** (`compatibility`)
- Compares output with markdownlint-cli2
- Uses Docker for reference implementation
- Ensures behavioral compatibility
- Runs only after fast checks (test, clippy, fmt) pass

**Security Audit** (`security`)
- Checks for known vulnerabilities
- Uses `cargo-audit` via rustsec

### 2. Tag and Release (`tag.yml`)

**Triggers**: Tags matching `v*.*.*` (e.g., `v0.1.0`)

Automated release process that ensures all CI checks pass before building and publishing binaries:

#### Release Process Flow:

1. **Run CI Checks** - Calls the `ci.yml` workflow to run all quality checks
2. **Create Release** - Only proceeds if CI passes
3. **Build Binaries** - Build for all platforms in parallel
4. **Publish to crates.io** - Optional final step

#### Safety Features:

**CI Enforcement**
- All tests, clippy, and formatting checks must pass before release
- Ensures only quality code gets released

**Concurrency Control**
- Uses commit SHA for concurrency group
- Prevents multiple tags on same commit from creating duplicate releases
- Second tag waits for first to complete

**Idempotent Release Creation**
- Checks if release already exists before creating
- If release exists, reuses it (useful for multiple tags)
- Prevents duplicate release errors

#### Jobs:

**Run CI Checks** (`ci`)
- Calls `ci.yml` workflow
- All CI jobs must pass before proceeding
- Blocks release if any quality checks fail

**Create Release** (`create-release`)
- Checks if release already exists (idempotent)
- Creates GitHub release with checksums instructions
- Provides upload URL for artifacts
- Skips if release already exists for this commit

**Build Release Binaries** (`build-release`)
- Builds optimized binaries for:
  - **Linux**: x86_64 (glibc), x86_64-musl (static), aarch64 (ARM64)
  - **macOS**: x86_64 (Intel), aarch64 (Apple Silicon ARM64)
  - **Windows**: x86_64 (Intel/AMD)
- Strips debug symbols for smaller binaries
- Generates SHA256 checksums for each binary
- Creates tarballs (Linux/macOS) and zips (Windows)
- Uploads all artifacts to GitHub release

**Publish to crates.io** (`publish-crates-io`)
- Only runs if this is a new release (not a duplicate tag)
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

#### Creating a Release

To create a new release:

```bash
# Create and push a version tag
git tag v0.1.0
git push origin v0.1.0
```

The workflow will automatically:
1. Run all CI checks
2. Create a GitHub release (if CI passes)
3. Build binaries for all platforms
4. Upload binaries with checksums
5. Publish to crates.io

#### Multiple Tags on Same Commit

If you push multiple tags to the same commit (e.g., `v1.0.0` and `v1.0.0-beta`):
- First tag triggers a full release process
- Second tag waits for first to complete (concurrency control)
- Second tag reuses the existing release, only uploads additional artifacts
- crates.io publish only happens once

### 3. Manual Release Testing (`release.yml`)

**Triggers**: Manual dispatch only

Test release workflow for debugging without creating a real release:

#### Purpose

Use this workflow ONLY for:
- Testing the release build process
- Debugging release issues
- Verifying cross-compilation works

**DO NOT** use this for actual releases - use version tags instead.

#### Jobs

Same as `tag.yml` but:
- Creates draft/prerelease (not a real release)
- Uses `-test` suffix on tags
- Requires manual version input
- Marked clearly as test releases

### 4. Dependency Updates (`dependencies.yml`)

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

### 5. Benchmarks (`benchmarks.yml`)

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

## Badges

Add these badges to your README:

```markdown
[![CI](https://github.com/YOUR_USERNAME/markdownlint-rs/workflows/CI/badge.svg)](https://github.com/YOUR_USERNAME/markdownlint-rs/actions/workflows/ci.yml)
[![Release](https://github.com/YOUR_USERNAME/markdownlint-rs/workflows/Tag%20and%20Release/badge.svg)](https://github.com/YOUR_USERNAME/markdownlint-rs/actions/workflows/tag.yml)
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
