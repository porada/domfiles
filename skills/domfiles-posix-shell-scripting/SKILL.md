---
name: posix-shell-scripting
description: Edit, review, audit, refactor, and diagnose POSIX `sh` code. Use it whenever code is intended to run under a POSIX shell or a migration targets POSIX `sh`, including extensionless entrypoints, sourced libraries, and scripts identified by POSIX shell hashbangs or syntax. Do not use when the requested output is only Fish or another non-POSIX shell.
metadata:
    internal: true
---

# POSIX shell scripting

Write portable POSIX `sh` on the language’s own terms. Preserve its expansion order, process boundaries, exit-status model, and deliberately small interface instead of relying on behavior from another shell.

Use the latest published POSIX shell specification unless the user or target environment establishes a narrower baseline. Treat the target shell implementation and available external utilities as separate compatibility constraints. Do not assume Bash, Zsh, or GNU behavior merely because a command works on one machine.

## Workflow

Choose the branch that matches the request. An explicit change takes precedence when the request also uses review or audit language.

- **Change:** Inspect the affected files, call sites, execution context, target shells, and project validation entrypoints before making the smallest complete edit.
- **Review:** Remain read-only and report only concrete correctness, portability, maintainability, or established-policy problems.
- **Audit:** Remain read-only, bound the file inventory first, apply every applicable rule to that inventory, and report evidence-backed findings rather than style preferences.
- **Diagnosis:** Remain read-only until the failure is reproduced or isolated. Trace expansion, environment changes, process boundaries, signals, and exit statuses before proposing a root-cause fix.

## POSIX shell context

1. Identify POSIX shell code from its hashbang and syntax rather than its filename alone. Include extensionless entrypoints and sourced files without hashbangs.
2. Classify the target as an executed entrypoint, sourced library, hook, startup fragment, or generated shell fragment. Determine whether state changes must affect the caller before choosing between sourcing, a function, a subshell, and an executed script.
3. Inspect each caller’s invocation form, argument contract, environment, working-directory assumptions, and handling of standard output, standard error, signals, and status.
4. Establish the target shell implementations and external utility set from project or environment evidence. Evaluate shell syntax separately from external commands and their options.
5. Prefer the project’s formatter, lint wrapper, tests, and conventions when they preserve POSIX semantics.

Use [POSIX-native idioms](references/posix-native-idioms.md) whenever a task touches variables, positional parameters, quoting, expansions, conditions, option parsing, functions, sourced files, input, output, exit status, paths, redirections, pipelines, shell options, process boundaries, traps, cleanup, background jobs, temporary resources, or command and utility selection.

## Design principles

### Keep POSIX targets in POSIX shell

Use POSIX syntax and interfaces. Do not introduce Bash, Zsh, Fish, or other shell-specific syntax unless the target contract explicitly permits it. Verify a feature against every established target rather than assuming that acceptance by one installed shell proves portability.

Keep a POSIX target in POSIX `sh` plus established target utilities. Do not invoke `python3`, Node.js, Perl, Ruby, another shell, or another language solely to avoid POSIX quoting, state, process, or utility constraints.

Treat changing the implementation language or interpreter as an architecture change. Proceed only when the task explicitly authorizes that boundary. When the required behavior cannot be expressed safely, clearly, and proportionately within POSIX shell, establish that a proposed runtime is already a target dependency, then stop for user direction rather than changing languages automatically.

### Choose the smallest boundary

Prefer direct arguments and streams to scalar reparsing, process indirection, or files. Do not introduce a temporary resource until the need satisfies [temporary-resource criteria](references/posix-native-idioms.md#justify-temporary-resources). Another language or an unestablished shell extension is not an alternative to a temporary resource that the contract genuinely requires.

Apply strict mode only when exit-on-error and unset-parameter behavior match the script’s contract. Account for conditional lists, functions, subshells, and command substitutions before enabling or changing it.

## Source conventions

For new or materially rewritten code:

- Represent boolean variables with the literal values `true` and `false`. Initialize them before use, and compare them explicitly with `=`. Do not use unset state, empty strings, or `0` and `1` as alternate boolean representations.
- Prefer the variable name `param` over `arg`.
- Treat 100 columns, including indentation, as the default threshold for introducing `\` continuations, not as a conformance limit for existing code. Do not report or reflow an existing command based on line length alone. Preserve a continuation when it communicates semantic grouping, control flow, or intentional alignment.
- Indent block bodies and continuation lines two spaces by default. Break at a meaningful argument, operator, or redirection boundary, and use the fewest lines that preserve clear grouping. Do not add `\` where shell syntax already continues the construct. Prefer a named helper over a long chain of continuations, and allow an overlong line when no useful break exists.

Do not report `eval` solely because another form could express generated shell syntax. Report it when it reparses data as code, loses required argument boundaries, or poses a concrete security risk.

## Human-facing text

Comments, help and usage text, diagnostics, warnings, prompts, labels, and test titles are human-facing technical copy.

Load `human-facing-writing` whenever a POSIX shell task creates, changes, or reviews human-facing text whose contract is in scope. POSIX semantics and project policy own what the text must communicate. `human-facing-writing` owns wording, reading order, terminology, tone, and surface-appropriate presentation within those facts. Preserve machine-readable output, exact command syntax, destination-supplied values, and upstream errors when the task does not change their contract.

## Validation

Run task-local behavioral checks only when they cannot modify user state. Cover empty and multiple arguments, whitespace and glob characters, option-like values, unset and empty values, command failures, partial output, pipeline behavior, ambient `CDPATH`, and sourced-versus-executed state when those cases matter.

### Define validation scope

- **Change:** Validate each changed POSIX shell file and every affected call site, execution context, and cross-file contract needed to establish the resulting behavior.
- **Review, Audit, or Diagnosis:** Validate the complete resolved read-only scope. For an audit, use the bounded inventory established before inspection. For a review or diagnosis, include every affected file, caller, execution context, and cross-file contract. Keep every check nonmutating.

### Run validation

1. Run the project’s narrowest applicable POSIX shell checks and diagnostics first.
2. Parse every changed or in-scope file with `sh -n -- <path>`. Skip this command only when the project workflow already invokes the target POSIX shell with its no-execute option for that file. Static analysis, including ShellCheck, does not replace this check.
3. Run the project’s ShellCheck configuration or `shellcheck --shell=sh -- <path>` when no project wrapper is established.
4. Check formatting with the project formatter’s nonmutating mode when one is established.
5. Exercise the applicable task-local behavioral checks in the target sourcing or execution context.
6. Recheck every human-facing string in scope under the [Human-facing text](#human-facing-text) contract.
