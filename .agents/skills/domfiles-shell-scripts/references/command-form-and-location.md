# Command form and location

## Choose the narrowest command surface

- Before choosing a repository location, load `fish-shell-scripting` and apply its wrapper-selection and execution-boundary policies.
- After that check, default a command used only through Fish to `.config/fish/functions/<command-name>.fish`.
- When the selected execution boundary is the current Fish process, the resulting function placement takes precedence over a desired `domfiles` or Git subcommand spelling.
- Add `bin/<command-name>` only when the selected execution boundary requires an executable, including invocation by a non-Fish caller or before Fish is available.
- Put a `domfiles` subcommand at `bin/domfiles-<command-name>`. For a Git subcommand, follow [Git helper form](../SKILL.md#choose-git-helper-form) before using `bin/git-<command-name>`.
