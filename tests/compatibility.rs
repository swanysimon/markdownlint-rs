// Compatibility tests with markdownlint-cli2
// These tests are slower as they require Docker to run the reference implementation
// Run with: cargo test --test compatibility -- --ignored --test-threads=1

use markdownlint_rs::lint::rules::{
    create_default_registry, MD001, MD003, MD004, MD005, MD006, MD007, MD009, MD010, MD011, MD012,
    MD013, MD014, MD018, MD019, MD020, MD021, MD022, MD023, MD024, MD025, MD026, MD027, MD028,
    MD029, MD030, MD031, MD032, MD033, MD034, MD035, MD036, MD037, MD038, MD039, MD040, MD041,
    MD042, MD043, MD044, MD045, MD046, MD047, MD048, MD049, MD050, MD051, MD052, MD053, MD054,
    MD055, MD056, MD058, MD059, MD060,
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
                    .trim()
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

/// Run our implementation and extract violations
fn run_our_implementation(file_path: &Path, rule: &dyn Rule) -> Vec<(String, usize)> {
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

#[test]
#[ignore] // Slower test requiring Docker
fn test_md018_compatibility() {
    let fixture = Path::new("tests/fixtures/md018_violations.md");
    if !fixture.exists() {
        panic!("Fixture file not found: {}", fixture.display());
    }

    let cli2_violations = run_markdownlint_cli2(fixture);
    if cli2_violations.is_empty() && !docker_available() {
        println!("Skipping test: Docker not available");
        return;
    }

    let rule = MD018;
    let our_violations = run_our_implementation(fixture, &rule);

    println!("markdownlint-cli2 violations: {:?}", cli2_violations);
    println!("Our violations: {:?}", our_violations);

    let cli2_md018: Vec<_> = cli2_violations
        .iter()
        .filter(|(name, _)| name == "MD018")
        .collect();

    assert_eq!(
        our_violations.len(),
        cli2_md018.len(),
        "Different number of MD018 violations detected"
    );

    for (our, cli2) in our_violations.iter().zip(cli2_md018.iter()) {
        assert_eq!(
            our.1, cli2.1,
            "Line number mismatch for MD018: ours={}, cli2={}",
            our.1, cli2.1
        );
    }
}

#[test]
#[ignore] // Slower test requiring Docker
fn test_md019_compatibility() {
    let fixture = Path::new("tests/fixtures/md019_violations.md");
    if !fixture.exists() {
        panic!("Fixture file not found: {}", fixture.display());
    }

    let cli2_violations = run_markdownlint_cli2(fixture);
    if cli2_violations.is_empty() && !docker_available() {
        println!("Skipping test: Docker not available");
        return;
    }

    let rule = MD019;
    let our_violations = run_our_implementation(fixture, &rule);

    println!("markdownlint-cli2 violations: {:?}", cli2_violations);
    println!("Our violations: {:?}", our_violations);

    let cli2_md019: Vec<_> = cli2_violations
        .iter()
        .filter(|(name, _)| name == "MD019")
        .collect();

    assert_eq!(
        our_violations.len(),
        cli2_md019.len(),
        "Different number of MD019 violations detected"
    );

    for (our, cli2) in our_violations.iter().zip(cli2_md019.iter()) {
        assert_eq!(
            our.1, cli2.1,
            "Line number mismatch for MD019: ours={}, cli2={}",
            our.1, cli2.1
        );
    }
}

#[test]
#[ignore] // Slower test requiring Docker
fn test_md022_compatibility() {
    let fixture = Path::new("tests/fixtures/md022_violations.md");
    if !fixture.exists() {
        panic!("Fixture file not found: {}", fixture.display());
    }

    let cli2_violations = run_markdownlint_cli2(fixture);
    if cli2_violations.is_empty() && !docker_available() {
        println!("Skipping test: Docker not available");
        return;
    }

    let rule = MD022;
    let our_violations = run_our_implementation(fixture, &rule);

    println!("markdownlint-cli2 violations: {:?}", cli2_violations);
    println!("Our violations: {:?}", our_violations);

    let cli2_md022: Vec<_> = cli2_violations
        .iter()
        .filter(|(name, _)| name == "MD022")
        .collect();

    assert_eq!(
        our_violations.len(),
        cli2_md022.len(),
        "Different number of MD022 violations detected"
    );

    for (our, cli2) in our_violations.iter().zip(cli2_md022.iter()) {
        assert_eq!(
            our.1, cli2.1,
            "Line number mismatch for MD022: ours={}, cli2={}",
            our.1, cli2.1
        );
    }
}

#[test]
#[ignore] // Slower test requiring Docker
fn test_md023_compatibility() {
    let fixture = Path::new("tests/fixtures/md023_violations.md");
    if !fixture.exists() {
        panic!("Fixture file not found: {}", fixture.display());
    }

    let cli2_violations = run_markdownlint_cli2(fixture);
    if cli2_violations.is_empty() && !docker_available() {
        println!("Skipping test: Docker not available");
        return;
    }

    let rule = MD023;
    let our_violations = run_our_implementation(fixture, &rule);

    println!("markdownlint-cli2 violations: {:?}", cli2_violations);
    println!("Our violations: {:?}", our_violations);

    let cli2_md023: Vec<_> = cli2_violations
        .iter()
        .filter(|(name, _)| name == "MD023")
        .collect();

    assert_eq!(
        our_violations.len(),
        cli2_md023.len(),
        "Different number of MD023 violations detected"
    );

    for (our, cli2) in our_violations.iter().zip(cli2_md023.iter()) {
        assert_eq!(
            our.1, cli2.1,
            "Line number mismatch for MD023: ours={}, cli2={}",
            our.1, cli2.1
        );
    }
}

#[test]
#[ignore] // Slower test requiring Docker
fn test_md025_compatibility() {
    let fixture = Path::new("tests/fixtures/md025_violations.md");
    if !fixture.exists() {
        panic!("Fixture file not found: {}", fixture.display());
    }

    let cli2_violations = run_markdownlint_cli2(fixture);
    if cli2_violations.is_empty() && !docker_available() {
        println!("Skipping test: Docker not available");
        return;
    }

    let rule = MD025;
    let our_violations = run_our_implementation(fixture, &rule);

    println!("markdownlint-cli2 violations: {:?}", cli2_violations);
    println!("Our violations: {:?}", our_violations);

    let cli2_md025: Vec<_> = cli2_violations
        .iter()
        .filter(|(name, _)| name == "MD025")
        .collect();

    assert_eq!(
        our_violations.len(),
        cli2_md025.len(),
        "Different number of MD025 violations detected"
    );

    for (our, cli2) in our_violations.iter().zip(cli2_md025.iter()) {
        assert_eq!(
            our.1, cli2.1,
            "Line number mismatch for MD025: ours={}, cli2={}",
            our.1, cli2.1
        );
    }
}

#[test]
#[ignore] // Slower test requiring Docker
fn test_md027_compatibility() {
    let fixture = Path::new("tests/fixtures/md027_violations.md");
    if !fixture.exists() {
        panic!("Fixture file not found: {}", fixture.display());
    }

    let cli2_violations = run_markdownlint_cli2(fixture);
    if cli2_violations.is_empty() && !docker_available() {
        println!("Skipping test: Docker not available");
        return;
    }

    let rule = MD027;
    let our_violations = run_our_implementation(fixture, &rule);

    println!("markdownlint-cli2 violations: {:?}", cli2_violations);
    println!("Our violations: {:?}", our_violations);

    let cli2_md027: Vec<_> = cli2_violations
        .iter()
        .filter(|(name, _)| name == "MD027")
        .collect();

    assert_eq!(
        our_violations.len(),
        cli2_md027.len(),
        "Different number of MD027 violations detected"
    );

    for (our, cli2) in our_violations.iter().zip(cli2_md027.iter()) {
        assert_eq!(
            our.1, cli2.1,
            "Line number mismatch for MD027: ours={}, cli2={}",
            our.1, cli2.1
        );
    }
}

#[test]
#[ignore] // Slower test requiring Docker
fn test_md028_compatibility() {
    let fixture = Path::new("tests/fixtures/md028_violations.md");
    if !fixture.exists() {
        panic!("Fixture file not found: {}", fixture.display());
    }

    let cli2_violations = run_markdownlint_cli2(fixture);
    if cli2_violations.is_empty() && !docker_available() {
        println!("Skipping test: Docker not available");
        return;
    }

    let rule = MD028;
    let our_violations = run_our_implementation(fixture, &rule);

    println!("markdownlint-cli2 violations: {:?}", cli2_violations);
    println!("Our violations: {:?}", our_violations);

    let cli2_md028: Vec<_> = cli2_violations
        .iter()
        .filter(|(name, _)| name == "MD028")
        .collect();

    assert_eq!(
        our_violations.len(),
        cli2_md028.len(),
        "Different number of MD028 violations detected"
    );

    for (our, cli2) in our_violations.iter().zip(cli2_md028.iter()) {
        assert_eq!(
            our.1, cli2.1,
            "Line number mismatch for MD028: ours={}, cli2={}",
            our.1, cli2.1
        );
    }
}

#[test]
#[ignore] // Slower test requiring Docker
fn test_md029_compatibility() {
    let fixture = Path::new("tests/fixtures/md029_violations.md");
    if !fixture.exists() {
        panic!("Fixture file not found: {}", fixture.display());
    }

    let cli2_violations = run_markdownlint_cli2(fixture);
    if cli2_violations.is_empty() && !docker_available() {
        println!("Skipping test: Docker not available");
        return;
    }

    let rule = MD029;
    let our_violations = run_our_implementation(fixture, &rule);

    println!("markdownlint-cli2 violations: {:?}", cli2_violations);
    println!("Our violations: {:?}", our_violations);

    let cli2_md029: Vec<_> = cli2_violations
        .iter()
        .filter(|(name, _)| name == "MD029")
        .collect();

    assert_eq!(
        our_violations.len(),
        cli2_md029.len(),
        "Different number of MD029 violations detected"
    );

    for (our, cli2) in our_violations.iter().zip(cli2_md029.iter()) {
        assert_eq!(
            our.1, cli2.1,
            "Line number mismatch for MD029: ours={}, cli2={}",
            our.1, cli2.1
        );
    }
}

#[test]
#[ignore] // Slower test requiring Docker
fn test_md030_compatibility() {
    let fixture = Path::new("tests/fixtures/md030_violations.md");
    if !fixture.exists() {
        panic!("Fixture file not found: {}", fixture.display());
    }

    let cli2_violations = run_markdownlint_cli2(fixture);
    if cli2_violations.is_empty() && !docker_available() {
        println!("Skipping test: Docker not available");
        return;
    }

    let rule = MD030;
    let our_violations = run_our_implementation(fixture, &rule);

    println!("markdownlint-cli2 violations: {:?}", cli2_violations);
    println!("Our violations: {:?}", our_violations);

    let cli2_md030: Vec<_> = cli2_violations
        .iter()
        .filter(|(name, _)| name == "MD030")
        .collect();

    assert_eq!(
        our_violations.len(),
        cli2_md030.len(),
        "Different number of MD030 violations detected"
    );

    for (our, cli2) in our_violations.iter().zip(cli2_md030.iter()) {
        assert_eq!(
            our.1, cli2.1,
            "Line number mismatch for MD030: ours={}, cli2={}",
            our.1, cli2.1
        );
    }
}

#[test]
#[ignore] // Slower test requiring Docker
fn test_md031_compatibility() {
    let fixture = Path::new("tests/fixtures/md031_violations.md");
    if !fixture.exists() {
        panic!("Fixture file not found: {}", fixture.display());
    }

    let cli2_violations = run_markdownlint_cli2(fixture);
    if cli2_violations.is_empty() && !docker_available() {
        println!("Skipping test: Docker not available");
        return;
    }

    let rule = MD031;
    let our_violations = run_our_implementation(fixture, &rule);

    println!("markdownlint-cli2 violations: {:?}", cli2_violations);
    println!("Our violations: {:?}", our_violations);

    let cli2_md031: Vec<_> = cli2_violations
        .iter()
        .filter(|(name, _)| name == "MD031")
        .collect();

    assert_eq!(
        our_violations.len(),
        cli2_md031.len(),
        "Different number of MD031 violations detected"
    );

    for (our, cli2) in our_violations.iter().zip(cli2_md031.iter()) {
        assert_eq!(
            our.1, cli2.1,
            "Line number mismatch for MD031: ours={}, cli2={}",
            our.1, cli2.1
        );
    }
}

#[test]
#[ignore] // Slower test requiring Docker
fn test_md014_compatibility() {
    let fixture = Path::new("tests/fixtures/md014_violations.md");
    if !fixture.exists() {
        panic!("Fixture file not found: {}", fixture.display());
    }

    let cli2_violations = run_markdownlint_cli2(fixture);
    if cli2_violations.is_empty() && !docker_available() {
        println!("Skipping test: Docker not available");
        return;
    }

    let rule = MD014;
    let our_violations = run_our_implementation(fixture, &rule);

    println!("markdownlint-cli2 violations: {:?}", cli2_violations);
    println!("Our violations: {:?}", our_violations);

    let cli2_md014: Vec<_> = cli2_violations
        .iter()
        .filter(|(name, _)| name == "MD014")
        .collect();

    assert_eq!(
        our_violations.len(),
        cli2_md014.len(),
        "Different number of MD014 violations detected"
    );

    for (our, cli2) in our_violations.iter().zip(cli2_md014.iter()) {
        assert_eq!(
            our.1, cli2.1,
            "Line number mismatch for MD014: ours={}, cli2={}",
            our.1, cli2.1
        );
    }
}

#[test]
#[ignore] // Slower test requiring Docker
fn test_md024_compatibility() {
    let fixture = Path::new("tests/fixtures/md024_violations.md");
    if !fixture.exists() {
        panic!("Fixture file not found: {}", fixture.display());
    }

    let cli2_violations = run_markdownlint_cli2(fixture);
    if cli2_violations.is_empty() && !docker_available() {
        println!("Skipping test: Docker not available");
        return;
    }

    let rule = MD024;
    let our_violations = run_our_implementation(fixture, &rule);

    println!("markdownlint-cli2 violations: {:?}", cli2_violations);
    println!("Our violations: {:?}", our_violations);

    let cli2_md024: Vec<_> = cli2_violations
        .iter()
        .filter(|(name, _)| name == "MD024")
        .collect();

    assert_eq!(
        our_violations.len(),
        cli2_md024.len(),
        "Different number of MD024 violations detected"
    );

    for (our, cli2) in our_violations.iter().zip(cli2_md024.iter()) {
        assert_eq!(
            our.1, cli2.1,
            "Line number mismatch for MD024: ours={}, cli2={}",
            our.1, cli2.1
        );
    }
}

#[test]
#[ignore] // Slower test requiring Docker
fn test_md026_compatibility() {
    let fixture = Path::new("tests/fixtures/md026_violations.md");
    if !fixture.exists() {
        panic!("Fixture file not found: {}", fixture.display());
    }

    let cli2_violations = run_markdownlint_cli2(fixture);
    if cli2_violations.is_empty() && !docker_available() {
        println!("Skipping test: Docker not available");
        return;
    }

    let rule = MD026;
    let our_violations = run_our_implementation(fixture, &rule);

    println!("markdownlint-cli2 violations: {:?}", cli2_violations);
    println!("Our violations: {:?}", our_violations);

    let cli2_md026: Vec<_> = cli2_violations
        .iter()
        .filter(|(name, _)| name == "MD026")
        .collect();

    assert_eq!(
        our_violations.len(),
        cli2_md026.len(),
        "Different number of MD026 violations detected"
    );

    for (our, cli2) in our_violations.iter().zip(cli2_md026.iter()) {
        assert_eq!(
            our.1, cli2.1,
            "Line number mismatch for MD026: ours={}, cli2={}",
            our.1, cli2.1
        );
    }
}

#[test]
#[ignore] // Slower test requiring Docker
fn test_md032_compatibility() {
    let fixture = Path::new("tests/fixtures/md032_violations.md");
    if !fixture.exists() {
        panic!("Fixture file not found: {}", fixture.display());
    }

    let cli2_violations = run_markdownlint_cli2(fixture);
    if cli2_violations.is_empty() && !docker_available() {
        println!("Skipping test: Docker not available");
        return;
    }

    let rule = MD032;
    let our_violations = run_our_implementation(fixture, &rule);

    println!("markdownlint-cli2 violations: {:?}", cli2_violations);
    println!("Our violations: {:?}", our_violations);

    let cli2_md032: Vec<_> = cli2_violations
        .iter()
        .filter(|(name, _)| name == "MD032")
        .collect();

    assert_eq!(
        our_violations.len(),
        cli2_md032.len(),
        "Different number of MD032 violations detected"
    );

    for (our, cli2) in our_violations.iter().zip(cli2_md032.iter()) {
        assert_eq!(
            our.1, cli2.1,
            "Line number mismatch for MD032: ours={}, cli2={}",
            our.1, cli2.1
        );
    }
}

#[test]
#[ignore] // Slower test requiring Docker
fn test_md040_compatibility() {
    let fixture = Path::new("tests/fixtures/md040_violations.md");
    if !fixture.exists() {
        panic!("Fixture file not found: {}", fixture.display());
    }

    let cli2_violations = run_markdownlint_cli2(fixture);
    if cli2_violations.is_empty() && !docker_available() {
        println!("Skipping test: Docker not available");
        return;
    }

    let rule = MD040;
    let our_violations = run_our_implementation(fixture, &rule);

    println!("markdownlint-cli2 violations: {:?}", cli2_violations);
    println!("Our violations: {:?}", our_violations);

    let cli2_md040: Vec<_> = cli2_violations
        .iter()
        .filter(|(name, _)| name == "MD040")
        .collect();

    assert_eq!(
        our_violations.len(),
        cli2_md040.len(),
        "Different number of MD040 violations detected"
    );

    for (our, cli2) in our_violations.iter().zip(cli2_md040.iter()) {
        assert_eq!(
            our.1, cli2.1,
            "Line number mismatch for MD040: ours={}, cli2={}",
            our.1, cli2.1
        );
    }
}

#[test]
#[ignore] // Slower test requiring Docker
fn test_md033_compatibility() {
    let fixture = Path::new("tests/fixtures/md033_violations.md");
    if !fixture.exists() {
        panic!("Fixture file not found: {}", fixture.display());
    }

    let cli2_violations = run_markdownlint_cli2(fixture);
    if cli2_violations.is_empty() && !docker_available() {
        println!("Skipping test: Docker not available");
        return;
    }

    let rule = MD033;
    let our_violations = run_our_implementation(fixture, &rule);

    println!("markdownlint-cli2 violations: {:?}", cli2_violations);
    println!("Our violations: {:?}", our_violations);

    let cli2_md033: Vec<_> = cli2_violations
        .iter()
        .filter(|(name, _)| name == "MD033")
        .collect();

    assert_eq!(
        our_violations.len(),
        cli2_md033.len(),
        "Different number of MD033 violations detected"
    );

    for (our, cli2) in our_violations.iter().zip(cli2_md033.iter()) {
        assert_eq!(
            our.1, cli2.1,
            "Line number mismatch for MD033: ours={}, cli2={}",
            our.1, cli2.1
        );
    }
}

#[test]
#[ignore] // Slower test requiring Docker
fn test_md034_compatibility() {
    let fixture = Path::new("tests/fixtures/md034_violations.md");
    if !fixture.exists() {
        panic!("Fixture file not found: {}", fixture.display());
    }

    let cli2_violations = run_markdownlint_cli2(fixture);
    if cli2_violations.is_empty() && !docker_available() {
        println!("Skipping test: Docker not available");
        return;
    }

    let rule = MD034;
    let our_violations = run_our_implementation(fixture, &rule);

    println!("markdownlint-cli2 violations: {:?}", cli2_violations);
    println!("Our violations: {:?}", our_violations);

    let cli2_md034: Vec<_> = cli2_violations
        .iter()
        .filter(|(name, _)| name == "MD034")
        .collect();

    assert_eq!(
        our_violations.len(),
        cli2_md034.len(),
        "Different number of MD034 violations detected"
    );

    for (our, cli2) in our_violations.iter().zip(cli2_md034.iter()) {
        assert_eq!(
            our.1, cli2.1,
            "Line number mismatch for MD034: ours={}, cli2={}",
            our.1, cli2.1
        );
    }
}

#[test]
#[ignore] // Slower test requiring Docker
fn test_md036_compatibility() {
    let fixture = Path::new("tests/fixtures/md036_violations.md");
    if !fixture.exists() {
        panic!("Fixture file not found: {}", fixture.display());
    }

    let cli2_violations = run_markdownlint_cli2(fixture);
    if cli2_violations.is_empty() && !docker_available() {
        println!("Skipping test: Docker not available");
        return;
    }

    let rule = MD036;
    let our_violations = run_our_implementation(fixture, &rule);

    println!("markdownlint-cli2 violations: {:?}", cli2_violations);
    println!("Our violations: {:?}", our_violations);

    let cli2_md036: Vec<_> = cli2_violations
        .iter()
        .filter(|(name, _)| name == "MD036")
        .collect();

    assert_eq!(
        our_violations.len(),
        cli2_md036.len(),
        "Different number of MD036 violations detected"
    );

    for (our, cli2) in our_violations.iter().zip(cli2_md036.iter()) {
        assert_eq!(
            our.1, cli2.1,
            "Line number mismatch for MD036: ours={}, cli2={}",
            our.1, cli2.1
        );
    }
}

#[test]
#[ignore] // Slower test requiring Docker
fn test_md037_compatibility() {
    let fixture = Path::new("tests/fixtures/md037_violations.md");
    if !fixture.exists() {
        panic!("Fixture file not found: {}", fixture.display());
    }

    let cli2_violations = run_markdownlint_cli2(fixture);
    if cli2_violations.is_empty() && !docker_available() {
        println!("Skipping test: Docker not available");
        return;
    }

    let rule = MD037;
    let our_violations = run_our_implementation(fixture, &rule);

    println!("markdownlint-cli2 violations: {:?}", cli2_violations);
    println!("Our violations: {:?}", our_violations);

    let cli2_md037: Vec<_> = cli2_violations
        .iter()
        .filter(|(name, _)| name == "MD037")
        .collect();

    assert_eq!(
        our_violations.len(),
        cli2_md037.len(),
        "Different number of MD037 violations detected"
    );

    for (our, cli2) in our_violations.iter().zip(cli2_md037.iter()) {
        assert_eq!(
            our.1, cli2.1,
            "Line number mismatch for MD037: ours={}, cli2={}",
            our.1, cli2.1
        );
    }
}

#[test]
#[ignore] // Slower test requiring Docker
fn test_md038_compatibility() {
    let fixture = Path::new("tests/fixtures/md038_violations.md");
    if !fixture.exists() {
        panic!("Fixture file not found: {}", fixture.display());
    }

    let cli2_violations = run_markdownlint_cli2(fixture);
    if cli2_violations.is_empty() && !docker_available() {
        println!("Skipping test: Docker not available");
        return;
    }

    let rule = MD038;
    let our_violations = run_our_implementation(fixture, &rule);

    println!("markdownlint-cli2 violations: {:?}", cli2_violations);
    println!("Our violations: {:?}", our_violations);

    let cli2_md038: Vec<_> = cli2_violations
        .iter()
        .filter(|(name, _)| name == "MD038")
        .collect();

    assert_eq!(
        our_violations.len(),
        cli2_md038.len(),
        "Different number of MD038 violations detected"
    );

    for (our, cli2) in our_violations.iter().zip(cli2_md038.iter()) {
        assert_eq!(
            our.1, cli2.1,
            "Line number mismatch for MD038: ours={}, cli2={}",
            our.1, cli2.1
        );
    }
}

#[test]
#[ignore] // Slower test requiring Docker
fn test_md020_compatibility() {
    let fixture = Path::new("tests/fixtures/md020_violations.md");
    if !fixture.exists() {
        panic!("Fixture file not found: {}", fixture.display());
    }

    let cli2_violations = run_markdownlint_cli2(fixture);
    if cli2_violations.is_empty() && !docker_available() {
        println!("Skipping test: Docker not available");
        return;
    }

    let rule = MD020;
    let our_violations = run_our_implementation(fixture, &rule);

    println!("markdownlint-cli2 violations: {:?}", cli2_violations);
    println!("Our violations: {:?}", our_violations);

    let cli2_md020: Vec<_> = cli2_violations
        .iter()
        .filter(|(name, _)| name == "MD020")
        .collect();

    assert_eq!(
        our_violations.len(),
        cli2_md020.len(),
        "Different number of MD020 violations detected"
    );

    for (our, cli2) in our_violations.iter().zip(cli2_md020.iter()) {
        assert_eq!(
            our.1, cli2.1,
            "Line number mismatch for MD020: ours={}, cli2={}",
            our.1, cli2.1
        );
    }
}

#[test]
#[ignore] // Slower test requiring Docker
fn test_md021_compatibility() {
    let fixture = Path::new("tests/fixtures/md021_violations.md");
    if !fixture.exists() {
        panic!("Fixture file not found: {}", fixture.display());
    }

    let cli2_violations = run_markdownlint_cli2(fixture);
    if cli2_violations.is_empty() && !docker_available() {
        println!("Skipping test: Docker not available");
        return;
    }

    let rule = MD021;
    let our_violations = run_our_implementation(fixture, &rule);

    println!("markdownlint-cli2 violations: {:?}", cli2_violations);
    println!("Our violations: {:?}", our_violations);

    let cli2_md021: Vec<_> = cli2_violations
        .iter()
        .filter(|(name, _)| name == "MD021")
        .collect();

    assert_eq!(
        our_violations.len(),
        cli2_md021.len(),
        "Different number of MD021 violations detected"
    );

    for (our, cli2) in our_violations.iter().zip(cli2_md021.iter()) {
        assert_eq!(
            our.1, cli2.1,
            "Line number mismatch for MD021: ours={}, cli2={}",
            our.1, cli2.1
        );
    }
}

#[test]
#[ignore] // Slower test requiring Docker
fn test_md041_compatibility() {
    let fixture = Path::new("tests/fixtures/md041_violations.md");
    if !fixture.exists() {
        panic!("Fixture file not found: {}", fixture.display());
    }

    let cli2_violations = run_markdownlint_cli2(fixture);
    if cli2_violations.is_empty() && !docker_available() {
        println!("Skipping test: Docker not available");
        return;
    }

    let rule = MD041;
    let our_violations = run_our_implementation(fixture, &rule);

    println!("markdownlint-cli2 violations: {:?}", cli2_violations);
    println!("Our violations: {:?}", our_violations);

    let cli2_md041: Vec<_> = cli2_violations
        .iter()
        .filter(|(name, _)| name == "MD041")
        .collect();

    assert_eq!(
        our_violations.len(),
        cli2_md041.len(),
        "Different number of MD041 violations detected"
    );

    for (our, cli2) in our_violations.iter().zip(cli2_md041.iter()) {
        assert_eq!(
            our.1, cli2.1,
            "Line number mismatch for MD041: ours={}, cli2={}",
            our.1, cli2.1
        );
    }
}

#[test]
#[ignore] // Slower test requiring Docker
fn test_md045_compatibility() {
    let fixture = Path::new("tests/fixtures/md045_violations.md");
    if !fixture.exists() {
        panic!("Fixture file not found: {}", fixture.display());
    }

    let cli2_violations = run_markdownlint_cli2(fixture);
    if cli2_violations.is_empty() && !docker_available() {
        println!("Skipping test: Docker not available");
        return;
    }

    let rule = MD045;
    let our_violations = run_our_implementation(fixture, &rule);

    println!("markdownlint-cli2 violations: {:?}", cli2_violations);
    println!("Our violations: {:?}", our_violations);

    let cli2_md045: Vec<_> = cli2_violations
        .iter()
        .filter(|(name, _)| name == "MD045")
        .collect();

    assert_eq!(
        our_violations.len(),
        cli2_md045.len(),
        "Different number of MD045 violations detected"
    );

    for (our, cli2) in our_violations.iter().zip(cli2_md045.iter()) {
        assert_eq!(
            our.1, cli2.1,
            "Line number mismatch for MD045: ours={}, cli2={}",
            our.1, cli2.1
        );
    }
}

#[test]
#[ignore] // Slower test requiring Docker
fn test_md047_compatibility() {
    let fixture = Path::new("tests/fixtures/md047_violations.md");
    if !fixture.exists() {
        panic!("Fixture file not found: {}", fixture.display());
    }

    let cli2_violations = run_markdownlint_cli2(fixture);
    if cli2_violations.is_empty() && !docker_available() {
        println!("Skipping test: Docker not available");
        return;
    }

    let rule = MD047;
    let our_violations = run_our_implementation(fixture, &rule);

    println!("markdownlint-cli2 violations: {:?}", cli2_violations);
    println!("Our violations: {:?}", our_violations);

    let cli2_md047: Vec<_> = cli2_violations
        .iter()
        .filter(|(name, _)| name == "MD047")
        .collect();

    assert_eq!(
        our_violations.len(),
        cli2_md047.len(),
        "Different number of MD047 violations detected"
    );

    for (our, cli2) in our_violations.iter().zip(cli2_md047.iter()) {
        assert_eq!(
            our.1, cli2.1,
            "Line number mismatch for MD047: ours={}, cli2={}",
            our.1, cli2.1
        );
    }
}

#[test]
#[ignore] // Slower test requiring Docker
fn test_md035_compatibility() {
    let fixture = Path::new("tests/fixtures/md035_violations.md");
    if !fixture.exists() {
        panic!("Fixture file not found: {}", fixture.display());
    }

    let cli2_violations = run_markdownlint_cli2(fixture);
    if cli2_violations.is_empty() && !docker_available() {
        println!("Skipping test: Docker not available");
        return;
    }

    let rule = MD035;
    let our_violations = run_our_implementation(fixture, &rule);

    println!("markdownlint-cli2 violations: {:?}", cli2_violations);
    println!("Our violations: {:?}", our_violations);

    let cli2_md035: Vec<_> = cli2_violations
        .iter()
        .filter(|(name, _)| name == "MD035")
        .collect();

    assert_eq!(
        our_violations.len(),
        cli2_md035.len(),
        "Different number of MD035 violations detected"
    );

    for (our, cli2) in our_violations.iter().zip(cli2_md035.iter()) {
        assert_eq!(
            our.1, cli2.1,
            "Line number mismatch for MD035: ours={}, cli2={}",
            our.1, cli2.1
        );
    }
}

#[test]
#[ignore] // Slower test requiring Docker
fn test_md039_compatibility() {
    let fixture = Path::new("tests/fixtures/md039_violations.md");
    if !fixture.exists() {
        panic!("Fixture file not found: {}", fixture.display());
    }

    let cli2_violations = run_markdownlint_cli2(fixture);
    if cli2_violations.is_empty() && !docker_available() {
        println!("Skipping test: Docker not available");
        return;
    }

    let rule = MD039;
    let our_violations = run_our_implementation(fixture, &rule);

    println!("markdownlint-cli2 violations: {:?}", cli2_violations);
    println!("Our violations: {:?}", our_violations);

    let cli2_md039: Vec<_> = cli2_violations
        .iter()
        .filter(|(name, _)| name == "MD039")
        .collect();

    assert_eq!(
        our_violations.len(),
        cli2_md039.len(),
        "Different number of MD039 violations detected"
    );

    for (our, cli2) in our_violations.iter().zip(cli2_md039.iter()) {
        assert_eq!(
            our.1, cli2.1,
            "Line number mismatch for MD039: ours={}, cli2={}",
            our.1, cli2.1
        );
    }
}

#[test]
#[ignore] // Slower test requiring Docker
fn test_md042_compatibility() {
    let fixture = Path::new("tests/fixtures/md042_violations.md");
    if !fixture.exists() {
        panic!("Fixture file not found: {}", fixture.display());
    }

    let cli2_violations = run_markdownlint_cli2(fixture);
    if cli2_violations.is_empty() && !docker_available() {
        println!("Skipping test: Docker not available");
        return;
    }

    let rule = MD042;
    let our_violations = run_our_implementation(fixture, &rule);

    println!("markdownlint-cli2 violations: {:?}", cli2_violations);
    println!("Our violations: {:?}", our_violations);

    let cli2_md042: Vec<_> = cli2_violations
        .iter()
        .filter(|(name, _)| name == "MD042")
        .collect();

    assert_eq!(
        our_violations.len(),
        cli2_md042.len(),
        "Different number of MD042 violations detected"
    );

    for (our, cli2) in our_violations.iter().zip(cli2_md042.iter()) {
        assert_eq!(
            our.1, cli2.1,
            "Line number mismatch for MD042: ours={}, cli2={}",
            our.1, cli2.1
        );
    }
}

#[test]
#[ignore] // Slower test requiring Docker
fn test_md046_compatibility() {
    let fixture = Path::new("tests/fixtures/md046_violations.md");
    if !fixture.exists() {
        panic!("Fixture file not found: {}", fixture.display());
    }

    let cli2_violations = run_markdownlint_cli2(fixture);
    if cli2_violations.is_empty() && !docker_available() {
        println!("Skipping test: Docker not available");
        return;
    }

    let rule = MD046;
    let our_violations = run_our_implementation(fixture, &rule);

    println!("markdownlint-cli2 violations: {:?}", cli2_violations);
    println!("Our violations: {:?}", our_violations);

    let cli2_md046: Vec<_> = cli2_violations
        .iter()
        .filter(|(name, _)| name == "MD046")
        .collect();

    assert_eq!(
        our_violations.len(),
        cli2_md046.len(),
        "Different number of MD046 violations detected"
    );

    for (our, cli2) in our_violations.iter().zip(cli2_md046.iter()) {
        assert_eq!(
            our.1, cli2.1,
            "Line number mismatch for MD046: ours={}, cli2={}",
            our.1, cli2.1
        );
    }
}

#[test]
#[ignore] // Slower test requiring Docker
fn test_md049_compatibility() {
    let fixture = Path::new("tests/fixtures/md049_violations.md");
    if !fixture.exists() {
        panic!("Fixture file not found: {}", fixture.display());
    }

    let cli2_violations = run_markdownlint_cli2(fixture);
    if cli2_violations.is_empty() && !docker_available() {
        println!("Skipping test: Docker not available");
        return;
    }

    let rule = MD049;
    let our_violations = run_our_implementation(fixture, &rule);

    println!("markdownlint-cli2 violations: {:?}", cli2_violations);
    println!("Our violations: {:?}", our_violations);

    let cli2_md049: Vec<_> = cli2_violations
        .iter()
        .filter(|(name, _)| name == "MD049")
        .collect();

    assert_eq!(
        our_violations.len(),
        cli2_md049.len(),
        "Different number of MD049 violations detected"
    );

    for (our, cli2) in our_violations.iter().zip(cli2_md049.iter()) {
        assert_eq!(
            our.1, cli2.1,
            "Line number mismatch for MD049: ours={}, cli2={}",
            our.1, cli2.1
        );
    }
}

#[test]
#[ignore] // Slower test requiring Docker
fn test_md006_compatibility() {
    let fixture = Path::new("tests/fixtures/md006.md");
    if !fixture.exists() {
        panic!("Fixture file not found: {}", fixture.display());
    }

    let cli2_violations = run_markdownlint_cli2(fixture);
    if cli2_violations.is_empty() && !docker_available() {
        println!("Skipping test: Docker not available");
        return;
    }

    let rule = MD006;
    let our_violations = run_our_implementation(fixture, &rule);

    println!("markdownlint-cli2 violations: {:?}", cli2_violations);
    println!("Our violations: {:?}", our_violations);

    let cli2_md006: Vec<_> = cli2_violations
        .iter()
        .filter(|(name, _)| name == "MD006")
        .collect();

    assert_eq!(
        our_violations.len(),
        cli2_md006.len(),
        "Different number of MD006 violations detected"
    );

    for (our, cli2) in our_violations.iter().zip(cli2_md006.iter()) {
        assert_eq!(
            our.1, cli2.1,
            "Line number mismatch for MD006: ours={}, cli2={}",
            our.1, cli2.1
        );
    }
}

#[test]
#[ignore] // Slower test requiring Docker
fn test_md043_compatibility() {
    let fixture = Path::new("tests/fixtures/md043.md");
    if !fixture.exists() {
        panic!("Fixture file not found: {}", fixture.display());
    }

    let cli2_violations = run_markdownlint_cli2(fixture);
    if cli2_violations.is_empty() && !docker_available() {
        println!("Skipping test: Docker not available");
        return;
    }

    let rule = MD043;
    let our_violations = run_our_implementation(fixture, &rule);

    println!("markdownlint-cli2 violations: {:?}", cli2_violations);
    println!("Our violations: {:?}", our_violations);

    let cli2_md043: Vec<_> = cli2_violations
        .iter()
        .filter(|(name, _)| name == "MD043")
        .collect();

    assert_eq!(
        our_violations.len(),
        cli2_md043.len(),
        "Different number of MD043 violations detected"
    );

    for (our, cli2) in our_violations.iter().zip(cli2_md043.iter()) {
        assert_eq!(
            our.1, cli2.1,
            "Line number mismatch for MD043: ours={}, cli2={}",
            our.1, cli2.1
        );
    }
}

#[test]
#[ignore] // Slower test requiring Docker
fn test_md044_compatibility() {
    let fixture = Path::new("tests/fixtures/md044.md");
    if !fixture.exists() {
        panic!("Fixture file not found: {}", fixture.display());
    }

    let cli2_violations = run_markdownlint_cli2(fixture);
    if cli2_violations.is_empty() && !docker_available() {
        println!("Skipping test: Docker not available");
        return;
    }

    let rule = MD044;
    let our_violations = run_our_implementation(fixture, &rule);

    println!("markdownlint-cli2 violations: {:?}", cli2_violations);
    println!("Our violations: {:?}", our_violations);

    let cli2_md044: Vec<_> = cli2_violations
        .iter()
        .filter(|(name, _)| name == "MD044")
        .collect();

    assert_eq!(
        our_violations.len(),
        cli2_md044.len(),
        "Different number of MD044 violations detected"
    );

    for (our, cli2) in our_violations.iter().zip(cli2_md044.iter()) {
        assert_eq!(
            our.1, cli2.1,
            "Line number mismatch for MD044: ours={}, cli2={}",
            our.1, cli2.1
        );
    }
}

#[test]
#[ignore] // Slower test requiring Docker
fn test_md048_compatibility() {
    let fixture = Path::new("tests/fixtures/md048.md");
    if !fixture.exists() {
        panic!("Fixture file not found: {}", fixture.display());
    }

    let cli2_violations = run_markdownlint_cli2(fixture);
    if cli2_violations.is_empty() && !docker_available() {
        println!("Skipping test: Docker not available");
        return;
    }

    let rule = MD048;
    let our_violations = run_our_implementation(fixture, &rule);

    println!("markdownlint-cli2 violations: {:?}", cli2_violations);
    println!("Our violations: {:?}", our_violations);

    let cli2_md048: Vec<_> = cli2_violations
        .iter()
        .filter(|(name, _)| name == "MD048")
        .collect();

    assert_eq!(
        our_violations.len(),
        cli2_md048.len(),
        "Different number of MD048 violations detected"
    );

    for (our, cli2) in our_violations.iter().zip(cli2_md048.iter()) {
        assert_eq!(
            our.1, cli2.1,
            "Line number mismatch for MD048: ours={}, cli2={}",
            our.1, cli2.1
        );
    }
}

#[test]
#[ignore] // Slower test requiring Docker
fn test_md050_compatibility() {
    let fixture = Path::new("tests/fixtures/md050.md");
    if !fixture.exists() {
        panic!("Fixture file not found: {}", fixture.display());
    }

    let cli2_violations = run_markdownlint_cli2(fixture);
    if cli2_violations.is_empty() && !docker_available() {
        println!("Skipping test: Docker not available");
        return;
    }

    let rule = MD050;
    let our_violations = run_our_implementation(fixture, &rule);

    println!("markdownlint-cli2 violations: {:?}", cli2_violations);
    println!("Our violations: {:?}", our_violations);

    let cli2_md050: Vec<_> = cli2_violations
        .iter()
        .filter(|(name, _)| name == "MD050")
        .collect();

    assert_eq!(
        our_violations.len(),
        cli2_md050.len(),
        "Different number of MD050 violations detected"
    );

    for (our, cli2) in our_violations.iter().zip(cli2_md050.iter()) {
        assert_eq!(
            our.1, cli2.1,
            "Line number mismatch for MD050: ours={}, cli2={}",
            our.1, cli2.1
        );
    }
}

#[test]
#[ignore] // Slower test requiring Docker
fn test_md051_compatibility() {
    let fixture = Path::new("tests/fixtures/md051.md");
    if !fixture.exists() {
        panic!("Fixture file not found: {}", fixture.display());
    }

    let cli2_violations = run_markdownlint_cli2(fixture);
    if cli2_violations.is_empty() && !docker_available() {
        println!("Skipping test: Docker not available");
        return;
    }

    let rule = MD051;
    let our_violations = run_our_implementation(fixture, &rule);

    println!("markdownlint-cli2 violations: {:?}", cli2_violations);
    println!("Our violations: {:?}", our_violations);

    let cli2_md051: Vec<_> = cli2_violations
        .iter()
        .filter(|(name, _)| name == "MD051")
        .collect();

    assert_eq!(
        our_violations.len(),
        cli2_md051.len(),
        "Different number of MD051 violations detected"
    );

    for (our, cli2) in our_violations.iter().zip(cli2_md051.iter()) {
        assert_eq!(
            our.1, cli2.1,
            "Line number mismatch for MD051: ours={}, cli2={}",
            our.1, cli2.1
        );
    }
}

#[test]
#[ignore] // Slower test requiring Docker
fn test_md052_compatibility() {
    let fixture = Path::new("tests/fixtures/md052.md");
    if !fixture.exists() {
        panic!("Fixture file not found: {}", fixture.display());
    }

    let cli2_violations = run_markdownlint_cli2(fixture);
    if cli2_violations.is_empty() && !docker_available() {
        println!("Skipping test: Docker not available");
        return;
    }

    let rule = MD052;
    let our_violations = run_our_implementation(fixture, &rule);

    println!("markdownlint-cli2 violations: {:?}", cli2_violations);
    println!("Our violations: {:?}", our_violations);

    let cli2_md052: Vec<_> = cli2_violations
        .iter()
        .filter(|(name, _)| name == "MD052")
        .collect();

    assert_eq!(
        our_violations.len(),
        cli2_md052.len(),
        "Different number of MD052 violations detected"
    );

    for (our, cli2) in our_violations.iter().zip(cli2_md052.iter()) {
        assert_eq!(
            our.1, cli2.1,
            "Line number mismatch for MD052: ours={}, cli2={}",
            our.1, cli2.1
        );
    }
}

#[test]
#[ignore] // Slower test requiring Docker
fn test_md053_compatibility() {
    let fixture = Path::new("tests/fixtures/md053.md");
    if !fixture.exists() {
        panic!("Fixture file not found: {}", fixture.display());
    }

    let cli2_violations = run_markdownlint_cli2(fixture);
    if cli2_violations.is_empty() && !docker_available() {
        println!("Skipping test: Docker not available");
        return;
    }

    let rule = MD053;
    let our_violations = run_our_implementation(fixture, &rule);

    println!("markdownlint-cli2 violations: {:?}", cli2_violations);
    println!("Our violations: {:?}", our_violations);

    let cli2_md053: Vec<_> = cli2_violations
        .iter()
        .filter(|(name, _)| name == "MD053")
        .collect();

    assert_eq!(
        our_violations.len(),
        cli2_md053.len(),
        "Different number of MD053 violations detected"
    );

    for (our, cli2) in our_violations.iter().zip(cli2_md053.iter()) {
        assert_eq!(
            our.1, cli2.1,
            "Line number mismatch for MD053: ours={}, cli2={}",
            our.1, cli2.1
        );
    }
}

#[test]
#[ignore] // Slower test requiring Docker
fn test_md054_compatibility() {
    let fixture = Path::new("tests/fixtures/md054.md");
    if !fixture.exists() {
        panic!("Fixture file not found: {}", fixture.display());
    }

    let cli2_violations = run_markdownlint_cli2(fixture);
    if cli2_violations.is_empty() && !docker_available() {
        println!("Skipping test: Docker not available");
        return;
    }

    let rule = MD054;
    let our_violations = run_our_implementation(fixture, &rule);

    println!("markdownlint-cli2 violations: {:?}", cli2_violations);
    println!("Our violations: {:?}", our_violations);

    let cli2_md054: Vec<_> = cli2_violations
        .iter()
        .filter(|(name, _)| name == "MD054")
        .collect();

    assert_eq!(
        our_violations.len(),
        cli2_md054.len(),
        "Different number of MD054 violations detected"
    );

    for (our, cli2) in our_violations.iter().zip(cli2_md054.iter()) {
        assert_eq!(
            our.1, cli2.1,
            "Line number mismatch for MD054: ours={}, cli2={}",
            our.1, cli2.1
        );
    }
}

#[test]
#[ignore] // Slower test requiring Docker
fn test_md055_compatibility() {
    let fixture = Path::new("tests/fixtures/md055.md");
    if !fixture.exists() {
        panic!("Fixture file not found: {}", fixture.display());
    }

    let cli2_violations = run_markdownlint_cli2(fixture);
    if cli2_violations.is_empty() && !docker_available() {
        println!("Skipping test: Docker not available");
        return;
    }

    let rule = MD055;
    let our_violations = run_our_implementation(fixture, &rule);

    println!("markdownlint-cli2 violations: {:?}", cli2_violations);
    println!("Our violations: {:?}", our_violations);

    let cli2_md055: Vec<_> = cli2_violations
        .iter()
        .filter(|(name, _)| name == "MD055")
        .collect();

    assert_eq!(
        our_violations.len(),
        cli2_md055.len(),
        "Different number of MD055 violations detected"
    );

    for (our, cli2) in our_violations.iter().zip(cli2_md055.iter()) {
        assert_eq!(
            our.1, cli2.1,
            "Line number mismatch for MD055: ours={}, cli2={}",
            our.1, cli2.1
        );
    }
}

#[test]
#[ignore] // Slower test requiring Docker
fn test_md056_compatibility() {
    let fixture = Path::new("tests/fixtures/md056.md");
    if !fixture.exists() {
        panic!("Fixture file not found: {}", fixture.display());
    }

    let cli2_violations = run_markdownlint_cli2(fixture);
    if cli2_violations.is_empty() && !docker_available() {
        println!("Skipping test: Docker not available");
        return;
    }

    let rule = MD056;
    let our_violations = run_our_implementation(fixture, &rule);

    println!("markdownlint-cli2 violations: {:?}", cli2_violations);
    println!("Our violations: {:?}", our_violations);

    let cli2_md056: Vec<_> = cli2_violations
        .iter()
        .filter(|(name, _)| name == "MD056")
        .collect();

    assert_eq!(
        our_violations.len(),
        cli2_md056.len(),
        "Different number of MD056 violations detected"
    );

    for (our, cli2) in our_violations.iter().zip(cli2_md056.iter()) {
        assert_eq!(
            our.1, cli2.1,
            "Line number mismatch for MD056: ours={}, cli2={}",
            our.1, cli2.1
        );
    }
}

#[test]
#[ignore] // Slower test requiring Docker
fn test_md058_compatibility() {
    let fixture = Path::new("tests/fixtures/md058.md");
    if !fixture.exists() {
        panic!("Fixture file not found: {}", fixture.display());
    }

    let cli2_violations = run_markdownlint_cli2(fixture);
    if cli2_violations.is_empty() && !docker_available() {
        println!("Skipping test: Docker not available");
        return;
    }

    let rule = MD058;
    let our_violations = run_our_implementation(fixture, &rule);

    println!("markdownlint-cli2 violations: {:?}", cli2_violations);
    println!("Our violations: {:?}", our_violations);

    let cli2_md058: Vec<_> = cli2_violations
        .iter()
        .filter(|(name, _)| name == "MD058")
        .collect();

    assert_eq!(
        our_violations.len(),
        cli2_md058.len(),
        "Different number of MD058 violations detected"
    );

    for (our, cli2) in our_violations.iter().zip(cli2_md058.iter()) {
        assert_eq!(
            our.1, cli2.1,
            "Line number mismatch for MD058: ours={}, cli2={}",
            our.1, cli2.1
        );
    }
}

#[test]
#[ignore] // Slower test requiring Docker
fn test_md059_compatibility() {
    let fixture = Path::new("tests/fixtures/md059.md");
    if !fixture.exists() {
        panic!("Fixture file not found: {}", fixture.display());
    }

    let cli2_violations = run_markdownlint_cli2(fixture);
    if cli2_violations.is_empty() && !docker_available() {
        println!("Skipping test: Docker not available");
        return;
    }

    let rule = MD059;
    let our_violations = run_our_implementation(fixture, &rule);

    println!("markdownlint-cli2 violations: {:?}", cli2_violations);
    println!("Our violations: {:?}", our_violations);

    let cli2_md059: Vec<_> = cli2_violations
        .iter()
        .filter(|(name, _)| name == "MD059")
        .collect();

    assert_eq!(
        our_violations.len(),
        cli2_md059.len(),
        "Different number of MD059 violations detected"
    );

    for (our, cli2) in our_violations.iter().zip(cli2_md059.iter()) {
        assert_eq!(
            our.1, cli2.1,
            "Line number mismatch for MD059: ours={}, cli2={}",
            our.1, cli2.1
        );
    }
}

#[test]
#[ignore] // Slower test requiring Docker
fn test_md060_compatibility() {
    let fixture = Path::new("tests/fixtures/md060.md");
    if !fixture.exists() {
        panic!("Fixture file not found: {}", fixture.display());
    }

    let cli2_violations = run_markdownlint_cli2(fixture);
    if cli2_violations.is_empty() && !docker_available() {
        println!("Skipping test: Docker not available");
        return;
    }

    let rule = MD060;
    let our_violations = run_our_implementation(fixture, &rule);

    println!("markdownlint-cli2 violations: {:?}", cli2_violations);
    println!("Our violations: {:?}", our_violations);

    let cli2_md060: Vec<_> = cli2_violations
        .iter()
        .filter(|(name, _)| name == "MD060")
        .collect();

    assert_eq!(
        our_violations.len(),
        cli2_md060.len(),
        "Different number of MD060 violations detected"
    );

    for (our, cli2) in our_violations.iter().zip(cli2_md060.iter()) {
        assert_eq!(
            our.1, cli2.1,
            "Line number mismatch for MD060: ours={}, cli2={}",
            our.1, cli2.1
        );
    }
}
