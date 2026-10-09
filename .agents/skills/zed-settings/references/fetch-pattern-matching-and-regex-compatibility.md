# Fetch Pattern Matching and Regex Compatibility

The pattern matcher checks whether a supplied Rust regex matches a supplied string.

This reference owns the matcher’s interface and manual version check guidance. Fetch policy and effective authorization belong to the [agent permission workflow](agent-permissions.md).

## Apply Matcher Contract

Use the `zed-match` command and Cargo target, implemented in `.agents/skills/zed-settings/scripts/zed_match.rs`, as a read-only pattern matcher with exactly these invocations:

```text
zed-match <case-sensitive> <pattern> <input>
zed-match --help
```

Require all three positional arguments. `<case-sensitive>` must be exactly `true` or `false`, with no default. Treat the pattern and input positions literally, including empty strings and values beginning with `--`. Recognize `--help` only as the sole argument.

Compile the supplied pattern with the repository’s pinned Rust `regex` dependency, using the explicit case sensitivity value as the builder’s initial setting. Preserve inline flag overrides and ordinary `is_match` behavior. Do not anchor, trim, normalize, or parse the input. Empty patterns retain the regex engine’s behavior rather than Zed’s special treatment of empty permission patterns. Rust regex does not support look-around.

For a successful evaluation, the compiled matcher prints `true` or `false` followed by a newline to standard output, leaves standard error empty, and exits with status `0`. A nonmatch is a successful evaluation. Invalid arguments or regexes produce a diagnostic on standard error, leave standard output empty, and exit with status `1`. Output failures also exit with status `1`. Standalone help describes the invocation and result semantics on standard output and exits with status `0`. Diagnostic wording is not a compatibility guarantee.

The compiled matcher accepts inert strings only. It does not read settings, evaluate permission precedence or defaults, compare configurations, create files, or make network requests. A match is not a configured permission decision or evidence of runtime network access.

## Check Patterns

Invoke the installed command from any working directory, with the pattern before the input:

```sh
zed-match '<case-sensitive>' '<pattern>' '<input>'
```

The `home/.local/bin/zed-match` wrapper builds the Cargo target from the domfiles root, then runs the compiled matcher in the caller’s directory. Cargo’s build preparation is separate from the read-only matching operation.

Build failures preserve Cargo’s exit status, such as `101`. Failure to read Cargo’s output or find its reported executable exits with status `1` and a launcher diagnostic on standard error. Executable launch failures may return other statuses. Determine the failure stage from the diagnostic rather than interpreting status `1` alone as invalid input or an invalid regex.

Read the pattern and `case_sensitive` value from the settings being checked rather than copying a documented regex. Compare the printed Boolean with the expected match, not merely the exit status. Separately [resolve the configured decision and native checks](agent-permissions.md#resolve-effective-permission-behavior).

## Check Zed Regex Compatibility

When changing the repository’s `regex` pin or investigating a regex discrepancy with Zed, manually compare the root `Cargo.toml` pin and its resolved direct dependency in `Cargo.lock` with the resolved version in the relevant Zed revision’s `Cargo.lock`. Use that release or revision rather than automatically following `main`. Retrieve only the needed official source, and report a verification limitation if it is unavailable.

Matching direct dependency versions does not establish identical fetch behavior or matching transitive dependencies. A mismatch is evidence to investigate, not authorization to change dependencies.

## Run Focused Contract Tests

Run from the domfiles root:

```sh
cargo test --locked --test zed-match-test
```

Keep the adjacent `zed_match.test.rs` focused on argument handling, help agreement, case sensitivity wiring and inline overrides, literal and empty inputs, and result and error behavior. Also cover the wrapper’s build location, Cargo-reported executable selection, caller working directory, output streams, and exit status propagation. These tests validate the command contract, not the checked-in fetch policy or live Zed behavior. Run them when changing the matcher or wrapper, then select root checks through the [skill-owned script validation policy](../../../../skills/.domfiles-skill-development/references/skill-owned-scripts.md#test-contracts).
