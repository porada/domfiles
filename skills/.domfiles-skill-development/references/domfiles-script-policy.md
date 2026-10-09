# Domfiles Script Policy

Apply these requirements only to skill-owned scripts maintained in domfiles. Compose them with the [shared script contract](skill-owned-scripts.md).

## Choose Implementation Language

Write project-authored skill scripts in Rust. Use another language only when a concrete ecosystem, interoperability, runtime, or tooling constraint makes it materially more correct, maintainable, or proportionate than Rust.

Record the exception and its durable reason in the owning skill before implementation. Avoiding migration, existing language use, familiarity, or shorter syntax alone does not justify an exception.

## Expose Portable Commands

Treat skill-owned Rust scripts as [portable commands](portable-skill-scripts.md) by default, regardless of the owning skill’s installation category. Expose each command through a thin `home/.local/bin/<command-name>` wrapper under the shared [ownership contract](skill-owned-scripts.md#keep-ownership-clear).

## Build With Cargo

Register every skill-owned Rust script and its adjacent contract tests in the root Cargo package, including standard-library-only commands. Follow the [Cargo build contract](rust-script-integration.md#build-with-cargo), including its shared output directory requirement.

## Align Command Names

Derive Rust source stems from the exposed CLI name using `snake_case`, with `.rs` for source and `.test.rs` for adjacent tests.

Name Cargo binary targets `<command-name>` and their test targets `<command-name>-test`, using the exposed CLI name without an additional prefix. Keep Cargo target names repository-unique. Help and command diagnostics use the exposed CLI name.

Keep established Cargo target and CLI names unchanged when only source filenames change.

## Run Command Wrappers

The `home/.local/bin/<command-name>` wrappers for skill-owned Rust commands source `home/.local/share/domlib` and call `__domfiles_cargo_target_exec` with the target name and original arguments. The shared launcher requires the reachable domfiles checkout, its prescribed Rust toolchain, and `jq`.

It runs `cargo build --bin <command-name> --locked --message-format=json-render-diagnostics --quiet` from the domfiles root, so an unrelated caller’s directory configuration does not select the build. It reads the selected target’s executable path from Cargo’s messages with `jq`, then runs that executable with the original arguments in the caller’s directory. Do not infer an executable location beneath `target/`.

Build preparation follows the [portable command boundary](portable-skill-scripts.md#apply-common-command-contract). It may write host build outputs and caches before help or argument validation. Read-only guarantees for the compiled operation do not cover build preparation. Build diagnostics go to standard error rather than the command’s data stream.

Cargo failures preserve Cargo’s exit status, such as `101`. Unreadable Cargo messages or a missing reported executable produce a launcher diagnostic on standard error and exit with status `1`. Executable launch failures can return other statuses. Once the executable runs, its own documented stream and status contract applies. Determine the failure stage from the diagnostic, not the status alone.

## Validate Command Wrappers

Apply the [general launcher checks](portable-skill-scripts.md#validate-portability) to each Rust command wrapper, with these domfiles-specific assertions:

- Record the Cargo invocation and assert the domfiles root as its working directory and the exact arguments documented in [Run Command Wrappers](#run-command-wrappers), including their order. Invoke the wrapper from an unrelated directory with conflicting Cargo configuration.
- Have Cargo report a stub executable that prints `pwd -P`, then assert that the wrapper prints the caller’s physical directory. This verifies both executable selection and runtime working directory without assuming Cargo’s output location.
- Return a nonzero Cargo status, such as `101`, and assert that the wrapper preserves it.

Apply the [shared refusal coverage rule](skill-owned-scripts.md#test-contracts) to `__domfiles_cargo_target_exec`: keep its refusal cases for unreadable Cargo messages and missing executables in `claude-signal-test`, with the shared wiring assertions above in every wrapper’s suite. Run that shared failure coverage alongside the affected wrapper tests when changing the launcher.
