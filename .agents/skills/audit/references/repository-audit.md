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

- Keep the endpoint analysis-only. Audit authority doesn’t cover fixes, source formatting, snapshot updates, or changes to repository configuration, dependency manifests and lockfiles, or tracked generated files.
- Don’t report findings outside the reportable scope.
- Base findings on the current repository contents under review. When current behavior must be verified, use authoritative installed tool behavior or official documentation and source as supporting evidence.
- Never speculate about intent or hypothetical implementations.

### Choose Automated Checks

Use relevant automated checks, including linter and formatter check modes, when their actual effects fit the audit’s authority and preserve the contents under review. Prefer existing project entrypoints and documented behavior. A `--check` or “read-only” label alone doesn’t establish eligibility.

- **Execution:** Establish the selected inputs, effective configuration, and material effects of the complete invocation, including its launcher, startup files, plugins, hooks, and lifecycle steps. Apply the global **Code Execution** and **Security** boundaries. Verify indirect reads, such as linked documents, against the reportable scope, supporting-evidence allowance, and remaining exclusions under [Resolve Audit Scope](#resolve-audit-scope).
- **Dependencies:** The global **Prescribed acquisition** policy covers prerequisite tool and dependency installation or reconciliation for audit checks, provided it preserves source, repository configuration, dependency manifests and lockfiles, and tracked generated files in both the audited and shared tool checkouts. Apply this condition to configured lifecycle steps too. [pnpm execution behavior](../../../PROJECT.md#pnpm-execution-and-lockfile-behavior) illustrates effects to account for, not a blanket exemption for a launcher. Explicit task restrictions and separate grants still govern.
- **Outputs:** Audit authority includes creating or updating bounded, disposable generated output and tool-managed caches needed for checks, and creating task-owned scratch data, while preserving audited inputs and unrelated state. Use `agent-task-directories` for agent-managed scratch data and its lifecycle. Ignore status alone doesn’t establish ownership, disposability, or permission to overwrite a path.
- **Eligibility:** Omit optional checks whose inputs or effects cannot be established or exceed the audit’s scope or authority, continue independent inspection, and report material limitations. Ask through the global **Execution checkpoints** policy only when a decision or additional authority is necessary to establish the evidence the user requested. Don’t bypass required entrypoints or build new wrappers or isolation infrastructure merely to make a check eligible.

### Interpret Check Evidence

Tie each result to the contents and revision actually checked, relevant configuration and dependencies, and effective input coverage. Apply the global **Validation evidence** policy before reusing results. For publication audits, preserve the [selected tree](publication-audit-trees.md) and its [isolation requirements](publication-audit-staging.md) when applicable. A check against working-tree files doesn’t establish a result for the selected commit unless the relevant inputs and execution conditions match.

Confirm that intended inputs were checked, accounting for ignored files, partial execution, and empty input sets. Treat diagnostics as candidate findings to validate against current contents and project rationale. A passing check establishes only its actual coverage, not completion of the audit or correctness beyond the properties it checks.

## Report Audit Results

Before reporting a finding, check `.agents/PROJECT.md` for project rationale that explains or settles it.

Continue until the entire scope has been reviewed, then report all findings together.

Follow the global [communication](../../../GLOBAL.md#communication) and [**Findings**](../../../GLOBAL.md#findings) requirements, then:

1. Lead with the findings. If there are none, state that the audit found no reportable issues.
2. State the resolved reportable scope.
3. Report material validation limitations, including relevant checks omitted because their effects or target fidelity could not be established. Distinguish inspection from executed checks without listing routine passing diagnostics.
