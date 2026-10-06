use std::{
    ffi::{OsStr, OsString},
    os::unix::ffi::OsStringExt,
    process::Command,
};

const BINARY: &str = env!("CARGO_BIN_EXE_domfiles-zed-settings-pattern-match");
const NAME: &str = "domfiles-zed-settings-pattern-match";

fn run<I, S>(arguments: I) -> (Option<i32>, String, String)
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    let output = Command::new(BINARY)
        .args(arguments)
        .output()
        .expect("The pattern matcher must start");

    (
        output.status.code(),
        String::from_utf8(output.stdout).expect("Standard output must be UTF-8"),
        String::from_utf8(output.stderr).expect("Standard error must be UTF-8"),
    )
}

fn assert_result(arguments: [&str; 3], expected: bool) {
    assert_eq!(
        run(arguments),
        (Some(0), format!("{expected}\n"), String::new()),
        "{arguments:?}"
    );
}

fn assert_refusal<I, S>(arguments: I)
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    let arguments: Vec<OsString> = arguments
        .into_iter()
        .map(|argument| argument.as_ref().to_owned())
        .collect();
    let (status, stdout, stderr) = run(&arguments);

    assert_eq!(status, Some(1), "{arguments:?}");
    assert!(stdout.is_empty(), "{arguments:?}");
    assert!(
        stderr.starts_with(&format!("{NAME}: ")) && stderr.ends_with('\n'),
        "{arguments:?}"
    );
}

fn help_section<'a>(help: &'a str, heading: &str) -> Vec<&'a str> {
    help.lines()
        .skip_while(|line| *line != heading)
        .skip(1)
        .take_while(|line| !line.is_empty())
        .collect()
}

fn sample_argument(token: &str) -> &str {
    match token {
        "<case-sensitive>" => "true",
        "<input>" | "<pattern>" => "a",
        literal => literal,
    }
}

#[test]
fn prints_help_alone() {
    let (status, stdout, stderr) = run(["--help"]);

    assert_eq!(status, Some(0));
    assert!(stdout.starts_with("Usage:\n"));
    assert!(stderr.is_empty());
}

#[test]
fn accepts_every_documented_invocation() {
    let (_, help, _) = run(["--help"]);
    let routes: Vec<Vec<&str>> = help_section(&help, "Usage:")
        .into_iter()
        .map(|line| {
            let mut tokens = line.split_whitespace();
            assert_eq!(tokens.next(), Some(NAME));
            tokens.collect()
        })
        .collect();
    let placeholders: Vec<&str> = routes
        .iter()
        .flatten()
        .copied()
        .filter(|token| token.starts_with('<'))
        .collect();
    let documented_arguments: Vec<&str> = help_section(&help, "Arguments:")
        .into_iter()
        .filter_map(|line| line.split_whitespace().next())
        .collect();

    assert!(!routes.is_empty());
    assert_eq!(documented_arguments, placeholders);

    for route in routes {
        let arguments: Vec<&str> = route.iter().copied().map(sample_argument).collect();
        let (status, stdout, stderr) = run(&arguments);

        assert_eq!(status, Some(0), "{arguments:?}");
        assert!(!stdout.is_empty(), "{arguments:?}");
        assert!(stderr.is_empty(), "{arguments:?}");
    }
}

#[test]
fn rejects_undocumented_argument_shapes() {
    assert_refusal(Vec::<&str>::new());
    assert_refusal(["-h"]);
    assert_refusal(["true"]);
    assert_refusal(["--help", "--help"]);
    assert_refusal(["true", "a"]);
    assert_refusal(["true", "a", "a", "a"]);
}

#[test]
fn rejects_invalid_case_sensitivity() {
    for value in ["", " true", "--help", "1", "TRUE", "False", "yes"] {
        assert_refusal([value, "a", "a"]);
    }
}

#[test]
fn rejects_non_utf8_arguments() {
    let invalid = OsString::from_vec(vec![0xff]);

    assert_refusal([invalid.clone(), "a".into(), "a".into()]);
    assert_refusal(["true".into(), invalid.clone(), "a".into()]);
    assert_refusal(["true".into(), "a".into(), invalid]);
}

#[test]
fn rejects_invalid_regexes() {
    for pattern in ["(", "[a", "(?=a)a"] {
        assert_refusal(["true", pattern, "a"]);
        assert_refusal(["false", pattern, "a"]);
    }
}

#[test]
fn reports_matches_and_nonmatches() {
    assert_result(["true", "b", "abc"], true);
    assert_result(["true", "^b", "abc"], false);
    assert_result(
        [
            "true",
            "^https://example\\.com/",
            "https://example.com/page",
        ],
        true,
    );
    assert_result(
        ["true", "^https://example\\.com/", "https://example.org/"],
        false,
    );
}

#[test]
fn applies_case_sensitivity_and_inline_overrides() {
    assert_result(["true", "abc", "ABC"], false);
    assert_result(["false", "abc", "ABC"], true);
    assert_result(["true", "(?i)abc", "ABC"], true);
    assert_result(["false", "(?-i)abc", "ABC"], false);
    assert_result(["true", "(?i:a)b", "Ab"], true);
    assert_result(["true", "(?i:a)b", "AB"], false);
    assert_result(["false", "(?-i:a)b", "aB"], true);
    assert_result(["false", "(?-i:a)b", "AB"], false);
}

#[test]
fn treats_patterns_and_inputs_literally() {
    assert_result(["true", "--help", "--help"], true);
    assert_result(["true", "--version", "--help"], false);
    assert_result(["true", "", ""], true);
    assert_result(["true", "", "abc"], true);
    assert_result(["true", "^$", ""], true);
    assert_result(["true", "^$", " "], false);
    assert_result(["true", "^a$", " a "], false);
    assert_result(["true", "^ a $", " a "], true);
    assert_result(
        ["true", "^https://a\\.example$", "HTTPS://a.example"],
        false,
    );
}
