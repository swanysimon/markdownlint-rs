# markdownlint-rs: Porting markdownlint-cli2 to Rust

## Project Philosophy

This project ports [markdownlint-cli2](https://github.com/DavidAnson/markdownlint-cli2) to Rust as a standalone executable. Drawing inspiration from successful Rust ports like `ripgrep` and `exa`, we prioritize:

- **Correctness over performance**: Ensure accurate linting before optimization
- **Type safety**: Leverage Rust's type system for modularity and safety
- **Minimal code**: Use external crates liberally to reduce maintenance burden
- **No duplication**: Avoid reimplementing functionality available in quality crates
- **Clean code**: Let types speak, minimize comments
- **Comprehensive testing**: Unit tests for all core functionality

## Architecture Principles

Based on ripgrep and exa patterns:
- Modular design with clear separation of concerns
- Strong typing for configuration and state
- Result/Option types for error handling
- Builder patterns for complex objects
- Iterator-based processing where appropriate
- Minimal allocations in hot paths

## Lessons Learned

### Testing Strategy
- **Test business logic, not libraries**: Focus tests on our custom logic (merge algorithms, discovery patterns, extraction logic), not on validating that external crates work
- **Integration tests should validate our glue code**: When integrating libraries, test that our error handling and data transformation works correctly
- **Prefer specific tests over library validation**: Rather than testing "JSONC parsing works", test "our Config struct deserializes correctly with our serde annotations"
- **Use lib.rs for unit testing**: Created `src/lib.rs` to enable `cargo test --lib` for modular testing
- **Compatibility testing via Docker**: Created `tests/compatibility.rs` to verify our implementation matches markdownlint-cli2 behavior using Docker container
- **Graceful test degradation**: Compatibility tests skip gracefully when Docker is unavailable, allowing CI to run without Docker
- **Binary uses library crate**: Changed `main.rs` to import from library crate instead of duplicating module declarations, avoiding compilation issues

### Configuration System Implementation
- **JSONC requires special handling**: Used `jsonc-parser` crate to convert JSONC to `serde_json::Value` before deserializing
- **Package.json extraction**: Custom logic to extract nested `markdownlint-cli2` key with fallback to empty Config
- **Hierarchical discovery**: Walk up directory tree using `PathBuf::pop()` until finding a config or reaching root
- **Config merging precedence**: Later configs override earlier, but arrays extend rather than replace

### File Discovery and Globbing
- **Relative path matching**: Must canonicalize root path and use `strip_prefix()` to get relative paths before glob matching
- **Exclude pattern normalization**: Simple directory names like "node_modules" need to be expanded to `**/node_modules/**` for recursive exclusion
- **Gitignore integration**: The `ignore` crate requires an actual git repository to respect .gitignore files, not just the presence of a .gitignore file
- **Markdown extensions**: Support 8 common extensions: md, markdown, mdown, mkdn, mkd, mdwn, mdtxt, mdtext

### Markdown Parsing
- **pulldown-cmark lifetime complexity**: The Parser type requires 2 lifetimes, simplified by returning `impl Iterator<Item = Event<'a>>` instead of concrete Parser type
- **Position tracking**: Implemented offset-to-line and offset-to-position mapping by tracking cumulative byte offsets with `line.len() + 1` (accounting for newlines)
- **Front matter detection**: Simple string-based detection for YAML (---) and TOML (+++) delimiters, avoiding regex overhead
- **CommonMark extensions**: Enabled tables, footnotes, strikethrough, tasklists, and heading attributes via pulldown-cmark Options

### Rule System Architecture
- **Rule trait design**: Rules receive `&MarkdownParser` for full access to content, lines, and AST events, plus optional JSON config
- **Fixable trait method**: Default implementation returns `false`, only override for fixable rules
- **Fix type**: Supports line and column ranges for precise text replacement with descriptive messages
- **Config flexibility**: Rules parse their own config from `Option<&Value>`, allowing rule-specific options like `br_spaces`, `strict`, `maximum`
- **Registry pattern**: Simple HashMap-based registry with `create_default_registry()` function to register all built-in rules

---

## Phase 1: Project Foundation ✅

### 1.1 Cargo Project Setup ✅
- [x] Initialize with `cargo init --bin`
- [x] Configure `Cargo.toml` with metadata (name, version, authors, edition)
- [x] Set up workspace if needed for multiple crates (not needed - single binary)
- [x] Add initial dependency categories (CLI, config, glob, markdown)
- [x] Configure release profile for optimization
- [x] Set up `.gitignore` for Rust projects (already present)

### 1.2 Core Dependencies Selection ✅
- [x] **CLI parsing**: `clap` v4 with derive feature for ergonomic argument handling
- [x] **Configuration**: `serde` + `serde_json` + `serde_yaml` + `jsonc-parser` crate
- [x] **Globbing**: `globset` or `ignore` crate (used by ripgrep) - Added both
- [x] **Markdown parsing**: Research options (pulldown-cmark, comrak, or markdown crate) - Selected pulldown-cmark
- [x] **Pattern matching**: `regex` crate for advanced patterns
- [x] **File I/O**: `walkdir` or use `ignore` crate for gitignore-aware traversal - Using ignore crate
- [x] **Error handling**: `anyhow` for application errors, `thiserror` for library errors
- [x] **Async runtime**: Evaluate if needed (likely not for initial version) - Not needed

### 1.3 Project Structure ✅
```
src/
  lib.rs            # Library root for unit testing
  main.rs           # Entry point, CLI setup
  config/           # Configuration loading and parsing
    mod.rs
    types.rs        # Config structure definitions
    loader.rs       # Load from various formats
    merge.rs        # Hierarchical config merging
  glob/             # File discovery
    mod.rs
    matcher.rs      # Pattern matching logic
    walker.rs       # File system traversal
  lint/             # Core linting engine
    mod.rs
    engine.rs       # Main linting orchestration
    rule.rs         # Rule trait and implementations
    result.rs       # Lint result types
  fix/              # Auto-fix functionality
    mod.rs
    fixer.rs        # Apply fixes to files
  format/           # Output formatting
    mod.rs
    default.rs      # Default formatter
    json.rs         # JSON output
    junit.rs        # JUnit format
    sarif.rs        # SARIF format
  error.rs          # Error types
  types.rs          # Shared types
```

### 1.4 Type System Foundation ✅
- [x] Define `Config` struct with all supported options
- [x] Define `LintResult` type for individual violations
- [x] Define `FileResult` type for per-file results
- [x] Define `Rule` trait for linting rules
- [x] Define `Formatter` trait for output formatters
- [x] Use `PathBuf` consistently for file paths
- [x] Use strongly-typed enums for options (e.g., `OutputFormat`)

---

## Phase 2: Configuration System ✅

### 2.1 Configuration File Formats ✅
- [x] Implement `.markdownlint-cli2.jsonc` parser (JSON with comments)
- [x] Implement `.markdownlint-cli2.yaml` parser
- [x] Implement `.markdownlint.json` parser (rules only)
- [x] Implement `.markdownlint.yaml` parser (rules only)
- [x] Implement `package.json` parser (extract `markdownlint-cli2` key)
- [x] Handle missing configuration gracefully (use defaults)

### 2.2 Configuration Properties ✅
Implement support for all markdownlint-cli2 config options:
- [x] `config`: Rule configuration object
- [x] `customRules`: Array of custom rule paths/modules
- [x] `fix`: Boolean to enable auto-fixing
- [x] `frontMatter`: Regex pattern for front matter
- [x] `gitignore`: Boolean to respect .gitignore
- [x] `globs`: Array of glob patterns
- [x] `ignores`: Array of ignore patterns
- [x] `markdownItPlugins`: Plugin configuration (defer to later phase)
- [x] `noBanner`: Suppress banner output
- [x] `noProgress`: Suppress progress output
- [x] `noInlineConfig`: Disable HTML comment configuration
- [x] `outputFormatters`: Array of formatter configurations

### 2.3 Hierarchical Configuration ✅
- [x] Discover config files in directory tree (walk up from cwd)
- [x] Merge configurations with correct precedence:
  1. Command-line options (highest) - ready for Phase 8
  2. Local directory config
  3. Parent directory configs
  4. Home directory config
  5. Default config (lowest)
- [x] Handle conflicts correctly (later configs override earlier)
- [x] Unit tests for merge logic

### 2.4 Configuration Validation (Deferred)
- [ ] Validate rule names against known rules (will do after implementing rules)
- [ ] Validate glob patterns are valid (will do in Phase 3)
- [ ] Validate regex patterns compile (will do when implementing front matter)
- [x] Provide helpful error messages for invalid config
- [x] Unit tests for validation logic (8 tests covering core functionality)

---

## Phase 3: File Discovery and Globbing ✅

### 3.1 Glob Pattern Processing ✅
- [x] Parse command-line glob patterns (ready for Phase 8)
- [x] Parse config file glob patterns
- [x] Support negation patterns (e.g., `#node_modules`)
- [x] Support `**` recursive wildcards
- [x] Support `?` and `*` single-level wildcards
- [x] Support character classes `[...]`
- [x] Handle Windows vs Unix path separators (globset handles this)

### 3.2 File System Traversal ✅
- [x] Implement directory walker using `ignore` crate
- [x] Respect `.gitignore` when `gitignore: true`
- [x] Apply ignore patterns from config
- [x] Filter for Markdown extensions: `.md`, `.markdown`, `.mdown`, etc.
- [x] Handle symlinks appropriately (ignore crate handles this)
- [x] Handle permission errors gracefully
- [x] Parallelize file discovery if beneficial (defer to Phase 10)

### 3.3 Front Matter Detection ✅
- [x] Implement front matter detection (string-based, not regex)
- [x] Support YAML front matter (`---` delimiters)
- [x] Support TOML front matter (`+++` delimiters)
- [x] Support custom regex patterns from config (defer to when needed)
- [x] Strip front matter before linting if configured
- [x] Preserve line numbers in results

### 3.4 File Discovery Tests ✅
- [x] Unit tests for glob pattern matching
- [x] Integration tests for file traversal
- [x] Tests for gitignore integration
- [x] Tests for front matter detection

---

## Phase 4: Markdown Parsing ✅

### 4.1 Parser Selection and Integration ✅
- [x] Research Rust markdown parsers (pulldown-cmark selected)
- [x] Evaluate CommonMark compliance (fully compliant with extensions)
- [x] Evaluate extensibility for custom rules (event-based API suitable)
- [x] Integrate chosen parser as dependency
- [x] Create parser wrapper with consistent interface

### 4.2 AST Processing ✅
- [x] Parse markdown into AST (event stream)
- [x] Provide AST traversal utilities (iterator-based)
- [x] Track line and column positions (offset_to_line, offset_to_position)
- [x] Handle inline HTML (pulldown-cmark handles natively)
- [x] Handle code blocks (fenced and indented, pulldown-cmark handles)
- [x] Handle lists (ordered and unordered, pulldown-cmark handles)
- [x] Handle emphasis and strong emphasis (pulldown-cmark handles)
- [x] Handle links and images (pulldown-cmark handles)
- [x] Handle tables (enabled via Options::ENABLE_TABLES)

### 4.3 Token and Line Processing ✅
- [x] Provide line-by-line access to content (get_line, lines())
- [x] Provide token stream access (parse(), parse_with_offsets())
- [x] Map tokens back to source positions (offset_to_line, offset_to_position)
- [x] Handle multi-byte UTF-8 characters correctly (using byte offsets)

### 4.4 Parser Tests ✅
- [x] Unit tests for AST generation (test_parse_events)
- [x] Tests for position tracking (test_offset_to_line, test_offset_to_position)
- [x] Tests for various markdown features (test_event_type_checks)
- [x] Edge case tests (basic_parsing, get_line boundary tests)

---

## Phase 5: Core Linting Rules (In Progress)

### 5.1 Rule System Architecture ✅
- [x] Define `Rule` trait with methods:
  - `name()`: Rule identifier (e.g., "MD001")
  - `description()`: Human-readable description
  - `tags()`: Rule categories
  - `check()`: Perform the check, return violations (takes MarkdownParser and config)
  - `fixable()`: Whether rule supports auto-fix
- [x] Define `Violation` type with:
  - Line and column position
  - Rule name
  - Error message
  - Fix information (if fixable)
- [x] Registry pattern for all rules
- [x] Rule configuration through Config types (JSON values)

### 5.2 Essential Rules (Priority 1) ✅
Port core rules from markdownlint library:
- [x] MD001: Heading levels increment by one
- [x] MD003: Heading style consistency (supports style config: atx, atx_closed, setext)
- [x] MD004: Unordered list style consistency (supports style config: asterisk, plus, dash)
- [x] MD005: List indentation consistency (checks items at same level have same indent)
- [x] MD007: Unordered list indentation (supports indent config, default 2)
- [x] MD009: Trailing spaces (supports br_spaces, strict config)
- [x] MD010: Hard tabs (supports code_blocks config)
- [x] MD011: Reversed link syntax (regex-based detection)
- [x] MD012: Multiple consecutive blank lines (supports maximum config)
- [x] MD013: Line length (supports line_length, heading_line_length, code_blocks, tables, headings config)

### 5.3 Important Rules (Priority 2) - Partial
- [x] MD018: No space after hash on atx heading
- [x] MD019: Multiple spaces after hash on atx heading
- [x] MD022: Headings surrounded by blank lines
- [x] MD023: Headings must start at the beginning of the line
- [x] MD025: Multiple top-level headings
- [ ] MD027: Multiple spaces after blockquote symbol
- [ ] MD028: Blank line inside blockquote
- [ ] MD029: Ordered list item prefix
- [ ] MD030: Spaces after list markers
- [ ] MD031: Fenced code blocks surrounded by blank lines

### 5.4 Additional Rules (Priority 3)
- [ ] Port remaining ~60 rules from markdownlint
- [ ] Ensure each rule matches original behavior
- [ ] Add configuration options for each rule
- [ ] Document each rule

### 5.5 Rule Tests
- [ ] Unit test for each rule with positive cases
- [ ] Unit test for each rule with negative cases
- [ ] Test rule configuration options
- [ ] Test edge cases
- [ ] Compare output with original markdownlint-cli2

---

## Phase 6: Auto-Fix Implementation

### 6.1 Fix Framework
- [ ] Define `Fix` type with:
  - Position range to replace
  - Replacement text
  - Fix description
- [ ] Implement fix application logic
- [ ] Handle multiple fixes in single file
- [ ] Ensure fixes don't conflict
- [ ] Sort fixes by position (reverse order for application)

### 6.2 Fixable Rules
Implement fixes for rules that support auto-fix:
- [ ] MD009: Remove trailing spaces
- [ ] MD010: Replace tabs with spaces
- [ ] MD012: Remove excess blank lines
- [ ] MD014: Remove dollar signs from shell commands
- [ ] MD018: Add space after hash
- [ ] MD019: Remove extra spaces after hash
- [ ] MD022: Add blank lines around headings
- [ ] MD023: Move heading to start of line
- [ ] MD027: Remove extra spaces in blockquote
- [ ] MD030: Fix spaces after list markers
- [ ] MD031: Add blank lines around code blocks
- [ ] MD034: Convert bare URLs to links
- [ ] MD037: Remove spaces inside emphasis markers
- [ ] MD038: Remove spaces inside code spans
- [ ] MD039: Remove spaces inside link text

### 6.3 Fix Safety
- [ ] Validate fixes don't corrupt file
- [ ] Preserve file encoding
- [ ] Preserve line endings (LF vs CRLF)
- [ ] Create backup before fixing (optional flag)
- [ ] Dry-run mode to preview fixes

### 6.4 Fix Tests
- [ ] Unit tests for each fixable rule
- [ ] Integration tests for multi-fix files
- [ ] Tests for fix conflicts
- [ ] Tests for line ending preservation

---

## Phase 7: Output Formatters

### 7.1 Formatter Trait
- [ ] Define `Formatter` trait with methods:
  - `format()`: Convert results to output string
  - `supports_color()`: Whether color output is supported
- [ ] Implement formatter registry
- [ ] Support multiple formatters simultaneously

### 7.2 Default Formatter
- [ ] Human-readable output with file paths
- [ ] Rule violations with line:column positions
- [ ] Color support using `termcolor` or `anstream` crate
- [ ] Summary statistics (files checked, errors found)
- [ ] Match markdownlint-cli2-formatter-default output

### 7.3 JSON Formatter
- [ ] Output results as JSON array
- [ ] Include all violation details
- [ ] Match markdownlint-cli2 JSON schema
- [ ] Pretty-print option

### 7.4 Additional Formatters
- [ ] JUnit XML formatter
- [ ] SARIF formatter (Static Analysis Results Interchange Format)
- [ ] GitHub Actions formatter (::error:: annotations)
- [ ] Codacy formatter
- [ ] Custom formatter plugin system (if time permits)

### 7.5 Formatter Tests
- [ ] Unit tests for each formatter
- [ ] Schema validation for structured formats
- [ ] Compare output with original formatters

---

## Phase 8: CLI Interface

### 8.1 Command-Line Arguments
Using `clap` with derive macros:
- [ ] Positional glob patterns (multiple allowed)
- [ ] `--config <path>`: Custom config file path
- [ ] `--fix`: Enable auto-fixing
- [ ] `--no-globs`: Ignore globs from config
- [ ] `--help`: Display usage information
- [ ] `--version`: Display version
- [ ] Consider: `--verbose` for debug output
- [ ] Consider: `--quiet` to suppress non-error output

### 8.2 Exit Codes
- [ ] Exit 0: Success, no errors found
- [ ] Exit 1: Success, but errors found
- [ ] Exit 2: Runtime error (bad config, file access, etc.)
- [ ] Document exit codes in help text

### 8.3 Progress and Banner
- [ ] Optional banner with tool name/version
- [ ] Progress indicator for large file sets
- [ ] Respect `noBanner` and `noProgress` config
- [ ] Detect TTY for appropriate output

### 8.4 Inline Configuration
- [ ] Parse HTML comments in Markdown:
  - `<!-- markdownlint-disable MD001 -->`
  - `<!-- markdownlint-enable MD001 -->`
  - `<!-- markdownlint-disable-next-line MD001 -->`
  - `<!-- markdownlint-configure-file { "MD013": false } -->`
- [ ] Apply inline config during linting
- [ ] Respect `noInlineConfig` option

### 8.5 CLI Tests
- [ ] Integration tests for CLI argument parsing
- [ ] Tests for exit codes
- [ ] Tests for inline configuration
- [ ] End-to-end tests with real files

---

## Phase 9: Testing Strategy

### 9.1 Unit Tests
- [ ] Test all configuration parsing
- [ ] Test all glob matching
- [ ] Test each linting rule independently
- [ ] Test fix application logic
- [ ] Test formatters
- [ ] Aim for >80% code coverage

### 9.2 Integration Tests
- [ ] End-to-end tests with sample repositories
- [ ] Test complete workflows (discovery → lint → format)
- [ ] Test with various configuration combinations
- [ ] Test auto-fix workflows

### 9.3 Compatibility Tests
- [ ] Create test suite comparing output with markdownlint-cli2
- [ ] Use identical inputs and configurations
- [ ] Verify results match (or document differences)
- [ ] Test against markdownlint's official test suite if available

### 9.4 Property-Based Tests
- [ ] Use `proptest` or `quickcheck` for fuzzing
- [ ] Generate random valid markdown
- [ ] Generate random invalid markdown
- [ ] Ensure no panics or crashes

### 9.5 Benchmark Tests
- [ ] Optional benchmarks using `criterion` crate
- [ ] Measure rule execution time
- [ ] Measure file discovery time
- [ ] Compare with markdownlint-cli2 (informational only)

---

## Phase 10: Advanced Features

### 10.1 Custom Rules
- [ ] Design plugin system for custom rules
- [ ] Support loading external rules (dynamic libraries or scripts)
- [ ] Document custom rule API
- [ ] Provide example custom rules
- [ ] Security considerations for untrusted rules

### 10.2 markdown-it Plugins
- [ ] Research markdown-it plugin equivalents in Rust
- [ ] Determine if extensible parsing is needed
- [ ] Implement plugin system or defer to future version
- [ ] Document limitations vs original

### 10.3 Performance Optimizations
(Only after correctness is proven)
- [ ] Parallel file processing using `rayon`
- [ ] Memory-mapped file I/O for large files
- [ ] Incremental linting (cache results)
- [ ] Profile and optimize hot paths

### 10.4 Watch Mode
- [ ] Implement file watching using `notify` crate
- [ ] Re-lint on file changes
- [ ] Debounce rapid changes
- [ ] Clear screen between runs

---

## Phase 11: Documentation

### 11.1 Code Documentation
- [ ] Add doc comments to all public items
- [ ] Generate docs with `cargo doc`
- [ ] Include examples in doc comments
- [ ] Document all configuration options

### 11.2 User Documentation
- [ ] README.md with overview and quick start
- [ ] Installation instructions
- [ ] Usage examples
- [ ] Configuration guide
- [ ] Migration guide from markdownlint-cli2
- [ ] Rule reference (can link to markdownlint docs initially)

### 11.3 Development Documentation
- [ ] CONTRIBUTING.md with development setup
- [ ] Architecture overview
- [ ] Adding new rules guide
- [ ] Release process

---

## Phase 12: Distribution and Release

### 12.1 Build Configuration
- [ ] Optimize release builds in Cargo.toml
- [ ] Enable LTO (Link Time Optimization)
- [ ] Set appropriate `opt-level`
- [ ] Strip debug symbols in release

### 12.2 Cross-Platform Builds
- [ ] Test on Linux
- [ ] Test on macOS
- [ ] Test on Windows
- [ ] Handle platform-specific path issues
- [ ] Handle platform-specific line endings

### 12.3 Distribution
- [ ] Publish to crates.io
- [ ] Create GitHub releases with binaries
- [ ] Build static binaries (musl on Linux)
- [ ] Consider: Homebrew formula
- [ ] Consider: Debian package
- [ ] Consider: Docker image
- [ ] Consider: npm wrapper package for Node.js compatibility

### 12.4 CI/CD
- [ ] Set up GitHub Actions for CI
- [ ] Run tests on all platforms
- [ ] Run clippy and rustfmt checks
- [ ] Automated binary builds on release
- [ ] Automated publishing to crates.io

---

## Phase 13: Polish and Refinement

### 13.1 Error Messages
- [ ] Review all error messages for clarity
- [ ] Add suggestions for common mistakes
- [ ] Include file path and location in errors
- [ ] Color code error messages

### 13.2 Performance Review
- [ ] Profile the application
- [ ] Identify bottlenecks
- [ ] Optimize if needed (maintain correctness)

### 13.3 Security Review
- [ ] Review for path traversal vulnerabilities
- [ ] Review regex for ReDoS vulnerabilities
- [ ] Review file I/O for TOCTOU issues
- [ ] Consider fuzzing for crash resistance

### 13.4 Compatibility Review
- [ ] Document differences from markdownlint-cli2
- [ ] Evaluate if differences are acceptable
- [ ] Add compatibility flags if needed

---

## Success Criteria

The project is complete when:
- ✓ All core markdownlint rules are implemented correctly
- ✓ Configuration system supports all markdownlint-cli2 formats
- ✓ Auto-fix works for all fixable rules
- ✓ Output formatters match original behavior
- ✓ Test coverage >80% with comprehensive integration tests
- ✓ Documentation is complete and accurate
- ✓ Published to crates.io
- ✓ Cross-platform binaries available
- ✓ Compatibility with markdownlint-cli2 verified on real projects

---

## Non-Goals (For Initial Release)

- Matching markdownlint-cli2 performance (correctness first)
- JavaScript custom rule compatibility (Rust API instead)
- markdown-it plugin system (use parser's native capabilities)
- Watch mode (can add later)
- Configuration file generation/wizards (can add later)

---

## References

- [markdownlint-cli2 GitHub](https://github.com/DavidAnson/markdownlint-cli2)
- [markdownlint library](https://github.com/DavidAnson/markdownlint)
- [ripgrep source](https://github.com/BurntSushi/ripgrep) - Reference for CLI structure
- [exa source (eza fork)](https://github.com/eza-community/eza) - Reference for code organization
- [CommonMark Spec](https://spec.commonmark.org/)
- [pulldown-cmark](https://github.com/raphlinus/pulldown-cmark) - Likely markdown parser
