# POSIX-native idioms

Choose the data representation and process boundary before choosing syntax. Preserve argument, text, pathname, state, and status contracts at every boundary.

## Choose the data boundary

The table moves from shell-managed values to boundaries that require a stream or file.

| Data | Representation | Boundary |
| --- | --- | --- |
| Argument vector | Positional parameters | Expand with `"$@"` |
| Scalar text | One shell variable | Quote every ordinary expansion |
| Arbitrary pathnames | Separate arguments | Use globs or `find -exec … {} +` |
| Line-oriented text | Standard input or a file | Read with `IFS= read -r` |
| Exact or opaque data | A pipe or file | Do not route it through a shell variable |

Shell variables cannot contain NUL bytes. Command substitution removes trailing newlines and executes in a subshell environment, so variable, directory, option, and trap changes inside it do not affect the caller. A newline-delimited stream cannot represent arbitrary pathnames. Do not change representations merely to make an operation syntactically convenient.

## Build argument vectors

Treat `"$@"` as the shell’s argument-vector container. It expands to one field per positional parameter and preserves empty arguments. Quoted `"$*"` joins all positional parameters into one field using the first character of `IFS`. Unquoted `$@` and `$*` perform field splitting and pathname expansion and do not preserve the original vector.

Build a command and its arguments with `set --`, then execute `"$@"`. Do not store a command or argument list in a scalar variable, reconstruct one through unquoted expansion, or use `eval` to recover boundaries that the representation discarded.

```sh
lint_named_shell_files() (
  if [ $# -ne 2 ] || [ -z "$1" ] || [ -z "$2" ]; then
    printf 'Usage: %s <directory> <name-pattern>\n' "${0##*/}" >&2
    return 2
  fi

  directory=$1
  name_pattern=$2

  case $directory in
    /* | ./* | ../*)
      ;;
    *)
      directory=./"$directory"
      ;;
  esac

  set -- find

  if [ -L "$directory" ]; then
    set -- "$@" -H
  fi

  set -- "$@" "$directory" -type f -name "$name_pattern" -exec shellcheck --shell=sh {} +
  "$@"
)
```

`-exec … {} +` batches matching pathnames as separate arguments to `shellcheck` rather than serializing them.

Validate positional cardinality before assigning required parameters. Leave `$#` unquoted in numeric tests. Own one mutable positional-parameter vector at a time. Each function invocation temporarily replaces the positional parameters and restores the caller’s parameters when it returns. An ordinary helper function can therefore use `set --` to own a separate vector without creating a process boundary.

## Validate parameters

The operator chooses what happens when the expansion is triggered. A colon widens the trigger from an unset value to an unset or empty value.

| Triggered effect | Unset only | Unset or empty |
| --- | --- | --- |
| Expand and substitute the provided text | `${parameter-provided text}` | `${parameter:-provided text}` |
| Expand the provided text, write it to standard error, and exit a noninteractive shell | `${parameter?provided text}` | `${parameter:?provided text}` |

Here, `parameter` stands for the parameter name, and `provided text` stands for the shell text after the operator. The space in `provided text` is deliberate. This text is not limited to one whitespace-delimited token. If the trigger does not apply, every form expands to the parameter’s current value.

Use `${optional-}` when absence is valid under `set -u`. Quote parameter expansions and command substitutions unless field splitting or pathname expansion is intentional and bounded. Use double quotes where expansion may occur and single quotes for literals containing characters that would otherwise need escaping. Treat every unquoted expansion as an explicit parsing operation.

Do not encode a list in a space- or newline-delimited scalar. That representation cannot preserve empty elements and makes the result depend on `IFS` and pathname expansion. Restrict an `IFS` change to the smallest applicable scope when a loop list comes from an unquoted parameter expansion, command substitution, or arithmetic expansion. Literal words and pathname-expansion results do not require that change.

## Write conditions and arithmetic

