# Fish completions

## Place completion definitions correctly

- Name a completion file after the command it completes and place it in an appropriate directory from `$fish_complete_path`.
- For software installation, use Fish’s vendor completion directory rather than writing into a user’s configuration directory. Resolve the vendor path through the installation environment instead of hardcoding a platform path.
- Keep completion loading free of observable side effects. Completion files can be sourced while Fish is discovering candidates.

## Define the command-line contract

Choose the registration target that matches how Fish resolves the completed command:

- Use `complete --command <command>` for a command name.
- Use `complete --path <absolute-path>` for an absolute path target, optionally containing wildcards.

Describe each option or operand from the target command’s verified interface.

- Distinguish short options, long options, old-style options, required arguments, optional arguments, and positional operands accurately.
- Use conditions when completion applicability depends on state that must be evaluated at completion time. That state may come from the current command line, variables, command availability, the filesystem, or another runtime source. Keep each condition quiet and fast.
- Disable or force file completion deliberately. Do not suppress file candidates merely because custom candidates also exist.
- Use wrapped-command completion only when the wrapper preserves the delegated command’s relevant interface.

Descriptions supplied to `complete` are human-facing technical copy. Apply the entrypoint’s `human-facing-writing` route and keep terminology aligned with the command’s help output.

## Control evaluation timing

`complete --arguments` receives one Fish expression string. Fish tokenizes and expands that string when it generates candidates.

- For static candidates, quote or escape inside the stored expression, not merely around the argument passed to `complete`. `--arguments 'alpha beta'` defines two candidates, while `--arguments 'alpha\ beta'` defines one candidate containing a space.
- For dynamic candidates, pass the command substitution literally so it runs at completion time:

```fish
complete --command example --arguments '$(example candidates)'
```

- Do not write `--arguments "$(example candidates)"`. It runs the generator as the definition loads, then stores the resulting text as a Fish expression that Fish tokenizes and expands again at completion time. If a definition-time snapshot is required, serialize it into an escaped stored expression explicitly.
- Distinguish escaping that preserves the stored Fish expression from escaping data that the completed command will later interpret.

## Generate candidates safely

- Prefer static candidates when the set is fixed.
- Generate dynamic candidates only from bounded, side-effect-free commands suitable for interactive latency.
- Emit one candidate per line from a dynamic command substitution. When using tab-separated candidate descriptions, ensure candidate values and descriptions cannot introduce ambiguous separators.
- Use `string`, `path`, and list operations to transform candidate data instead of importing POSIX word splitting.
- Do not call undocumented helpers whose names begin with `__fish_`. A prefixed helper is allowed only when official Fish documentation exposes it as a supported interface.

## Validate completions

- Parse and format the completion file through the project workflow.
- Exercise representative command lines with `complete --do-complete`, including empty input, partial options, `--`, option arguments, and paths containing whitespace.
- Confirm that descriptions, conditions, file-completion behavior, and wrapped-command behavior match the target command.
- Check interactive latency when candidate generation runs external commands.

## Official sources

The Fish behavior underlying this guidance is documented in the official [completion guide](https://fishshell.com/docs/current/completions.html), [`complete` reference](https://fishshell.com/docs/current/cmds/complete.html), and [Fish language](https://fishshell.com/docs/current/language.html).
