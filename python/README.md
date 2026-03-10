# mdlint — Python package

This directory contains the Python distribution for [mdlint](https://github.com/swanysimon/mdlint),
an opinionated Markdown formatter and linter written in Rust.

The package wraps the pre-built `mdlint` binary. No Rust toolchain is required to install or use it.

## Installation

```shell
pip install mdlint
```

Or with uv:

```shell
uv tool install mdlint
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

`pip install mdlint` downloads a platform-specific wheel that bundles the correct pre-built
`mdlint` binary for your OS and architecture. The `mdlint` command is a thin Python wrapper
that locates and execs that binary.

Supported platforms:

| Platform | Architecture |
| --- | --- |
| Linux | x86_64 (glibc) |
| Linux | aarch64 (glibc) |
| macOS | x86_64 |
| macOS | arm64 (Apple Silicon) |
| Windows | x86_64 |

## Development

### Prerequisites

- [uv](https://docs.astral.sh/uv/)

### Build a wheel locally

```shell
cd python

# Pure-Python wheel (no binary bundled — for metadata validation only)
uv build --wheel

# Platform-specific wheel with a binary
cp /path/to/mdlint-binary mdlint/mdlint
MDLINT_PLATFORM_TAG=macosx_11_0_arm64 uv build --wheel
```

`MDLINT_PLATFORM_TAG` is read by `hatch_build.py` to stamp the correct platform tag onto the
wheel. Without it, the wheel is tagged `py3-none-any` and contains no binary — useful for
metadata validation in CI but not for distribution.

### Validate package metadata

```shell
cd python
uv build --wheel
uvx twine check dist/*.whl
```

### Platform tags

| Asset | `MDLINT_PLATFORM_TAG` |
| --- | --- |
| `mdlint-linux-x86_64` | `manylinux_2_17_x86_64.manylinux2014_x86_64` |
| `mdlint-linux-aarch64` | `manylinux_2_17_aarch64.manylinux2014_aarch64` |
| `mdlint-macos-x86_64` | `macosx_10_12_x86_64` |
| `mdlint-macos-aarch64` | `macosx_11_0_arm64` |
| `mdlint-windows-x86_64.exe` | `win_amd64` |

### Release

Releases are automated via `.github/workflows/build-python.yml`. On a version tag push, the
workflow downloads each pre-built binary from the GitHub release, builds a platform-specific
wheel, uploads it to the GitHub release, and publishes it to PyPI.

A `PYPI_TOKEN` secret must be configured in the repository settings.
