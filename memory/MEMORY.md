# markdownlint-rs Project Memory

## Project Pivot (Feb 2026)

mdlint is now positioned as an **opinionated formatter first, linter second** — like ruff/gofmt, not
markdownlint-cli2. The formatter (`mdlint format`) is the hero feature; the linter (`mdlint check`)
is secondary.

## Key Files

- `AIDEV.md` — task checklist as AI-addressable prompts (authoritative source of truth for work)
- `CLAUDE.md` — lessons learned and architecture notes for AI context (kept concise, ~200 line limit)
- `FORMAT_SPEC.md` — canonical formatter style decisions; source of truth for formatter behavior

## Architecture

- Config: TOML (`mdlint.toml` / `.mdlint.toml`), hierarchical discovery
- File discovery: `ignore` crate (gitignore-aware)
- Markdown parsing: `pulldown-cmark` wrapper in `src/markdown/`
- Formatter: `src/formatter/mod.rs` (canonical rewriter); `src/format/` (output formatters — different concerns)
- Rules: `src/lint/rules/md*.rs` through md060 (MD057 deliberately absent), registered in `create_default_registry()`

## Tooling

- `mise install` bootstraps all tools (`prek`, `tombi`, `hadolint`) — only Rust and mise are manual prereqs
- `prek run -a` runs ALL quality checks (TOML fmt, rustfmt, clippy --fix, tests, mdlint dogfood, hadolint)
- This is the single command for both local dev and CI quality gates
- Config: `mise.toml` (tool versions) + `prek.toml` (hook definitions)

## User Preferences

- Tasks tracked in AIDEV.md as AI-readable prompts, not numbered phases
- CLAUDE.md should stay under 200 lines (truncation limit)
- No compatibility goal with markdownlint-cli2; mdlint defines its own canonical format
