# markdownlint-rs

[![CI](https://github.com/swanysimon/markdownlint-rs/workflows/CI/badge.svg)](https://github.com/swanysimon/markdownlint-rs/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/markdownlint-rs.svg)](https://crates.io/crates/markdownlint-rs)

A fast, flexible, configuration-based command-line interface for linting Markdown files, written in Rust.

**Project Status**: Active development. Compatible with [markdownlint-cli2](https://github.com/DavidAnson/markdownlint-cli2) configuration and rule behavior.

## Features

- ✨ **Fast**: Written in Rust for performance
- 🔧 **Flexible**: Supports multiple configuration formats (JSONC, YAML, package.json)
- 📏 **54 Built-in Rules**: Comprehensive Markdown linting with [markdownlint](https://github.com/DavidAnson/markdownlint) compatibility
- 🔨 **Auto-fix**: Automatically fix many common issues with `--fix`
- 🌳 **Gitignore Support**: Respects `.gitignore` files by default
- 📦 **Cross-platform**: Linux (x86_64, ARM64), macOS (Intel, Apple Silicon), Windows

## Compatibility

markdownlint-rs aims for full compatibility with [markdownlint-cli2](https://github.com/DavidAnson/markdownlint-cli2):

- ✅ Same configuration file formats and locations
- ✅ Same rule behavior and naming (MD001-MD059)
- ✅ Same configuration options for rules
- ✅ Compatible exit codes

## Installation

### From GitHub Releases (Recommended)

Download the latest release for your platform from the [releases page](https://github.com/swanysimon/markdownlint-rs/releases):

**Linux (x86_64)**:
```bash
curl -LO https://github.com/swanysimon/markdownlint-rs/releases/latest/download/markdownlint-rs-linux-x86_64.tar.gz
tar xzf markdownlint-rs-linux-x86_64.tar.gz
sudo mv markdownlint-rs /usr/local/bin/
```

**Linux (ARM64)**:
```bash
curl -LO https://github.com/swanysimon/markdownlint-rs/releases/latest/download/markdownlint-rs-linux-aarch64.tar.gz
tar xzf markdownlint-rs-linux-aarch64.tar.gz
sudo mv markdownlint-rs /usr/local/bin/
```

**macOS (Intel)**:
```bash
curl -LO https://github.com/swanysimon/markdownlint-rs/releases/latest/download/markdownlint-rs-macos-x86_64.tar.gz
tar xzf markdownlint-rs-macos-x86_64.tar.gz
sudo mv markdownlint-rs /usr/local/bin/
```

**macOS (Apple Silicon)**:
```bash
curl -LO https://github.com/swanysimon/markdownlint-rs/releases/latest/download/markdownlint-rs-macos-aarch64.tar.gz
tar xzf markdownlint-rs-macos-aarch64.tar.gz
sudo mv markdownlint-rs /usr/local/bin/
```

**Windows**:
Download `markdownlint-rs-windows-x86_64.exe.zip` from the releases page and extract it to a directory in your PATH.

**Verify checksum** (optional but recommended):
```bash
# Linux/macOS
sha256sum -c markdownlint-rs-*.sha256

# Windows (PowerShell)
$expected = (Get-Content markdownlint-rs-*.sha256).Split()[0]
$actual = (Get-FileHash markdownlint-rs.exe).Hash.ToLower()
if ($expected -eq $actual) { "OK" } else { "FAILED" }
```

### From crates.io

```bash
cargo install markdownlint-rs
```

### From Source

```bash
git clone https://github.com/swanysimon/markdownlint-rs.git
cd markdownlint-rs
cargo build --release
sudo cp target/release/markdownlint-rs /usr/local/bin/
```

### Docker

Pull from GitHub Container Registry:

```bash
docker pull ghcr.io/swanysimon/markdownlint-rs:latest
```

Run on files in the current directory:

```bash
docker run --rm -v "$PWD:/workspace" ghcr.io/swanysimon/markdownlint-rs:latest
```

Run with auto-fix:

```bash
docker run --rm -v "$PWD:/workspace" ghcr.io/swanysimon/markdownlint-rs:latest --fix
```

Run with custom config:

```bash
docker run --rm -v "$PWD:/workspace" ghcr.io/swanysimon/markdownlint-rs:latest --config .markdownlint.json
```

**Available tags:**
- `latest` - Latest stable release
- `1.x.x` - Specific version (e.g., `1.0.0`)
- `1.x` - Latest patch version in minor release (e.g., `1.0`)
- `1` - Latest minor version in major release

The Docker image supports both `linux/amd64` and `linux/arm64` platforms.

## Usage

### Basic Usage

Lint all Markdown files in the current directory:
```bash
markdownlint-rs
```

Lint specific files or directories:
```bash
markdownlint-rs README.md docs/
```

Lint with auto-fix:
```bash
markdownlint-rs --fix
```

### Command-Line Options

```
markdownlint-rs [OPTIONS] [PATTERNS]...

Arguments:
  [PATTERNS]...  Glob patterns for files to lint (defaults to current directory)

Options:
      --config <PATH>     Path to configuration file
      --fix               Apply fixes to files
      --no-globs          Ignore globs from configuration
      --format <FORMAT>   Output format: default or json [default: default]
      --no-color          Disable color output
  -h, --help              Print help
  -V, --version           Print version
```

### Examples

**Lint with custom config file:**
```bash
markdownlint-rs --config .markdownlint.json
```

**Output as JSON:**
```bash
markdownlint-rs --format json
```

**Lint specific glob patterns:**
```bash
markdownlint-rs "**/*.md" "!node_modules/**"
```

**Fix issues automatically:**
```bash
markdownlint-rs --fix docs/
```

**Disable color output (for CI):**
```bash
markdownlint-rs --no-color
```

## Configuration

markdownlint-rs discovers configuration files automatically by searching up from the current directory:

### Configuration File Locations

The tool searches for these files in order (first found wins per directory level):
1. `.markdownlint-cli2.jsonc`
2. `.markdownlint-cli2.yaml`
3. `.markdownlint-cli2.json`
4. `.markdownlint.jsonc`
5. `.markdownlint.json`
6. `.markdownlint.yaml`
7. `package.json` (in `markdownlint-cli2` key)

### Configuration File Format

**JSONC/JSON** (`.markdownlint-cli2.jsonc`):
```jsonc
{
  // Rule configuration
  "config": {
    "default": true,              // Enable all rules by default
    "MD013": false,               // Disable line length rule
    "MD003": { "style": "atx" }   // Configure heading style
  },

  // File selection
  "globs": ["**/*.md"],
  "ignores": ["node_modules/**", "dist/**"],

  // Options
  "fix": false,
  "gitignore": true,
  "noInlineConfig": false
}
```

**YAML** (`.markdownlint-cli2.yaml`):
```yaml
config:
  default: true
  MD013: false
  MD003:
    style: atx

globs:
  - "**/*.md"
ignores:
  - "node_modules/**"
  - "dist/**"

fix: false
gitignore: true
```

### Configuration Hierarchies

Configurations are discovered by walking up the directory tree. When multiple configs are found, they are merged with the following precedence (highest to lowest):

1. Command-line options (`--config`, `--fix`, etc.)
2. Local directory config (`.markdownlint-cli2.jsonc` in current dir)
3. Parent directory configs (walking up to root)
4. Default configuration

Arrays (like `globs` and `ignores`) are **extended**, not replaced.

### Rule Configuration

Each rule can be configured in multiple ways:

```jsonc
{
  "config": {
    "MD001": true,                    // Enable rule
    "MD002": false,                   // Disable rule
    "MD003": { "style": "atx" },      // Configure with options
    "MD007": { "indent": 4 },         // Set specific parameters
    "default": true                   // Enable all rules by default
  }
}
```

See the [markdownlint rules documentation](https://github.com/DavidAnson/markdownlint/blob/main/doc/Rules.md) for details on each rule and its configuration options.

## Exit Codes

- **0**: Success - no linting errors found
- **1**: Linting errors found
- **2**: Runtime error (invalid config, file not found, etc.)

Use exit codes in CI/CD pipelines:
```bash
markdownlint-rs || exit 1  # Fail build on linting errors
```

## Supported Rules

markdownlint-rs implements 54 rules compatible with markdownlint:

| Rule | Description | Fixable |
|------|-------------|---------|
| MD001 | Heading levels should only increment by one level at a time | ❌ |
| MD003 | Heading style | ❌ |
| MD004 | Unordered list style | ❌ |
| MD005 | Inconsistent indentation for list items at the same level | ❌ |
| MD007 | Unordered list indentation | ❌ |
| MD009 | Trailing spaces | ✅ |
| MD010 | Hard tabs | ✅ |
| MD011 | Reversed link syntax | ❌ |
| MD012 | Multiple consecutive blank lines | ✅ |
| MD013 | Line length | ❌ |
| MD018 | No space after hash on atx style heading | ✅ |
| MD019 | Multiple spaces after hash on atx style heading | ✅ |
| MD022 | Headings should be surrounded by blank lines | ✅ |
| MD023 | Headings must start at the beginning of the line | ✅ |
| MD025 | Multiple top-level headings in the same document | ❌ |
| ... | See [markdownlint rules](https://github.com/DavidAnson/markdownlint/blob/main/doc/Rules.md) for full list | ... |

## Differences from markdownlint-cli2

While markdownlint-rs aims for compatibility, there are some intentional differences:

- **No JavaScript custom rules**: Use Rust API instead (future feature)
- **No markdown-it plugins**: Uses CommonMark-compliant parser with standard extensions
- **Faster execution**: Compiled binary vs Node.js runtime
- **Single binary**: No npm/node dependencies required

## Contributing

Contributions are welcome! See [CONTRIBUTING.md](CONTRIBUTING.md) for:
- Development setup
- Code quality standards
- How to add new rules
- Release process

## License

MIT License - see [LICENSE](LICENSE) for details.

## Acknowledgments

- [markdownlint](https://github.com/DavidAnson/markdownlint) by David Anson - Original rule definitions
- [markdownlint-cli2](https://github.com/DavidAnson/markdownlint-cli2) - Configuration format and behavior
- [pulldown-cmark](https://github.com/raphlinus/pulldown-cmark) - Markdown parsing

## Resources

- [Documentation](https://github.com/swanysimon/markdownlint-rs/tree/main/.github)
- [Issue Tracker](https://github.com/swanysimon/markdownlint-rs/issues)
- [Changelog](https://github.com/swanysimon/markdownlint-rs/releases)
- [markdownlint Rules Reference](https://github.com/DavidAnson/markdownlint/blob/main/doc/Rules.md)
