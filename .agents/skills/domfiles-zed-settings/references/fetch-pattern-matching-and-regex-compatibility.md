# Fetch Pattern Matching and Regex Compatibility

The pattern matcher checks whether a supplied Rust regex matches a supplied string.

This reference owns the matcher’s interface and manual version-check guidance. Fetch policy and effective authorization belong to the [agent permission workflow](agent-permissions.md).

## Apply Matcher Contract

Retain `.agents/skills/domfiles-zed-settings/scripts/pattern_match.rs` and the Cargo target `domfiles-zed-settings-pattern-match` as a read-only pattern matcher with exactly these invocations:

```text
domfiles-zed-settings-pattern-match <case-sensitive> <pattern> <input>
domfiles-zed-settings-pattern-match --help
```

Require all three positional arguments. `<case-sensitive>` must be exactly `true` or `false`, with no default. Treat the pattern and input positions literally, including empty strings and values beginning with `--`. Recognize `--help` only as the sole argument.

Compile the supplied pattern with the repository’s pinned Rust `regex` dependency, using the explicit case-sensitivity value as the builder’s initial setting. Preserve inline flag overrides and ordinary `is_match` behavior. Do not anchor, trim, normalize, or parse the input. Empty patterns retain the regex engine’s behavior rather than Zed’s special treatment of empty permission patterns. Rust regex does not support look-around.

For a successful evaluation, print `true` or `false` followed by a newline to standard output, leave standard error empty, and exit with status `0`. A nonmatch is a successful evaluation. Invalid arguments or regexes produce a diagnostic on standard error, leave standard output empty, and exit with status `1`. Standalone help describes the invocation and result semantics on standard output and exits with status `0`. Diagnostic wording is not a compatibility guarantee.

The pattern matcher accepts inert strings only. It does not read settings, evaluate permission precedence or defaults, compare configurations, create files, or make network requests. A match is not a configured permission decision or evidence of runtime network access.

## Check a Pattern

Run from the repository root, with the pattern before the input:

```sh
cargo run --locked --quiet \
    --bin domfiles-zed-settings-pattern-match -- \
    '<case-sensitive>' '<pattern>' '<input>'
```

Read the pattern and `case_sensitive` value from the settings being checked rather than copying a documented regex. Compare the printed Boolean with the expected match, not merely the exit status. Separately [resolve the configured decision and native checks](agent-permissions.md#resolve-effective-permission-behavior).

## Check Zed Regex Compatibility

When changing the repository’s `regex` pin or investigating a regex discrepancy with Zed, manually compare the root `Cargo.toml` pin and its resolved direct dependency in `Cargo.lock` with the resolved version in the relevant Zed revision’s `Cargo.lock`. Use that release or revision rather than automatically following `main`. Retrieve only the needed official source, and report a verification limitation if it is unavailable.

Matching direct dependency versions does not establish identical fetch behavior or matching transitive dependencies. A mismatch is evidence to investigate, not authorization to change dependencies.

## Run Focused Contract Tests

```sh
cargo test --locked --test domfiles-zed-settings-pattern-match-test
```

Keep the adjacent `pattern_match.test.rs` focused on argument handling, help agreement, case-sensitivity wiring and inline overrides, literal and empty inputs, and result and error behavior. These tests validate the wrapper contract, not the checked-in fetch policy or live Zed behavior. Run them when changing the matcher, then select root checks through the [skill-owned script validation policy](../../../../skills/.domfiles-skill-development/references/skill-owned-scripts.md#test-contracts).
