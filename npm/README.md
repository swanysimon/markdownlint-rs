# mdlint — npm package

This directory contains the npm distribution for [mdlint](https://github.com/swanysimon/mdlint),
an opinionated Markdown formatter and linter written in Rust.

The package wraps the pre-built `mdlint` binary via platform-specific optional dependencies.
No Rust toolchain is required to install or use it.

## Installation

```shell
npm install --save-dev markdownlint-rs
```

Or globally:

```shell
npm install -g markdownlint-rs
```

## Usage

```shell
# Format Markdown files
mdlint format

# Check for issues
mdlint check

# Check and auto-fix
mdlint check --fix
```

See the [full documentation](https://github.com/swanysimon/mdlint) for all options,
configuration, and CI integration examples.

## How it works

`npm install mdlint` also installs the platform-specific optional dependency that bundles the
correct pre-built `mdlint` binary for your OS and architecture. The `mdlint` command is a thin
Node.js wrapper that locates and execs that binary.

Supported platforms:

| Platform | Architecture | Optional dependency |
| --- | --- | --- |
| Linux | x64 | `markdownlint-rs-linux-x64` |
| Linux | arm64 | `markdownlint-rs-linux-arm64` |
| macOS | x64 | `markdownlint-rs-darwin-x64` |
| macOS | arm64 (Apple Silicon) | `markdownlint-rs-darwin-arm64` |
| Windows | x64 | `markdownlint-rs-win32-x64` |

## Development

### Prerequisites

- [Node.js](https://nodejs.org/) >= 14

### Validate package locally

```shell
cd npm
npm pack --dry-run
```

### Release

Releases are automated via `.github/workflows/build-npm.yml`. On a version tag push, the
workflow downloads each pre-built binary from the GitHub release, creates a platform-specific
npm package, and publishes it alongside the main package to npm.

The npm package uses trusted publishing via GitHub Actions OIDC — no token is required.
