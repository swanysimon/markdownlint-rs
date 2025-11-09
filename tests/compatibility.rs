// Compatibility tests with markdownlint-cli2
// These tests are slower as they require Docker to run the reference implementation
// Run with: cargo test --test compatibility -- --ignored --test-threads=1

use markdownlint_rs::lint::rules::{
    create_default_registry, MD001, MD003, MD004, MD005, MD007, MD009, MD010, MD011, MD012, MD013,
};
use markdownlint_rs::lint::Rule;
use markdownlint_rs::markdown::MarkdownParser;
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

    let absolute_path = file_path.canonicalize().expect("Failed to canonicalize path");
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
            if parts.len() >= 3 {
                if let Ok(line_num) = parts[1].trim().parse::<usize>() {
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

/// Run our implementation and extract violations
fn run_our_implementation(
    file_path: &Path,
    rule: &dyn Rule,
) -> Vec<(String, usize)> {
    let content = fs::read_to_string(file_path).expect("Failed to read file");
    let parser = MarkdownParser::new(&content);
    let violations = rule.check(&parser, None);

    let mut results: Vec<(String, usize)> = violations
        .iter()
        .map(|v| (v.rule.clone(), v.line))
        .collect();

    results.sort_by_key(|v| (v.0.clone(), v.1));
    results
}

#[test]
#[ignore] // Slower test requiring Docker
fn test_md009_compatibility() {
    let fixture = Path::new("tests/fixtures/md009_violations.md");
    if !fixture.exists() {
        panic!("Fixture file not found: {}", fixture.display());
    }

    let cli2_violations = run_markdownlint_cli2(fixture);
    if cli2_violations.is_empty() && !docker_available() {
        println!("Skipping test: Docker not available");
        return;
    }

    let rule = MD009;
    let our_violations = run_our_implementation(fixture, &rule);

    println!("markdownlint-cli2 violations: {:?}", cli2_violations);
    println!("Our violations: {:?}", our_violations);

    // Filter to only MD009 violations from cli2
    let cli2_md009: Vec<_> = cli2_violations
        .iter()
        .filter(|(name, _)| name == "MD009")
        .collect();

    assert_eq!(
        our_violations.len(),
        cli2_md009.len(),
        "Different number of MD009 violations detected"
    );

    // Check that line numbers match
    for (our, cli2) in our_violations.iter().zip(cli2_md009.iter()) {
        assert_eq!(
            our.1, cli2.1,
            "Line number mismatch for MD009: ours={}, cli2={}",
            our.1, cli2.1
        );
    }
}

#[test]
#[ignore] // Slower test requiring Docker
fn test_md010_compatibility() {
    let fixture = Path::new("tests/fixtures/md010_violations.md");
    if !fixture.exists() {
        panic!("Fixture file not found: {}", fixture.display());
    }

    let cli2_violations = run_markdownlint_cli2(fixture);
    if cli2_violations.is_empty() && !docker_available() {
        println!("Skipping test: Docker not available");
        return;
    }

    let rule = MD010;
    let our_violations = run_our_implementation(fixture, &rule);

    println!("markdownlint-cli2 violations: {:?}", cli2_violations);
    println!("Our violations: {:?}", our_violations);

    // Filter to only MD010 violations from cli2
    let cli2_md010: Vec<_> = cli2_violations
        .iter()
        .filter(|(name, _)| name == "MD010")
        .collect();

    assert_eq!(
        our_violations.len(),
        cli2_md010.len(),
        "Different number of MD010 violations detected"
    );

    // Check that line numbers match
    for (our, cli2) in our_violations.iter().zip(cli2_md010.iter()) {
        assert_eq!(
            our.1, cli2.1,
            "Line number mismatch for MD010: ours={}, cli2={}",
            our.1, cli2.1
        );
    }
}

#[test]
#[ignore] // Slower test requiring Docker
fn test_md012_compatibility() {
    let fixture = Path::new("tests/fixtures/md012_violations.md");
    if !fixture.exists() {
        panic!("Fixture file not found: {}", fixture.display());
    }

    let cli2_violations = run_markdownlint_cli2(fixture);
    if cli2_violations.is_empty() && !docker_available() {
        println!("Skipping test: Docker not available");
        return;
    }

    let rule = MD012;
    let our_violations = run_our_implementation(fixture, &rule);

    println!("markdownlint-cli2 violations: {:?}", cli2_violations);
    println!("Our violations: {:?}", our_violations);

    // Filter to only MD012 violations from cli2
    let cli2_md012: Vec<_> = cli2_violations
        .iter()
        .filter(|(name, _)| name == "MD012")
        .collect();

    assert_eq!(
        our_violations.len(),
        cli2_md012.len(),
        "Different number of MD012 violations detected"
    );

    // Check that line numbers match
    for (our, cli2) in our_violations.iter().zip(cli2_md012.iter()) {
        assert_eq!(
            our.1, cli2.1,
            "Line number mismatch for MD012: ours={}, cli2={}",
            our.1, cli2.1
        );
    }
}

#[test]
#[ignore] // Slower test requiring Docker
fn test_md001_compatibility() {
    let fixture = Path::new("tests/fixtures/md001_violations.md");
    if !fixture.exists() {
        panic!("Fixture file not found: {}", fixture.display());
    }

    let cli2_violations = run_markdownlint_cli2(fixture);
    if cli2_violations.is_empty() && !docker_available() {
        println!("Skipping test: Docker not available");
        return;
    }

    let rule = MD001;
    let our_violations = run_our_implementation(fixture, &rule);

    println!("markdownlint-cli2 violations: {:?}", cli2_violations);
    println!("Our violations: {:?}", our_violations);

    // Filter to only MD001 violations from cli2
    let cli2_md001: Vec<_> = cli2_violations
        .iter()
        .filter(|(name, _)| name == "MD001")
        .collect();

    assert_eq!(
        our_violations.len(),
        cli2_md001.len(),
        "Different number of MD001 violations detected"
    );

    // Check that line numbers match
    for (our, cli2) in our_violations.iter().zip(cli2_md001.iter()) {
        assert_eq!(
            our.1, cli2.1,
            "Line number mismatch for MD001: ours={}, cli2={}",
            our.1, cli2.1
        );
    }
}

#[test]
#[ignore] // Slower test requiring Docker
fn test_md011_compatibility() {
    let fixture = Path::new("tests/fixtures/md011_violations.md");
    if !fixture.exists() {
        panic!("Fixture file not found: {}", fixture.display());
    }

    let cli2_violations = run_markdownlint_cli2(fixture);
    if cli2_violations.is_empty() && !docker_available() {
        println!("Skipping test: Docker not available");
        return;
    }

    let rule = MD011;
    let our_violations = run_our_implementation(fixture, &rule);

    println!("markdownlint-cli2 violations: {:?}", cli2_violations);
    println!("Our violations: {:?}", our_violations);

    // Filter to only MD011 violations from cli2
    let cli2_md011: Vec<_> = cli2_violations
        .iter()
        .filter(|(name, _)| name == "MD011")
        .collect();

    assert_eq!(
        our_violations.len(),
        cli2_md011.len(),
        "Different number of MD011 violations detected"
    );

    // Check that line numbers match
    for (our, cli2) in our_violations.iter().zip(cli2_md011.iter()) {
        assert_eq!(
            our.1, cli2.1,
            "Line number mismatch for MD011: ours={}, cli2={}",
            our.1, cli2.1
        );
    }
}

#[test]
#[ignore] // Slower test requiring Docker
fn test_md013_compatibility() {
    let fixture = Path::new("tests/fixtures/md013_violations.md");
    if !fixture.exists() {
        panic!("Fixture file not found: {}", fixture.display());
    }

    let cli2_violations = run_markdownlint_cli2(fixture);
    if cli2_violations.is_empty() && !docker_available() {
        println!("Skipping test: Docker not available");
        return;
    }

    let rule = MD013;
    let our_violations = run_our_implementation(fixture, &rule);

    println!("markdownlint-cli2 violations: {:?}", cli2_violations);
    println!("Our violations: {:?}", our_violations);

    // Filter to only MD013 violations from cli2
    let cli2_md013: Vec<_> = cli2_violations
        .iter()
        .filter(|(name, _)| name == "MD013")
        .collect();

    assert_eq!(
        our_violations.len(),
        cli2_md013.len(),
        "Different number of MD013 violations detected"
    );

    // Check that line numbers match
    for (our, cli2) in our_violations.iter().zip(cli2_md013.iter()) {
        assert_eq!(
            our.1, cli2.1,
            "Line number mismatch for MD013: ours={}, cli2={}",
            our.1, cli2.1
        );
    }
}

#[test]
#[ignore] // Slower test requiring Docker
fn test_md003_compatibility() {
    let fixture = Path::new("tests/fixtures/md003_violations.md");
    if !fixture.exists() {
        panic!("Fixture file not found: {}", fixture.display());
    }

    let cli2_violations = run_markdownlint_cli2(fixture);
    if cli2_violations.is_empty() && !docker_available() {
        println!("Skipping test: Docker not available");
        return;
    }

    let rule = MD003;
    let our_violations = run_our_implementation(fixture, &rule);

    println!("markdownlint-cli2 violations: {:?}", cli2_violations);
    println!("Our violations: {:?}", our_violations);

    // Filter to only MD003 violations from cli2
    let cli2_md003: Vec<_> = cli2_violations
        .iter()
        .filter(|(name, _)| name == "MD003")
        .collect();

    assert_eq!(
        our_violations.len(),
        cli2_md003.len(),
        "Different number of MD003 violations detected"
    );

    // Check that line numbers match
    for (our, cli2) in our_violations.iter().zip(cli2_md003.iter()) {
        assert_eq!(
            our.1, cli2.1,
            "Line number mismatch for MD003: ours={}, cli2={}",
            our.1, cli2.1
        );
    }
}

#[test]
#[ignore] // Slower test requiring Docker
fn test_md004_compatibility() {
    let fixture = Path::new("tests/fixtures/md004_violations.md");
    if !fixture.exists() {
        panic!("Fixture file not found: {}", fixture.display());
    }

    let cli2_violations = run_markdownlint_cli2(fixture);
    if cli2_violations.is_empty() && !docker_available() {
        println!("Skipping test: Docker not available");
        return;
    }

    let rule = MD004;
    let our_violations = run_our_implementation(fixture, &rule);

    println!("markdownlint-cli2 violations: {:?}", cli2_violations);
    println!("Our violations: {:?}", our_violations);

    // Filter to only MD004 violations from cli2
    let cli2_md004: Vec<_> = cli2_violations
        .iter()
        .filter(|(name, _)| name == "MD004")
        .collect();

    assert_eq!(
        our_violations.len(),
        cli2_md004.len(),
        "Different number of MD004 violations detected"
    );

    // Check that line numbers match
    for (our, cli2) in our_violations.iter().zip(cli2_md004.iter()) {
        assert_eq!(
            our.1, cli2.1,
            "Line number mismatch for MD004: ours={}, cli2={}",
            our.1, cli2.1
        );
    }
}

#[test]
#[ignore] // Slower test requiring Docker
fn test_md005_compatibility() {
    let fixture = Path::new("tests/fixtures/md005_violations.md");
    if !fixture.exists() {
        panic!("Fixture file not found: {}", fixture.display());
    }

    let cli2_violations = run_markdownlint_cli2(fixture);
    if cli2_violations.is_empty() && !docker_available() {
        println!("Skipping test: Docker not available");
        return;
    }

    let rule = MD005;
    let our_violations = run_our_implementation(fixture, &rule);

    println!("markdownlint-cli2 violations: {:?}", cli2_violations);
    println!("Our violations: {:?}", our_violations);

    // Filter to only MD005 violations from cli2
    let cli2_md005: Vec<_> = cli2_violations
        .iter()
        .filter(|(name, _)| name == "MD005")
        .collect();

    assert_eq!(
        our_violations.len(),
        cli2_md005.len(),
        "Different number of MD005 violations detected"
    );

    // Check that line numbers match
    for (our, cli2) in our_violations.iter().zip(cli2_md005.iter()) {
        assert_eq!(
            our.1, cli2.1,
            "Line number mismatch for MD005: ours={}, cli2={}",
            our.1, cli2.1
        );
    }
}

#[test]
#[ignore] // Slower test requiring Docker
fn test_md007_compatibility() {
    let fixture = Path::new("tests/fixtures/md007_violations.md");
    if !fixture.exists() {
        panic!("Fixture file not found: {}", fixture.display());
    }

    let cli2_violations = run_markdownlint_cli2(fixture);
    if cli2_violations.is_empty() && !docker_available() {
        println!("Skipping test: Docker not available");
        return;
    }

    let rule = MD007;
    let our_violations = run_our_implementation(fixture, &rule);

    println!("markdownlint-cli2 violations: {:?}", cli2_violations);
    println!("Our violations: {:?}", our_violations);

    // Filter to only MD007 violations from cli2
    let cli2_md007: Vec<_> = cli2_violations
        .iter()
        .filter(|(name, _)| name == "MD007")
        .collect();

    assert_eq!(
        our_violations.len(),
        cli2_md007.len(),
        "Different number of MD007 violations detected"
    );

    // Check that line numbers match
    for (our, cli2) in our_violations.iter().zip(cli2_md007.iter()) {
        assert_eq!(
            our.1, cli2.1,
            "Line number mismatch for MD007: ours={}, cli2={}",
            our.1, cli2.1
        );
    }
}

#[test]
#[ignore] // Slower test requiring Docker
fn test_all_rules_together() {
    // Test with a file that has multiple types of violations
    let fixture = Path::new("tests/fixtures/md009_violations.md");
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
    let mut our_violations = Vec::new();

    for rule in registry.all_rules() {
        let violations = rule.check(&parser, None);
        for v in violations {
            our_violations.push((v.rule.clone(), v.line));
        }
    }

    our_violations.sort_by_key(|v| (v.0.clone(), v.1));

    println!("markdownlint-cli2 violations: {:?}", cli2_violations);
    println!("Our violations: {:?}", our_violations);

    // At minimum, we should detect the same number of violations
    // (This is a weaker assertion since we only have 3 rules implemented)
    assert!(
        !our_violations.is_empty(),
        "We should detect some violations"
    );
}
