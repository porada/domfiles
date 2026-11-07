# Repository Audit

## Resolve Audit Scope

The table lists higher-precedence rules first. Apply publication audit scope when that mode is selected and default scope otherwise.

| Priority | Rule |
| --- | --- |
| Security exclusions | Preserve credential protection, access controls, and higher-priority read restrictions, including the [exclusion of `home/.config/fish/local.fish`](../../../../AGENTS.md#scope). Explicit scope doesn’t bypass these boundaries. |
| Explicit scope | Override only the corresponding publication audit or default scope rules. Interpret the request through [Audit Path Selection](audit-path-selection.md). |
| Publication audit | Use this mode when an audit evaluates the tracked `HEAD` tree for public disclosure. Resolve the reportable scope from the tree’s regular files rather than the active checkout, and exclude every untracked or ignored path. |
| Default scope | Start with Git-tracked regular files, exclude symbolic links and untracked or ignored paths, and apply every other default inclusion, exclusion, and exemption from applicable `AGENTS.md` files. |

1. Apply the precedence table above to resolve the reportable scope.
2. Inspect content outside the reportable scope only when needed as supporting evidence for a path in that scope. Security exclusions still apply, and supporting evidence doesn’t become reportable.

## Audit Contents

For every path in the reportable scope:

- Check for redundancies, inconsistencies, typos, and structural or type issues.
- Ensure there is no dead or unused code.
- Report code that reimplements behavior already available in the language, standard library, or existing shared utilities in this repository. When the audit has a comparison baseline, apply this check specifically to new code.
- Include comments and documentation in the analysis. Report factual claims in comments or documentation that no longer match current repository behavior, the supported environment, or applicable project rationale, or that no longer make sense in their current context.
- Report documentation that duplicates durable details or violates the [documented authority and ownership boundaries](../../../../AGENTS.md#agent-documentation).
- Apply every relevant repository instruction and loaded domain skill policy. Use domain skills to supplement the audit with domain-specific checks and verification, rather than as separate audit workflows.

## Keep Audits Read-Only

- Don’t modify repository files or run linters or formatters as part of the analysis, including read-only check modes.
- Don’t report findings outside the reportable scope.
- Base findings on the current repository contents under review. When current behavior must be verified, use authoritative installed tool behavior or official documentation and source as supporting evidence.
- Never speculate about intent or hypothetical implementations.

## Report Audit Results

Before reporting a finding, check `.agents/PROJECT.md` for project rationale that explains or settles it.

Continue until the entire scope has been reviewed, then report all findings together.

Follow the global [communication](../../../GLOBAL.md#communication) and [**Findings**](../../../GLOBAL.md#findings) requirements, then:

1. Lead with the findings. If there are none, state that the audit found no reportable issues.
2. State the resolved reportable scope.
