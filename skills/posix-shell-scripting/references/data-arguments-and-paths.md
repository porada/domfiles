# Data, Arguments, and Paths

Choose a representation according to what the data must preserve. POSIX shell handles argument vectors and scalar text directly. Streams and files take over when those forms cannot carry the required boundaries or bytes.

## Data Boundaries

The available representations progress from shell-managed values to stream and file boundaries.

| Data | Representation | Boundary |
| --- | --- | --- |
| Argument vector | Positional parameters | Expand with `"$@"` |
| Scalar text | One shell variable | Quote every ordinary expansion |
| Arbitrary pathnames | Separate arguments | Use globs or `find -exec … {} +` |
| Line-oriented text | Standard input or a file | Read with `IFS= read -r` |
| Exact or opaque data | A pipe or file | Do not route it through a shell variable |

Shell variables cannot contain NUL bytes. Command substitution removes trailing newlines and runs in a subshell environment, so changes to variables, directories, options, and traps do not reach the caller. Newline-delimited text cannot represent arbitrary pathnames.

Choose the boundary that preserves the contract rather than the one that makes the next command shortest.

## Argument Vectors

Treat `"$@"` as the shell’s argument-vector container. It expands to one field per positional parameter and preserves empty arguments. Quoted `"$*"` joins the parameters into one field using the first character of `IFS`. Unquoted `$@` and `$*` perform field splitting and pathname expansion, so they cannot preserve the original vector.

Build a command and its arguments with `set --`, then execute `"$@"`. Do not store a command or argument list in a scalar, reconstruct one through unquoted expansion, or use `eval` to recover boundaries that the representation has already discarded.

```sh
search_named_files() (
  if [ $# -ne 3 ] || [ -z "$1" ] || [ -z "$2" ] || [ -z "$3" ]; then
    printf 'Usage: %s <directory> <name-pattern> <search-pattern>\n' "${0##*/}" >&2
    return 2
  fi

  directory=$1
  name_pattern=$2
  search_pattern=$3

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

  set -- "$@" "$directory" -type f -name "$name_pattern" -exec grep -n -e "$search_pattern" {} +
  "$@"
)
```

`-exec … {} +` gives `grep` each matching pathname as a separate argument instead of serializing the matches.

Iterate over `"$@"` when inspection must leave the original vector intact. This rebase helper can discover `--autosquash` and the requested commit count while preserving every argument for a later command:

```sh
autosquash=false
count_param=''

for param in "$@"; do
  if [ "$param" = '--autosquash' ]; then
    autosquash=true
  elif [ -z "$count_param" ]; then
    count_param=$param
  fi
done
```

Use a separate rebuilding pass when arguments must be removed, inserted, or reordered. Record the original argument count before shifting or appending, process only that many original arguments, then execute the rebuilt vector. Without that boundary, an appended argument can be mistaken for original input.

Validate positional cardinality before assigning required parameters. Leave `$#` unquoted in numeric tests. Own one mutable positional-parameter vector at a time. A function invocation temporarily replaces the positional parameters and restores the caller’s parameters when it returns, so an ordinary helper function can use `set --` without adding a process boundary.

## Parameter Validation

The operator controls both the trigger and the result. Adding `:` expands the trigger from an unset parameter to an unset or empty parameter.

| Triggered effect | Unset only | Unset or empty |
| --- | --- | --- |
| Expand and substitute the provided text | `${parameter-provided text}` | `${parameter:-provided text}` |
| Expand the provided text, write it to standard error, and exit a noninteractive shell | `${parameter?provided text}` | `${parameter:?provided text}` |

`parameter` stands for the parameter name. `provided text` stands for the shell text after the operator, and the space in the placeholder is deliberate. The text may contain more than one whitespace-delimited token. When the trigger does not apply, each form expands to the parameter’s current value.

Use `${optional-}` when absence is valid under `set -u`. Quote parameter expansions and command substitutions unless field splitting or pathname expansion is both intentional and bounded. Use double quotes where expansion may occur and single quotes for literals that would otherwise require escaping. Treat every unquoted expansion as an explicit parsing operation.

Do not encode a list in a space- or newline-delimited scalar. It cannot preserve empty elements, and its result depends on `IFS` and pathname expansion. Restrict an `IFS` change to the smallest applicable scope when a loop list comes from an unquoted parameter expansion, command substitution, or arithmetic expansion. Literal words and pathname-expansion results do not require that change.

