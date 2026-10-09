use std::{
    env,
    ffi::{OsStr, OsString},
    fs,
    os::unix::{ffi::OsStringExt, fs::PermissionsExt},
    path::{Path, PathBuf},
    process::{self, Command},
    sync::atomic::{AtomicUsize, Ordering},
};

const BINARY: &str = env!("CARGO_BIN_EXE_zed-match");
const CARGO_STUB: &str = concat!(
    "#!/bin/sh\n",
    "pwd -P > \"${0%/*}/cargo-directory\"\n",
    "printf '%s\\n' \"$@\" > \"${0%/*}/cargo-arguments\"\n",
    "printf '%s\\n' 'cargo: building' >&2\n",
    "printf '{\"reason\":\"compiler-artifact\",\"target\":{\"name\":\"%s\"},\"executable\":\"%s\"}\\n' \"$3\" \"$(cat \"${0%/*}/cargo-executable\")\"\n",
    "exit \"$(cat \"${0%/*}/cargo-status\")\"\n",
);
const LAUNCHER: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/home/.local/bin/zed-match");
const NAME: &str = "zed-match";

static FIXTURES: AtomicUsize = AtomicUsize::new(0);

struct Fixture {
    bin: PathBuf,
    root: PathBuf,
}

impl Fixture {
    fn new() -> Self {
        let root = env::temp_dir().join(format!(
            "{NAME}-test-{}-{}",
            process::id(),
            FIXTURES.fetch_add(1, Ordering::Relaxed)
        ));
        let bin = root.join("bin");
        let stub = bin.join("cargo");

        // A previous run that was interrupted before cleanup may have left state behind
        let _ = fs::remove_dir_all(&root);
        fs::create_dir_all(&bin).expect("The stub directory must be created");
        fs::write(&stub, CARGO_STUB).expect("The stub must be written");
        fs::set_permissions(&stub, fs::Permissions::from_mode(0o755))
            .expect("The stub must be executable");

        Self { bin, root }
    }

    fn cargo(&self, status: u8, executable: &str) {
        fs::write(self.bin.join("cargo-status"), status.to_string())
            .expect("The Cargo status must be written");
        fs::write(self.bin.join("cargo-executable"), executable)
            .expect("The Cargo executable must be written");
    }

    fn recorded(&self, name: &str) -> String {
        fs::read_to_string(self.bin.join(name)).expect("The stub must record its invocation")
    }
}

impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.root);
    }
}

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

fn execute(
    program: &str,
    path: &str,
    directory: Option<&Path>,
    arguments: &[&str],
) -> (Option<i32>, String, String) {
    let mut command = Command::new(program);

    command.args(arguments).env("PATH", path);

    if let Some(directory) = directory {
        command.current_dir(directory);
    }

    let output = command.output().expect("The command must start");

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

#[test]
fn launches_from_the_domfiles_root_and_runs_in_the_caller_directory() {
    let fixture = Fixture::new();
    let caller = fixture.root.join("caller");
    let root = fs::canonicalize(env!("CARGO_MANIFEST_DIR")).expect("The root must resolve");
    // The launcher also needs the provisioned `jq`, so it keeps the inherited search path after the stubs
    let path = format!(
        "{}:{}",
        fixture.bin.display(),
        env::var("PATH").unwrap_or_default()
    );
    let launch = |arguments: &[&str]| execute(LAUNCHER, &path, Some(&caller), arguments);
    let reported = fixture.bin.join("reported");

    fs::create_dir_all(caller.join(".cargo")).expect("The caller configuration must be created");
    fs::write(
        caller.join(".cargo/config.toml"),
        "[build]\ntarget-dir = \"conflicting\"\n",
    )
    .expect("The caller configuration must be written");
    fixture.cargo(0, BINARY);

    assert_eq!(
        launch(&["true", "--help", "--help"]),
        (Some(0), "true\n".to_owned(), "cargo: building\n".to_owned())
    );
    assert_eq!(
        fixture.recorded("cargo-directory"),
        format!("{}\n", root.display())
    );
    assert_eq!(
        fixture.recorded("cargo-arguments"),
        "build\n--bin\nzed-match\n--locked\n--message-format=json-render-diagnostics\n--quiet\n"
    );

    let (status, stdout, stderr) = launch(&["maybe", "a", "a"]);

    assert_eq!((status, stdout.as_str()), (Some(1), ""));
    assert_eq!(
        stderr,
        format!("cargo: building\n{NAME}: `<case-sensitive>` must be `true` or `false`\n")
    );

    fs::write(&reported, "#!/bin/sh\npwd -P\n").expect("The reported command must be written");
    fs::set_permissions(&reported, fs::Permissions::from_mode(0o755))
        .expect("The reported command must be executable");
    fixture.cargo(0, reported.to_str().expect("The path must be UTF-8"));
    assert_eq!(
        launch(&["true", "a", "a"]),
        (
            Some(0),
            format!(
                "{}\n",
                fs::canonicalize(&caller)
                    .expect("The caller directory must resolve")
                    .display()
            ),
            "cargo: building\n".to_owned()
        )
    );

    fixture.cargo(101, BINARY);
    assert_eq!(
        launch(&["true", "a", "a"]),
        (Some(101), String::new(), "cargo: building\n".to_owned())
    );
}
