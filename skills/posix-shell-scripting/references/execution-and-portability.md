# Execution and Portability

Evaluate syntax, utilities, and process boundaries as separate contracts.

## POSIX Syntax Boundaries

Use this table to distinguish portable forms from target-gated extensions. The routed guidance for each need owns the behavior behind its POSIX form. Rows are alphabetized by need.

| Need | POSIX form | Reject unless the target contract permits it |
| --- | --- | --- |
| Alias and function bypass | `command <name>` with direct status handling | Treating `command -v` as a matching preflight |
| Any invocable command | `command -v <name>`, then ordinary invocation | `which` or invoking through `command` after that check |
| Arithmetic | `$((…))` | `((…))` or `let` |
| Combined redirection | `>file 2>&1` | `&>` |
| Conditions | `[ … ]`, `test`, or `case` | `[[ … ]]` |
| Function declaration | `name() { …; }` | `function name` |
| Function state isolation | A subshell-bodied function | `local` |
| Generated input | A pipe or redirection | Here-strings or process substitution |
| Load shell code | `.` | `source` |
| Mutable argument vector | `set --` and `"$@"` | Arrays |
| Option parsing | `getopts` or an explicit `case` loop | External `getopt` or shell-specific parsers |
| Wait for a child | Capture `$!`, then `wait "$pid"` | `wait -n` without established support |

A feature’s presence in a newer POSIX edition does not establish its availability in every target implementation.

## Utility Selection

Prefer parameter expansion, `case`, and arithmetic expansion when their exact edge case semantics match the operation. Use an established external utility when its documented behavior is the required interface. Do not replace clear shell logic mechanically. Likewise, do not force structured parsing, binary processing, or a complex algorithm into shell merely to avoid a suitable established tool.

Use `command -v <name>` when any command selected by normal shell resolution may satisfy the dependency, then invoke `<name>` normally. When an alias or function must be bypassed, invoke `command <name>` and handle that invocation’s status instead of treating `command -v` as a matching preflight. `command` may still select a built-in, so it is not an external-only resolver.

Do not reset `PATH` indiscriminately. Doing so can hide intentionally installed dependencies.

Before a programmatic `cd`, reject an empty operand. Preserve absolute paths and explicit relative paths beginning with `./` or `../`. Prefix every other relative path with `./` so option-like names and `-` remain directory operands. Invoke `CDPATH='' cd "$directory"` to disable directory search and emitted path output. Add `-P` only when the interface requires physical path behavior.

Scope `LC_ALL=C` to commands that deliberately need bytewise sorting, matching, or character classes. Do not change user-facing behavior globally.

Portable syntax does not make a GNU-only option portable.

## Pipeline Contracts

Treat a pipeline’s status as the final command’s status unless every target establishes another mechanism. Do not assume an earlier failure propagates. Do not depend on variable, directory, trap, or function state changed inside a pipeline component after that process exits.

Avoid feeding command output into a loop through a bare pipeline when upstream failure detection or post-loop state matters. A `printf` pipeline into `while` is acceptable when input generation cannot fail meaningfully and loop state does not need to persist.

When an optional command in strict mode produces either a usable nonempty scalar or no usable output on failure, scope `|| true` inside the command substitution before testing the result:

```sh
branch_name="$(git symbolic-ref --quiet --short HEAD || true)"

if [ -z "$branch_name" ]; then
  branch_name="$(git rev-parse --short HEAD)"
fi
```

Use direct status control flow when successful empty output or partial output on failure must remain distinguishable.

Do not probe for and enable optional `pipefail` behavior when correctness depends on it. Either establish the required semantics for every target or restructure the operation so each important status is checked independently.

## Background Jobs

Capture `$!` immediately after each background command. Wait for every recorded PID explicitly, and define how multiple failures combine. `wait` without operands does not provide a useful per-operation failure policy.

## Official Sources

The [POSIX Shell Command Language](https://pubs.opengroup.org/onlinepubs/9799919799/utilities/V3_chap02.html) defines expansion, execution, functions, pipelines, and shell state. The [Utility Syntax Guidelines](https://pubs.opengroup.org/onlinepubs/9799919799/basedefs/V1_chap12.html) define the option conventions that individual utilities may adopt. Use each utility’s POSIX reference together with target implementation evidence before relying on its options or edge case behavior.
