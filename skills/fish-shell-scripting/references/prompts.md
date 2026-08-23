# Fish prompts

## Preserve prompt status and contracts

Fish builds prompts from named functions:

- `fish_prompt` writes the left prompt.
- `fish_right_prompt` writes the right prompt.
- `fish_mode_prompt` writes mode information when modal bindings use it.

Give every prompt function and helper the entrypoint’s required source docstring.

- Capture `$status` and, when needed, `$pipestatus` before any other status-producing command. Prompt work must not erase the result it intends to display.
- Keep prompt functions deterministic and fast because Fish invokes them repeatedly during interactive use.
- Write prompt content to standard output and keep diagnostics off the normal prompt path.
- Restore color and style state deliberately with `set_color` so one segment does not leak presentation into another.
- Use Fish prompt helpers such as `prompt_pwd` and `prompt_hostname` when their documented behavior matches the design.
- Treat literal labels, failure text, and user-facing symbols as human-facing text under the entrypoint’s `human-facing-writing` route. Pure control sequences and exact glyph tokens remain syntax or presentation data.

## Handle prompt variants explicitly

- Enable transient prompts with `set --global fish_transient_prompt 1`. Fish then reruns the prompt functions with the `--final-rendering` argument before executing a command line.
- In the final-rendering branch, keep the reduced prompt semantically consistent with the normal prompt.
- Preserve behavior across ordinary users, privileged users, remote sessions, missing repository state, and failed previous commands when those states are represented.
- Keep version-control and environment probes bounded. Avoid spawning expensive processes for information Fish already exposes through variables or builtins.
- Do not print startup banners or unrelated diagnostics from prompt functions.

## Validate rendered behavior

- Parse and format every changed prompt file.
- Exercise success and failure statuses and verify that the prompt does not replace the captured value before rendering it.
- Check remote, root, version-control, multiline, right-prompt, mode-prompt, and transient states that the implementation supports.
- Inspect color resets and line endings in an interactive Fish session or an established prompt test harness.
- Confirm that prompt output remains absent from noninteractive startup paths.

## Official sources

The Fish behavior underlying this guidance is documented in the official [prompt guide](https://fishshell.com/docs/current/prompt.html), [Fish language](https://fishshell.com/docs/current/language.html), and [`set_color` reference](https://fishshell.com/docs/current/cmds/set_color.html).
