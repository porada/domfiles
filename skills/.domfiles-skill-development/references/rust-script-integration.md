# Rust Script Integration

Follow an explicit repository build policy before choosing a route. A policy that registers all skill-owned Rust scripts with Cargo also governs standard-library-only commands. Otherwise, use the direct compilation route below for standard-library-only scripts and Cargo for scripts with non-standard-library dependencies.

## Compile Directly

When the direct compilation route applies:

- Compile the script directly with stable `rustc` and compile its adjacent test with `rustc --test`.
- Include both compile commands in root static validation. Do not register Cargo targets merely because the repository otherwise uses Cargo.
- Pass the repository’s supported Rust edition explicitly. Pass an explicit valid crate name when the repository’s filename pattern contains characters that Rust crate names do not accept.
- Resolve compiled binaries and other transient output through the [ephemeral artifact rule](script-artifact-boundaries.md#bound-artifact-locations).

## Build With Cargo

When the Cargo route applies:

- Keep target registration and applicable root manifests committed, follow the repository’s lockfile policy, and keep generated `target/` contents ignored and uncommitted.
- Let a root package own the targets in a package root workspace. In a virtual workspace, let a package member own them because the virtual manifest cannot define targets.
- Use Cargo through an existing repository-owned tooling package or, when the current task authorizes it, one shared tooling package for skill-owned scripts.
- Follow the repository’s target naming policy. Without one, use repository-unique, skill-qualified target names. Use the repository’s normal shared Cargo target directory. Do not override `CARGO_TARGET_DIR` merely to isolate a script.
