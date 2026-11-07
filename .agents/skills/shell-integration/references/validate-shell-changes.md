# Validate Shell Changes

After editing, use the narrowest applicable validation scope:

1. Pass changed paths explicitly to the matching lint wrapper. Omit paths only when repository-wide validation is intended. Explicit paths bypass [default discovery and its pathname limits](../../../PROJECT.md#development-lint-wrapper-architecture). Explicitly pass paths that Git C-quotes. When no paths are supplied, file-oriented wrappers discover tracked files and non-ignored untracked files. The Fish wrapper restricts that inventory to `*.fish` files. The POSIX wrapper uses `*.sh`, `home/.local/bin/*`, and `home/.local/share/*` to include extensionless entrypoints and `domlib`. File-oriented wrappers skip non-files and symlinks.
2. For Fish, run `pnpm run lint:fish <changed-fish-files>`, respecting the [repository scope exclusions](../../../../AGENTS.md#scope). The wrapper already runs `fish --no-execute`, so don’t repeat that check.
3. For POSIX shell, run `sh -n -- <file>` for each changed file and `pnpm run lint:sh <changed-posix-files>`. The wrapper supplies the complementary ShellCheck analysis. For POSIX hooks without hashbangs under `.hooks`, use `env SHELLCHECK_OPTS=--shell=sh pnpm run lint:sh <changed-hook-files>`.
4. For changed `.fish` and `.sh` files and extensionless Fish and POSIX shell scripts with hashbangs, check formatting with `pnpm exec prettier --check <changed-shell-files>`. The configured Fish plugin infers extensionless Fish scripts from their `fish` hashbang, so don’t force `--parser fish`. For POSIX hooks without hashbangs under `.hooks`, run a separate invocation of the formatting command with `--parser sh` and only those hook paths.
5. [Check integration contracts](#check-integration-contracts).
6. Run `git --no-pager diff --check` and, when task-owned changes are staged, `git --no-pager diff --cached --check`. Inspect task-owned untracked files directly without staging them because Git diff doesn’t include them.

## Check Integration Contracts

For affected shell targets, verify:

- Caller migration and contract documentation when command form changes under [Command Form and Location](command-form-and-location.md).
- Caller use of shared defaults and paired-helper alignment under [Shared Helper Design](shared-helper-design.md).
- Entrypoint sourcing, helper placement and ordering, contract comments, repository-wide usage, and required `$DOMFILES_*` counterparts under [`domlib` Integration](domlib-integration.md).