Put a command directly in `if`, `while`, or `until` when its status is the condition. Keep each `test` or `[ … ]` invocation to one predicate, then compose predicates with the shell’s `!`, `&&`, and `||` syntax. Avoid `test`’s ambiguous `-a` and `-o` operators, and use `=` rather than `==`.

Use `case` for patterns, enumerated values, and numeric input validation. Validate numeric syntax and the target-supported range before arithmetic expansion. The following example accepts decimal values from `0` through `999` without leading zeros, leaving room for the increment to produce `1000`.

```sh
case $count in
  [0-9] | [1-9][0-9] | [1-9][0-9][0-9])
    ;;
  *)
    printf 'Invalid count: %s\n' "$count" >&2
    exit 2
    ;;
esac

count=$((count + 1))
```

Arithmetic expansion is POSIX. The arithmetic command form `((…))` is not available unless the target contract explicitly permits it.

## Isolate function state

POSIX does not define `local`. Use a subshell-bodied function when assignments, directory changes, shell options, traps, file descriptors, or the file-creation mask must not affect the caller.

```sh
run_make_in_directory() (
  if [ $# -lt 1 ] || [ -z "$1" ]; then
    printf 'Usage: %s <directory> [<target>…]\n' "${0##*/}" >&2
    return 2
  fi

  directory=$1
  shift

  case $directory in
    /* | ./* | ../*)
      ;;
    *)
      directory=./"$directory"
      ;;
  esac

  CDPATH='' cd "$directory" || return 1
  command make -- "$@"
)
```

When a function intentionally modifies caller state, make that behavior part of its interface and namespace every shared variable. Avoid generic library variables such as `result`, `status`, `file`, or `tmp`.

Define a function as `name() { …; }`, not with another shell’s `function` syntax. Never prefix a POSIX shell function or special built-in invocation with a variable assignment. Use an explicit assignment inside a subshell when a function needs temporary variable state. Assignment prefixes remain available for regular built-ins and external commands when their normal temporary environment is intended.

## Keep sourced interfaces explicit

Use the `.` command, not `source`. A sourceable file should normally define functions and documented variables without parsing the caller’s positional parameters, installing global traps, changing directories, enabling shell options, or running its main operation. Permit those effects only when they are an explicit part of the sourced interface and every caller is evaluated against them.

Keep reusable library definitions separate from an executable entrypoint when combining them would require a nonportable test for whether the file was sourced or executed.

## Separate output and status

Treat standard output, standard error, and exit status as separate interfaces unless the receiving contract says otherwise. The interfaces progress from returned data through diagnostics to outcome.

| Interface       | Default role       |
| --------------- | ------------------ |
| Standard output | Returned data      |
| Standard error  | Diagnostics        |
| Exit status     | Success or failure |

A value-producing function writes only its value to standard output. Prefer direct status control flow such as `if command` or `command || fallback` when only success or failure matters. Capture a numeric status before logging, cleanup, or another command overwrites it. Use `return` from a function and `exit` from an executed script. Leave `$?` unquoted when passing it directly to `exit`.

An assignment containing command substitution receives the substitution’s status, so this form can capture a scalar value and enforce failure together:

```sh
value=$(produce_value) || exit
```

Use it only when command substitution’s trailing-newline removal and scalar representation preserve the output contract.

## Produce output portably

Use `printf`, not `echo`, for program output. Pass externally influenced data as an operand to a fixed format string.

```sh
printf '%s\n' "$value"
printf '%s: %s\n' "${0##*/}" "$message" >&2
```

Do not use an externally influenced value as the format string.

## Parse options deliberately

Use `getopts` for a conventional portable short-option interface. Begin the option specification with `:` when the command owns parser diagnostics, reset `OPTIND=1` when a reusable function parses more than once in one shell, and shift by `OPTIND - 1` after parsing.

Parse required long options with an explicit `case` loop that defines `--`, missing option values, unknown options, and the first positional operand. Do not use the external `getopt` utility as a portable parser.

