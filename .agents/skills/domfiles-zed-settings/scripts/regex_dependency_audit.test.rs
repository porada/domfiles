#[path = "regex_dependency_audit.rs"]
mod helper;

use helper::{Parameter, RouteKind};
use std::{
    env,
    ffi::OsString,
    fs,
    io::{self, Write},
    path::{Path, PathBuf},
    process,
    sync::atomic::{AtomicUsize, Ordering},
    time::{SystemTime, UNIX_EPOCH},
};

const LOCAL_MANIFEST: &str = concat!(
    "[package]\n",
    "name = \"domfiles\"\n",
    "version = \"0.0.0\"\n",
    "\n",
    "[dependencies]\n",
    "regex = \"=1.12.3\"\n",
);

const REGEX_LOCK: &str = concat!(
    "version = 4\n",
    "\n",
    "[[package]]\n",
    "name = \"domfiles\"\n",
    "version = \"0.0.0\"\n",
    "dependencies = [\n",
    " \"regex\",\n",
    "]\n",
    "\n",
    "[[package]]\n",
    "name = \"aho-corasick\"\n",
    "version = \"1.1.3\"\n",
    "source = \"registry+https://github.com/rust-lang/crates.io-index\"\n",
    "checksum = \"8e60d3430d3a69478ad0993f19238d2df97c507009a52b3c10addcd7f6bcb916\"\n",
    "dependencies = [\n",
    " \"memchr\",\n",
    "]\n",
    "\n",
    "[[package]]\n",
    "name = \"memchr\"\n",
    "version = \"2.7.6\"\n",
    "source = \"registry+https://github.com/rust-lang/crates.io-index\"\n",
    "checksum = \"f52b00d39961fc5b2736ea853c9cc86238e165017a493d1d5c8eac6bdc4cc273\"\n",
    "\n",
    "[[package]]\n",
    "name = \"regex\"\n",
    "version = \"1.12.3\"\n",
    "source = \"registry+https://github.com/rust-lang/crates.io-index\"\n",
    "checksum = \"e10754a14b9137dd7b1e3e5b0493cc9171fdd105e0ab477f51b72e7f3ac0e276\"\n",
    "dependencies = [\n",
    " \"aho-corasick\",\n",
    " \"memchr\",\n",
    " \"regex-automata\",\n",
    " \"regex-syntax\",\n",
    "]\n",
    "\n",
    "[[package]]\n",
    "name = \"regex-automata\"\n",
    "version = \"0.4.14\"\n",
    "source = \"registry+https://github.com/rust-lang/crates.io-index\"\n",
    "checksum = \"6e1dd4122fc1595e8162618945476892eefca7b88c52820e74af6262213cae8f\"\n",
    "dependencies = [\n",
    " \"aho-corasick\",\n",
    " \"memchr\",\n",
    " \"regex-syntax\",\n",
    "]\n",
    "\n",
    "[[package]]\n",
    "name = \"regex-syntax\"\n",
    "version = \"0.8.8\"\n",
    "source = \"registry+https://github.com/rust-lang/crates.io-index\"\n",
    "checksum = \"7a2d987857b319362043e95f5353c0535c1f58eec5336fdfcf626430af7def58\"\n",
);

const LOCAL_LOCK: &str = REGEX_LOCK;
const UPSTREAM_LOCK: &str = REGEX_LOCK;

static NEXT_FIXTURE_ID: AtomicUsize = AtomicUsize::new(0);

struct Fixture {
    root: PathBuf,
}

impl Fixture {
    fn new() -> Self {
        let fixture_id = NEXT_FIXTURE_ID.fetch_add(1, Ordering::Relaxed);
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("System clock must be after the Unix epoch")
            .as_nanos();
        let root = env::temp_dir().join(format!(
            "domfiles-regex-dependency-audit-{}-{timestamp}-{fixture_id}",
            process::id()
        ));
        fs::create_dir(&root).expect("Failed to create fixture directory");

        Self { root }
    }

