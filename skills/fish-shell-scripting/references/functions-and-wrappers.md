# Functions and Wrappers

Apply the shared [runtime state rules](fish-native-idioms.md#runtime-state) to functions and their variables.

## Function Contracts

Apply the [function documentation contract](../SKILL.md#function-documentation) to every explicit function on this surface.

## Function Autoloading

Put an autoloaded function named `<function-name>` in `<function-name>.fish` within `$fish_function_path`, and treat it as that file’s owner. Put every other exposed function in its own matching autoload file. Replace each `-` in `<function-name>` with `_` to derive `<function-namespace>`. Choose a stable underscore-form `<owner>` namespace token for the owning tool or configuration rather than deriving it mechanically from its name. The token must be unique in the target Fish function namespace. Outside source maintained by Fish itself, `<owner>` must neither equal `fish` nor begin with `fish_`, keeping generated names outside the [Fish-owned `__fish_` namespace](fish-native-idioms.md#runtime-state). Prefix every private helper owned by the function with `__<owner>_<function-namespace>_`. Within one `<owner>`, no two helper-owning functions may derive the same `<function-namespace>`. For example, private helpers for `start-app` under owner `app_tools` use names such as `__app_tools_start_app_find_root`, while a `fish_prompt` under owner `shell_theme` uses `__shell_theme_fish_prompt_git`.

Fish first loads the file when resolving the matching function name and automatically reloads a changed definition after detecting the change. A helper does not independently trigger the initial load. Once loaded, every function in the file remains callable by name. Fish omits underscore-prefixed names from the default `functions` listing, but the prefix is an internal-use and namespace convention rather than access control.

Put a shared helper that must autoload independently in its own matching file. When no single function owns it, use `__<owner>_` followed by a role-specific name instead of assigning it a `<function-namespace>`.

## `argparse` Contracts

Use `argparse` as the parsing boundary for conventional command interfaces. After successful parsing, `$argv` contains the remaining positional arguments, while `$argv_opts` contains consumed options and their values by default. An `&` modifier in an option specification keeps that option and any attached values out of both `$argv` and `$argv_opts` without affecting the corresponding `_flag_` variables.

```fish
argparse \
    --strict-longopts \
    --min-args=1 \
    --max-args=1 \
    --name=start-app \
    o/open \
    'p/port=' \
    -- $argv
or return
```

- Use `--min-args` and `--max-args` to set the accepted number of positional arguments. Repeat `--exclusive` for each set of incompatible options.
- Use `--name` when diagnostics must identify a stable public interface rather than the current helper function. Otherwise, keep the default function name.
- Use `--strict-longopts` when abbreviated or single-dash long options are outside the interface.
- Use option validators for constraints on individual option values. Validate relationships between positional arguments, rules spanning several options, and other command semantics after parsing when they do not belong to one option value.
- Write validator error fragments to standard output because `argparse` consumes them. `argparse` reports the resulting failure to standard error.

Return immediately when parsing fails unless the function deliberately translates the parser’s diagnostic or status contract.

## Wrapper Selection

Choose the smallest Fish mechanism that matches the behavior:

| Need | Mechanism |
| --- | --- |
| Interactive command line expansion visible before execution | `abbr` |
| Lazily loaded named behavior | Autoloaded function file |
| Reusable runtime behavior | Function |
| Simple function-shaped wrapper | `alias`, which Fish implements as a function |
| Startup or event registration | Explicitly sourced configuration or `conf.d` snippet |

By default, the completion pager describes a literal abbreviation with its expansion and a function-backed abbreviation with the expansion function’s name. That default satisfies the [completion description principle](../SKILL.md#keep-interactive-behavior-deliberate) when it makes the abbreviation’s purpose clear. Otherwise, add a concise custom description by placing the attached option `--description='<text>'` before the abbreviation name. Apply the [human-facing text contract](../SKILL.md#human-facing-text) to its wording.

Define a maintained wrapper with an observable contract as an explicit function. Use `function --wraps <command>` only when the wrapper preserves the delegated command’s relevant completion interface.

When wrapping an external program, invoke it through `command` and forward `$argv` unless the wrapper intentionally changes that interface. Use the [command resolution operation](builtin-selection.md#input-and-command-state) that matches whether functions, builtins, or only external programs may satisfy the dependency.

Before rewriting a delegated command’s arguments, establish which input forms the wrapper supports. Validate `--` handling, abbreviated and full option names, and attached and separate option values against that declared interface. Keep supported arguments as separate list elements, and reject unsupported forms before rewriting rather than partially parsing them and forwarding an altered command.

When a wrapper takes over a command’s terminal presentation, preserve the configuration and environment variable precedence required by its declared interface. Check empty and unset values separately because they may select different behavior. Prefer the underlying command’s supported resolver when it supplies that behavior rather than duplicating its resolution logic.

## Loading Diagnosis

- Use `fish_trace` for execution tracing without source edits or persisted state during read-only diagnosis.
- Use `status print-stack-trace` at an existing breakpoint when call context matters.
- Use `type --all <name>` and `functions <name>` to inspect command resolution and loaded function definitions.

## Performance Profiling

With explicit authorization to create profile artifacts, use `fish --profile=<path>` to measure commands executed after startup and `fish --profile-startup=<path>` to measure startup and configuration loading. Cache only measured repeated work with a defined validity and invalidation contract. Do not add mutable cache state merely because a path is performance-sensitive. Remove temporary breakpoints, profiles, or tracing introduced under explicit authorization when the authorized work is complete unless the task explicitly adds a durable debugging mode.

## Official Sources

Function behavior is documented in the official [Fish language](https://fishshell.com/docs/current/language.html), [`function` reference](https://fishshell.com/docs/current/cmds/function.html), and [`status` reference](https://fishshell.com/docs/current/cmds/status.html). Argument parsing and profiling are documented in the [`argparse` reference](https://fishshell.com/docs/current/cmds/argparse.html) and [`fish` reference](https://fishshell.com/docs/current/cmds/fish.html).
