// Compatibility tests with markdownlint-cli2
// These tests are slower as they require Docker to run the reference implementation
// Run with: cargo test --test compatibility -- --ignored

use markdownlint_rs::lint::rules::create_default_registry;
use markdownlint_rs::markdown::MarkdownParser;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::process::Command;

/// Check if Docker is available on the system
fn docker_available() -> bool {
    Command::new("docker")
        .arg("--version")
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

/// Run markdownlint-cli2 via Docker and parse the output
/// Returns a list of (rule_name, line_number) tuples
fn run_markdownlint_cli2(file_path: &Path) -> Vec<(String, usize)> {
    if !docker_available() {
        eprintln!("Docker not available, skipping markdownlint-cli2 comparison");
        return Vec::new();
    }

    let absolute_path = file_path
        .canonicalize()
        .expect("Failed to canonicalize path");
    let parent_dir = absolute_path.parent().expect("Failed to get parent dir");
    let file_name = absolute_path.file_name().expect("Failed to get filename");

    // Run markdownlint-cli2 in Docker
    let output = Command::new("docker")
        .arg("run")
        .arg("--rm")
        .arg("-v")
        .arg(format!("{}:/workdir", parent_dir.display()))
        .arg("-w")
        .arg("/workdir")
        .arg("davidanson/markdownlint-cli2:latest")
        .arg(file_name)
        .output()
        .expect("Failed to run markdownlint-cli2 via Docker");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    // Parse the output to extract violations
    // markdownlint-cli2 outputs in format: "filename:line:column MD### message"
    let mut violations = Vec::new();

    for line in stdout.lines().chain(stderr.lines()) {
        if line.contains("MD0") {
            let parts: Vec<&str> = line.split(':').collect();
            if parts.len() >= 2 {
                // parts[1] could be "5 MD001/..." or "5" (if column is present)
                // Extract just the line number by taking first whitespace-separated token
                if let Ok(line_num) = parts[1]
                    .split_whitespace()
                    .next()
                    .unwrap_or("")
                    .parse::<usize>()
                {
                    // Extract rule name (MD###)
                    if let Some(md_part) = line.split_whitespace().find(|s| s.starts_with("MD")) {
                        let rule_name = md_part.split('/').next().unwrap_or(md_part);
                        violations.push((rule_name.to_string(), line_num));
                    }
                }
            }
        }
    }

    violations.sort_by_key(|v| (v.0.clone(), v.1));
    violations
}

#[test]
#[ignore] // Slower test requiring Docker
fn test_all_rules_compatibility() {
    let fixture = Path::new("tests/fixtures/all_rules.md");
    if !fixture.exists() {
        panic!("Fixture file not found: {}", fixture.display());
    }

    let cli2_violations = run_markdownlint_cli2(fixture);
    if cli2_violations.is_empty() && !docker_available() {
        println!("Skipping test: Docker not available");
        return;
    }

    // Run all our rules
    let content = fs::read_to_string(fixture).expect("Failed to read file");
    let parser = MarkdownParser::new(&content);

    let registry = create_default_registry();
    let mut our_violations: Vec<(String, usize)> = Vec::new();

    for rule in registry.all_rules() {
        let violations = rule.check(&parser, None);
        for v in violations {
            our_violations.push((v.rule.clone(), v.line));
        }
    }

    our_violations.sort_by_key(|v| (v.0.clone(), v.1));

    println!("=== markdownlint-cli2 violations ===");
    for v in &cli2_violations {
        println!("  {:?}", v);
    }
    println!("\n=== Our violations ===");
    for v in &our_violations {
        println!("  {:?}", v);
    }

    // Group violations by rule for comparison
    let mut cli2_by_rule: HashMap<String, Vec<usize>> = HashMap::new();
    for (rule, line) in &cli2_violations {
        cli2_by_rule.entry(rule.clone()).or_default().push(*line);
    }

    let mut our_by_rule: HashMap<String, Vec<usize>> = HashMap::new();
    for (rule, line) in &our_violations {
        our_by_rule.entry(rule.clone()).or_default().push(*line);
    }

    // Get all unique rule names
    let mut all_rules: Vec<String> = cli2_by_rule
        .keys()
        .chain(our_by_rule.keys())
        .cloned()
        .collect();
    all_rules.sort();
    all_rules.dedup();

    println!("\n=== Per-rule comparison ===");

    let mut mismatches = Vec::new();

    for rule in &all_rules {
        let cli2_lines = cli2_by_rule.get(rule).cloned().unwrap_or_default();
        let our_lines = our_by_rule.get(rule).cloned().unwrap_or_default();

        if cli2_lines != our_lines {
            println!(
                "{}: MISMATCH - cli2={:?}, ours={:?}",
                rule, cli2_lines, our_lines
            );
            mismatches.push(format!(
                "{}: cli2 found {} at {:?}, we found {} at {:?}",
                rule,
                cli2_lines.len(),
                cli2_lines,
                our_lines.len(),
                our_lines
            ));
        } else {
            println!("{}: OK ({} violations)", rule, cli2_lines.len());
        }
    }

    if !mismatches.is_empty() {
        panic!("Compatibility mismatches found:\n{}", mismatches.join("\n"));
    }
}
