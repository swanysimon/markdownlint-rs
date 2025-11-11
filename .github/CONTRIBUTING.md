# Contributing to markdownlint-rs

Thank you for your interest in contributing to markdownlint-rs! This document provides guidelines and instructions for contributors.

## Development Setup

1. **Install Rust**: Install Rust via [rustup](https://rustup.rs/)
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **Clone the repository**:
   ```bash
   git clone https://github.com/YOUR_USERNAME/markdownlint-rs.git
   cd markdownlint-rs
   ```

3. **Build the project**:
   ```bash
   cargo build
   ```

4. **Run tests**:
   ```bash
   cargo test
   ```

## Code Quality Standards

All contributions must meet the following quality standards:

### Testing
- **Unit tests**: All new functionality must include unit tests
- **Integration tests**: Complex features should include integration tests
- **Run tests**: `cargo test --all-features`
- **Test coverage**: Aim for high test coverage on new code

### Code Style
- **Formatting**: Code must be formatted with `rustfmt`
  ```bash
  cargo fmt
  ```
- **Linting**: Code must pass `clippy` checks
  ```bash
  cargo clippy --all-targets --all-features
  ```

### Documentation
- Public APIs must have doc comments
- Complex logic should include inline comments
- Update relevant documentation files

## Continuous Integration

Our CI pipeline runs automatically on all pull requests and includes:

### 1. **Test Suite** (`ci.yml`)
- Runs on Linux, macOS, and Windows
- Tests with stable and beta Rust
- Includes doc tests

### 2. **Clippy Linting** (`ci.yml`)
- Enforces Rust best practices
- All warnings treated as errors in CI

### 3. **Code Formatting** (`ci.yml`)
- Ensures consistent code style
- Must pass `cargo fmt --check`

### 4. **Build Verification** (`ci.yml`)
- Builds for multiple targets
- Tests binary execution

### 5. **Compatibility Tests** (`ci.yml`)
- Verifies compatibility with markdownlint-cli2
- Uses Docker for reference implementation

### 6. **Security Audit** (`ci.yml`, `dependencies.yml`)
- Checks for known security vulnerabilities
- Runs on PRs and weekly schedule

## Pull Request Process

1. **Fork the repository** and create a feature branch
   ```bash
   git checkout -b feature/my-new-feature
   ```

2. **Make your changes** following the code quality standards

3. **Run the full test suite**:
   ```bash
   cargo test --all-features
   cargo clippy --all-targets --all-features
   cargo fmt --check
   ```

4. **Commit your changes** with clear, descriptive messages:
   ```bash
   git commit -m "feat: add new rule MD999"
   ```

5. **Push to your fork**:
   ```bash
   git push origin feature/my-new-feature
   ```

6. **Open a Pull Request** with:
   - Clear title describing the change
   - Description of what changed and why
   - Link to any related issues
   - Screenshots/examples if applicable

7. **Wait for CI checks** to pass (all green ✓)

8. **Address review feedback** if requested

## Commit Message Convention

We follow conventional commits:

- `feat:` New features
- `fix:` Bug fixes
- `docs:` Documentation changes
- `style:` Code style changes (formatting, etc.)
- `refactor:` Code refactoring
- `perf:` Performance improvements
- `test:` Adding or updating tests
- `chore:` Maintenance tasks

Examples:
```
feat: implement MD999 rule for checking markdown syntax
fix: correct line number reporting in MD001
docs: update README with installation instructions
test: add unit tests for MD042 rule
```

## Adding New Rules

To add a new markdownlint rule:

1. Create a new file in `src/lint/rules/` (e.g., `md999.rs`)
2. Implement the `Rule` trait
3. Add comprehensive unit tests
4. Register the rule in `src/lint/rules/mod.rs`
5. Add documentation for the rule
6. If possible, implement auto-fix functionality

See existing rules for examples.

## Release Process

Releases are automated via GitHub Actions:

1. **Create a version tag**:
   ```bash
   git tag v0.1.0
   git push origin v0.1.0
   ```

2. **GitHub Actions will**:
   - Build binaries for all supported platforms
   - Generate SHA256 checksums
   - Create a GitHub release
   - Publish to crates.io (if configured)

## Getting Help

- **Issues**: Check existing issues or create a new one
- **Discussions**: Use GitHub Discussions for questions
- **Documentation**: See `CLAUDE.md` for architecture details

## Code of Conduct

- Be respectful and inclusive
- Focus on constructive feedback
- Help others learn and grow
- Keep discussions professional

Thank you for contributing! 🎉
