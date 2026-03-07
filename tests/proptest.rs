use markdownlint_rs::formatter;
use proptest::prelude::*;

proptest! {
    /// The formatter must never panic on any input.
    #[test]
    fn formatter_never_panics(s in ".*") {
        let _ = formatter::format(&s);
    }

    /// format(format(x)) == format(x) for all inputs.
    #[test]
    fn formatter_is_idempotent(s in ".*") {
        let once = formatter::format(&s);
        let twice = formatter::format(&once);
        prop_assert_eq!(&once, &twice, "formatter not idempotent on input: {:?}", s);
    }

    /// Non-empty output always ends with exactly one newline.
    #[test]
    fn formatter_trailing_newline(s in ".*") {
        let out = formatter::format(&s);
        if !out.is_empty() {
            prop_assert!(
                out.ends_with('\n'),
                "output does not end with newline: {out:?}"
            );
            prop_assert!(
                !out.ends_with("\n\n"),
                "output ends with multiple newlines: {out:?}"
            );
        }
    }

    /// No line in the output has trailing whitespace.
    #[test]
    fn formatter_no_trailing_whitespace(s in ".*") {
        let out = formatter::format(&s);
        for line in out.lines() {
            prop_assert_eq!(
                line,
                line.trim_end(),
                "line has trailing whitespace: {:?}",
                line
            );
        }
    }
}
