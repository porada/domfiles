---
name: posix-shell-scripting
description: |-
    Write, review, audit, refactor, and diagnose POSIX `sh` code and configuration. Use it whenever code is intended to run under a POSIX shell or a migration targets POSIX `sh`.

    Use it across `.sh` files, `#!/bin/sh` entrypoints, extensionless commands, sourced libraries, hooks, startup fragments, generated shell fragments, and other shell code with no identifiable shell dialect.

    Do not use when the requested output is only code for a non-POSIX shell.
---

# POSIX Shell Scripting

Portable POSIX `sh` starts with the language’s actual contracts: expansion, process boundaries, exit statuses, and caller-visible state. Write to those contracts directly instead of translating habits from a different shell or runtime.

Use the latest published POSIX shell specification unless the user or target environment establishes a narrower baseline. Treat the shell implementation and available external utilities as separate compatibility constraints. A command that works on one machine does not establish portable shell syntax or utility behavior.

## Secrets and Authentication

Never add literal credentials, access tokens, private keys, secret-bearing URLs, or private machine or account values to tracked files, proposed repository artifacts, patches, relays, command literals, environment assignments, configuration values, or task artifacts. Never directly retrieve, inspect, enumerate, echo, transmit, create, rotate, or load a real credential or authentication identity. Use established machine-local authentication only through ordinary non-disclosing tool operations. When direct credential handling is required, provide a command for the user to run instead.

## Typography

Apply the [typography conventions](references/typography.md) to all prose.

## Workflow

Choose the branch that matches the requested outcome. An explicit change takes precedence when the request also uses review or audit language.

- **Change:** Inspect the affected files, call sites, execution context, target shells, and project validation entrypoints before making the smallest complete edit. Remove temporary `set -x`, diagnostic output, tracing traps, and generated trace files introduced during the change before completion unless the task explicitly establishes a durable debugging mode.
- **Review:** Remain read-only and report only concrete correctness, portability, maintainability, or established-policy problems.
- **Audit:** Remain read-only, bound the file inventory first, apply every applicable rule to that inventory, and report evidence-backed findings rather than style preferences.
- **Diagnosis:** Remain read-only throughout. Reproduce or isolate the failure by tracing expansion, environment changes, process boundaries, signals, and exit statuses. Reserve source instrumentation and every other edit for an explicitly requested Change.

## POSIX Shell Context

1. Identify POSIX shell code from its hashbang and syntax rather than its filename alone. Include extensionless entrypoints and sourced files without hashbangs.
2. Classify the target as an executed entrypoint, sourced library, hook, startup fragment, or generated shell fragment. Determine whether state changes must affect the caller before choosing sourcing, a function, a subshell, or an executed script.
3. Inspect each caller’s invocation form, argument contract, environment, working-directory assumptions, standard streams, signal handling, and status handling.
4. Establish the target shell implementations and external utility set from project or environment evidence. Evaluate shell syntax separately from external commands and their options.
5. Prefer the project’s formatter, lint wrapper, tests, and conventions when they preserve POSIX semantics.

Load bundled guidance when the corresponding decision enters scope:

- Use [Data, Arguments, and Paths](references/data-arguments-and-paths.md) for data representation, exact or opaque streams, variables, positional parameters, quoting, expansions, command-substitution data, conditions, arithmetic, option parsing, strict mode, `set -e`, `set -u`, line-oriented input, and pathnames.
- Use [Functions and Interfaces](references/functions-and-interfaces.md) for functions, subshells, sourced files, caller state, standard output, standard error, exit status, command-substitution status, and terminal-dependent input or output.
- Use [Execution and Resources](references/execution-and-resources.md) for non-POSIX forms, utility selection, command lookup, environment and locale state, working-directory changes, redirections, pipelines, strict mode or other shell options, pipeline or background process boundaries, temporary resources, traps, recovery, cleanup, and background jobs.

## Design Principles

### Keep POSIX Targets in POSIX Shell

Use POSIX syntax and interfaces unless the target contract explicitly permits an extension. Verify each permitted feature against every established target. Acceptance by one installed shell does not establish portability.

Keep a POSIX target in POSIX `sh` plus its established utilities. Do not invoke `python3`, Node.js, Perl, Ruby, another shell, or another language merely to avoid POSIX quoting, state, process, or utility constraints.

Changing the implementation language or interpreter is an architecture change. Proceed only when the task explicitly authorizes that boundary. If the required behavior cannot be expressed safely, clearly, and proportionately in POSIX shell, establish that the proposed runtime is already a target dependency, then ask the user whether to change languages.

### Choose the Smallest Boundary