Validate option relationships and positional cardinality after parsing. Protect externally supplied operands with `--` only when the receiving utility supports it. Prefer a dedicated option such as `grep -e "$pattern"` when an operand could otherwise be parsed as an option. Prefix a relative pathname with `./` when the utility has no suitable option boundary.

## Read text lines

Read line-oriented text with `IFS= read -r`, and capture its status immediately. Status `1` indicates EOF, while a greater status indicates an error. Clear `line` before each `read` so an unsuccessful call cannot reuse a record from the preceding iteration. Accept a nonempty value returned with status `1` only when a final unterminated line is valid input. Keep each line-reading loop body as short as possible so the read boundary remains easy to follow. Move longer processing into a named helper.

In an executable entrypoint, open the file on a dedicated descriptor before the loop. This handles an open failure without placing the `while` command in an AND-OR list, where `set -e` would be ignored throughout the loop. The descriptor also prevents a command inside the body from consuming the loop’s next line through standard input. Close it after the loop, and propagate a saved `read` error. Inside a function, use `return` instead of `exit`.

```sh
exec 3< "$file" || exit

while
  line=
  IFS= read -r line <&3
  read_status=$?
  [ "$read_status" -eq 0 ] \
    || { [ "$read_status" -eq 1 ] && [ -n "$line" ]; }
do
  process_line "$line"
done

exec 3<&-

if [ "$read_status" -gt 1 ]; then
  exit "$read_status"
fi
```

Do not iterate over `$(cat "$file")`. Command substitution removes trailing newlines, field splitting discards line boundaries, and pathname expansion can reinterpret the resulting text.

## Preserve pathname boundaries

Never serialize arbitrary pathnames through newline-delimited output. For direct traversal, a shell glob preserves each match as one argument:

```sh
for file in "$directory"/*.json; do
  [ -f "$file" ] || continue
  process_file "$file"
done
```

Before either recursive form, normalize a relative search root so that an unqualified pathname beginning with `-`, `!`, or `(` cannot be parsed as an option or expression:

```sh
case $root in
  /* | ./* | ../*)
    ;;
  *)
    root=./"$root"
    ;;
esac
```

Use `find -exec … {} +` when it expresses the operation:

```sh
find "$root" -type f -exec ./bin/process-files {} +
```

When an inline shell is necessary, use a fixed script and reserve `$0` so pathnames begin at `$1`. Never interpolate `{}` into shell source.

```sh
find "$root" -type f -exec sh -c '
  for file do
    [ -r "$file" ] || exit 1
  done
' sh {} +
```

## Recognize non-POSIX forms

Use this table as a syntax boundary. The preceding sections own the behavior behind each positive form. Rows are alphabetized by need.

| Need | POSIX form | Reject unless the target contract permits it |
| --- | --- | --- |
| Alias and function bypass | `command <name>` with direct status handling | Treating `command -v` as a matching preflight |
| Any invocable command | `command -v <name>`, then ordinary invocation | `which` or invoking through `command` after that check |
| Arithmetic | `$((…))` | `((…))` or `let` |
| Combined redirection | `>file 2>&1` | `&>` |
| Conditions | `[ … ]`, `test`, or `case` | `[[ … ]]` |
| Function declaration | `name() { …; }` | `function name` |
| Function-state isolation | A subshell-bodied function | `local` |
| Generated input | A pipe or redirection | Here-strings or process substitution |
| Load shell code | `.` | `source` |
| Mutable argument vector | `set --` and `"$@"` | Arrays |
| Option parsing | `getopts` or an explicit `case` loop | External `getopt` or shell-specific parsers |
| Wait for a child | Capture `$!`, then `wait "$pid"` | `wait -n` without established support |

A feature’s presence in a newer POSIX edition does not establish availability in every target implementation. Verify syntax and utility behavior against every established target before using a target-gated form.

## Use utilities selectively

