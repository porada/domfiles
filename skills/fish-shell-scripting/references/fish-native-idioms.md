# Fish-native idioms

## Choose variable scope and state explicitly

Fish treats scope and exportedness as separate properties.

- Explicitly scope the assignment that introduces important state. After that declaration, an unscoped `set` may intentionally update the narrowest existing variable.
- Use `set --local` for a value confined to the current block and `set --function` for a value needed across blocks in the current function.
- Use `set --global` for session state that functions in the current Fish process must share.
- Use `set --universal` only when state must persist across sessions and synchronize between Fish processes.
- Add `--export` only when child processes require the value. Uppercase names conventionally identify exported variables.

Choose the narrowest predicate that establishes the required property, from definition through content:

| Required property           | Check                              |
| --------------------------- | ---------------------------------- |
| Variable is defined         | `set --query <name>`               |
| At least one element exists | `set --query <name>[1]`            |
| Exact element count         | `test $(count $value) -eq <count>` |
| Joined content is nonempty  | `test -n "$value"`                 |

An undefined variable, a defined empty list, and a list containing one empty string are distinct states. Do not pass an unquoted potentially empty list to `string length --quiet` because zero operands can make it consume piped or redirected standard input. Use `set --erase <name>` to remove a variable or list element. `set -e` is shorthand for erase, not POSIX-style error handling.

A single-command override such as `LC_ALL=C command` is valid Fish syntax. Use it only when the value must exist for exactly that command. Fish applies the override before expanding the rest of the command line.

## Construct commands as argument lists

Every Fish variable is a one-dimensional list. Keep ordinary command arguments in that representation from construction through execution. Set one logical argument per element, then expand the list unquoted when the receiving command should get those elements separately.

A command and its fixed arguments use the same representation:

```fish
set --local editor emacs --no-window-system
$editor README.md
```

- Do not route ordinary argv through `eval`. Use `eval` only when generated Fish syntax itself, such as a pipeline or compound construct, must be parsed again.
- Use 1-based indices and slices such as `$items[1]`, `$items[2..-1]`, and `$items[-1]`.
- Do not rely on `$IFS` for ordinary variable expansion. Fish performs no post-expansion word splitting.

## Control expansion cardinality

Decide how many arguments an expansion may produce before composing it with other text.

- Quote an expansion when the receiving command must get exactly one argument. A double-quoted empty or undefined variable becomes one empty argument, and a quoted multi-element list joins with spaces. A quoted path variable joins with colons.
- Adjacent list expansions form a cartesian product. Attached text combines with every element, while an empty unquoted list can remove the complete token.
- For pairwise operations, require equal list lengths and index both lists explicitly. Adjacent expansions do not zip lists.
- Before attaching text to a sensitive value, establish the cardinality and content the operation requires. For example, require one nonempty root before constructing a path:

```fish
test $(count $root) -eq 1
or return 2
test -n "$root"
or return 2

set --local target "$root/cache"
```

## Preserve text and record boundaries

Choose whether command output represents lines, one opaque document, or delimited records before capturing it:

```fish
set --local lines $(command tool)
set --local document "$(command tool)"
set --local exact_document $(
    command tool |
        string collect --allow-empty --no-trim-newlines
)
```

A normal command substitution splits on newlines, produces no elements for empty output, and does not create an additional empty element for the final terminating newline. A quoted substitution produces exactly one argument but still trims trailing newlines. A final `string collect --allow-empty --no-trim-newlines` preserves empty output as one element and retains trailing newlines. Treat JSON, SQL, generated source, and similar opaque documents as text rather than line lists unless their interface says otherwise.

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

- Keep `string split0` as the final pipeline stage when collecting a NUL stream into a Fish list so its element boundaries survive command substitution.
- Use `path`’s `--null-in` and `--null-out` options while NUL-delimited data remains a stream. Do not send NUL output directly to a terminal or command substitution. Pipe it to a final `string split0` when collecting it.
- Direct `path` output captured by command substitution preserves item boundaries, including embedded newlines. An intervening command can serialize those boundaries away, and ordinary `path` standard input remains newline-delimited unless NUL input is selected or detected.

## Select the purpose-built operation

Do not replace an external command mechanically. Select the Fish builtin when it expresses the required semantics without losing portability or behavior. The tables group operations by data manipulation, shell boundaries, and queries. Within each group, entries are alphabetical by need.

### Manipulate Fish data

| Need | Prefer | Avoid when Fish owns the operation |
| --- | --- | --- |
| Count arguments or list elements | `count` | `$#`, scalar counters, `wc -w` |
| Inspect or transform paths | `path` | routine `basename`, `dirname`, `realpath`, or string slicing |
| Inspect or transform strings | `string` | `${…}` operators, routine `grep`, `sed`, `tr`, or `awk` pipelines |
| Perform arithmetic | `math` | `$((…))`, `((…))`, `expr` |

### Cross shell boundaries

