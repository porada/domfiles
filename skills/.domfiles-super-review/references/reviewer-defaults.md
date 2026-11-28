# Reviewer Defaults

## Profiles

The default reviewer models are Claude Opus and Codex GPT Sol. On every review interface, use the model’s default effort from global **Thread effort** for each reviewer profile. Apply the user’s runtime overrides to every role they cover. When the user selects only Claude or Codex, use that provider’s default reviewer profile.

By default, use the Claude profile for every peer review group. Add the Codex profile when the change takes effect through how agents or people read its wording. Executable work can be checked by running it. Instructions depend on interpretation, and agents from both providers read them. A group whose wording only accompanies an executable change uses Claude alone.

Each cold review group uses the Claude profile in a fresh reviewer session.

For each new assessment, resolve the reviewer’s model against the current catalog in that review unit’s bound environment. Honor an explicitly requested version. Otherwise, identify the newest available version within the selected provider and model family. If the initial listing is partial, search the catalog for that family and verify the version order. If the required version cannot be established or run at the selected effort, pause the affected reviewer and report the limitation. Keep the resolved profile when resuming or replacing a reviewer for the same assessment.

## Titles

Apply global **Thread titles** when naming reviewer sessions. Use the caller’s task title wording throughout the review. When none was supplied, select a task title from the initiating thread. Use the following titles in stage order for reviewer sessions. When the interface has no session title field, retain the title with the session reference in the user-visible review record.

- **Peer reviewers:** `⤷ Peer Review: <title>`
- **First cold review:** `⤷ Cold Review: <title>`
- **Repeated cold reviews:** `⤷ Cold Review <n>: <title>`, beginning with `2`.

A replacement retains its stage’s title and cycle number. Follow-ups do not change the session’s title or increment the number.

When first naming a reviewer role in a stage with multiple groups, append a short group label, including the repository when needed to distinguish units. When the caller supplies a group label, append it even in a single-group stage. Keep existing titles when groups are added. Number cold reviews separately for each group.
