# `domlib` Integration

## Inspect Shared State

When `domlib` or `home/.config/fish/config.fish` is relevant, inspect both files before evaluating shared variables or functions.

## Integrate POSIX Entrypoints

- Treat every domfiles shell script not written in Fish as a POSIX `sh` target.
- Ensure every POSIX shell entrypoint sources `domlib`. Exempt `.hooks` scripts. Treat `bin/domlib` as the shared library rather than an entrypoint, and keep strict mode there so sourced scripts inherit it.

## Maintain `domlib`

- Keep all functions defined in `domlib` alphabetized in natural order.
- When a `domlib` function changes, keep its adjacent contract comment aligned with the resulting behavior.
- Whenever a reusable `domlib` helper or its Fish counterpart is in scope, follow [shared helper design](shared-helper-design.md).
- Require every `$DOMFILES_*` variable defined in Fish configuration to have a same-named counterpart in `domlib`. Variables defined only in `domlib` require no Fish counterpart.

## Apply Domlib Reporting Rules

- Search repository-wide call sites before reporting a `domlib` function or variable as unused. More than one call site is sufficient reuse and must not be reported on usage-count grounds.
- Report unused functions or variables defined in `domlib`.
    - Do not treat a `domlib` variable as unused when it exists solely as the required counterpart to a Fish-defined variable.
- Report every POSIX shell function prefixed with `__` when it is defined outside `domlib`.

## Apply Domlib-Specific POSIX Conventions

- Apply portable continuation policy only to executable POSIX shell code. `domlib` contract comments retain their 80-column limit.
- Parse user-supplied values for domfiles-authored boolean environment variables with `__read_boolean_from_env` at the input boundary. Keep the supported-value sets owned by POSIX `__normalize_boolean` and Fish `home/.config/fish/functions/__domfiles_normalize_boolean.fish` rather than repeating them in policy or project documentation. Third-party environment variables remain outside this rule.
- Use `__suppress <command>` rather than an assignment-prefixed function invocation to suppress command echo for one command. See [suppressed command output](../../../PROJECT.md#suppressed-command-output).
    - Never wrap `__domfiles_exec` in `__suppress`. The subshell would absorb its `exec` and let the caller resume. Omit that function’s opt-in `--print` flag instead.

## Validate `domlib`

When `domlib` is in scope, verify every function has an adjacent contract comment and its comment prose stays within 80 columns.
