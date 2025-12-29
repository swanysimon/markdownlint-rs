# All Rules Test Fixture

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
