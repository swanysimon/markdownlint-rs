// Compatibility tests with markdownlint-cli2
// These tests require Docker to run the reference implementation
// Run with: cargo test --features compatibility-tests

#[cfg(test)]
#[cfg(feature = "compatibility-tests")]
mod tests {
    use std::{collections::HashMap, fs, path::Path, process::Command};

    use markdownlint_rs::{lint::rules::create_default_registry, markdown::MarkdownParser};

    /// Consolidated test fixture containing test cases for all rules
    const TEST_FIXTURE: &str = r#"# All Rules Test Fixture

This consolidated file contains test cases for all rules.

## MD001 Section

### Level 3 is OK

##### Level 5 - Skipped H4!

Back to text.

### Back to Level 3

###### Level 6 - Skipped H4 and H5

## MD003 Section

### ATX Style

### Also ATX

### Closed Style ###

## MD004 Section

* Item with asterisk
+ Item with plus
- Item with dash

## MD005 Section

* Item 1
 * Item 2 with 1 space indent (wrong)
* Item 3

## MD007 Section

* Item 1
   * Nested with 3 spaces (should be 2)
  * Properly nested with 2 spaces
* Item 2

## MD009 Section

This line has trailing spaces
This line is fine
Another line with spaces

## MD010 Section

This line has a	tab in it
This line is fine
	Tab at start

```javascript
	// Tab in code
	function test() {
		return true;
	}
```

Normal text again	with tab

## MD011 Section

This is a correct [link](http://example.com).

But this is (reversed)[http://example.com] which is wrong.

Here is (another reversed)[url] link.

## MD012 Section

This is a paragraph.


Multiple blank lines above.

Another paragraph.



Three blank lines above.

## MD013 Section

This is a short line.

This is a very long line that definitely exceeds the default eighty character limit and should be flagged as a violation.

Another short line.

This is another extremely long line that goes on and on and on and continues past the normal eighty character maximum line length limit.

## MD014 Section

```bash
$ ls -la
$ echo hello
$ pwd
```

Good example with output:

```bash
$ ls -la
total 64
```

## MD018 Section

###No space here

Normal content.

## MD019 Section

###  Two spaces after hash

Normal content.

## MD020 Section

### Correct closed heading ###

### Missing space before close###

### Missing space after open###

## MD021 Section

### Multiple spaces  ###

### Also multiple   ###

## MD022 Section

Text without blank line before heading.
### Heading Without Blank Before

Another paragraph.

### Heading Without Blank After
More content directly after.

## MD023 Section

 ### Indented with 1 space

  ### Indented with 2 spaces

## MD024 Section

### Duplicate Heading

Some content.

### Duplicate Heading

Different content but same heading text.

## MD026 Section

### Bad Heading.

### Another Bad One?

### Yet Another!

## MD027 Section

>  Two spaces after blockquote marker

> Normal blockquote

>    Four spaces after marker

## MD028 Section

> First blockquote line

> Continued after blank - violation

## MD029 Section

1. First item
3. Wrong number - should be 2
4. Fourth item

## MD030 Section

*  Two spaces after marker

1.  Two spaces after ordered marker

## MD031 Section

Text before code block
```
code
```

```
another code block
```
Text after code block

## MD032 Section

Text before list
* Item 1
* Item 2

Good list below:

* Item A
* Item B

Text after list without blank

## MD033 Section

Normal markdown text.

Text with <br> tag.

<div>Block HTML</div>

More content with <span>inline</span> tags.

## MD034 Section

Check out https://example.com for more info.

Multiple URLs: https://test.com and https://demo.com

Good link: [example](https://example.com)

## MD035 Section

---

Content here.

***

More content.

## MD036 Section

**Summary**

Some content here.

*Introduction*

More content.

**Note:** This is fine with punctuation.

## MD037 Section

This is ** bold ** with spaces.

This is * italic * with spaces.

Correct: **bold** and *italic*.

## MD038 Section

Use the ` function()` with leading space.

Use the `function() ` with trailing space.

Correct: `function()` without spaces.

## MD039 Section

[ Link with leading space](https://example.com)

[Link with trailing space ](https://example.com)

[ Both spaces ](https://example.com)

[Correct link](https://example.com)

## MD040 Section

```
code without language
```

Good code block:

```rust
let x = 5;
```

## MD042 Section

[Good link](https://example.com)

[](https://empty-link.com)

## MD045 Section

![Good alt text](image1.png)

![](image2.png)

## MD046 Section

```
Fenced code block
```

    Indented code block

## MD047 Section

This content is at the end.

## MD049 Section

This is *italic* with asterisks.

This is _italic_ with underscores.
"#;

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
                        if let Some(md_part) = line.split_whitespace().find(|s| s.starts_with("MD"))
                        {
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
    fn test_all_rules_compatibility() {
        if !docker_available() {
            eprintln!("Skipping compatibility test: Docker not available");
            return;
        }

        // Write fixture to tests directory (accessible by Docker on macOS)
        let fixture_path = Path::new("tests/compatibility_fixture.md");
        fs::write(fixture_path, TEST_FIXTURE).expect("Failed to write fixture");

        let cli2_violations = run_markdownlint_cli2(fixture_path);

        // Clean up fixture file
        let _ = fs::remove_file(fixture_path);

        // Run all our rules on the inline fixture
        let parser = MarkdownParser::new(TEST_FIXTURE);

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
}
