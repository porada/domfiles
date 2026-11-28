# Agent Instructions

## Overview

- **Repository:** Contains all actively used dotfiles and is also called domfiles.

## Public Repository Boundary

- **Public surfaces:** Treat every tracked file, proposed repository artifact, and patch as publicly disclosed. Apply that requirement to communication intended for inclusion in those artifacts or another public surface. Other agent communication follows the authorized recipient and processing boundaries under global **External services** and **Agent communication**. Permission to communicate context doesn’t authorize including it in a public artifact.
- **Authentication review:** Before recommending or implementing a tool that uses authentication or private configuration, establish:
    - Which secrets or private values it requires.
    - How those values enter at runtime without appearing in command literals or repository files.
    - Where the tool persists credentials and generated configuration.
    - Whether any generated or modified file can enter the repository.
- **Feasibility:** Treat a tool as feasible only when its public configuration can remain separate from secret material through an established machine-local source, external credential store, or qualifying mechanism under global **Client-mediated operations**. Temporary client delivery must remain outside the repository and does not establish persistent credential storage.
    - **Ignored-file boundary:** A Git-ignored file qualifies as an established machine-local source only when public repository provisioning creates or adopts it without embedding secret values, restricts it to user-only access, and tracked configuration refers only to its path. Ignore status alone isn’t enough.
- **No safe route:** If no established public-safe route exists, report the tool as infeasible or ask the user to choose a secret storage boundary.

## Agent Documentation

| Source | Authority and Ownership |
| --- | --- |
| `.agents/GLOBAL.md` | Defines global user defaults. Follow [Maintain Global Instructions](skills/.domfiles-agent-documentation/references/maintain-global-instructions.md) when editing, reviewing, or auditing this file and before publishing a change to it. |
| `.agents/ANTIGRAVITY.md` | Directs Antigravity to load `.agents/GLOBAL.md` as user instructions. |
| `AGENTS.md` | Defines project instructions, scope, and documentation authority. Applicable project instructions override global defaults. |
| Project-authored skill directories under `.agents/skills/` and `skills/` | Own delegated domain policy, workflows, validation, and reporting exceptions without contradicting applicable `AGENTS.md` instructions. |
| `.agents/PROJECT.md` | Records constraints, durable facts, maintenance decisions, and rationale. It doesn’t override agent instructions. |
| `.agents/project/` | Each reference document has the authority of the section it replaces in `.agents/PROJECT.md`. The [composition rules](skills/.domfiles-agent-documentation/references/compose-documentation.md) apply as if its contents still appeared under the original heading. |

Unqualified phrases such as “global agent instructions,” “global `AGENTS.md`,” and “global `AGENTS` document,” along with equivalent wording, refer to `.agents/GLOBAL.md`.

## General

