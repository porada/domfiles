# Fish-Native Idioms

## Variable Scope and State

Fish treats scope and exportedness as separate properties. Use `set` to create, update, export, scope, query, and erase variables. Do not write bare assignments except for the supported single-command `NAME=value command` override when that exact lifetime is intended.

- Explicitly scope the assignment that introduces important state. After that declaration, an unscoped `set` may intentionally update the narrowest existing variable.
- Use `set --local` for a value confined to the current block and `set --function` for one needed across blocks in the current function.
- Use `set --global` for session state shared by functions in the current Fish process.
- Use `set --universal` only when state must persist across sessions and synchronize between Fish processes.
- Add `--export` only when child processes require the value. Uppercase names conventionally identify exported variables.

Choose the narrowest check that establishes the required property, from definition through content:

| Required Property           | Check                              |
| --------------------------- | ---------------------------------- |
| Variable is defined         | `set --query <name>`               |
| At least one element exists | `set --query <name>[1]`            |
| Exact element count         | `test $(count $value) -eq <count>` |
| Joined content is nonempty  | `test -n "$value"`                 |

An undefined variable, a defined empty list, and a list containing one empty string are different states. Do not pass an unquoted, potentially empty list as the only input to `string length --quiet`. If the list expands to zero arguments, the command reads piped or redirected standard input instead. Use `set --erase <name>` to remove a variable or list element. `set -e` is shorthand for erase, not POSIX-style error handling.

When Fish code owns the representation of stored boolean state and neither an applicable policy nor the user selects another form, use the literal values `true` and `false`. Initialize the variable before use, compare it explicitly with `=`, and do not encode owned boolean state through unset or empty values or `0` and `1`.

A single-command override such as `LC_ALL=C command` is valid Fish syntax. Use it only when the value must exist for that command alone. Fish applies the override before expanding the rest of the command line.

## Argument Lists

Every Fish variable is a one-dimensional list. Keep ordinary command arguments in that representation from construction through execution. Store one logical argument per element, then expand the list unquoted when the receiving command should get those elements separately.

Use `$argv` for positional arguments and whole-list forwarding. Use `count` instead of `$#`, and do not use `$1`, `$@`, `$*`, or arrays from another shell.

A command and its fixed arguments use the same representation:

```fish
set --local editor emacs --no-window-system
$editor README.md
```

Do not route ordinary arguments through `eval`. Use it only when generated Fish syntax, such as a pipeline or compound construct, must be parsed again. Use 1-based indices and slices such as `$items[1]`, `$items[2..-1]`, and `$items[-1]`. Do not rely on `$IFS` for ordinary variable expansion because Fish performs no post-expansion word splitting.

## Expansion Cardinality

Decide how many arguments an expansion may produce before combining it with other text.

Use quotes for Fish semantics rather than visual consistency. Leave literal tokens unquoted when Fish parses them identically. Use single quotes for literal text that must remain unexpanded. Use double quotes when interpolation must remain one argument. Do not require or restore quotes that `fish_indent` removes without changing semantics.

Quote an expansion when the receiving command must get exactly one argument. A double-quoted empty or undefined variable becomes one empty argument. A quoted multi-element list joins with spaces, while a quoted path variable joins with colons.

Adjacent list expansions form a cartesian product. Attached text combines with every element, while an empty unquoted list can remove the entire token. For pairwise operations, require equal list lengths and index both lists explicitly because adjacent expansions do not zip lists.

Before attaching text to a sensitive value, establish the required element count and content. For example, require one nonempty root before constructing a path:

```fish
test $(count $root) -eq 1
or return 2
test -n "$root"
or return 2

set --local target "$root/cache"
```

## Text and Record Boundaries

Choose whether command output represents lines, one opaque document, or delimited records before capturing it. Use `$(command)` for command substitution, including inside double quotes. Use `string split` or `string split0` when another delimiter defines records. Use `string collect` when output must be collected without newline splitting.

```fish
set --local lines $(command tool)
set --local document "$(command tool)"
set --local exact_document $(
    command tool |
        string collect --allow-empty --no-trim-newlines
)
```

A normal command substitution splits on newlines and produces no elements for empty output. A final terminating newline does not create another empty element. A quoted substitution produces exactly one argument but still trims trailing newlines. A final `string collect --allow-empty --no-trim-newlines` preserves empty output as one element and retains trailing newlines.

Treat JSON, SQL, generated source, and similar opaque documents as text rather than line lists unless their interface says otherwise.

Use NUL-delimited streams when records can contain newlines, especially for filenames:

```fish
find . -type f -print0 |
    while read --null file
        process_file $file
    end

set --local files $(
    find . -type f -print0 |
        string split0
)
```

Keep `string split0` as the final pipeline stage when collecting a NUL stream into a Fish list so its element boundaries survive command substitution. Use `path`’s `--null-in` and `--null-out` options while NUL-delimited data remains a stream. Do not send NUL output directly to a terminal or command substitution. Pipe it to a final `string split0` when collecting it.

Direct `path` output captured by command substitution preserves item boundaries, including embedded newlines. An intervening command can serialize those boundaries away. Ordinary `path` standard input remains newline-delimited unless NUL input is selected or detected.

## Purpose-Built Operations

Do not replace an external command mechanically. Use a Fish builtin when it expresses the required semantics without losing portability or behavior. The tables group operations under three headings: Fish Data, Shell Boundaries, and Input and Command State. Entries within each group are alphabetical by need.

### Fish Data

| Need | Prefer | Avoid When Fish Owns the Operation |
| --- | --- | --- |
| Count arguments or list elements | `count` | `$#`, scalar counters, `wc -w` |
| Inspect or transform paths | `path` | Routine `basename`, `dirname`, `realpath`, or string slicing |
| Inspect or transform strings | `string` | `${…}` operators or routine `grep`, `sed`, `tr`, or `awk` pipelines |
| Perform arithmetic | `math` | `$((…))`, `((…))`, `expr` |

