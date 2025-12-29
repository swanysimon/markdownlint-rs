# Integration Tests

This directory contains integration tests for markdownlint-rs.

## Compatibility Tests

The `compatibility.rs` tests verify that our Rust implementation produces the same results as the original [markdownlint-cli2](https://github.com/DavidAnson/markdownlint-cli2).

### Requirements

- Docker installed and running
- Internet connection (to pull the markdownlint-cli2 Docker image)

### Running Compatibility Tests

These tests are marked with `#[ignore]` because they are slower and require Docker. To run them:

```bash
# Run compatibility test
cargo test --test compatibility -- --ignored

# Run with output visible
cargo test --test compatibility -- --ignored --nocapture
```

### How It Works

1. A consolidated test fixture (`tests/fixtures/all_rules.md`) contains test cases for all rules
2. The markdownlint-cli2 Docker container is run on the fixture
3. Our implementation is run on the same fixture
4. Violations are grouped by rule and compared
5. The test passes if both implementations detect violations on the same lines for each rule

### What Gets Tested

All implemented rules are tested in a single consolidated run. The test output shows per-rule comparison results.

### Behavior Without Docker

If Docker is not available, the test will:
1. Check for Docker availability
2. Print "Docker not available, skipping markdownlint-cli2 comparison"
3. Pass the test gracefully

This allows the test suite to run in CI environments without Docker while still being useful in development.

### Troubleshooting

**"Docker not available" message:**
- Ensure Docker is installed: `docker --version`
- Ensure Docker daemon is running
- Ensure your user has permission to run Docker commands

**Tests fail with different violation counts:**
- Check the test output for which rules differ
- The output shows both cli2 and our line numbers for each rule
- Check if our rule implementation differs from markdownlint-cli2
- Consider if the difference is intentional (document in CLAUDE.md if so)
