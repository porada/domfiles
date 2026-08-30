# Functions and Interfaces

Choose each function, sourcing, and process boundary according to the state and interfaces that must cross it.

## Function State

POSIX does not define `local`. Use a subshell-bodied function when assignments, directory changes, shell options, traps, file descriptors, or the file-creation mask must not affect the caller.

`exit` executed while a subshell-bodied function’s own subshell is the active execution environment terminates that function subshell. On direct invocation of the function, the resulting status returns to its caller, whose own error handling may still terminate the caller’s shell. An `exit` in a nested subshell or command substitution terminates only that nested environment. This containment describes process behavior and does not create an exception to the function-level [output and status rule](#output-and-status).

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

When a function intentionally changes caller state, make that behavior part of its interface and namespace every shared variable. Avoid generic library variables such as `result`, `status`, `file`, or `tmp`.

Declare a function as `name() { …; }`, not with another shell’s `function` syntax. Do not prefix a POSIX shell function or special built-in invocation with a variable assignment. When a function needs temporary variable state, assign it explicitly inside a subshell. Assignment prefixes remain available for regular built-ins and external commands when their normal temporary environment is intended.

## Sourced Interfaces

Use `.`, not `source`. A sourceable file should normally define functions and documented variables without parsing the caller’s positional parameters, installing global traps, changing directories, enabling shell options, or running its main operation. Allow those effects only when the sourced interface declares them and every caller is evaluated against them.

Separate reusable library definitions from an executable entrypoint when combining them would require a nonportable test for whether the file was sourced or executed.

Do not assume `$0` identifies the entrypoint file. When `sh` finds a slashless command file through `PATH`, POSIX still sets `$0` to the original command-file operand. Establish the library location through an installation contract, a caller-provided interface, or an invocation contract that guarantees `$0` denotes the actual file. Define the contract’s symlink behavior as well.

ShellCheck’s static source route serves a different consumer. Add one only after the source belongs to the resolved validation scope, and keep it aligned with the runtime location contract.

## Output and Status

Treat standard output, standard error, and exit status as separate interfaces unless the receiving contract says otherwise. They progress from returned data through diagnostics to outcome.

| Interface       | Default role       |
| --------------- | ------------------ |
| Standard output | Returned data      |
| Standard error  | Diagnostics        |
| Exit status     | Success or failure |

A value-producing function writes only its value to standard output. When only success or failure matters, prefer direct status control flow such as `if command` or `command || fallback`. Capture a numeric status before logging, cleanup, or another command overwrites it. Use `return` from every function, including a subshell-bodied function, and `exit` from an executed script. Leave `$?` unquoted when passing it directly to `exit`.

Some commands use a nonzero status for a domain result rather than an operational error. Classify every documented status, and propagate unexpected values. Do not put the command behind `!` when the original status matters because `!` replaces it with the inverted result.

For example, `git merge-base --is-ancestor` returns `0` when the upstream commit is an ancestor of `HEAD`, `1` when it is not, and another status when the comparison fails:

```sh
upstream_update_required=false
merge_base_status=0

git merge-base --is-ancestor '@{u}' HEAD || merge_base_status=$?

if [ "$merge_base_status" -eq 1 ]; then
  upstream_update_required=true
elif [ "$merge_base_status" -ne 0 ]; then
  printf '%s\n' 'Failed to compare the local and upstream revisions' >&2
  exit "$merge_base_status"
fi
```

An assignment containing command substitution receives the substitution’s status. This form can capture scalar output and enforce failure together:

```sh
repository_root=$(git rev-parse --show-toplevel) || exit
```

Use this form only when trailing-newline removal and a scalar representation preserve the command’s output contract.

## Portable Output

Use `printf`, not `echo`, for program output. Pass externally influenced data as an operand to a fixed format string.

```sh
printf '%s\n' "$repository_root"
printf '%s: %s\n' "${0##*/}" "$diagnostic" >&2
```

Never use an externally influenced value as the format string.

## Terminal Destinations

Test the file descriptor whose behavior will change. Use `[ -t 0 ]` for standard-input decisions, `[ -t 1 ]` for standard-output decisions, and `[ -t 2 ]` for diagnostics. One terminal descriptor does not imply that either of the others also refers to a terminal.

Use a terminal test only when the script itself changes behavior. Gate script-controlled standard-output paging, color, and terminal-fit calculations on `[ -t 1 ]`. Do not duplicate a command’s own automatic terminal behavior.

When standard output is not a terminal, avoid terminal-size queries that cannot improve the redirected result.