### Shell Boundaries

| Need | Prefer | Avoid When Fish Owns the Operation |
| --- | --- | --- |
| Inspect shell, command, or script context | `status` | `$0` or shell-specific context variables |
| Manage path-list additions | `fish_add_path` or list-valued path variables | Manual colon concatenation |
| Parse function or script options | `argparse` | `getopts`, `getopt`, or hand-written option shifting |
| Read Fish’s process ID | `$fish_pid` | `$$` or another shell’s PID variable |

### Input and Command State

| Need | Prefer | Avoid When Fish Owns the Operation |
| --- | --- | --- |
| Read structured input | `read` with an explicit delimiter or tokenization mode | Non-Fish `read` flags or implicit `$IFS` assumptions |
| Resolve an external path despite shadowing | `type --force-path` | Assuming `type --path` bypasses functions |
| Resolve any command Fish would invoke | `type --query` | `which` |
| Resolve external program availability | `command --query` | Accepting a function or builtin by mistake |
| Test list membership | `contains` | Regex or loop-based membership checks |

When a verified command interface supports it, place `--` after fixed options and before externally supplied positional arguments.

## Command Conditions

Put a command directly after `if` or `while`, and invert its status with `not`. Prefer a structured block over a long `and` or `or` chain, and close every block with `end`. Use `test` for scalar checks, `string` for string checks, `path` for filesystem checks, `contains` for membership, and `type` for command resolution. Write explicit checks such as `test -n "$value"` rather than the ambiguous one-argument `test "$value"` form.

Use `switch` for pattern-based branches. Fish executes the first matching `case` and has no fallthrough.

Recognize `&&`, `||`, `!`, and `$()` as valid Fish syntax during review. For an equivalent two-command status dependency in new or materially rewritten code, write `command; and next` or `command; or fallback` instead of `&&` or `||`, and use `not` instead of `!`. Preserve semantics and precedence rather than replacing symbolic forms mechanically.

## Output and Failure Contracts

Fish has no direct equivalent of `set -euo pipefail`. Do not add that option sequence or invent a blanket strict mode.

Use `return` from a function and `exit` from a script. Preserve a failing status deliberately instead of allowing a logging or cleanup command to overwrite it. Treat stdout as returned data and stderr as diagnostics unless the receiving interface defines another contract. An `argparse` validator is an exception because it writes its diagnostic fragment to stdout for `argparse` to consume.

Capture one command substitution’s output and status together when both matter:

```fish
set --local output $(command tool $argv)
or return
```

Assignment-mode `set` preserves the status of its final command substitution. Copy that status immediately when logging, cleanup, or another command must run before returning it.

Put a required command directly in `if` or `while`, or use `command; or return` when failure should end the current function. Handle optional failure at the operation that permits it instead of suppressing a broad region of code.

Inspect `$pipestatus` only when individual pipeline stages matter. Do not reinterpret every nonzero upstream status as whole-pipeline failure.

After a pipeline, `$status` is the pipeline result. It normally comes from the final foreground process and then reflects any `not` or `!` negation. `$pipestatus` contains one unnegated status per pipeline process. Inspect or copy these values immediately.

Fish deliberately has no `pipefail` mode because an early-closing consumer can make an upstream process report `SIGPIPE` even when the pipeline is semantically correct.

## Paths

Treat variables whose names end in `PATH` as lists internally and as colon-delimited values only when quoted or exported. Derive script-relative paths from `status filename` or `status dirname` and `path` operations rather than the caller’s working directory.

## Globs

Use `*` for one path segment and `**` when recursive descent is intended. Do not introduce `?` globs because current Fish treats `?` as an ordinary character by default.

Expect an ordinary unmatched glob to stop the command with a nonzero status. An unmatched glob expands to zero arguments when it is an argument to `set`, `path`, `count`, or `for`, or when it appears in the value of a single-command variable override. The override does not exempt other globs in the same command.

Quote a wildcard that the called program or remote system must interpret. Do not store a wildcard in a variable expecting Fish to expand it later because Fish does not re-glob expanded variables.

## Redirections

Use `2>` for standard error. Use `&>` or explicit descriptor redirections only when combining output streams is intentional. Preserve their order because Fish evaluates redirections from left to right after establishing a pipe.

## Processes

Prefer a pipe when a consumer accepts standard input. Use `$(producer | psub)` only when the consumer requires a filename. Use `begin … end` to group commands for redirection or scope, knowing that it does not create a subprocess. Use an explicit `fish --command '<code>'` only when process isolation is required. Replace heredocs with a pipe, `printf`, or a quoted multiline string according to the receiving command’s interface.

## Official Sources

Fish’s overall design is documented in the official [design principles](https://fishshell.com/docs/current/design.html). Expansion and state behavior are documented in the [Fish language](https://fishshell.com/docs/current/language.html), [`set` reference](https://fishshell.com/docs/current/cmds/set.html), [`string collect` reference](https://fishshell.com/docs/current/cmds/string-collect.html), and [`string split0` reference](https://fishshell.com/docs/current/cmds/string-split0.html).

Path, input, and command-resolution behavior are documented in the [`path` reference](https://fishshell.com/docs/current/cmds/path.html), [`read` reference](https://fishshell.com/docs/current/cmds/read.html), [`command` reference](https://fishshell.com/docs/current/cmds/command.html), and [`type` reference](https://fishshell.com/docs/current/cmds/type.html). Generated Fish syntax is covered by the [`eval` reference](https://fishshell.com/docs/current/cmds/eval.html).
