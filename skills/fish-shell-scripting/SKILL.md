---
name: fish-shell-scripting
description: |-
    Write, review, audit, refactor, and diagnose Fish code and configuration. Use it whenever code is intended to run in Fish or a migration targets Fish.

    Use it across `.fish` files, Fish hashbangs, `config.fish`, autoloaded functions, prompts, and completions.

    Do not use when the requested output is only code for another shell.
---

# Fish shell scripting

Treat Fish as its own language. Preserve its list semantics, command-oriented conditions, explicit variable scopes, and purpose-built builtins rather than importing overlapping syntax from another shell.

Use the latest stable Fish release as the behavioral baseline unless the user or an authoritative target environment establishes another version. Do not introduce legacy forms, compatibility branches, historical caveats, or migration guidance without that evidence.

## Follow Fish’s design opinions

Apply the official [Fish design principles](https://fishshell.com/docs/current/design.html) as defaults for code and configuration decisions.

For language structure:

- **Orthogonality:** Prefer one powerful Fish-native construct over overlapping aliases, expansion families, heredocs, subshell forms, or hand-built variants. Use functions as the reusable abstraction.
- **User focus:** Express loops, conditions, assignments, and scopes through Fish’s uniform command model instead of exposing lower-level process machinery.
- **Selective POSIX alignment:** Follow POSIX where Fish already does, but never weaken Fish semantics or emulate unsupported syntax merely to resemble another shell.

For interactive behavior and configuration:

- **Responsiveness:** Keep startup, prompt, and completion paths fast. Minimize forks, disk access, and synchronous work that is not directly initiated by the user.
- **Discoverability:** Give every completion item a useful description when Fish’s API allows one, and make errors identify what went wrong and the relevant action or help surface.
- **Deliberate configuration:** Do not add a setting when the code can infer one reliable behavior. Treat every new configuration branch as a maintenance and compatibility cost.

## Choose the workflow

- For an explicit change, including a request that also uses review or audit language, inspect the affected Fish files, call sites, execution context, current Fish behavior, and project validation entrypoints before making the smallest complete edit.
- For a standalone review, remain read-only and report only concrete correctness, compatibility, maintainability, or established-policy problems.
- For a standalone audit, remain read-only, bound the file inventory first, apply every applicable rule to that inventory, and report evidence-backed findings rather than style preferences.
- For a diagnosis, remain read-only until the failure is reproduced or isolated. Trace expansion, scope, status, startup context, and command resolution before proposing a root-cause fix.

## Establish the Fish context

1. Identify Fish from the hashbang and syntax rather than the filename alone. Include extensionless entrypoints with a Fish hashbang.
2. Classify the target as a noninteractive script, interactive configuration, autoloaded function, event handler, prompt, or completion. These surfaces have different loading, status, output, and performance constraints.
3. Choose the execution boundary from state ownership. Use a function for reusable behavior that must affect the current Fish process, source a file only when file-based code must affect its caller, and execute a script when process isolation is intended. A sourced file has no process boundary. An executed Fish script still reads startup configuration by default and inherits its environment.
4. Prefer the project’s existing formatter, lint wrapper, tests, and conventions when they preserve Fish semantics. Do not import POSIX-shell policy merely because another shell exists in the same repository.

Load the applicable bundled guidance at the decision that needs it:

- For nontrivial variables, lists, expansions, conditions, paths, globs, process boundaries, redirections, pipelines, or builtin selection, use [Fish-native idioms](references/fish-native-idioms.md).
- For startup files, autoloaded functions, wrappers, abbreviations, universal variables, or event handlers, use [configuration, functions, and events](references/configuration-functions-and-events.md).
- For completion definitions, use [completions](references/completions.md).
- For prompt functions, use [prompts](references/prompts.md).

## Write Fish-native code

### Follow repository authoring conventions

- When applicable project or global policy requires alphabetization, apply it to order-independent Fish declarations, completion candidates, option lists, and configuration entries. Preserve order that communicates or controls precedence, lifecycle, dependency, or presentation.
- In new or materially rewritten commands, prefer a supported full-length option name such as `--all` over its short form such as `-a`. Keep the short form only when no equivalent long option exists or exact syntax is part of the interface being preserved.
- When no applicable project or formatter rule sets another limit, treat 100 columns, including indentation, as the default threshold for wrapping new or materially rewritten commands. Do not reflow existing code solely for length. Break at a meaningful argument, operator, pipe, or redirection boundary, rely on Fish’s grammatical continuation where available, and use `\` only when the line would otherwise terminate. Let the formatter own indentation, and allow an overlong line when no useful break exists.

### Preserve values and arguments

- Use `set` to create, update, export, scope, query, and erase variables. Do not write bare assignment syntax except the supported single-command `NAME=value command` override when that exact lifetime is intended.
- Model argument collections as lists. Use `$argv`, `count`, 1-based indices, slices, and whole-list forwarding instead of `$1`, `$@`, `$*`, `$#`, arrays from another shell, or implicit word splitting.
- Use quotes for Fish semantics rather than visual consistency. Leave literal tokens unquoted when Fish parses them identically, use single quotes for literal text that must remain unexpanded, and use double quotes when interpolation must remain one argument. Do not require or restore quotes that the project formatter or `fish_indent` removes without changing semantics.
- Decide expansion cardinality before quoting. Unquoted list expansion passes one argument per element. Double-quoted expansion produces exactly one argument, joining multiple elements with spaces or colons for path variables.
- Use `$(command)` for command substitution, including inside double quotes. Use `string split` or `string split0` when another delimiter defines records, and use `string collect` when output must be collected without newline splitting.

### Use Fish control and operations

- Express conditions as commands and statuses. Use `if`, `while`, `switch`, `test`, `string`, `path`, `contains`, `type`, and `not`, closing every block with `end`. Prefer a structured block over a long `and` or `or` chain.
- Reach for `string`, `math`, `path`, `argparse`, `count`, `contains`, `read`, `set`, `status`, and `source` before parameter-expansion tricks or an external text-processing pipeline. Keep an external command when it provides required semantics the Fish builtin does not.
- Capture `$status` or `$pipestatus` before another status-producing command runs. Prefer direct status control flow over storing textual booleans.
- When Fish code owns the representation of stored boolean state and neither an applicable policy nor the user selects another form, default to the literal values `true` and `false`. Initialize the variable before use and compare it explicitly with `=`. Do not encode owned boolean state through unset or empty values or `0` and `1`.
- Treat process boundaries as explicit design choices rather than assuming POSIX subshell behavior.

### Respect parsing boundaries

- Treat glob expansion as a potentially failing operation rather than string construction.
- Recognize `&&`, `||`, `!`, and `$()` as valid Fish syntax during review. For an equivalent two-command status dependency in new or materially rewritten code, write `command; and next` or `command; or fallback` instead of `&&` or `||`, and use `not` instead of `!`. Preserve semantics and precedence rather than replacing symbolic forms mechanically.

## Document every function

A Fish function docstring is a contiguous block of `#` comment lines immediately above an explicit `function` declaration, with no blank line between the docstring and declaration.

- Give every explicit function definition a docstring, including private helpers, wrappers, event handlers, prompt functions, completion helpers, and intentionally empty overrides.
- State the function’s purpose, observable contract, compatibility boundary, or non-obvious constraint. Do not narrate its implementation or repeat an obvious name.
- Keep the docstring adjacent when moving or refactoring the function.
- Treat `function --description` as optional runtime metadata. It may supplement but never replaces the source docstring.

```fish
# Resolve a repository path to its canonical form
function resolve_repository
    path resolve -- $argv[1]
end
```

## Compose human-facing text

Treat function docstrings, explanatory comments, help text, usage text, diagnostics, warnings, prompts, completion descriptions, interactive labels, and test titles as human-facing technical copy.

- Load `human-facing-writing` whenever a task creates, changes, or reviews human-facing text whose contract is in scope for the Fish task, including adjacent tests written in another language. Provide the Fish surface, required semantics, and relevant evidence, then let that skill select its applicable routes.
- Fish semantics and project policy own what the text must communicate. `human-facing-writing` owns wording, reading order, terminology, tone, and surface-appropriate presentation within those facts.
- Do not rewrite machine-readable output, exact command syntax, destination-supplied values, or preserved upstream errors merely for prose style.
- If `human-facing-writing` is unavailable locally and available evidence shows that remote use would materially improve the current wording, follow [the optional public-peer workflow](references/skill-human-facing-writing.md).
- If the peer remains unavailable, keep standalone behavior complete. Write concise, neutral text that leads with the purpose or outcome, explains non-obvious intent rather than control flow, preserves exact technical tokens, and gives an actionable reason only when evidence establishes one.

## Validate Fish behavior

Apply the relevant behavioral checks only when they can run without modifying user state:

- Cover empty and multi-element lists, paths containing whitespace, failed commands, unmatched globs, and option boundaries when those cases matter.
- When a standalone target must not depend on startup configuration, exercise it under `fish --no-config` as well as its normal target context. Treat this as a configuration-independence check, not a hermetic environment.
- For configuration, prompts, completions, and events, validate the relevant interactive, noninteractive, login, autoload, or event-loading context without persisting universal variables or overwriting user configuration.

### Select the validation scope

- For a change, validate each changed Fish file, function, and human-facing string, plus every affected call site, execution context, and cross-file contract needed to establish the resulting behavior.
- For a review, audit, or diagnosis, validate the complete resolved read-only scope. For an audit, use the bounded inventory established before inspection. For a review or diagnosis, include every affected file, call site, execution context, and cross-file contract. Keep every check nonmutating.

### Run validation

1. Run the project’s narrowest applicable Fish checks and diagnostics first.
2. Parse each script in the validation scope with `fish --no-config --no-execute <path>` when the project workflow does not already do so.
3. Check formatting with the project formatter’s check mode or `fish_indent --check <path>` when no project formatter is established.
4. Exercise the relevant behavioral checks above.
5. Recheck every function in the validation scope for its adjacent docstring and every human-facing string in the validation scope through the required writing route.