Prefer parameter expansion, `case`, and arithmetic expansion when their exact edge-case semantics match the operation. Use an established external utility when its documented behavior is the required interface. Do not replace clear shell logic mechanically, and do not force structured parsing, binary processing, or a complex algorithm into shell merely to avoid an established suitable tool.

Use `command -v <name>` when any command that normal shell resolution would invoke may satisfy the dependency, then invoke `<name>` normally. When an alias or function must be bypassed, invoke `command <name>` and handle that invocation’s status instead of treating `command -v` as a matching preflight. `command` may still select a built-in, so it is not an external-only resolver. Do not reset `PATH` indiscriminately because doing so can hide intentionally installed dependencies.

Before a programmatic `cd`, reject an empty operand. Preserve absolute paths and explicit relative paths beginning with `./` or `../`. Prefix every other relative path with `./` so option-like names and `-` remain directory operands. Invoke `CDPATH='' cd "$directory"` to disable directory search and emitted path output. Add `-P` only when physical-path behavior is required. Scope `LC_ALL=C` to commands that deliberately need bytewise sorting, matching, or character classes rather than changing user-facing behavior globally.

Evaluate each external command and option against the target utility set separately from shell-language syntax. Do not assume GNU behavior merely because the shell syntax is portable.

## Preserve pipeline contracts

Treat a pipeline’s status as the final command’s status unless every target establishes another mechanism. Do not assume an earlier failure propagates. Do not depend on variable, directory, trap, or function state changed inside a pipeline component after that process exits.

Avoid feeding command output into a loop through a bare pipeline when upstream failure detection or post-loop state matters. A `printf` pipeline into `while` is acceptable when input generation cannot fail meaningfully and loop state need not persist.

When an optional command in strict mode produces either a usable nonempty scalar or no usable output on failure, scope `|| true` inside the command substitution before testing the result:

```sh
value="$(optional-command || true)"
[ -z "$value" ] && value="$(fallback-command)"
```

Use status control flow instead when successful empty output or partial output on failure must remain distinguishable.

Do not probe and enable an optional `pipefail` mode when correctness depends on it. Either establish that every target provides the required semantics or restructure the operation so each important status is checked independently.

## Justify temporary resources

Use the first boundary that preserves the contract. The order moves from direct transfer to durable staging:

1. Pass separate arguments directly.
2. Stream through standard input, standard output, a pipe, or redirection.
3. Use command substitution for suitable scalar text.
4. Reuse a destination or work directory the operation already owns.
5. Create a temporary resource only when the receiving interface requires a pathname, exact data cannot survive a shell variable, the operation needs multiple passes, statuses must be checked independently, or atomic replacement or rollback requires staging.

Do not introduce a temporary resource merely because it makes quoting or control flow easier.

When temporary storage is necessary and the target provides `mktemp`, keep its creation, use, and cleanup in one subshell-isolated operation. Set `umask 077` immediately before creating a private directory under `${TMPDIR:-/tmp}`. Do not change the mask for an entire executable merely because the entrypoint owns cleanup. Do not check whether a predictable pathname is unused and then create it because that sequence has a race.

## Own cleanup and background jobs

Let the executable entrypoint own global cleanup traps. A sourceable library should release resources before returning rather than replace the caller’s trap configuration. Keep trap bodies simple and preserve the operation’s status deliberately.

Capture `$!` immediately after each background command. Wait for each recorded PID explicitly and define how multiple failures combine. `wait` without operands does not provide a useful per-operation failure policy.

## Official sources

The [POSIX Shell Command Language](https://pubs.opengroup.org/onlinepubs/9799919799/utilities/V3_chap02.html) defines expansion, execution, functions, pipelines, and shell state. The [Utility Syntax Guidelines](https://pubs.opengroup.org/onlinepubs/9799919799/basedefs/V1_chap12.html) define the option conventions that individual utilities may adopt. Use each utility’s POSIX reference together with target implementation evidence before relying on its options or edge-case behavior.
