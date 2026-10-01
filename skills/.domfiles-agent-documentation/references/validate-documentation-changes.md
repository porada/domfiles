# Validate Documentation Changes

After capturing all task-authorized documentation updates intended for the current change:

1. Resolve every unjustified direct-path increase found by the complete-scope footprint check. Move conditional guidance into a reference in the existing skill, and remove obsolete direct-path wording in the same change.
2. Resolve every missing behavioral distinction, condition, exception, or route found by the complete-scope moved guidance check.
3. Run targeted diagnostics and `git diff --check` for the changed documentation without formatting unrelated files. Use repeatable checks for deterministic invariants such as links, anchors, skill identities, template substitutions, and verbatim mirrors. Treat their success as evidence only for those invariants, not as proof of factual accuracy, routing completeness, preserved authority, or readable prose. Inspect task-owned untracked documentation directly because Git diff checks do not include it. Do not stage files solely for validation.
4. Perform one bounded final alignment pass over the changed documentation against [Apply Documentation Principles](../SKILL.md#apply-documentation-principles), the resolved local authority model, applicable project values, and explicit user decisions. Include [Workflow Compatibility](#workflow-compatibility) in that pass. Correct concrete discrepancies within the authorized scope before delivery. Treat this as a completion check rather than a drafting gate: do not withhold useful documentation, reopen settled decisions, repeatedly rewrite compliant content, or expand scope for speculative improvements. If a correction requires new authorization, preserve the completed changes and report that boundary.

## Workflow Compatibility

Before finalizing new or materially changed operational guidance, identify the failure it is meant to prevent and the supported work it must still permit. Derive expected outcomes from governing instructions, explicit user decisions, and verified behavior, not from the proposed wording alone.

Within the existing bounded alignment pass, trace concrete cases through the applicable inherited rules and routed workflows. When a change alters how a workflow is entered, revisit the assumptions that depended on the previous entry point. Verify the default path and how each remaining branch is selected. Cover the permitted path, the relevant stop or approval boundary, and any affected exception, recovery, or continuation. Check that prerequisites are available at the phase that requires them and that valid authorization remains effective through composition. Trace mode-specific effects and cleanup through completion or stopping.

Validate a documented command procedure as a complete sequence for each materially different input class it claims to support. Verify effective configuration and implicit effects in the supported invocation context, not merely accepted syntax. A successful individual command does not validate later stages. Bound claims such as “every wrapper” or “all scripts” by verified shared behavior. When execution is unauthorized or unavailable, distinguish inspection from execution and report the remaining verification gap.

Reuse established evidence and choose only cases needed for the changed behavior and its direct integration boundaries. Preserve existing complete-scope checks and approval requirements. Report uncertainty or conflicts rather than inventing a workaround.