- **User direction:** The global **Explicit user direction** policy applies to this repository’s workflow requirements, conventions, and non-secret content exclusions, including those in project-authored skills. The [public repository boundary](#public-repository-boundary) remains applicable.
- **Environment:** Follow the [supported environment](.agents/PROJECT.md#supported-environment), including its default-shell requirement.
- **Worktree for agent instructions and skills:** In T3 Code, a task starting in the project root that will edit `.agents/GLOBAL.md` or anything under `skills/` is authorized to [move its thread into a new worktree](skills/.domfiles-skill-development/references/domfiles-skill-policy.md#enter-a-task-worktree) before its first edit. Make that move unless the task reviews, commits, or continues changes already pending in the root checkout.
- **Command entrypoints:** Use the entrypoint documented for each task, including validation, rather than another route to the same implementation. Invoke [repository-scoped commands](.agents/PROJECT.md#repository-scoped-commands) through `PATH`, overriding global **Direct invocation** for `skills` and `vercel` in this repository. For domfiles source validation, follow [Checkout Command Selection](#checkout-command-selection) so the command exercises the task checkout.
- **Navigation:** Consult the relevant sections of `.agents/PROJECT.md` and follow their links to reference documents in `.agents/project/` as needed.
- **Sync recovery:** Consult [Interrupted Sync Recovery](.agents/project/interrupted-sync-recovery.md) when restoring a checkout after an interrupted repository update or changing the marker or private refs used for that restore.
- **Durable knowledge:** Record project knowledge only when it meets the global **Durable knowledge** threshold. Don’t record optimization recaps, routine implementation details, or workflow summaries. Prefer updating existing material over adding another explanation. If the task doesn’t permit the edit, report deferred documentation work only if it meets that threshold.
- **Separate reference documents:** Keep project knowledge in `.agents/PROJECT.md` by default. Record knowledge within a moved section’s scope in its reference document. When a section moves, keep its original heading with a link that says what the document holds, which tasks need it, and why it is separate. Point references in instructions and skills directly to the document. Move a whole section to `.agents/project/` only when all three conditions hold.
    - The section serves one identifiable kind of task.
    - An instruction in `AGENTS.md` or a project skill tells readers when to consult it.
    - Given the section’s length and how often tasks would use it or skip it, the context saved on unrelated tasks outweighs the extra lookup for its readers.
- **Zed settings:** Keep `.zed/settings.json` and `home/.config/zed/settings.json` free of redundant entries. An entry is redundant only when omitting it preserves the supported workflow, including tool-managed state and later invocations. Matching a default or inherited value alone doesn’t make an entry redundant. See the [CLI persistence rationale](.agents/PROJECT.md#zed-cli-open-behavior).

## Validation

For changed JSON files, run `pnpm run lint:json <changed-json-files>`. Pass paths explicitly unless repository-wide validation is intended.

For Markdown validation, invoke `doclint` and follow [Check Markdown](skills/.domfiles-agent-documentation/references/check-markdown.md).

### Checkout Command Selection

When invoking a repository-scoped launcher to validate domfiles source, prepend the task checkout’s absolute `home/.local/bin` path to `PATH` for that invocation and verify that the launcher resolves there. Other prescribed validation entrypoints retain their documented invocation. From the task checkout root, for example:

```sh
env PATH="$PWD/home/.local/bin:$PATH" doclint --no-ignore --max-warnings 0 -- AGENTS.md
```

For a check whose working directory must be another project, retain the absolute launcher directory from the task checkout rather than deriving it from that project’s `$PWD`. Leave persistent `PATH` settings, the provisioned `~/.local/bin` link, and global skill discovery unchanged. Ordinary use outside domfiles development continues through the provisioned installation.

If a check cannot run within the task’s authority, report the validation limitation rather than substituting another checkout’s implementation.

## Scope

- **Fish:** Treat [`home/.config/fish/local.fish`](.agents/project/shell-integration.md#fish-local-configuration) as secret-bearing machine-local configuration. Exclude its contents from analysis, execution, and validation, including indirect sourcing through Fish startup.
- **Symlink:** Exclude the contents of `home/.local/bin/git-diff-highlight` by default because its target is outside the repository. Include its target only when the user explicitly requests that analysis and access boundaries permit it.
- **Secret-bearing local files:** Don’t read, analyze, echo, or stage Git-ignored files that public provisioning and tracked configuration designate for machine-local secret material. Path-level metadata and public provisioning code remain in scope.

## Reporting

- **Empty configuration:** Don’t report empty configuration files.
- **Fixed locations:** Report cases that would tie this repository to a fixed filesystem location, except:
    - `$HOME/*` paths, system paths, or vendor paths.
    - Symlinks created through `domfiles sync`.
    - `home/.config/fish/fish_variables`.
    - Documentation.
- **Intentional stops:** When evaluating workflow interruptions, consult [Intentional Workflow Stops](.agents/project/intentional-workflow-stops.md). Treat the recorded tradeoffs as settled within their stated scope. Don’t report a stop as a defect or recommend its removal solely because it interrupts autonomous work. Reconsider the decision when the user requests it or concrete evidence challenges its rationale, including fulfillment of a recorded reconsideration condition. Still report incorrect implementation, application outside the intended scope, redundant approval requests, or conflicts with governing instructions. Identify what falls outside the recorded decision or what evidence warrants reconsideration.

## Skills

Follow the [domfiles skill policy](skills/.domfiles-skill-development/references/domfiles-skill-policy.md) for project-authored skill work, including classification, supported installation, scripts, and test integration.
