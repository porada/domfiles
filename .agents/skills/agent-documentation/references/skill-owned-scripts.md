# Skill-owned scripts

A skill-owned script is a small program that gathers recurring repository evidence, produces declared artifacts, or does both for one project-authored skill. Use one when it makes the workflow more deterministic, efficient, or repeatable than a sequence of manual operations. Do not turn a narrow skill need into a general repository utility.

## Choose the operation route

- Use a read route to gather and report evidence without creating or updating repository artifacts.
- Use a write route to produce or update declared artifacts when the current task permits that mutation.
- Combine reading and writing in one bounded invocation when that avoids duplicate collection work.
- Let the agent choose the least expensive permitted route. Do not require a separate evidence pass before every write.

A script with both read and write modes must require an explicit write selection such as `--write` or `--output`. A writer-only generator does not need a redundant mode flag when its name and help text make the mutation clear.

Generating a declared artifact is not a repair. When evidence indicates that authoritative source, configuration, or documentation should change, the owning skill must define a separate repair branch and enter it only when the current request authorizes those changes. Findings alone do not authorize repair.

## Keep ownership clear

- Store each script and its adjacent contract test directly under `<skill>/scripts`. Do not create a per-script directory for a single script-and-test pair. Use a subdirectory only when an established repository layout or multiple supporting source or fixture files make a flat pair insufficient.
- Let the skill own the source, tests, purpose, invocation, operation routes, artifact contract, and repair workflow.
- Let the repository root own toolchain configuration, dependencies, static validation, optional Cargo integration, and repository build-output policy.
- Do not give the scripts directory its own package, crate, manifest, TypeScript configuration, lockfile, or workspace membership.
- Keep multiple related scripts directly in the same scripts directory rather than creating infrastructure around each one.

## Make the interface discoverable

- Support `--help` and describe the script’s purpose, modes, accepted inputs, output behavior, destination selection, overwrite policy, and relevant exit behavior.
- Keep each operation bounded and deterministic for the same repository state and inputs.
- Choose human-readable or structured output according to the consumer’s needs. Do not require JSON without a consumer that needs it.
- Do not invent an output filename in the current directory. Use a declared repository destination or require the caller to supply one.

## Bound artifact locations

Every filesystem write must remain within one of these authorized categories:

- A declared repository artifact at its canonical path, whether intentionally tracked or stored in an established repository-owned output location.
- An explicit file or directory supplied by the caller.
- A short-lived sibling used exclusively to atomically replace an otherwise authorized destination.

Do not repurpose a location merely because it is ignored. Never modify `.gitignore` while running the script, and do not add an ignore rule solely to accommodate script-specific output. If repository-wide toolchain output exposes a missing ignore policy, handle that as a separate repository configuration change under the current task’s authorization.

For ephemeral artifacts, follow the applicable temporary-file policy. Accept the resolved task-specific destination from the caller instead of establishing a separate temporary-output convention.

Before writing:

- Confirm that the resolved destination remains within the authorized location. Reject traversal or symlink redirection outside it.
- Do not write Git metadata or files unrelated to the declared artifact contract.
- Inspect the status and diff of repository destinations, preserving user or concurrent changes.
- Replace an existing path only when it is a declared generated artifact or the current request explicitly authorizes overwriting it.
- Leave byte-identical output unchanged.

## Write artifacts safely

- When atomic replacement requires a same-filesystem temporary file, use a short-lived sibling as an internal implementation detail and remove it after success or a handled failure. The applicable temporary-file policy continues to govern ephemeral task artifacts.
- Remove stale paths only when they belong to a declared script-owned output set and exact synchronization is part of the documented contract.
- Do not provide a generic `--force` or `--fix` escape hatch that bypasses ownership or destination checks.
- Report every artifact created, updated, unchanged, or removed.

## Test the contracts

- Pair each script with an adjacent test file that documents its behavior and edge cases.
- Use the language’s native test framework when it is sufficient. Use `node:test` for JavaScript or TypeScript and Rust’s built-in `#[test]` harness for Rust.
- Test applicable read and write modes, destination resolution, overwrite refusal, unchanged output, cleanup, and failure behavior.
- Keep durable repository-owned fixture inputs narrow and deterministic under `<skill>/scripts`. Contain runtime-created fixture outputs, repositories, and scratch state through the applicable temporary-file policy.
- Run focused tests and the repository’s root static validation. Direct execution and focused tests do not replace root typechecking or compilation.

Document focused script and test commands in the owning skill or its repair reference.

Prefer standard-library and repository-owned dependencies. Obtain explicit user approval before adding a third-party runtime package or Rust crate, then declare an approved dependency through the repository root rather than beside the script. Declare host-language types needed for static validation through the root toolchain as well.

## Integrate root validation

When a script reveals that root validation omits a source category, extend the root contract for that complete category rather than hardcoding one skill path. For example, add `.agents/**` to a TypeScript repository’s root include patterns when they do not cover skill-owned sources. Ensure the root check covers both the script and its test. Do not broaden configuration prospectively before a real script establishes the need.

For standard-library-only Rust scripts:

- Compile the script directly with stable `rustc` and compile its adjacent test with `rustc --test`.
- Pass the repository’s supported Rust edition explicitly. Pass an explicit valid crate name when the repository’s filename pattern contains characters that Rust crate names do not accept.
- Keep testable behavior in functions that the adjacent test can load from the script instead of duplicating the implementation.
- Resolve compiled binaries and other transient output through the applicable temporary-file policy.
- Include both compile commands in root static validation. Do not register Cargo targets merely because the repository otherwise uses Cargo.

When a Rust script requires a non-standard-library dependency:

- Use Cargo through an existing repository-owned tooling package or, when the current task authorizes it, one shared tooling package for skill-owned scripts.
- Let a root package own the targets in a package-root workspace. In a virtual workspace, let a package member own them because the virtual manifest cannot define targets.
- Keep the script and its test under the owning skill. Do not create a manifest, package, or workspace member per skill or script.
- Use repository-unique, skill-qualified target names and the repository’s normal shared Cargo target directory. Do not override `CARGO_TARGET_DIR` merely to isolate a script.
- Keep target registration and applicable root manifests committed, follow the repository’s lockfile policy, and keep generated `target/` contents ignored and uncommitted.

## Resolve file names

Resolve the filename stem and test suffix independently before considering language-native naming:

1. Preserve an explicit user-selected path or applicable project instruction for the current pair. Treat it as a broader convention only when the user or project policy says so.
2. Follow an existing script-and-test pair in the same skill unless it is documented as exceptional.
3. Treat a pattern shared by at least two project-authored skills as cross-skill precedent.
4. Derive the stem style from repository-owned standalone scripts and executable helpers. Derive test-suffix placement from sidecar tests, including tests written in another language. Treat a single pair from another skill as supporting evidence rather than an automatic winner. When applicable patterns conflict, prefer files with the same role and closest scope, then ask only when equally applicable evidence remains unresolved.
5. When no repository pattern governs the pair, use the portable fallback `script-name.<extension>` and `script-name.test.<extension>`.

Treat skill-owned scripts as repository tooling rather than ordinary language modules. Do not infer language-native filenames merely from a manifest, package, or file extension. Use a language-native alternative only when existing repository files, explicit project policy, or a tooling constraint requires it. The presence of Cargo alone does not establish Rust-native filename conventions.