| Need | Prefer | Avoid when Fish owns the operation |
| --- | --- | --- |
| Inspect shell, command, or script context | `status` | `$0` or shell-specific context variables |
| Manage path-list additions | `fish_add_path` or list-valued path variables | Manual colon concatenation |
| Parse function or script options | `argparse` | `getopts`, `getopt`, hand-written option shifting |
| Read Fish’s process ID | `$fish_pid` | `$$` or another shell’s PID variable |

### Query input and command state

| Need | Prefer | Avoid when Fish owns the operation |
| --- | --- | --- |
| Read structured input | `read` with an explicit delimiter or tokenization mode | Non-Fish `read` flags or implicit `$IFS` assumptions |
| Resolve an external path despite shadowing | `type --force-path` | Assuming `type --path` bypasses functions |
| Resolve any command Fish would invoke | `type --query` | `which` |
| Resolve external program availability | `command --query` | Accepting a function or builtin by mistake |
| Test list membership | `contains` | regex or loop-based membership checks |

When a verified command interface supports it, place `--` after fixed options and before externally supplied positional operands.

## Use command-oriented conditions

- Put a command directly after `if` or `while`, and invert its status with `not`.
- Use `test` for scalar predicates, `string` for string predicates, `path` for filesystem predicates, `contains` for membership, and `type` for command resolution.
- Write explicit predicates such as `test -n "$value"`. Avoid the ambiguous one-argument `test "$value"` form.
- Use `switch` for pattern-based branches. Fish executes the first matching `case` and has no fallthrough.

## Define output and failure boundaries

Fish has no direct equivalent of `set -euo pipefail`. Do not add that option sequence or invent a blanket strict mode.

- Use `return` from a function and `exit` from a script. Preserve a failing status deliberately rather than allowing a logging or cleanup command to overwrite it.
- Treat stdout as returned data and stderr as diagnostics unless the receiving interface defines another contract. For example, an `argparse` validator writes its diagnostic fragment to stdout because `argparse` consumes it.
- Capture one command substitution’s output and status together when both matter:

```fish
set --local output $(command tool $argv)
or return
```

Assignment-mode `set` preserves the status of its last command substitution. Copy that status immediately when later logging, cleanup, or another command must run before returning it.

- Put a required command directly in `if` or `while`, or use `command; or return` when its failure should end the current function.
- Handle optional failure at the operation that permits it instead of suppressing a broad region of code.
- Inspect `$pipestatus` only when individual pipeline stages matter. Do not reinterpret every nonzero upstream status as whole-pipeline failure.

After a pipeline, `$status` is the pipeline result. It normally derives from the final foreground process and then reflects any `not` or `!` negation. `$pipestatus` contains one unnegated status per pipeline process. Inspect or copy them immediately. Fish deliberately has no `pipefail` mode because early-closing consumers can make upstream processes report `SIGPIPE` without the pipeline being semantically wrong.

## Handle paths deliberately

- Treat variables whose names end in `PATH` as lists internally and colon-delimited values only when quoted or exported.
- Derive script-relative paths from `status filename` or `status dirname` and `path` operations rather than the caller’s working directory.

## Handle globs deliberately

- Use `*` for a path segment and `**` when recursive descent is intended. Do not introduce `?` globs because current Fish treats `?` as an ordinary character by default.
- Expect an ordinary unmatched glob to stop the command with a nonzero status. An unmatched glob expands to zero arguments when it appears as an argument to `set`, `path`, `count`, or `for`, or in the value of a single-command variable override. The override does not exempt other globs in the same command.
- Quote a wildcard that the called program or remote system must interpret.
- Do not store a wildcard in a variable expecting it to be expanded later. Fish does not re-glob expanded variables.

## Compose redirections

- Use `2>` for standard error.
- Use `&>` or explicit descriptor redirections only when combining output streams is intentional. Preserve ordering because Fish evaluates redirections from left to right after establishing a pipe.

## Compose processes

- Prefer a pipe when a consumer accepts standard input. Use `$(producer | psub)` only when it requires a filename.
- Use `begin … end` to group commands for redirection or scope. It does not create a subprocess.
- Use an explicit `fish --command '<code>'` only when process isolation is actually required.
- Replace heredocs with a pipe, `printf`, or a quoted multiline string according to the receiving command’s interface.

## Official sources

Expansion and state behavior are documented in the official [Fish language](https://fishshell.com/docs/current/language.html), [`set` reference](https://fishshell.com/docs/current/cmds/set.html), [`string collect` reference](https://fishshell.com/docs/current/cmds/string-collect.html), and [`string split0` reference](https://fishshell.com/docs/current/cmds/string-split0.html).

Path, input, and command-resolution behavior are documented in the [`path` reference](https://fishshell.com/docs/current/cmds/path.html), [`read` reference](https://fishshell.com/docs/current/cmds/read.html), [`command` reference](https://fishshell.com/docs/current/cmds/command.html), and [`type` reference](https://fishshell.com/docs/current/cmds/type.html). Generated Fish syntax is covered by the [`eval` reference](https://fishshell.com/docs/current/cmds/eval.html).
