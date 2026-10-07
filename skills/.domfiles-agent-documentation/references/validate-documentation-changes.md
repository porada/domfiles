# Validate Documentation Changes

## Select Validation Scope

For changes, reviews, and audits, select checks by the contract being changed or evaluated before loading detailed validation procedures. Use these cumulative categories, ordered by the behavior affected:

| Change or Evaluation | Required Coverage |
| --- | --- |
| Wording or formatting | Verify preserved meaning, prose conventions, and applicable mechanical checks. Do not reopen unrelated routing, recovery, or peer behavior merely because the file belongs to a skill. |
| Headings, links, or routing | Also verify affected anchors, inbound and directional references, discovery paths, and availability of required guidance from each affected entry point. When routing changes how a workflow is entered or which guidance loads, also apply **Check Workflow Compatibility**. |
| Behavior or authorization | Also trace affected execution paths, scope, permissions, exceptions, recovery, and standalone fallbacks through **Check Workflow Compatibility** below. |

An explicitly broader audit retains its requested scope. When impact is uncertain, inspect enough context to classify it rather than assuming a wording-only change. Keep applicable formatting, identity, link, and mirror checks, and validate the complete scope of every affected invariant, including unchanged members. A shared wording or template change still reaches every governed copy. Reuse established evidence under the global **Validation evidence** policy rather than restarting an unchanged review or check at each phase.

## Complete Change Checks

After capturing all task-authorized documentation updates intended for the current change:

1. Resolve every unjustified direct-path increase found by the complete-scope footprint check. Move conditional guidance into a reference in an existing skill that owns the relevant domain, and remove obsolete direct-path wording in the same change.
2. Resolve every missing behavioral distinction, condition, exception, or route found by the complete-scope moved guidance check.
3. Run targeted diagnostics and `git diff --check` for the changed documentation without formatting unrelated files. Use repeatable checks for deterministic invariants such as links, anchors, skill identities, template substitutions, and verbatim mirrors. Treat their success as evidence only for those invariants, not as proof of factual accuracy, routing completeness, preserved authority, or readable prose. Inspect task-owned untracked documentation directly because Git diff checks do not include it. Do not stage files solely for validation.
4. Perform one bounded final alignment pass over the changed documentation against [Apply Documentation Principles](../SKILL.md#apply-documentation-principles), the resolved local authority model, applicable project values, and explicit user decisions. Include [Check Workflow Compatibility](#check-workflow-compatibility) when the selected scope affects operational behavior or routing, and [Check Repository Independence](#check-repository-independence) when selecting, relocating, or aligning project requirements or establishing project agent documentation. Correct concrete discrepancies within the authorized scope before delivery. Treat this as a completion check rather than a drafting gate: do not withhold useful documentation, reopen settled decisions, repeatedly rewrite compliant content, or expand scope for speculative improvements. If a correction requires new authorization, preserve the completed changes and report that boundary.

## Check Workflow Compatibility

Before finalizing new or materially changed operational guidance, identify the failure it is meant to prevent and the supported work it must still permit. Derive expected outcomes from governing instructions, explicit user decisions, and verified behavior, not from the proposed wording alone.

Within the existing bounded alignment pass, trace concrete cases through the applicable inherited rules and routed workflows. When a change alters how a workflow is entered, revisit the assumptions that depended on the previous entry point. Verify the default path and how each remaining branch is selected. Cover the permitted path, the relevant stop or approval boundary, and any affected exception, recovery, or continuation. Check that prerequisites are available at the phase that requires them and that valid authorization remains effective through composition. Trace mode-specific effects and cleanup through completion or stopping.

Validate a documented command procedure as a complete sequence for each materially different input class it claims to support. Verify effective configuration and implicit effects in the supported invocation context, not merely accepted syntax. A successful individual command does not validate later stages. Bound claims such as “every wrapper” or “all scripts” by verified shared behavior. When execution is unauthorized or unavailable, distinguish inspection from execution and report the remaining verification gap.

Reuse established evidence and choose only cases needed for the changed behavior and its direct integration boundaries. Preserve existing complete-scope checks and approval requirements. Report uncertainty or conflicts rather than inventing a workaround.

### Evaluate Workflow Behavior

When a change aims to reduce repetition or interruption, define the expected actions for a small set of representative cases before evaluating the result. Pair a case that should continue or reuse evidence with one that must rerun a check, pause, or ask for a decision. For validation reuse, vary a relevant input. For proportional checking, contrast a wording edit with a routing or authority change. For recovery, contrast an evidence-backed in-scope correction with a new approval requirement or an exhausted retry limit.

Use available, authorized task evidence or safe fixtures. Assess redundant checks, unnecessary approvals, continued progress, and preserved correctness and permission boundaries. Fewer tool calls or prompts alone do not establish improvement. Keep instruction walkthroughs distinct from observed execution, and report when live behavior has not been exercised. Do not introduce mandatory per-task metrics, new infrastructure, optional remote processing, or simulated approvals to perform this evaluation.

## Check Repository Independence

Evaluate the complete affected requirement family and its instruction paths under the [repository independence contract](repository-independence.md), not just the edited files. Trace representative tasks using the destination’s supported client entrypoints, local documentation, and declared peers, without the maintainer’s global instructions or domfiles installation. Also trace the maintainer’s environment so local rules and personal defaults do not create contradictory outcomes.

Check ordinary tasks outside skills as well as relevant skill routes, including shortcuts and wording-only work. Vary personal defaults where they could change an accepted artifact requirement. When an optional peer supplies assistance, verify that required local behavior remains reachable under its supported availability and fallback cases. Installed guidance alone is not proof of invocation or precedence.

For each selected requirement, confirm its local owner, how each affected route reaches it, and whether its authority is sufficient for the expected result. Confirm that equivalent copies retain the complete canonical wording and that deliberate local exceptions remain distinct. Check that a moved rule was not left only in a maintainer-specific global layer, an optional peer default, or a source-only baseline reference.

For new repository documentation, verify that only necessary surfaces were created and that client bridges reach the same canonical instructions without adding independent policy. Report any unavailable instruction path or unsettled output requirement. Distinguish source-based traces from observed client behavior, and do not turn this check into an unrelated repository audit or require live client experiments for every documentation change.
