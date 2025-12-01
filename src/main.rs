use clap::Parser;
use markdownlint_rs::config::{Config, ConfigLoader, merge_many_configs};
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

    #[arg(
        long,
        help = "Output format: default or json",
        default_value = "default"
    )]
    format: String,

    #[arg(long, help = "Disable color output")]
    no_color: bool,
}

fn main() {
    process::exit(
        run()
            .map(|had_errors| return if had_errors { 1 } else { 0 })
            .unwrap_or(2),
    );
}

fn run() -> Result<bool> {
    let cli = Cli::parse();
    let config = load_config(&cli)?;

    let files = find_files(&cli, &config)?;
    if files.is_empty() {
        eprintln!("No markdown files found");
        return Ok(false);
    }

    let lint_result = lint_files(config, &files)?;
    if cli.fix && lint_result.has_errors() {
        apply_fixes(&lint_result)?;
    }

    let use_color = !cli.no_color && io::stdout().is_terminal();
    let formatter: Box<dyn Formatter> = match cli.format.as_str() {
        "json" => Box::new(JsonFormatter::new(true)),
        _ => Box::new(DefaultFormatter::new(use_color)),
    };

    let output = formatter.format(&lint_result);
    print!("{}", output);

    Ok(lint_result.has_errors())
}

fn load_config(cli: &Cli) -> Result<Config> {
    if let Some(config_path) = &cli.config {
        let path = PathBuf::from(config_path);
        return ConfigLoader::load_from_file(&path);
    }

    let configs = ConfigLoader::find_all_configs(&env::current_dir()?)?;
    if configs.is_empty() {
        return Ok(Config::default());
    }

    let config_list: Vec<Config> = configs.into_iter().map(|(_, cfg)| cfg).collect();
    Ok(merge_many_configs(config_list))
}

fn find_files(cli: &Cli, config: &Config) -> Result<Vec<PathBuf>> {
    if cli.patterns.is_empty() {
        let walker = FileWalker::new(config.gitignore);
        return walker.find_markdown_files(&env::current_dir()?);
    }

    let mut all_files = Vec::new();
    let mut add_to_file = |path: PathBuf| {
        if !all_files.contains(&path) {
            all_files.push(path);
        }
    };

    for pattern in &cli.patterns {
        let path = PathBuf::from(pattern);
        if path.is_dir() {
            let walker = FileWalker::new(config.gitignore);
            walker
                .find_markdown_files(&path)?
                .into_iter()
                .for_each(|path| add_to_file(path));
        } else if path.is_file() {
            add_to_file(path);
        } else {
            eprintln!("Warning: Path not found: {}", pattern);
        }
    }

    Ok(all_files)
}

fn lint_files(config: Config, files: &Vec<PathBuf>) -> Result<LintResult> {
    let engine = LintEngine::new(config.clone());

    let mut lint_result = LintResult::new();
    for file_path in files {
        let content = fs::read_to_string(file_path)?;
        let violations = engine.lint_content(&content)?;

        if !violations.is_empty() {
            lint_result.add_file_result(file_path.clone(), violations);
        }
    }
    Ok(lint_result)
}

fn apply_fixes(lint_result: &LintResult) -> Result<()> {
    let fixer = Fixer::new(); // Not dry-run

    for file_result in &lint_result.file_results {
        let fixable_violations: Vec<_> = file_result
            .violations
            .iter()
            .filter(|v| v.fix.is_some())
            .collect();
        if fixable_violations.is_empty() {
            continue;
        }

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
    Ok(())
}
