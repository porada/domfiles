# Rust Script Integration

For standard-library-only Rust scripts:

- Compile the script directly with stable `rustc` and compile its adjacent test with `rustc --test`.
- Include both compile commands in root static validation. Do not register Cargo targets merely because the repository otherwise uses Cargo.
- Pass the repository’s supported Rust edition explicitly. Pass an explicit valid crate name when the repository’s filename pattern contains characters that Rust crate names do not accept.
- Resolve compiled binaries and other transient output through the [ephemeral artifact rule](script-artifact-boundaries.md#bound-artifact-locations).

When a Rust script requires a non-standard-library dependency:

- Keep target registration and applicable root manifests committed, follow the repository’s lockfile policy, and keep generated `target/` contents ignored and uncommitted.
- Let a root package own the targets in a package root workspace. In a virtual workspace, let a package member own them because the virtual manifest cannot define targets.
- Use Cargo through an existing repository-owned tooling package or, when the current task authorizes it, one shared tooling package for skill-owned scripts.
- Use repository-unique, skill-qualified target names and the repository’s normal shared Cargo target directory. Do not override `CARGO_TARGET_DIR` merely to isolate a script.
