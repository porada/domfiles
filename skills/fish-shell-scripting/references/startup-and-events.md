# Startup and Events

Apply the [runtime state rules](fish-native-idioms.md#runtime-state) when the task involves Fish internals, functions, or shared variables.

## Configuration Lifecycle

Fish selects configuration snippets across the configured `conf.d` directories before reading system and user `config.fish` files. If the same basename appears in more than one directory, only the first file in directory precedence order runs. Fish then runs the selected snippets in natural filename order.

Put independent startup snippets in `conf.d/*.fish` when their order and override behavior are deliberate. Put user-level overrides and coordination in `$__fish_config_dir/config.fish`. Fish resolves that directory from `$XDG_CONFIG_HOME` or its `$HOME/.config/fish` fallback.

Keep setup required by noninteractive shells outside interactive-only guards. Guard prompts, abbreviations, bindings, and other interactive behavior with `status is-interactive` so remote commands and file transfer sessions do not receive unrelated output or state. Guard login-only behavior with `status is-login`.

## Declarative Startup

Make startup mutations idempotent. Re-sourcing a configuration file should produce the same state unless accumulation is its documented purpose. Use duplicate-safe operations, rebuild owned lists, or guard one-time work instead of repeatedly appending values that may already exist.

Keep startup code quiet, deterministic, and fast without suppressing errors broadly merely to keep startup silent. Derive configuration-relative paths using the [path rules](fish-native-idioms.md#paths).

Choose whether version-controlled startup files should recreate state for each session or Fish should preserve a mutable preference across sessions. Use global variables when version-controlled startup files should determine each session’s state. `fish_add_path --global` ignores nonexistent directories, normalizes accepted paths, avoids duplicates, and leaves an existing entry in place unless directed to move it.

Use universal variables for intentionally mutable, cross-session preferences managed independently of version-controlled startup files. Do not append to them on every startup. Manage them through `set --universal`, and never edit `fish_variables` directly.

## Event Handlers

Functions can handle job exits, named events, process exits, signals, and variable changes through options such as `--on-event`, `--on-job-exit`, `--on-process-exit`, `--on-signal`, and `--on-variable`.

Use `--on-process-exit <pid>` for a child process of the current Fish instance and `--on-job-exit <pid>` for a job containing a child with that process ID. Neither handler fires for a disowned job. Use the named `fish_exit` event for the current Fish instance’s exit.

An `--on-signal <signal>` handler receives only a signal delivered to Fish. Registering the handler also prevents Fish from exiting through its normal response to that signal.

Treat `--on-variable <name>` as a notification that Fish may combine or delay, not as a callback for every assignment. Fish guarantees neither exact timing nor one invocation for each `set`. It may skip intermediate values or run after a same-value assignment. Use it to invalidate or synchronize derived state, not as a transaction log or correctness-critical trigger.

Load the defining file before the event can occur because Fish cannot discover an unloaded handler from its declaration. Ordinary autoloading by function name is insufficient. Do not depend on handler order when several functions subscribe to the same event.

Keep handlers fast and avoid unexpected interactive output unless that output is the feature. Treat event names, variable names, and process targets as part of the handler contract. Document non-obvious lifetimes in the handler’s source docstring.

## Official Sources

Startup and event behavior are documented in the official [Fish language](https://fishshell.com/docs/current/language.html), [`fish_add_path` reference](https://fishshell.com/docs/current/cmds/fish_add_path.html), [`function` reference](https://fishshell.com/docs/current/cmds/function.html), and [`status` reference](https://fishshell.com/docs/current/cmds/status.html).
