# Command form and location

## Choose the narrowest command surface

- Before choosing a repository location, load `fish-shell-scripting` and apply its wrapper-selection and execution-boundary policies.
- After that check, default a command used only through Fish to `.config/fish/functions/<command-name>.fish`.
- When an executable entrypoint becomes an autoloaded Fish function, place the function in its matching file without a hashbang or executable bit. Preserve its documentation and observable contract, and migrate every caller before removing the entrypoint.
- When the selected execution boundary is the current Fish process, the resulting function placement takes precedence over a desired `domfiles` or Git subcommand spelling.
- Add `bin/<command-name>` only when the selected execution boundary requires an executable, including invocation by a non-Fish caller or before Fish is available.
- Put a `domfiles` subcommand at `bin/domfiles-<command-name>`. For a Git subcommand, follow [Git helper form](#choose-git-helper-form) before using `bin/git-<command-name>`.

## Choose Git helper form

- Before adding or reviewing a `bin/git-*` entrypoint, inspect `.config/git/config` and determine whether a plain Git alias preserves the required behavior.
- Prefer a plain Git alias when the helper invokes one Git subcommand with fixed options and relies on Git’s normal argument forwarding.
- Keep a script when the behavior requires shell control flow, dynamic values, safety checks, external commands, or shared `domlib` behavior.
- If a Git alias would require the `!` shell-command form, implement it as a `bin/git-*` script instead. Never define shell commands inside `.config/git/config` aliases.
- Do not retain a script solely for custom argument-count validation unless strict arity is required behavior.
