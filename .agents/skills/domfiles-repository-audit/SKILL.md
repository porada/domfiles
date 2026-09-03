---
name: domfiles-repository-audit
description: |-
    Audit this repository.
disable-model-invocation: true
metadata:
    internal: true
---

# Domfiles repository audit

Changing the default scope does not change the workflow’s gates.

## Resolve the scope

| Priority | Rule |
| --- | --- |
| Absolute exclusions | Apply every applicable instruction that explicitly prohibits reading or analyzing content. Explicit user scope cannot override these exclusions. |
| Explicit scope | When the user specifies paths, categories, inclusions, or exclusions, treat them as authoritative over publication-audit and default scope rules. Apply every other applicable `AGENTS.md` instruction within that scope. Include explicitly named untracked paths and symbolic links without dereferencing a link unless the request or applicable policy requires its target. |
| Publication audit | Use this mode when an audit evaluates the tracked `HEAD` tree for public disclosure. Resolve the reportable scope from its regular files rather than the active checkout, and exclude every untracked path, including `.config/fish/local.fish`. When the audit requires an isolated copy of that tree, follow [Publication audit staging](references/publication-audit-staging.md). |
| Default scope | Without explicit scope, start with Git-tracked regular files, exclude symbolic links and untracked paths except `.config/fish/local.fish` when repository scope rules include it, and apply every other default inclusion, exclusion, and exemption from applicable `AGENTS.md` files. |

1. Read every applicable `AGENTS.md` file before reviewing any other repository content. Consult `.agents/PROJECT.md` for relevant project rationale before resolving the audit scope.
2. Apply the precedence table above to resolve the reportable scope.
3. Exclude these paths in publication-audit mode and in every default repository scope, including `/domfiles-repository-audit` without an explicit scope. An explicit exhaustive scope such as “every tracked file” includes them, subject to the absolute exclusions above:
    - `.agents/skills/domfiles-zed-settings/scripts` and its descendants otherwise require an explicit request for that subtree or the Zed-settings skill scripts. Agent documentation or Zed settings alone does not count as explicit inclusion.
    - `.config/zed/settings.json` and `.zed/settings.json` otherwise require explicit inclusion of either file or Zed settings.
4. Inspect content outside the reportable scope only when needed as supporting evidence for a path in the reportable scope. Absolute exclusions still apply, and supporting evidence does not become reportable.

## Partition a large audit

- Divide a large scope into complete, non-overlapping passes and treat them as one continuous audit.
- Resolve each pass’s scope before execution so supported clients can discover every applicable project-local `domfiles-*` skill from its description. When delegating a pass, identify those skills for the delegate instead of accumulating their bodies in the coordinating context.
- Apply the global “Prompt contract” policy to every delegated pass. Identify the applicable `AGENTS.md` files and relevant domain skills for the delegate to load. Identify this audit workflow by its project-relative path, `.agents/skills/domfiles-repository-audit/SKILL.md`. Do not copy those documents into the prompt. Keep the reportable scope, coverage tracking, cross-pass synthesis, and issue IDs in the coordinating context.

## Audit the contents

For every path in the reportable scope:

- Check for redundancies, inconsistencies, typos, and structural or type issues.
- Ensure there is no dead or unused code.
- Report any cases where in-scope code reimplements behavior already available in the language, standard library, or existing shared utilities in this repository. When the audit has a comparison baseline, apply this check specifically to new code.
- Include comments and documentation in the analysis. Report factual claims in either that no longer match current repository behavior, the supported environment, or applicable project rationale, or that no longer make sense in their current context.
- Report documentation that duplicates durable details or violates the [documented authority and ownership boundaries](../../../AGENTS.md#agent-documentation).
- Apply every relevant repository instruction and loaded domain-skill policy, treating domain skills as supplements for domain-specific checks and verification rather than separate audit workflows.

## Preserve the read-only process

- Do not modify repository files or run linters or formatters as part of the analysis.
- Do not report findings outside the reportable scope.
- Base findings on the current repository contents under review. When current behavior must be verified, use authoritative installed-tool behavior or official documentation and source as supporting evidence.
- Never speculate about intent or hypothetical implementations.
- Do not stop after individual findings. Continue until the entire scope has been reviewed, then report all findings together.

## Report the result

Follow the global [communication](../../../.config/zed/AGENTS.md#communication) and [issue-reporting](../../../.config/zed/AGENTS.md#documentation) requirements, then:

1. Lead with the findings. If there are none, state that the audit found no reportable issues.
2. State the resolved reportable scope.
