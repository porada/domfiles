# Fish Completions

Fish loads completion definitions on demand when it discovers candidates. Match the target command’s verified interface, defer dynamic work until completion time, keep loading free of side effects, and keep candidate generation bounded for interactive use.

## File Placement

Name each autoloaded completion file `<command>.fish` and place it in a directory from `$fish_complete_path`. For software installation, use Fish’s vendor completion directory rather than a user configuration directory. Resolve that directory through the installation environment instead of hardcoding a platform path. Keep completion loading free of observable side effects because Fish may source the file while discovering candidates.

## Registration Contract

Choose the registration target that matches how Fish resolves the completed command. Use `complete --command <command>` for a command name and `complete --path <absolute-path>` for an absolute path target, which may contain wildcards.

Model every option and operand from the target command’s verified interface. Represent short, GNU-style long, and old-style options accurately, distinguishing required arguments, optional arguments, and positional operands. Use a condition only when applicability depends on state evaluated at completion time, such as the current command line, variables, command availability, or the filesystem, and keep each condition quiet and fast.

Disable or force file completion deliberately because custom candidates do not disable file candidates by themselves. Use wrapped-command completion only for a command-name target whose relevant interface matches the delegated command. Fish ignores wrapping for `complete --path`.

Descriptions supplied to `complete` are human-facing technical copy. Apply the [human-facing text contract](../SKILL.md#human-facing-text) and use the target command’s help terminology.

## Evaluation Timing

`complete --arguments` receives one Fish expression string. Fish tokenizes and expands that stored expression when it generates candidates.

For static candidates, quote or escape inside the stored expression, not merely around the argument passed to `complete`. `--arguments 'alpha beta'` defines two candidates, while `--arguments 'alpha\ beta'` defines one candidate containing a space.

For dynamic candidates, pass the command substitution literally so it runs at completion time:

```fish
complete --command example --arguments '$(example candidates)'
```

Do not write `--arguments "$(example candidates)"`. It runs the generator while the definition loads, then stores the output as an expression that Fish tokenizes and expands again at completion time. When a definition-time snapshot is intentional, serialize it explicitly as an escaped stored expression.

Keep the two escaping boundaries distinct. First preserve the stored Fish expression, then escape any data that the completed command will interpret later.

## Candidate Generation

Prefer static candidates when the set is fixed. Generate dynamic candidates only with bounded, side-effect-free commands suitable for interactive latency. Emit one candidate per line from a dynamic command substitution. For tab-separated descriptions, ensure neither candidate values nor descriptions can introduce ambiguous separators.

Use `string`, `path`, and list operations to transform candidate data instead of importing POSIX word splitting. Do not call an undocumented helper whose name begins with `__fish_`. Use one only when official Fish documentation exposes its behavior for completion authors.

## Validation

- Exercise representative command lines with `complete --do-complete`, including empty input, partial options, `--`, option arguments, and paths containing whitespace.
- Confirm that descriptions, conditions, file-completion behavior, and wrapped-command behavior match the target command.
- Check interactive latency when candidate generation runs external commands.

## Official Sources

Use Fish’s official [completion guide](https://fishshell.com/docs/current/completions.html) for authoring and placement, the [`complete` reference](https://fishshell.com/docs/current/cmds/complete.html) for registration and candidate behavior, and the [Fish language](https://fishshell.com/docs/current/language.html) for expression, expansion, and escaping semantics.