    fn write(&self, name: &str, contents: &str) -> PathBuf {
        let path = self.root.join(name);
        fs::write(&path, contents).expect("Failed to write fixture file");
        path
    }
}

impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.root);
    }
}

struct FailingWriter;

impl Write for FailingWriter {
    fn write(&mut self, _buffer: &[u8]) -> io::Result<usize> {
        Err(io::Error::other("intentional write failure"))
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

fn comparison_arguments(
    local_manifest: &Path,
    upstream_lock: &Path,
    upstream_revision: &str,
) -> [OsString; 6] {
    [
        OsString::from("--local-manifest"),
        local_manifest.as_os_str().to_owned(),
        OsString::from("--upstream-lock"),
        upstream_lock.as_os_str().to_owned(),
        OsString::from("--upstream-revision"),
        OsString::from(upstream_revision),
    ]
}

fn run_arguments<I>(arguments: I) -> (u8, String, String)
where
    I: IntoIterator<Item = OsString>,
{
    let mut stdout = Vec::new();
    let mut stderr = Vec::new();
    let status = helper::run(arguments, &mut stdout, &mut stderr);

    (
        status,
        String::from_utf8(stdout).expect("Standard output must be valid UTF-8"),
        String::from_utf8(stderr).expect("Standard error must be valid UTF-8"),
    )
}

fn help_options() -> Vec<String> {
    helper::HELP
        .lines()
        .skip_while(|line| *line != "Options:")
        .skip(1)
        .take_while(|line| !line.is_empty())
        .filter(|line| line.trim_start().starts_with("--"))
        .map(|line| {
            line.split_whitespace()
                .next()
                .expect("Each help option line must name an option")
                .to_owned()
        })
        .collect()
}

fn usage_options() -> Vec<Vec<String>> {
    helper::HELP
        .lines()
        .skip(1)
        .take_while(|line| !line.is_empty())
        .map(|line| {
            line.split_whitespace()
                .filter(|token| token.starts_with("--"))
                .map(str::to_owned)
                .collect()
        })
        .collect()
}

fn route_options(route: RouteKind) -> Vec<String> {
    Parameter::ALL
        .into_iter()
        .filter(|parameter| parameter.route() == route)
        .map(|parameter| parameter.option().to_owned())
        .collect()
}

fn run_with_files(
    local_manifest: &Path,
    upstream_lock: &Path,
    upstream_revision: &str,
) -> (u8, String, String) {
    run_arguments(comparison_arguments(
        local_manifest,
        upstream_lock,
        upstream_revision,
    ))
}

fn write_version_files(
    fixture: &Fixture,
    local_lock: &str,
    upstream_lock: &str,
) -> (PathBuf, PathBuf) {
    let local_manifest = fixture.write("Cargo.toml", LOCAL_MANIFEST);
    fixture.write("Cargo.lock", local_lock);
    let upstream_lock = fixture.write("upstream.lock", upstream_lock);

    (local_manifest, upstream_lock)
}

#[test]
fn documents_every_accepted_option() {
    let accepted = Parameter::ALL
        .map(|parameter| parameter.option().to_owned())
        .to_vec();

    assert_eq!(
        help_options(),
        accepted,
        "The help option list must match the accepted options in alphabetical order"
    );
}

#[test]
fn documents_every_supported_route() {
    let expected = vec![
        route_options(RouteKind::Comparison),
        route_options(RouteKind::Help),
    ];

    assert_eq!(
        usage_options(),
        expected,
        "Each usage line must name exactly the options its route accepts"
    );
}

#[test]
fn rejects_removed_pattern_options() {
    for option in ["--case-sensitive", "--pattern-file"] {
        let mut stdout = Vec::new();
        let mut stderr = Vec::new();

        let status = helper::run([OsString::from(option)], &mut stdout, &mut stderr);

        assert_eq!(status, 2);
        assert!(stdout.is_empty());
        assert_eq!(
            String::from_utf8(stderr).expect("Standard error must be valid UTF-8"),
            format!(
                "regex-dependency-audit: Unknown option `{option}`. Run `regex-dependency-audit --help` for usage\n"
            )
        );
    }
}

#[test]
fn rejects_help_combined_with_comparison() {
    let (status, stdout, stderr) = run_arguments([
        OsString::from("--help"),
        OsString::from("--local-manifest"),
        OsString::from("Cargo.toml"),
    ]);

    assert_eq!(status, 2);
    assert!(stdout.is_empty());
    assert_eq!(
        stderr,
        "regex-dependency-audit: Option `--help` must be used alone\n"
    );
}

#[test]
fn rejects_missing_option_values() {
    for (option, requirement) in [
        ("--local-manifest", "requires a path"),
        ("--upstream-lock", "requires a path"),
        ("--upstream-revision", "requires a commit reference"),
    ] {
        let (status, stdout, stderr) = run_arguments([OsString::from(option)]);

        assert_eq!(status, 2);
        assert!(stdout.is_empty());
        assert_eq!(
            stderr,
            format!("regex-dependency-audit: Option `{option}` {requirement}\n")
        );
    }
}

#[test]
fn rejects_missing_required_options() {
    for (arguments, missing_option) in [
        (Vec::new(), "`--local-manifest <path>`"),
        (
            vec![
                OsString::from("--local-manifest"),
                OsString::from("Cargo.toml"),
            ],
            "`--upstream-lock <path>`",
        ),
        (
            vec![
                OsString::from("--local-manifest"),
                OsString::from("Cargo.toml"),
                OsString::from("--upstream-lock"),
                OsString::from("Cargo.lock"),
            ],
            "`--upstream-revision <commit>`",
        ),
    ] {
        let (status, stdout, stderr) = run_arguments(arguments);

        assert_eq!(status, 2);
        assert!(stdout.is_empty());
        assert!(stderr.contains(&format!("Missing required option {missing_option}")));
    }
}

#[test]
fn rejects_positional_arguments() {
    let (status, stdout, stderr) = run_arguments([OsString::from("Cargo.toml")]);

    assert_eq!(status, 2);
    assert!(stdout.is_empty());
    assert_eq!(
        stderr,
        "regex-dependency-audit: Unknown option `Cargo.toml`. Run `regex-dependency-audit --help` for usage\n"
    );
}

#[test]
fn rejects_repeated_options() {
    for arguments in [
        [
            "--local-manifest",
            "first.toml",
            "--local-manifest",
            "second.toml",
        ],
        [
            "--upstream-lock",
            "first.lock",
            "--upstream-lock",
            "second.lock",
        ],
        [
            "--upstream-revision",
            "abcdef1",
            "--upstream-revision",
            "abcdef2",
        ],
    ] {
        let option = arguments[0];
        let (status, stdout, stderr) = run_arguments(arguments.map(OsString::from));

        assert_eq!(status, 2);
        assert!(stdout.is_empty());
        assert_eq!(
            stderr,
            format!("regex-dependency-audit: Option `{option}` may be specified only once\n")
        );
    }
}

#[test]
fn extracts_exact_local_pin() {
    assert_eq!(
        helper::local_regex_version(LOCAL_MANIFEST).expect("Local pin must be valid"),
        "1.12.3"
    );
}

#[test]
fn extracts_upstream_package_version() {
    assert_eq!(
        helper::upstream_regex_version(UPSTREAM_LOCK)
            .expect("Upstream package version must be valid"),
        "1.12.3"
    );
}

#[test]
fn rejects_multiple_upstream_versions() {
    let lockfile =
        format!("{UPSTREAM_LOCK}\n[[package]]\nname = \"regex\"\nversion = \"1.13.0\"\n");

    let error = helper::upstream_regex_version(&lockfile)
        .expect_err("Multiple upstream versions must fail");

    assert!(error.contains("multiple `regex` versions"));
    assert!(error.contains("`1.12.3`"));
    assert!(error.contains("`1.13.0`"));
}

#[test]
fn rejects_malformed_upstream_lockfile() {
    let fixture = Fixture::new();
    let malformed_upstream = format!("this is invalid TOML\n{UPSTREAM_LOCK}");
    let (local_manifest, upstream_lock) =
        write_version_files(&fixture, LOCAL_LOCK, &malformed_upstream);

    let (status, stdout, stderr) = run_with_files(&local_manifest, &upstream_lock, "abcdef1");

    assert_eq!(status, 2);
    assert!(stdout.is_empty());
    assert!(stderr.contains("Upstream lockfile contains invalid TOML:\n\n"));
}

#[test]
fn rejects_unpinned_local_dependency() {
    let manifest = LOCAL_MANIFEST.replace("regex = \"=1.12.3\"", "regex = \"1.12.3\"");

    let error = helper::local_regex_version(&manifest).expect_err("Unpinned version must fail");

    assert!(error.contains("exact `=VERSION`"));
}

#[test]
fn rejects_unpinned_dependency_despite_exact_metadata_key() {
    let manifest = concat!(
        "[package]\n",
        "name = \"domfiles\"\n",
        "version = \"0.0.0\"\n",
        "\n",
        "[dependencies]\n",
        "regex = \"1.12.3\"\n",
        "\n",
        "[package.metadata.audit]\n",
        "regex = \"=1.12.3\"\n",
    );

    let error = helper::local_regex_version(manifest)
        .expect_err("An unrelated metadata key must not satisfy the dependency pin");

    assert!(error.contains("`[dependencies].regex`"));
    assert!(error.contains("exact `=VERSION`"));
}

#[test]
fn extracts_exact_local_pin_from_detailed_dependency() {
    let manifest = LOCAL_MANIFEST.replace(
        "regex = \"=1.12.3\"",
        "regex = { version = \"=1.12.3\", default-features = false }",
    );

    assert_eq!(
        helper::local_regex_version(&manifest).expect("Detailed local pin must be valid"),
        "1.12.3"
    );
}

#[test]
fn rejects_invalid_upstream_revision() {
    let mut stdout = Vec::new();
    let mut stderr = Vec::new();
    let arguments = [
        OsString::from("--local-manifest"),
        OsString::from("Cargo.toml"),
        OsString::from("--upstream-lock"),
        OsString::from("Cargo.lock"),
        OsString::from("--upstream-revision"),
        OsString::from("main"),
    ];

    let status = helper::run(arguments, &mut stdout, &mut stderr);

    assert_eq!(status, 2);
    assert!(stdout.is_empty());
    assert!(
        String::from_utf8(stderr)
            .expect("Standard error must be valid UTF-8")
            .contains("lowercase hexadecimal commit")
    );
}

#[test]
fn accepts_unrelated_local_regex_version() {
    let fixture = Fixture::new();
    let local_lock = LOCAL_LOCK.replace(
        " \"regex\",\n",
        " \"regex 1.12.3 (registry+https://github.com/rust-lang/crates.io-index)\",\n",
    );
    let local_lock = format!(
        "{local_lock}\n[[package]]\nname = \"regex\"\nversion = \"1.11.1\"\nsource = \"registry+https://github.com/rust-lang/crates.io-index\"\nchecksum = \"1111111111111111111111111111111111111111111111111111111111111111\"\n"
    );
    let (local_manifest, upstream_lock) = write_version_files(&fixture, &local_lock, UPSTREAM_LOCK);

    let (status, stdout, stderr) = run_with_files(&local_manifest, &upstream_lock, "abcdef1");

    assert_eq!(status, 0);
    assert_eq!(
        stdout,
        "Zed commit `abcdef1` and `Cargo.toml` use `regex` `1.12.3`\n"
    );
    assert!(stderr.is_empty());
}

#[test]
fn rejects_pinned_regex_package_not_selected_by_root_dependency() {
    let fixture = Fixture::new();
    let local_lock = format!(
        "{LOCAL_LOCK}\n[[package]]\nname = \"regex\"\nversion = \"1.11.1\"\nsource = \"registry+https://github.com/rust-lang/crates.io-index\"\nchecksum = \"1111111111111111111111111111111111111111111111111111111111111111\"\n"
    );
    let local_lock = local_lock.replace(
        " \"regex\",\n",
        " \"regex 1.11.1 (registry+https://github.com/rust-lang/crates.io-index)\",\n",
    );
    let (local_manifest, upstream_lock) = write_version_files(&fixture, &local_lock, UPSTREAM_LOCK);

    let (status, stdout, stderr) = run_with_files(&local_manifest, &upstream_lock, "abcdef1");

    assert_eq!(status, 2);
    assert!(stdout.is_empty());
    assert!(stderr.contains("adjacent `Cargo.lock` resolves `1.11.1`"));
}

#[test]
fn reports_matching_versions() {
    let fixture = Fixture::new();
    let (local_manifest, upstream_lock) = write_version_files(&fixture, LOCAL_LOCK, UPSTREAM_LOCK);

    let (status, stdout, stderr) = run_with_files(&local_manifest, &upstream_lock, "abcdef1");

    assert_eq!(status, 0);
    assert_eq!(
        stdout,
        "Zed commit `abcdef1` and `Cargo.toml` use `regex` `1.12.3`\n"
    );
    assert!(stderr.is_empty());
}

#[test]
fn accepts_transitive_dependency_checksum_mismatch() {
    let fixture = Fixture::new();
    let upstream_lock = UPSTREAM_LOCK.replace(
        "6e1dd4122fc1595e8162618945476892eefca7b88c52820e74af6262213cae8f",
        "0000000000000000000000000000000000000000000000000000000000000000",
    );
    let (local_manifest, upstream_lock) = write_version_files(&fixture, LOCAL_LOCK, &upstream_lock);

    let (status, stdout, stderr) = run_with_files(&local_manifest, &upstream_lock, "abcdef1");

    assert_eq!(status, 0);
    assert_eq!(
        stdout,
        "Zed commit `abcdef1` and `Cargo.toml` use `regex` `1.12.3`\n"
    );
    assert!(stderr.is_empty());
}

#[test]
fn accepts_source_less_transitive_dependency() {
    let fixture = Fixture::new();
    let upstream_lock = UPSTREAM_LOCK.replace(
        concat!(
            "source = \"registry+https://github.com/rust-lang/crates.io-index\"\n",
            "checksum = \"7a2d987857b319362043e95f5353c0535c1f58eec5336fdfcf626430af7def58\"\n",
        ),
        "",
    );
    let (local_manifest, upstream_lock) = write_version_files(&fixture, LOCAL_LOCK, &upstream_lock);

    let (status, stdout, stderr) = run_with_files(&local_manifest, &upstream_lock, "abcdef1");

    assert_eq!(status, 0);
    assert_eq!(
        stdout,
        "Zed commit `abcdef1` and `Cargo.toml` use `regex` `1.12.3`\n"
    );
    assert!(stderr.is_empty());
}

#[test]
fn accepts_transitive_dependency_edge_mismatch() {
    let fixture = Fixture::new();
    let upstream_lock = UPSTREAM_LOCK.replace(
        concat!(
            "dependencies = [\n",
            " \"aho-corasick\",\n",
            " \"memchr\",\n",
            " \"regex-automata\",\n",
            " \"regex-syntax\",\n",
            "]",
        ),
        concat!(
            "dependencies = [\n",
            " \"aho-corasick\",\n",
            " \"regex-automata\",\n",
            " \"regex-syntax\",\n",
            "]",
        ),
    );
    let (local_manifest, upstream_lock) = write_version_files(&fixture, LOCAL_LOCK, &upstream_lock);

    let (status, stdout, stderr) = run_with_files(&local_manifest, &upstream_lock, "abcdef1");

    assert_eq!(status, 0);
    assert_eq!(
        stdout,
        "Zed commit `abcdef1` and `Cargo.toml` use `regex` `1.12.3`\n"
    );
    assert!(stderr.is_empty());
}

#[test]
fn accepts_transitive_dependency_version_mismatch() {
    let fixture = Fixture::new();
    let upstream_lock = UPSTREAM_LOCK.replace("version = \"0.4.14\"", "version = \"0.4.13\"");
    let (local_manifest, upstream_lock) = write_version_files(&fixture, LOCAL_LOCK, &upstream_lock);

    let (status, stdout, stderr) = run_with_files(&local_manifest, &upstream_lock, "abcdef1");

    assert_eq!(status, 0);
    assert_eq!(
        stdout,
        "Zed commit `abcdef1` and `Cargo.toml` use `regex` `1.12.3`\n"
    );
    assert!(stderr.is_empty());
}

#[test]
fn reports_version_mismatch() {
    let fixture = Fixture::new();
    let upstream_lock = UPSTREAM_LOCK.replace("version = \"1.12.3\"", "version = \"1.13.0\"");
    let (local_manifest, upstream_lock) = write_version_files(&fixture, LOCAL_LOCK, &upstream_lock);

    let (status, stdout, stderr) = run_with_files(&local_manifest, &upstream_lock, "abcdef1");

    assert_eq!(status, 1);
    assert!(stdout.is_empty());
    assert_eq!(
        stderr,
        "Zed commit `abcdef1` uses `regex` `1.13.0`, but `Cargo.toml` pins `1.12.3`\n"
    );
}

#[test]
fn rejects_stale_local_lockfile() {
    let fixture = Fixture::new();
    let local_lock = LOCAL_LOCK.replace("version = \"1.12.3\"", "version = \"1.13.0\"");
    let (local_manifest, upstream_lock) = write_version_files(&fixture, &local_lock, UPSTREAM_LOCK);

    let (status, stdout, stderr) = run_with_files(&local_manifest, &upstream_lock, "abcdef1");

    assert_eq!(status, 2);
    assert!(stdout.is_empty());
    assert!(stderr.contains("adjacent `Cargo.lock` resolves `1.13.0`"));
}

#[test]
fn returns_success_for_help() {
    let mut stdout = Vec::new();
    let mut stderr = Vec::new();

    let status = helper::run(vec![OsString::from("--help")], &mut stdout, &mut stderr);

    assert_eq!(status, 0);
    assert_eq!(
        String::from_utf8(stdout).expect("Standard output must be valid UTF-8"),
        helper::HELP
    );
    assert!(stderr.is_empty());
}

#[test]
fn returns_error_when_help_output_fails() {
    let mut stdout = FailingWriter;
    let mut stderr = Vec::new();

    let status = helper::run([OsString::from("--help")], &mut stdout, &mut stderr);

    assert_eq!(status, 2);
    assert!(
        String::from_utf8(stderr)
            .expect("Standard error must be valid UTF-8")
            .contains("Failed to write help to standard output")
    );
}

#[test]
fn returns_error_when_match_output_fails() {
    let fixture = Fixture::new();
    let (local_manifest, upstream_lock) = write_version_files(&fixture, LOCAL_LOCK, UPSTREAM_LOCK);
    let arguments = comparison_arguments(&local_manifest, &upstream_lock, "abcdef1");
    let mut stdout = FailingWriter;
    let mut stderr = Vec::new();

    let status = helper::run(arguments, &mut stdout, &mut stderr);

    assert_eq!(status, 2);
    assert!(
        String::from_utf8(stderr)
            .expect("Standard error must be valid UTF-8")
            .contains("Failed to write audit result to standard output")
    );
}

#[test]
fn returns_error_when_mismatch_output_fails() {
    let fixture = Fixture::new();
    let upstream_lock_contents =
        UPSTREAM_LOCK.replace("version = \"1.12.3\"", "version = \"1.13.0\"");
    let (local_manifest, upstream_lock) =
        write_version_files(&fixture, LOCAL_LOCK, &upstream_lock_contents);
    let arguments = comparison_arguments(&local_manifest, &upstream_lock, "abcdef1");
    let mut stdout = Vec::new();
    let mut stderr = FailingWriter;

    let status = helper::run(arguments, &mut stdout, &mut stderr);

    assert_eq!(status, 2);
    assert!(stdout.is_empty());
}
