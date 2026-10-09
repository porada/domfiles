# Domfiles Script Policy

Apply these requirements only to skill-owned scripts maintained in domfiles. Compose them with the [shared script contract](skill-owned-scripts.md).

## Choose Implementation Language

Write project-authored skill scripts in Rust. Use another language only when a concrete ecosystem, interoperability, runtime, or tooling constraint makes it materially more correct, maintainable, or proportionate than Rust.

Record the exception and its durable reason in the owning skill before implementation. Avoiding migration, existing language use, familiarity, or shorter syntax alone does not justify an exception.

## Expose Portable Commands

Treat skill-owned Rust scripts as [portable commands](portable-skill-scripts.md) by default, regardless of the owning skill’s installation category. Expose each command through a thin `home/.local/bin/<command-name>` wrapper under the shared [ownership contract](skill-owned-scripts.md#keep-ownership-clear).

## Build With Cargo

Register every skill-owned Rust script and its adjacent contract tests in the root Cargo package, including standard-library-only commands. Follow the [Cargo build contract](rust-script-integration.md#build-with-cargo), including its shared output directory requirement.

Wrappers build from the domfiles root, then execute the compiled command in the caller’s directory.

## Align Command Names

Derive Rust source stems from the exposed CLI name using `snake_case`, with `.rs` for source and `.test.rs` for adjacent tests.

Name Cargo binary targets `<command-name>` and their test targets `<command-name>-test`, using the exposed CLI name without an additional prefix. Keep Cargo target names repository-unique. Help and command diagnostics use the exposed CLI name.

Keep established Cargo target and CLI names unchanged when only source filenames change.
