---
name: domfiles-repository-audit
description: Perform a read-only audit of the default domfiles repository scope or an explicitly scoped subset. Use this skill whenever the user requests an audit—including the bare `Audit` command—for redundancies, inconsistencies, typos, outdated or duplicated documentation, dead or unused code, structural or type issues, or reimplemented behavior. Do not use it for commit reviews, ordinary code review, debugging, or implementation tasks.
---

# Domfiles repository audit

Audit the resolved reportable scope without modifying it.

## Resolve the scope

| Priority | Rule |
| --- | --- |
| Absolute exclusions | Apply every applicable instruction that explicitly prohibits reading or analyzing content. Exclude symbolic links without reading or resolving their targets. Exclude untracked paths except `.config/fish/local.fish` when repository scope rules include it. Explicit user scope cannot override these exclusions. |
| Explicit scope | When the user specifies paths, categories, inclusions, or exclusions, treat them as authoritative over default scope rules. Apply every other applicable `AGENTS.md` instruction within that scope. |
| Default scope | Without explicit scope, start with Git-tracked paths and apply every default inclusion, exclusion, and exemption from applicable `AGENTS.md` files. |

1. Read every applicable `AGENTS.md` file before reviewing any other repository content.
2. Apply the precedence table above to resolve the reportable scope.
3. Treat `.config/zed/settings.json` and `.zed/settings.json` as a default exclusion that explicit scope may override by explicitly including either file or Zed settings. Repository-wide scope alone does not count as explicit inclusion.
4. Inspect content outside the reportable scope only when needed as supporting evidence for a path in the reportable scope. Absolute exclusions still apply, and supporting evidence does not become reportable.

## Partition a large audit

- Divide a large scope into complete, non-overlapping passes and treat them as one continuous audit.
- Load only the domain skills relevant to a pass immediately before auditing it. When delegating a pass, have the delegate load those skills instead of accumulating their bodies in the coordinating context.
- Give each delegate only the exact pass scope, applicable `AGENTS.md` files, this audit workflow, and relevant domain skills. Keep the reportable scope, coverage tracking, cross-pass synthesis, and issue IDs in the coordinating context.

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

## Stage a publication audit

When a publication audit requires a clean copy of tracked `HEAD`:

1. Create a tar archive with `git archive --format=tar --output=<temporary-path> HEAD`.
2. Keep the archive and extraction destination beneath Zed agent terminal temporary directories.
3. Extract only with `tar -xf <temporary-archive> -C <temporary-directory>`.
4. Do not use alternate archive formats, refs, paths, or broader extraction options.

## Report the result

Follow the [global issue-reporting requirements](../../../.config/zed/AGENTS.md#documentation), then:

1. Lead with the findings. If there are none, state that the audit found no reportable issues.
2. State the resolved reportable scope and identify anything within it that could not be verified.
