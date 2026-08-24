# Fish Prompts

Fish invokes prompt functions throughout interactive use. Preserve the previous command’s state before doing any other prompt work, then keep each render fast and free of unrelated output.

## Prompt Functions

Fish builds the prompt from three named functions and displays what each writes to standard output:

- `fish_prompt` renders the left prompt.
- `fish_right_prompt` renders the right prompt.
- `fish_mode_prompt` renders the current mode when Vi key bindings use it.

Give every prompt function and helper the [required source docstring](../SKILL.md#function-documentation).

## Rendering Contract

Before any status-producing prompt work, capture `$status` and, when the prompt reports the entire previous pipeline, `$pipestatus`. Prompt rendering must not erase the state it intends to display. Keep rendering deterministic for the same inputs, relevant state, and rendering mode.

Write only prompt content to standard output. Keep diagnostics, startup banners, and unrelated messages out of prompt functions. Keep version-control and environment probes bounded. Prefer variables, builtins, and documented Fish helpers such as `prompt_pwd` and `prompt_hostname` when their behavior matches the design.

Set and reset color or style at deliberate boundaries with `set_color` so one segment cannot alter another segment’s presentation accidentally. Treat literal labels, failure text, and user-facing symbols as human-facing text under the [human-facing text contract](../SKILL.md#human-facing-text). Pure control sequences and exact glyph tokens remain syntax or presentation data.

## Prompt States

Enable transient prompts with `set --global fish_transient_prompt 1`. Fish then reruns the prompt functions with the `--final-rendering` argument before executing a command line. The `--final-rendering` branch may simplify the prompt left in terminal scrollback, but any information it keeps must mean the same thing as in the normal rendering.

Define and validate the behavior for both states of every distinction the design communicates, including root versus non-root users, local versus remote sessions, version-control state present versus absent, and successful versus failed commands. A state may intentionally produce no visible segment.

## Validation

- Parse and format every changed prompt file.
- Exercise successful and failed previous commands, including pipelines when the prompt renders `$pipestatus`. Confirm that no prompt operation replaces captured state before rendering it.
- Check each supported state and layout, including local and remote sessions, root and non-root users, version-control state present and absent, multiline prompts, right prompts, mode prompts, and transient rendering.
- Check inherited prompt color defaults in an interactive Fish process. A noninteractive `fish --command` invocation does not establish the interactive default theme state.
- Inspect color and style boundaries and line endings in an interactive Fish session or an established prompt test harness.
- Confirm that prompt output remains absent from noninteractive startup paths.

## Official Sources

Use Fish’s official [prompt guide](https://fishshell.com/docs/current/prompt.html) for prompt lifecycle and transient rendering, the [Fish language](https://fishshell.com/docs/current/language.html) for `$status` and `$pipestatus`, and the [`set_color` reference](https://fishshell.com/docs/current/cmds/set_color.html) for color and style control.
