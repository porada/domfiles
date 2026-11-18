# Reviewer Defaults

## Profiles

The default reviewer profiles are Claude Opus and Codex GPT Sol, both at `xhigh` effort. Each initial group uses both profiles. Each final group uses the Claude profile in a fresh reviewer session. Apply the user’s runtime overrides to every role they cover. When the user selects only Claude or Codex, use that provider’s default reviewer profile.

For each new assessment, resolve the reviewer’s model against the current catalog in that review unit’s bound environment. Honor an explicitly requested version. Otherwise, identify the newest available version within the selected provider and model family. If the initial listing is partial, search the catalog for that family and verify the version order. If the required version cannot be established or run at the selected effort, pause the affected reviewer and report the limitation. Keep the resolved profile when resuming or replacing a reviewer for the same assessment.

## Titles

Capture the initiating thread’s title when the run begins. Use the following titles in stage order for reviewer sessions. When the interface has no session title field, retain the title with the session reference in the user-visible review record.

- **Initial reviewers:** `⤷ Review: <original thread title>`
- **First final review:** `⤷ Final Review: <original thread title>`
- **Repeated final reviews:** `⤷ Final Review <n>: <original thread title>`, beginning with `2`.

A replacement retains its stage’s title and cycle number. Follow-ups do not change the session’s title or increment the number.

When first naming a reviewer role in a stage with multiple groups, append a short group label, including the repository when needed to distinguish units. Keep existing titles when groups are added. Final-review numbering belongs to each final group.
