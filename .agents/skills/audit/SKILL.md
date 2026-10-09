---
name: audit
description: |-
    Audit this repository.

disable-model-invocation: true
metadata:
    internal: true
---

# Domfiles Repository Audit

## Resolve Audit Scope

The table lists higher-precedence rules first. Apply publication audit scope when that mode is selected and default scope otherwise.

| Priority | Rule |
| --- | --- |
| Security exclusions | Preserve credential protection, access controls, and higher-priority read restrictions. Explicit scope does not bypass these boundaries. |
| Explicit scope | Override only the corresponding publication audit or default scope rules. Interpret the request through [Audit Path Selection](references/audit-path-selection.md). |
| Publication audit | Use this mode when an audit evaluates the tracked `HEAD` tree for public disclosure. Resolve the reportable scope from its regular files rather than the active checkout, and exclude every untracked or ignored path, including `home/.config/fish/local.fish`. Apply the [publication audit requirements](#inspect-publication-audit-trees). |
| Default scope | Start with Git-tracked regular files, exclude symbolic links and untracked or ignored paths except `home/.config/fish/local.fish` when repository scope rules include it, and apply every other default inclusion, exclusion, and exemption from applicable `AGENTS.md` files. |

1. Read every applicable `AGENTS.md` file before reviewing any other repository content. Consult `.agents/PROJECT.md` for relevant project rationale before resolving the audit scope.
2. Apply the precedence table above to resolve the reportable scope.
3. Exclude these paths in publication audit mode and in every default repository scope, including `/audit` without an explicit scope. An explicit exhaustive scope such as “every tracked file” includes them, subject to the security exclusions above:
    - `.agents/skills/zed-settings/scripts` and its descendants otherwise require an explicit request for that subtree or the Zed settings skill scripts. Agent documentation or Zed settings alone does not count as explicit inclusion.
    - `home/.config/zed/settings.json` and `.zed/settings.json` otherwise require explicit inclusion of either file or Zed settings.
4. Inspect content outside the reportable scope only when needed as supporting evidence for a path in the reportable scope. Security exclusions still apply, and supporting evidence does not become reportable.

## Inspect Publication Audit Trees

By default, inspect tracked `HEAD` directly rather than materializing an isolated filesystem copy. Capture its exact commit once and use that commit for every read-only Git inspection. Select the repository root through the tool’s working directory parameter or `git -C <repository-root>`, and do not let a moving ref, the current shell subdirectory, or the working tree narrow or alter the evidence.

An explicit user direction may select another route, but it does not establish that an existing tool preserves the selected tree. Do not design or implement a materializer without authorization for that additional work. When the audit requires an isolated copy, follow [Publication Audit Staging](references/publication-audit-staging.md).

## Partition Large Audits

- Divide a large scope into complete, nonoverlapping passes and treat them as one continuous audit.
- Resolve each pass’s scope before execution so supported clients can discover every applicable project-local skill from its description. When delegating a pass, identify those skills for the delegate without loading their bodies into the coordinating context.
- Apply the global **Prompt contract** policy to every delegated pass. Identify this command-only audit workflow by its project-relative path, `.agents/skills/audit/SKILL.md`. Keep coverage tracking, cross-pass synthesis, issue IDs, and the reportable scope in the coordinating context.

## Audit Contents

For every path in the reportable scope:

- Check for redundancies, inconsistencies, typos, and structural or type issues.
- Ensure there is no dead or unused code.
- Report any cases where in-scope code reimplements behavior already available in the language, standard library, or existing shared utilities in this repository. When the audit has a comparison baseline, apply this check specifically to new code.
- Include comments and documentation in the analysis. Report factual claims in comments or documentation that no longer match current repository behavior, the supported environment, or applicable project rationale, or that no longer make sense in their current context.
- Report documentation that duplicates durable details or violates the [documented authority and ownership boundaries](../../../AGENTS.md#agent-documentation).
- Apply every relevant repository instruction and loaded domain skill policy, treating domain skills as supplements for domain-specific checks and verification rather than separate audit workflows.

## Keep Audits Read-Only

- Do not modify repository files or run linters or formatters as part of the analysis, including read-only check modes.
- Do not report findings outside the reportable scope.
- Base findings on the current repository contents under review. When current behavior must be verified, use authoritative installed tool behavior or official documentation and source as supporting evidence.
- Never speculate about intent or hypothetical implementations.
- Do not stop after individual findings. Continue until the entire scope has been reviewed, then report all findings together.

## Report Audit Results

Follow the global [communication](../../GLOBAL.md#communication) and [**Findings**](../../GLOBAL.md#documentation) requirements, then:

1. Lead with the findings. If there are none, state that the audit found no reportable issues.
2. State the resolved reportable scope.
