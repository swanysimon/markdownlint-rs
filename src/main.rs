use clap::Parser;
use markdownlint_rs::config::{merge_many_configs, Config, ConfigLoader};
use markdownlint_rs::error::Result;
use markdownlint_rs::fix::Fixer;
use markdownlint_rs::format::{DefaultFormatter, Formatter, JsonFormatter};
use markdownlint_rs::glob::FileWalker;
use markdownlint_rs::lint::{LintEngine, LintResult};
use std::env;
use std::fs;
use std::io::{self, IsTerminal};
use std::path::PathBuf;
use std::process;

#[derive(Parser, Debug)]
#[command(
    name = "markdownlint-rs",
    version,
    about = "A fast, flexible, configuration-based command-line interface for linting Markdown files"
)]
struct Cli {
    #[arg(help = "Glob patterns for files to lint (defaults to current directory)")]
    patterns: Vec<String>,

    #[arg(long, help = "Path to configuration file")]
    config: Option<String>,

    #[arg(long, help = "Apply fixes to files")]
    fix: bool,

    #[arg(long, help = "Ignore globs from configuration")]
    no_globs: bool,

    #[arg(long, help = "Output format: default or json", default_value = "default")]
    format: String,

    #[arg(long, help = "Disable color output")]
    no_color: bool,
}

fn main() {
    let exit_code = match run() {
        Ok(had_errors) => {
            if had_errors {
                1 // Errors found
            } else {
                0 // Success
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            2 // Runtime error
        }
    };

    process::exit(exit_code);
}

fn run() -> Result<bool> {
    let cli = Cli::parse();

    // Load configuration
    let config = if let Some(config_path) = &cli.config {
        // Load specific config file
        let path = PathBuf::from(config_path);
        ConfigLoader::load_from_file(&path)?
    } else {
        // Discover and merge configs from current directory
        let cwd = env::current_dir()?;
        let configs = ConfigLoader::find_all_configs(&cwd)?;
        if configs.is_empty() {
            Config::default()
        } else {
            let config_list: Vec<Config> = configs.into_iter().map(|(_, cfg)| cfg).collect();
            merge_many_configs(config_list)
        }
    };

    // Discover files to lint
    let files = if cli.patterns.is_empty() {
        // Default to current directory
        let walker = FileWalker::new(config.gitignore);
        walker.find_markdown_files(&env::current_dir()?)?
    } else {
        // Use provided patterns
        let mut all_files = Vec::new();
        let walker = FileWalker::new(config.gitignore);

        for pattern in &cli.patterns {
            let path = PathBuf::from(pattern);

            // If it's a directory, walk it
            if path.is_dir() {
                let files = walker.find_markdown_files(&path)?;
                all_files.extend(files);
            } else if path.is_file() {
                // If it's a file, add it directly
                all_files.push(path);
            } else {
                eprintln!("Warning: Path not found: {}", pattern);
            }
        }

        all_files.sort();
        all_files.dedup();
        all_files
    };

    if files.is_empty() {
        eprintln!("No markdown files found");
        return Ok(false);
    }

    // Create lint engine
    let engine = LintEngine::new(config.clone());

    // Lint all files
    let mut lint_result = LintResult::new();
    for file_path in &files {
        let content = fs::read_to_string(file_path)?;
        let violations = engine.lint_content(&content)?;

        if !violations.is_empty() {
            lint_result.add_file_result(file_path.clone(), violations);
        }
    }

    // Apply fixes if requested
    if cli.fix && lint_result.has_errors() {
        let fixer = Fixer::new(); // Not dry-run

        for file_result in &lint_result.file_results {
            let fixable_violations: Vec<_> = file_result
                .violations
                .iter()
                .filter(|v| v.fix.is_some())
                .collect();

            if !fixable_violations.is_empty() {
                let content = fs::read_to_string(&file_result.path)?;
                let fixes: Vec<_> = fixable_violations
                    .iter()
                    .filter_map(|v| v.fix.clone())
                    .collect();

                match fixer.apply_fixes_to_content(&content, &fixes) {
                    Ok(fixed_content) => {
                        fs::write(&file_result.path, fixed_content)?;
                        eprintln!("Fixed: {}", file_result.path.display());
                    }
                    Err(e) => {
                        eprintln!(
                            "Failed to apply fixes to {}: {}",
                            file_result.path.display(),
                            e
                        );
                    }
                }
            }
        }
    }

    // Determine if we should use color
    let use_color = !cli.no_color && io::stdout().is_terminal();

    // Format and output results
    let formatter: Box<dyn Formatter> = match cli.format.as_str() {
        "json" => Box::new(JsonFormatter::new(true)),
        _ => Box::new(DefaultFormatter::new(use_color)),
    };

    let output = formatter.format(&lint_result);
    print!("{}", output);

    Ok(lint_result.has_errors())
}
