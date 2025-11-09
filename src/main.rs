use clap::Parser;
use markdownlint_rs::error::Result;

#[derive(Parser, Debug)]
#[command(
    name = "markdownlint-rs",
    version,
    about = "A fast, flexible, configuration-based command-line interface for linting Markdown files"
)]
struct Cli {
    #[arg(help = "Glob patterns for files to lint")]
    patterns: Vec<String>,

    #[arg(long, help = "Path to configuration file")]
    config: Option<String>,

    #[arg(long, help = "Apply fixes to files")]
    fix: bool,

    #[arg(long, help = "Ignore globs from configuration")]
    no_globs: bool,
}

fn main() -> Result<()> {
    let _cli = Cli::parse();

    println!("markdownlint-rs initialized");
    println!("Project structure created successfully");

    Ok(())
}