## Conditions and Arithmetic

Put a command directly in `if`, `while`, or `until` when its status is the condition. Keep each `test` or `[ … ]` invocation to one predicate, then compose predicates with the shell’s `!`, `&&`, and `||` syntax. Avoid the ambiguous `-a` and `-o` operators, and use `=` rather than `==`.

Do not create an `if`, `elif`, or loop branch solely to hold a no-op command. Prefer a negated condition or an early `continue`, `return`, or `exit` so each remaining branch performs required work while preserving the [output and status contract](functions-and-interfaces.md#output-and-status). Empty `case` arms may represent accepted patterns without a command.

When shell grammar genuinely requires a no-op command, use `true` for success or `false` for failure instead of `:`. Account for `false` under `set -e`. Reserve `:` for parameter-expansion or redirection side effects.

Use `case` for patterns, enumerated values, and numeric validation. Validate both the syntax and the target-supported range before arithmetic expansion. This example accepts decimal values from `0` through `999` without leading zeros, leaving room for the increment to produce `1000`:

```sh
case $count in
  [0123456789] | [123456789][0123456789] | [123456789][0123456789][0123456789])
    ;;
  *)
    printf 'Count must be a decimal integer from 0 through 999 without leading zeros: %s\n' \
      "$count" >&2
    exit 2
    ;;
esac

count=$((count + 1))
```

Arithmetic expansion is POSIX. The arithmetic command form `((…))` requires an explicit target extension.

## Option Parsing

Use `getopts` for a conventional portable short-option interface. Start the option specification with `:` when the command owns parser diagnostics. Reset `OPTIND=1` when a reusable function may parse more than once in the same shell, then shift by `OPTIND - 1` after parsing.

Parse required long options with an explicit `case` loop. Define the behavior for `--`, missing option values, unknown options, and the first positional operand. Do not use the external `getopt` utility as a portable parser.

Validate option relationships and positional cardinality after parsing. Protect externally supplied operands with `--` only when the receiving utility supports it. Prefer a dedicated option such as `grep -e "$pattern"` when an operand could otherwise be parsed as an option. Prefix a relative pathname with `./` when the utility has no suitable option boundary.

## Line-Oriented Input

Read line-oriented text with `IFS= read -r`, and capture its status immediately. Status `1` indicates EOF. A greater status indicates an error. Clear `line` before each call so an unsuccessful `read` cannot reuse the previous record. Accept a nonempty value returned with status `1` only when a final unterminated line is valid input.

Keep the loop body short enough that the read boundary remains visible. Move longer processing into a named helper.

In an executable entrypoint, open the file on a dedicated descriptor before the loop when a command in the body may consume standard input. Treat `read` and line processing as independent failure sources. When the loop owns a dedicated descriptor, treat opening and closing it as additional failure sources, and close it after the loop.

Capture each processing or input/output error status immediately. Stop processing when the interface requires it, and propagate statuses according to the interface’s stated precedence and the [output and status contract](functions-and-interfaces.md#output-and-status). Do not rely on `set -e` to supply that contract.

Do not iterate over `$(cat "$file")`. Command substitution removes trailing newlines, field splitting discards line boundaries, and pathname expansion can reinterpret the resulting text.

## Pathname Boundaries

Never serialize arbitrary pathnames as newline-delimited text. To count visible regular files in an already validated directory, place the glob directly in the loop so the shell passes each pathname intact:

```sh
file_count=0

for file in "$directory"/*; do
  [ -f "$file" ] || continue
  file_count=$((file_count + 1))
done

printf '%s\n' "$file_count"
```

Before recursive traversal, normalize a relative search root so an unqualified pathname beginning with `-`, `!`, or `(` cannot be parsed as an option or expression:

```sh
case $root in
  /* | ./* | ../*)
    ;;
  *)
    root=./"$root"
    ;;
esac
```

Use `find -exec … {} +` when it expresses the operation directly:

```sh
find "$root" -type f -exec ./bin/check-license-headers {} +
```

When an inline shell is necessary, use fixed shell source and reserve `$0` so pathnames begin at `$1`. Never interpolate `{}` into the shell source.

```sh
find "$root" -type f -exec sh -c '
  for file do
    [ -r "$file" ] || exit 1
  done
' sh {} +
```
