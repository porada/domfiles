# Configuration, functions, and events

## Place configuration by lifecycle

Fish selects configuration snippets across the configured `conf.d` directories before reading system and user `config.fish` files. When directories contain the same basename, only the first file in directory-precedence order runs. The selected snippets then run together in natural filename order.

- Put independent startup snippets in `conf.d/*.fish` when ordering and override behavior are deliberate.
- Put user-level overrides and coordination in `$__fish_config_dir/config.fish`. Fish resolves that directory from `$XDG_CONFIG_HOME` or its `$HOME/.config/fish` fallback.
- Keep setup required by noninteractive shells outside interactive-only guards. Guard prompt, abbreviation, binding, and other interactive behavior with `status is-interactive` so remote commands and file-transfer sessions do not receive unrelated output or state.
- Guard login-only behavior with `status is-login`.

## Keep startup declarative

- Make startup mutations idempotent. Re-sourcing a configuration file should produce the same resulting state unless accumulation is its documented purpose. Use duplicate-safe operations, rebuild owned lists, or guard one-time work instead of repeatedly appending values that may already exist.
- Keep startup code quiet, deterministic, and fast. Do not suppress errors broadly merely to keep startup silent.
- Derive configuration-relative paths from Fish’s status and path builtins rather than the caller’s working directory.

Choose the persistent source of truth deliberately:

- Use global variables when version-controlled startup files should determine each session’s state. `fish_add_path --global` ignores nonexistent directories, normalizes accepted paths, avoids duplicates, and leaves an existing entry in place unless directed to move it.
- Use universal variables for intentionally mutable, cross-session user preferences managed independently of version-controlled startup files. Do not append to them on every startup.
- Manage universal variables through `set --universal`. Never edit `fish_variables` directly.

## Define and autoload functions deliberately

Apply the entrypoint’s mandatory function-docstring rule to every explicit function definition on this surface.

- Put an autoloaded function named `example` in `example.fish` within `$fish_function_path`.
- Prefer one public autoload target per file. Helpers may share the file when they are private to that target. Fish initially loads the file when it resolves the matching function name and automatically reloads an altered definition after detecting the change. A helper does not independently trigger the initial load.
- Put a shared helper that must autoload independently in its own matching file. Namespace private user functions for the owning tool or configuration to reduce collisions.
- Use `$argv` for function arguments and `return` to report the function’s status.
- Add `function --description` when runtime discovery through `functions` or completion tooling benefits from it. Keep the source docstring regardless.

## Define argument contracts with `argparse`

Use `argparse` as the parsing and option-value-validation boundary for conventional command interfaces. After successful parsing, `$argv` contains the remaining positional operands and `$argv_opts` contains consumed options retained by their specifications.

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
- Use `--min-args` and `--max-args` for positional cardinality, and repeat `--exclusive` for each set of incompatible options.
- Use option validators for constraints on individual option values. Validate positional relationships, cross-option rules, and other semantics after parsing when they do not belong to one option value.
- Write validator error fragments to stdout because `argparse` consumes them. `argparse` itself reports the resulting failure to stderr.
- Use `--name` when diagnostics must identify a stable public interface rather than the current helper function. Otherwise retain the default function name.

Return immediately when parsing fails unless the function deliberately translates the parser’s diagnostic or status contract.

## Choose wrappers and interactive substitutions deliberately

Choose the smallest Fish mechanism that matches the behavior:

| Need | Mechanism |
| --- | --- |
| Interactive command-line expansion visible before execution | `abbr` |
| Lazily loaded named behavior | Autoloaded function file |
| Reusable runtime behavior | Function |
| Simple function-shaped wrapper | `alias`, recognizing that Fish implements it as a function |
| Startup or event registration | Explicitly sourced configuration or `conf.d` snippet |

For a maintained wrapper with an observable contract, define the function explicitly. Use `function --wraps <command>` only when the wrapper preserves the delegated command’s relevant completion interface. For a wrapper around an external program, invoke it through `command` and forward `$argv` unless the wrapper intentionally changes that interface. Use the command-resolution operation from the Fish-native guidance that matches whether functions, builtins, or only external programs may satisfy the dependency.

## Activate event handlers explicitly

Functions can handle job exits, named events, process exits, signals, and variable changes through `function` options such as `--on-event`, `--on-job-exit`, `--on-process-exit`, `--on-signal`, and `--on-variable`.

- Use `--on-process-exit <pid>` for a child process of the current Fish instance and `--on-job-exit <pid>` for a job containing a child with that process ID. These handlers do not fire for disowned jobs. Use the named `fish_exit` event for the current Fish instance’s exit.
- An `--on-signal <signal>` handler receives only a signal delivered to Fish. Registering the handler also prevents Fish from exiting in its normal response to that signal.
- Treat `--on-variable <name>` as a coalesced variable notification rather than a callback for every assignment. Fish guarantees neither exact timing nor one invocation for each `set`. It may skip intermediate values or run after a same-value assignment. Use it to invalidate or synchronize derived state, not as a transaction log or correctness-critical trigger.
- Ensure the defining file is loaded before the event can occur because Fish cannot discover an unloaded handler from its declaration. Ordinary autoloading by function name is insufficient. Do not rely on handler order when several functions subscribe to the same event.
- Keep handlers fast and avoid unexpected interactive output unless that output is the feature. Treat event names, variable names, and process targets as part of the handler contract, and document non-obvious lifetimes in the function docstring.

## Preserve startup and function state

- Capture `$status` as the first command in any function whose behavior depends on the preceding command, especially prompt and event functions.
- Avoid mutable global state when a local or function-scoped value is sufficient.
- Remember that a global variable can shadow a universal variable of the same name.
- Do not modify undocumented Fish internals. A name beginning with `__fish` is a review signal rather than proof that the interface is private. Allow interfaces that official Fish documentation exposes for user configuration.

## Diagnose loading

- Use `type --all <name>` and `functions <name>` to inspect command resolution and loaded function definitions.
- Use `status print-stack-trace` at a breakpoint when call context matters.
- Use `fish_trace` for execution tracing.

## Profile measured work

- Use `fish --profile=<path>` to measure executed commands after startup, and `fish --profile-startup=<path>` to measure startup and configuration loading.
- Cache only measured repeated work with a defined validity and invalidation contract. Do not add mutable cache state merely because a path is performance-sensitive.
- Remove temporary tracing, profiles, or breakpoints after diagnosis unless the task explicitly adds a durable debugging mode.

## Official sources

Startup, function, and event behavior are documented in the official [Fish language](https://fishshell.com/docs/current/language.html), [`fish_add_path` reference](https://fishshell.com/docs/current/cmds/fish_add_path.html), [`function` reference](https://fishshell.com/docs/current/cmds/function.html), and [`status` reference](https://fishshell.com/docs/current/cmds/status.html). Argument parsing and profiling are documented in the [`argparse` reference](https://fishshell.com/docs/current/cmds/argparse.html) and [`fish` reference](https://fishshell.com/docs/current/cmds/fish.html).
