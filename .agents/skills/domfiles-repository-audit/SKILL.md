---
name: domfiles-repository-audit
description: Perform a read-only audit of the default domfiles repository scope or an explicitly scoped subset. Use this skill whenever the user requests an audit—including the bare `Audit` command—for redundancies, inconsistencies, typos, outdated or duplicated documentation, dead or unused code, structural or type issues, or reimplemented behavior. Do not use it for commit reviews, ordinary code review, debugging, or implementation tasks.
---

# Domfiles repository audit

Audit the resolved repository scope without modifying it. Treat an explicit user scope as authoritative; otherwise audit the default repository scope.

## Resolve the scope

1. Read every applicable `AGENTS.md` file before reviewing any other repository content.
2. Start with Git-tracked paths, then apply the user’s scope modifiers and every inclusion, exclusion, and exemption from the applicable `AGENTS.md` files.
3. Exclude `.config/zed/settings.json` and `.zed/settings.json` unless the user explicitly includes either file or Zed settings in the requested scope. A repository-wide scope alone does not count as explicit inclusion.
4. Exclude symbolic links without reading or resolving their targets.
5. For a large scope, use complete, non-overlapping passes. Treat those passes as one continuous audit and preserve the resolved scope until every pass is complete.

## Audit the contents

For every in-scope path:

- Check for redundancies, inconsistencies, typos, and structural or type issues.
- Ensure there is no dead or unused code.
- Report any cases where in-scope code reimplements behavior already available in the language, standard library, or existing shared utilities in this repository; when the audit has a comparison baseline, apply this check specifically to new code.
- Include comments and documentation in the analysis. Report factual claims in either that no longer match current repository behavior, the supported environment, or applicable project rationale, or that no longer make sense in their current context.
- Report documentation that duplicates or paraphrases the same durable detail in multiple places. Each detail must have one canonical home; other locations should link to that source instead of restating it.
- Apply every relevant repository instruction and loaded domain-skill policy.

## Preserve the read-only process

- Do not modify files or run linters or formatters as part of the analysis.
- Do not report findings outside the resolved scope.
- Base findings on the current repository contents under review. When an applicable domain skill requires current behavior to be verified, use authoritative installed-tool behavior or official documentation and source as supporting evidence without expanding the reportable scope.
- Never speculate about intent or hypothetical implementations.
- Do not stop after individual findings. Continue until the entire scope has been reviewed, then report all findings together.
- Defer required `.agents/PROJECT.md` updates and report them as follow-up work.

## Report the result

Follow the [global issue-reporting requirements](../../../.config/zed/AGENTS.md#documentation), then:

1. Lead with the findings. If there are none, state that the audit found no reportable issues.
2. State the resolved scope and identify anything within it that could not be verified.
