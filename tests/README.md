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
# Run all compatibility tests
cargo test --test compatibility -- --ignored

# Run with output visible
cargo test --test compatibility -- --ignored --nocapture

# Run a specific compatibility test
cargo test --test compatibility test_md009_compatibility -- --ignored --nocapture
```

### How It Works

1. Test fixtures are created in `tests/fixtures/` with known violations
2. The markdownlint-cli2 Docker container is run on each fixture
3. Our implementation is run on the same fixture
4. The violations (rule name and line number) are compared
5. Tests pass if both implementations detect violations on the same lines

### What Gets Tested

Currently tested rules:
- **MD009**: Trailing spaces
- **MD010**: Hard tabs
- **MD012**: Multiple consecutive blank lines

### Behavior Without Docker

If Docker is not available, the tests will:
1. Check for Docker availability
2. Print "Docker not available, skipping markdownlint-cli2 comparison"
3. Pass the test gracefully

This allows the test suite to run in CI environments without Docker while still being useful in development.

### Adding New Compatibility Tests

To add compatibility tests for a new rule:

1. Create a test fixture in `tests/fixtures/` with violations
2. Add a new test function following the pattern:
   ```rust
   #[test]
   #[ignore]
   fn test_mdXXX_compatibility() {
       let fixture = Path::new("tests/fixtures/mdXXX_violations.md");
       let cli2_violations = run_markdownlint_cli2(fixture);
       // ... compare with our implementation
   }
   ```

### Troubleshooting

**"Docker not available" message:**
- Ensure Docker is installed: `docker --version`
- Ensure Docker daemon is running
- Ensure your user has permission to run Docker commands

**Tests fail with different violation counts:**
- Check the test output for which lines differ
- Verify the fixture file has the expected violations
- Check if our rule implementation differs from markdownlint-cli2
- Consider if the difference is intentional (document in CLAUDE.md if so)