Prefer direct arguments and streams to scalar reparsing, process indirection, or files. Introduce a temporary resource only when the need satisfies the [temporary-resource criteria](references/execution-and-resources.md#temporary-resources). Another language or an unestablished shell extension is not an alternative to a temporary resource that the contract genuinely requires.

Do not add an option, environment variable, or configuration file when the script can infer one reliable behavior. Each new configuration branch adds an interface and a portability cost.

Enable or change strict mode only when exit-on-error and unset-parameter behavior match the script’s contract. Account for conditional lists, functions, subshells, and command substitutions first.

## Source Conventions

When an applicable policy requires alphabetization, apply it to order-independent entries such as declarations, option lists, lookup tables, and inventories. Preserve order when it controls parsing, expansion, dependencies, execution, or precedence, or when it communicates lifecycle or presentation.

Follow established project conventions when they preserve POSIX semantics. For each choice the project does not establish, use the corresponding default for new or materially rewritten code:

- Represent booleans with the literal values `true` and `false`. Initialize each boolean before use, and compare it explicitly with `=`. Do not use unset state, an empty string, or `0` and `1` as alternate representations.
- Prefer the variable name `param` over `arg`.
- Treat 100 columns, including indentation, as the default threshold for introducing `\` continuations, not as a conformance limit for existing code. Do not report or reflow an existing command for length alone. Preserve a continuation that communicates semantic grouping, control flow, or intentional alignment.
- Indent block bodies and continuation lines two spaces by default. Break at a meaningful argument, operator, or redirection boundary, using the fewest lines that preserve clear grouping. Do not add `\` where shell grammar already continues the construct. Prefer a named helper to a long continuation chain, and keep an overlong line when no useful break exists.

Do not report `eval` merely because another form could express generated shell syntax. Report it when it reparses data as code, loses required argument boundaries, or creates a concrete security risk.

## Human-Facing Text

Comments, help and usage text, diagnostics, warnings, prompts, labels, and test titles are human-facing technical copy.

Load `human-facing-writing` whenever a POSIX shell task creates, changes, or reviews human-facing text whose contract is in scope, including adjacent tests written in another language. Provide the POSIX shell surface, required semantics, and relevant evidence, then let that skill select its applicable routes.

POSIX shell semantics and project policy own what the text must communicate. `human-facing-writing` owns wording, reading order, terminology, tone, and surface-appropriate presentation within those facts. Do not rewrite machine-readable output, exact command syntax, destination-supplied values, or preserved upstream errors merely for prose style.

If `human-facing-writing` is unavailable locally and available evidence shows that remote use would materially improve the wording, follow the [optional public-peer workflow](references/optional-peer-human-facing-writing.md). If the peer remains unavailable, preserve complete standalone behavior. Write concise, neutral text that leads with the purpose or outcome, explains non-obvious intent rather than control flow, preserves exact technical tokens, and gives an actionable reason only when evidence establishes one.

## Validation

Run task-local behavioral checks only when they cannot modify user state. Cover empty and multiple arguments, whitespace and glob characters, option-like values, unset and empty values, command failures, partial output, pipeline behavior, ambient `CDPATH`, and sourced-versus-executed state when those cases matter.

### Define Validation Scope

- **Change:** Validate each changed POSIX shell file and every affected call site, execution context, and cross-file contract needed to establish the resulting behavior.
- **Review, Audit, or Diagnosis:** Validate the complete resolved read-only scope. For an audit, use the bounded inventory established before inspection. For a review or diagnosis, include every affected file, caller, execution context, and cross-file contract. Keep every check nonmutating.

### Run Validation

1. Inspect the project’s narrowest applicable POSIX shell checks and diagnostics without running them. Identify every project wrapper, configuration file, `SHELLCHECK_OPTS` value, or command-line option that may invoke ShellCheck or enable external-source following.
2. Resolve ShellCheck’s read scope before invoking it directly or through a project entrypoint. Include every sourced file ShellCheck may read in the resolved validation scope. If the read scope cannot be established or constrained, skip each affected check and report the validation limitation.
3. Run each of the project’s narrowest applicable checks and diagnostics only after every ShellCheck path it can reach has passed the scope gate. A check that cannot invoke ShellCheck does not require that gate. When no project ShellCheck configuration is established, pass the complete resolved source set explicitly to `shellcheck --norc --shell=sh -- <path>…` only after the gate passes and only when ShellCheck is already available. Permit external-source following in any invocation only after constraining every source ShellCheck can reach to that resolved set. Do not add or install a validator without explicit user authorization. If ShellCheck is unavailable, report that validation limitation.
4. Parse every changed or in-scope file with each established target shell’s no-execute option. When the target command is `sh`, use `sh -n -- <path>`. Skip this check only when a permitted project workflow has already parsed the file with that target shell’s no-execute option. Static analysis, including ShellCheck, does not replace this check.
5. Check formatting with the project formatter’s nonmutating mode when one is established.
6. Exercise the applicable task-local behavioral checks in the target sourcing or execution context.
7. Recheck every human-facing string in scope under the [human-facing text contract](#human-facing-text).

## Stale Guidance

Classify each part of this skill’s guidance used by the selected workflow as required, optional, or supporting. Treat missing local targets, malformed destinations, and HTTP responses that report a resource as missing or permanently unavailable as broken references. Broken references and verified conflicts with the current interface or behavior mean the guidance is stale. Use any failure response the guidance defines. Otherwise, report the stale guidance and evidence, recommend updating this skill, and follow the appropriate recovery below.

When required guidance is stale, stop only the affected branch and use any complete fallback provided by the available guidance. Without one, ask whether to continue. The choice applies only to this conversation and to work independent of the stale guidance. Stale optional or supporting guidance does not stop the workflow.

Access restrictions, authentication problems, network failures, and HTTP server errors are not evidence of staleness. Use any relevant access or retrieval guidance. If none applies, stop retrieving the resource and report the resource, attempted method, exact error, and smallest corrective action.

Never infer missing content. Never substitute an unverified location. Never weaken scope, approval, mutation, or security boundaries.
