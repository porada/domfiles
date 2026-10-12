# Collect Documentation Checks

Record mechanical documentation validation attempts, whether command-driven or manual, regardless of outcome. Do not change validation choices or perform additional checks for collection.

Mechanical checks test explicit, reproducible conditions such as link and anchor resolution, frontmatter shape, naming, ordering, or prescribed punctuation. Searches and comparisons used to interpret meaning, assess policy conflicts, or judge wording against intent are semantic review, not mechanical checks.

Exclude ordinary context gathering, routine formatter invocations, repository bookkeeping, patch inspection, and generic patch hygiene checks, including `git diff --check`. Documentation-specific checks remain in scope when they use Git to select or read their inputs.

Each agent records its own attempts. Every delegating agent must ensure that delegates whose work may include mechanical documentation validation receive this requirement, including its read-only exception, and propagate it through further delegation. This applies during commit reviews as well as other documentation work.

## Capture

At a natural validation boundary, before temporary script cleanup, save one Markdown record per invocation or coherent manual check under `~/.local/state/domfiles/agent-documentation/`.

Include the working directory, using a home-relative path where applicable. For executed checks, preserve the exact command in a fenced code block and any ad hoc script contents it invokes. Keep compound commands, pipelines, whole scripts, and coherent manual checks intact when they include an in-scope check, even if they also contain excluded work. For manual checks, briefly describe the mechanical condition checked and the procedure used, identifying the relevant files. Do not include results, additional metadata, or reasoning narratives.

## Save

Generate a `<unix-time-ms>-<short-random-suffix>.md` filename and create the record without overwriting an existing file, all within the same save operation. Use the runtime’s built-in clock and random source. The timestamp represents when the record is saved. Multiple records may be saved together.

Do not make separate naming or clock queries, scan the destination, inspect existing records, maintain shared counters or indexes, or deduplicate the collection.

## Finish

Successful completion of the write is sufficient. Do not read back, validate, format, lint, hash, or execute the resulting record. Treat collected records as retained data, exempt from documentation validation and ordinary task-artifact cleanup.

## Boundaries

The global **Task authorization** policy permits record creation during otherwise read-only work. This does not permit edits to reviewed material. Preserve explicit restrictions on collection and existing access and security requirements.

Skip the entire record if its contents would include a secret or other material excluded by the global **Secrets and authentication** policy. Do not save a redacted substitute or retrieve excluded values to determine whether to record it. Treat the omission as an intentional exclusion, not a collection failure requiring a report, retry, or repair.

If recording otherwise cannot proceed, continue the primary task and report the collection gap once. Do not rerun validation, provision storage, or begin a repair workflow merely to complete collection.
