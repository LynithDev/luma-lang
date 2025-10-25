use luma_diagnostic::{DiagnosticKind, Reporter};

use crate::helpers;

#[test]
pub fn test_parsing_if() {
    const INPUT: &str = r#"
        // errors as it returns a type and no semi-colon therefore its a "scope" return type
        if arg == "1" {
            arg = "1a"
        }
    "#;

    let reporter = Reporter::new();
    let parsed = helpers::parse_source(&reporter, INPUT);
    helpers::analyze_source(&reporter, &parsed);

    assert_eq!(reporter.diagnostic_count(DiagnosticKind::Error), 1);
}