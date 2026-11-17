# Reviewer Defaults

## Profiles

Use these model IDs with `xhigh` effort:

- **Claude:** `claude-opus-5-5`.
- **Codex:** `gpt-6.1-sol`.

Each initial group uses both profiles. Each final group uses the Claude profile in a fresh reviewer session.

Apply invocation-specific selections to the requested roles without changing these defaults. Resolve the requested models and settings through the host’s supported identifiers. Do not silently substitute a different provider, model, or effort.

## Titles

Capture the initiating thread’s title when the run begins. Use the following titles in stage order for reviewer sessions. When the interface has no session title field, retain the title with the session reference in the user-visible review record.

- **Initial reviewers:** `⤷ Review: <original thread title>`
- **First final review:** `⤷ Final Review: <original thread title>`
- **Repeated final reviews:** `⤷ Final Review <n>: <original thread title>`, beginning with `2`.

A replacement retains its stage’s title and cycle number. Follow-ups do not change the session’s title or increment the number.

When first naming a reviewer role in a stage with multiple groups, append a short group label, including the repository when needed to distinguish units. Keep existing titles when groups are added. Final-review numbering belongs to each final group.
