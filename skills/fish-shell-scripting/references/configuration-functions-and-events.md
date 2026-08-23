# Configuration, Functions, and Events

## Configuration Lifecycle

Fish selects configuration snippets across the configured `conf.d` directories before reading system and user `config.fish` files. If the same basename appears in more than one directory, only the first file in directory-precedence order runs. Fish then runs the selected snippets in natural filename order.

Put independent startup snippets in `conf.d/*.fish` when their order and override behavior are deliberate. Put user-level overrides and coordination in `$__fish_config_dir/config.fish`. Fish resolves that directory from `$XDG_CONFIG_HOME` or its `$HOME/.config/fish` fallback.

Keep setup required by noninteractive shells outside interactive-only guards. Guard prompts, abbreviations, bindings, and other interactive behavior with `status is-interactive` so remote commands and file-transfer sessions do not receive unrelated output or state. Guard login-only behavior with `status is-login`.

## Declarative Startup

Make startup mutations idempotent. Re-sourcing a configuration file should produce the same state unless accumulation is its documented purpose. Use duplicate-safe operations, rebuild owned lists, or guard one-time work instead of repeatedly appending values that may already exist.

Keep startup code quiet, deterministic, and fast without suppressing errors broadly merely to keep startup silent. Derive configuration-relative paths from Fish’s `status` and `path` builtins rather than the caller’s working directory.

Choose whether version-controlled startup files should recreate state for each session or Fish should preserve a mutable preference across sessions. Use global variables when version-controlled startup files should determine each session’s state. `fish_add_path --global` ignores nonexistent directories, normalizes accepted paths, avoids duplicates, and leaves an existing entry in place unless directed to move it.

Use universal variables for intentionally mutable, cross-session preferences managed independently of version-controlled startup files. Do not append to them on every startup. Manage them through `set --universal`, and never edit `fish_variables` directly.

## Function Contracts

Apply the [function-docstring contract](../SKILL.md#function-documentation) to every explicit function on this surface. Use `$argv` for function arguments and `return` to report the function’s status. Add `function --description` when runtime discovery through `functions` or completion tooling benefits from it, while keeping the source docstring regardless.

## Function Autoloading

Put an autoloaded function named `example` in `example.fish` within `$fish_function_path`, and prefer one public autoload target per file. Private helpers may share that file. Fish first loads it when resolving the matching function name and automatically reloads a changed definition after detecting the change. A helper does not independently trigger the initial load.

Put a shared helper that must autoload independently in its own matching file. Namespace private user functions for the owning tool or configuration to reduce collisions.

## `argparse` Contracts

Use `argparse` as the parsing boundary for conventional command interfaces. After successful parsing, `$argv` contains the remaining positional arguments, while `$argv_opts` contains consumed options and their values by default. An `&` modifier in an option specification keeps that option and any attached values out of both `$argv` and `$argv_opts` without affecting the corresponding `_flag_` variables.

```fish
argparse \
    --strict-longopts \
    --min-args=1 \
    --max-args=1 \
    --name=extract_archive \
    f/force \
    'o/output=' \
    -- $argv
or return
```

- Use `--strict-longopts` when abbreviated or single-dash long options are outside the interface.
- Use `--min-args` and `--max-args` to set the accepted number of positional arguments. Repeat `--exclusive` for each set of incompatible options.
- Use option validators for constraints on individual option values. Validate relationships between positional arguments, rules spanning several options, and other command semantics after parsing when they do not belong to one option value.
- Write validator error fragments to stdout because `argparse` consumes them. `argparse` reports the resulting failure to stderr.
- Use `--name` when diagnostics must identify a stable public interface rather than the current helper function. Otherwise, keep the default function name.

Return immediately when parsing fails unless the function deliberately translates the parser’s diagnostic or status contract.

## Wrapper Selection

Choose the smallest Fish mechanism that matches the behavior:

| Need | Mechanism |
| --- | --- |
| Interactive command-line expansion visible before execution | `abbr` |
| Lazily loaded named behavior | Autoloaded function file |
| Reusable runtime behavior | Function |
| Simple function-shaped wrapper | `alias`, which Fish implements as a function |
| Startup or event registration | Explicitly sourced configuration or `conf.d` snippet |

Define a maintained wrapper with an observable contract as an explicit function. Use `function --wraps <command>` only when the wrapper preserves the delegated command’s relevant completion interface.

When wrapping an external program, invoke it through `command` and forward `$argv` unless the wrapper intentionally changes that interface. Use the command-resolution operation from the Fish-native guidance that matches whether functions, builtins, or only external programs may satisfy the dependency.

## Event Handlers

Functions can handle job exits, named events, process exits, signals, and variable changes through options such as `--on-event`, `--on-job-exit`, `--on-process-exit`, `--on-signal`, and `--on-variable`.

Use `--on-process-exit <pid>` for a child process of the current Fish instance and `--on-job-exit <pid>` for a job containing a child with that process ID. Neither handler fires for a disowned job. Use the named `fish_exit` event for the current Fish instance’s exit.

An `--on-signal <signal>` handler receives only a signal delivered to Fish. Registering the handler also prevents Fish from exiting through its normal response to that signal.

Treat `--on-variable <name>` as a notification that Fish may combine or delay, not as a callback for every assignment. Fish guarantees neither exact timing nor one invocation for each `set`. It may skip intermediate values or run after a same-value assignment. Use it to invalidate or synchronize derived state, not as a transaction log or correctness-critical trigger.

Load the defining file before the event can occur because Fish cannot discover an unloaded handler from its declaration. Ordinary autoloading by function name is insufficient. Do not depend on handler order when several functions subscribe to the same event.

Keep handlers fast and avoid unexpected interactive output unless that output is the feature. Treat event names, variable names, and process targets as part of the handler contract. Document non-obvious lifetimes in the function docstring.

## Runtime State

Capture `$status` as the first command in any function whose behavior depends on the preceding command, especially prompt and event functions. Avoid mutable global state when a local or function-scoped value is sufficient, and remember that a global variable can shadow a universal variable of the same name.

Do not modify undocumented Fish internals. A name beginning with `__fish` is a review signal rather than proof that the interface is private. Allow interfaces that official Fish documentation exposes for user configuration.

## Loading Diagnosis

- Use `type --all <name>` and `functions <name>` to inspect command resolution and loaded function definitions.
- Use `status print-stack-trace` at a breakpoint when call context matters.
- Use `fish_trace` for execution tracing.

## Performance Profiling

Use `fish --profile=<path>` to measure commands executed after startup and `fish --profile-startup=<path>` to measure startup and configuration loading. Cache only measured repeated work with a defined validity and invalidation contract. Do not add mutable cache state merely because a path is performance-sensitive. Remove temporary tracing, profiles, or breakpoints after diagnosis unless the task explicitly adds a durable debugging mode.

## Official Sources

Startup, function, and event behavior are documented in the official [Fish language](https://fishshell.com/docs/current/language.html), [`fish_add_path` reference](https://fishshell.com/docs/current/cmds/fish_add_path.html), [`function` reference](https://fishshell.com/docs/current/cmds/function.html), and [`status` reference](https://fishshell.com/docs/current/cmds/status.html). Argument parsing and profiling are documented in the [`argparse` reference](https://fishshell.com/docs/current/cmds/argparse.html) and [`fish` reference](https://fishshell.com/docs/current/cmds/fish.html).
