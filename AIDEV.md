# AIDEV.md — Development Task Checklist

Tasks are phrased as prompts you might give an AI coding assistant. Keep this file up to date as
work progresses. See CONTRIBUTING.md for development setup.

## Status Key

- ✅ Complete
- 🔄 In Progress
- ⬜ Todo

---

## Completed Foundation

- ✅ Initialize Cargo project with optimized release profile (LTO, strip, opt-level=3)
- ✅ Implement TOML-based configuration system with hierarchical directory discovery
- ✅ Implement file discovery using the `ignore` crate (gitignore-aware)
- ✅ Implement markdown parsing wrapper around pulldown-cmark with position tracking
- ✅ Implement linting rule framework: `Rule` trait, registry, and `Violation` type
- ✅ Implement auto-fix framework (`Fix` type, `fixer.rs`)
- ✅ Implement default and JSON output formatters
- ✅ Implement CLI with `check` and `format` subcommands using clap
- ✅ Set up GitHub Actions CI: test, clippy, fmt, dogfooding, and multi-platform build jobs
- ✅ Set up multi-platform binary builds (Linux x86/ARM, macOS x86/ARM, Windows) and Docker images
- ✅ Port the majority of the 54 markdownlint rules (see `src/lint/rules/`)
- ✅ Write user and developer documentation (README, CONTRIBUTING)

---

## Priority 1: Pivot and Positioning

These tasks update framing to reflect that mdlint is an **opinionated formatter first, linter second** —
analogous to ruff or gofmt, not markdownlint-cli2.

- ⬜ Update README.md to lead with "opinionated Markdown formatter" messaging. Remove the "Differences
  from markdownlint-cli2" section. Reframe the rules table to emphasize that most violations are
  auto-fixable via `mdlint format`. Keep existing structure but make the formatter the hero.

- ⬜ Write FORMAT_SPEC.md that specifies exactly what canonical mdlint-formatted markdown looks like.
  Document every opinionated choice: ATX headings only, dash list markers, backtick code fences,
  asterisk emphasis, trailing newline, blank lines around block elements, etc. This spec is the north
  star for all formatter implementation work.

- ⬜ Delete IMPROVEMENTS.md — its content has been moved into AIDEV.md. Remove any references to it
  from other documentation.

- ⬜ Update CONTRIBUTING.md to include a section on adding formatter rules (not just linting rules).
  Explain the distinction: a formatting rule is enforced by `mdlint format`; a linting rule is
  reported by `mdlint check`. Many rules should be both.

---

## Priority 2: Formatter Core

The formatter is the centerpiece of mdlint. It reads markdown, parses it to an AST, and emits canonical
text. This is what makes mdlint a formatter rather than a linter with `--fix`.

- ⬜ Design the formatter architecture. Decide whether to: (a) emit canonical text directly from
  pulldown-cmark events, (b) build an intermediate representation then emit, or (c) another approach.
  Key constraint: idempotency is a hard requirement — formatting an already-formatted file must
  produce no changes. Document the architectural decision in FORMAT_SPEC.md.

- ⬜ Implement `src/formatter/mod.rs` — a formatter that takes `&str` and returns a `String` in
  canonical form. Wire it into the existing `format` CLI command, which currently is a placeholder.

- ⬜ Implement heading canonicalization in the formatter: always emit ATX style (`# Heading`), never
  setext style (`Heading\n===`). MD003-equivalent.

- ⬜ Implement list marker canonicalization: always use dashes (`- item`), never asterisks or plus
  signs. MD004-equivalent.

- ⬜ Implement code fence canonicalization: always use backticks (` ``` `), never tildes (`~~~`).
  MD048-equivalent.

- ⬜ Implement emphasis marker canonicalization: always use asterisks (`*text*`, `**text**`), never
  underscores. MD049/MD050-equivalent.

- ⬜ Implement blank line normalization: exactly one blank line before and after block elements
  (headings, code blocks, lists, blockquotes); no multiple consecutive blank lines. MD012/MD022/MD031/
  MD032-equivalent.

- ⬜ Implement trailing whitespace removal: strip trailing spaces and tabs from every line.
  MD009-equivalent.

- ⬜ Implement trailing newline normalization: exactly one newline at end of file. MD047-equivalent.

- ⬜ Write tests verifying formatter idempotency: format a file, format it again, assert outputs are
  identical. Then add a property-based test using proptest that generates random CommonMark input and
  verifies idempotency.

- ⬜ Implement `mdlint format --check`: read each file, format in memory, exit 1 if any file would
  change. No files written. This is the CI-friendly verification mode.

---

## Priority 3: Linting Rules

The `check` command is the secondary workflow. Rules report violations; violations enforceable by the
formatter should also be marked fixable.

- ⬜ Audit all existing rule implementations against the markdownlint reference. For each of the 54
  rules, verify: (1) the rule file exists in `src/lint/rules/`, (2) it is registered in
  `create_default_registry()`, (3) it has at least one passing test. Fix any gaps found.

- ⬜ Mark all formatter-enforceable rules as fixable (return `true` from `fixable()`) and ensure their
  fix logic is consistent with what the formatter does — no divergence between `--fix` and
  `mdlint format`.

- ⬜ Fix the task list checkbox detection bug: `[ ]` in link position is being detected as a link.
  It should be recognized as a GFM task list checkbox and excluded from link-related rules (MD011, etc.).

- ⬜ Implement inline configuration comments: parse `<!-- mdlint-disable MD001 -->`,
  `<!-- mdlint-enable MD001 -->`, and `<!-- mdlint-disable-next-line MD001 -->` HTML comments
  during the check pass to suppress violations on specific lines.

- ⬜ Implement any remaining rules from the 54-rule set that are missing: review `src/lint/rules/mod.rs`
  to confirm registration count against the full markdownlint rule list.

---

## Priority 4: Testing

- ⬜ Write integration tests for the full check workflow: use test fixtures in `tests/fixtures/` to run
  discovery → lint → format output and compare against golden output files.

- ⬜ Write integration tests for the full format workflow: for each fixture, run the formatter and
  assert output matches a golden file. Assert that formatting the golden file again is a no-op
  (idempotency check).

- ⬜ Add property-based tests for the formatter using proptest: generate random strings and verify the
  formatter (1) never panics, (2) is idempotent, (3) produces output that parses as valid CommonMark.

- ⬜ Add regression tests for all known bugs as they are discovered and fixed. Each bug gets a fixture
  and a test.

- ⬜ Consider running compatibility tests against reference markdown files from real open-source projects
  to verify the formatter produces consistent, expected output.

---

## Priority 5: CLI Polish

- ⬜ Add color support to check output using the `anstream` crate. Colors for: error/warning severity,
  rule names, file paths. Respect the `NO_COLOR` environment variable and the `--no-color` flag.

- ⬜ Add summary statistics to check output: "X files checked, Y errors found across Z files" at the
  end of a run.

- ⬜ Add a `--verbose` flag that prints the name of each file as it is processed.

- ⬜ Improve error messages with structured context: file path, line number, column, and the offending
  text snippet highlighted.

---

## Priority 6: Distribution

- ⬜ Publish first release to crates.io by setting the `CARGO_REGISTRY_TOKEN` GitHub Actions secret
  and running `cargo release patch --execute`.

- ⬜ Write a Homebrew formula in a tap repository so macOS and Linux users can install with
  `brew install`.

- ⬜ Add pre-commit hook configuration examples to the documentation so teams can run
  `mdlint format --check` as a git pre-commit hook.

- ⬜ Consider an npm wrapper package so Node.js projects can add mdlint as a devDependency and use it
  without a separate Rust toolchain installation.
