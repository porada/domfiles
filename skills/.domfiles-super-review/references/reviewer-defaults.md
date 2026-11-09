# Reviewer Defaults

## Profiles

Use these model IDs with `xhigh` effort:

- **Claude:** `claude-opus-5-5`.
- **Codex:** `gpt-6.1-sol`.

The initial stage uses both profiles. The final stage uses the Claude profile in a fresh thread.

Apply invocation-specific selections to the requested roles without changing these defaults. Resolve the requested models and settings through the host’s supported identifiers. Do not silently substitute a different provider, model, or effort.

## Titles

Capture the initiating thread’s title when the run begins. Use the following titles in stage order:

- **Initial reviewers:** `⤷ Review: <original thread title>`
- **First final review:** `⤷ Final Review: <original thread title>`
- **Repeated final reviews:** `⤷ Final Review <n>: <original thread title>`, beginning with `2`.

A replacement retains its stage’s title and cycle number. Follow-ups do not rename the thread or increment the number.
