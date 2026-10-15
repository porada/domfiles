---
name: subagent-review
description: |-
    Use when preparing or performing a review delegated to a subagent, or coordinating its findings, authorized fixes, and re-review. Applies to native subagents and Claude or Codex CLI review sessions, including review phases within another task.

    Do not use merely for a main agent’s direct review or self-review, externally supplied reports handled by `agent-task-relay`, or standalone findings verification.

metadata:
    internal: true
---

# Subagent Review

## Bound the Review

The calling workflow determines whether a review is required, what it examines, and which corrections are authorized. This skill adds no review checkpoint. The execution interface owns dispatch, monitoring, and session mechanics.

The coordinator owns the shared review record, findings validation, and integration. Each reviewer evaluates the assigned target independently and owns the standing status of findings in that reviewer’s thread. Apply the steps for the assigned role. A standalone review remains read-only. During authorized implementation, continue through resolution only for corrections covered by the existing task authority.

## Prepare the Review

Keep one conversation-local review record: baseline first, then scope and exclusions, settled decisions, and current finding classifications. For each reviewer, track the session, latest reviewed state, and standing findings. Use a shared starting baseline for a group of reviewers.

Give a first-pass reviewer the minimum neutral context needed to evaluate the target: baseline and scope, intended behavior, applicable instructions, execution boundaries, and material evidence limitations. Do not front-load coordinator conclusions, self-assessment, anticipated findings, implementation defenses, or decision history unless needed to establish a governing requirement or the expressly requested review subject. Keep other decision history with the coordinator rather than forwarding it automatically.

Require complete reporting under the global **Findings** policy, including adjacent opportunities, and an explicit statement of whether any in-scope findings remain standing. Carry that reporting contract into further delegation under **Prompt contract**.

## Consolidate Each Round

Collect every reviewer’s report for the round, then reconcile and validate the combined findings in one coordinator pass before applying a fix batch or starting the next review round. Merge duplicates without losing the originating reviewer threads or distinct issues. Do not run separate validation and fix cycles as individual reports arrive.

## Resolve Disagreements

Validate returned findings against current evidence before adopting them. When conclusions conflict, compare target revisions, governing requirements, settled decisions, and material assumptions. Check whether an established decision still applies and whether a finding supplies materially new evidence. Intent alone does not rebut a finding, and a prior decision does not excuse a genuine defect.

The coordinator weighs evidence, not reviewer counts, and uses the smallest decisive inspection or check within the existing scope and limits. Do not recruit additional reviewers merely to obtain agreement. If evidence remains insufficient, report the verification gap rather than forcing a verdict. Ask the user only for a material choice that the evidence and existing authority cannot settle, or a separately required approval.

Record the decisive reason with each finding’s classification. Prepare relevant decisions, evidence, and rationale for the next follow-up, including evidence-backed objections that require no edits. Return a dispute to its originating reviewer thread rather than closing the finding through the coordinator’s judgment alone.

## Coordinate Fixes and Re-Review

For a review-only task, do not apply fixes. Return evidence-backed disputes to the reviewer threads and report any validated findings that need separately authorized changes as standing. When fixes are authorized, consolidate required corrections from the combined pass into one complete batch, apply the fixes, and run relevant checks before the follow-up round. Ask a pre-fix clarification only when available evidence cannot settle something needed for a safe fix, continuing independent fixes. Leave working tree fixes there until the global **Commit gate** authorizes a commit update.

Resume every original reviewer in the same session. Give each follow-up the complete current edited scope and affected integration boundaries, together with relevant prior findings and dispute evidence. Do not limit a reviewer to that reviewer’s own findings or resulting fixes. Keep the review within its established scope, exclusions, and access boundaries.

A no-standing-findings result applies only to the state that reviewer examined. Any subsequent edit within the reviewed scope requires another look at the complete edited scope from every original reviewer, including threads that previously reported no standing findings. Collect the new reports before the next combined validation and fix pass.

If an original reviewer session cannot continue, report the affected thread and review limitation, and ask how to proceed. Do not substitute local verification or a replacement reviewer for that thread’s completion without user direction.

Prefer convergence over a fixed total round cap. Keep optional hygiene changes out of the required fix batch. If a second fix round finds another issue in the same construct, stop extending it. Narrow, replace, or remove it within the existing authority, or ask the user to choose among materially different options.

## Complete the Review

Resolution is complete only when every original reviewer thread reports no standing findings for the same final edited state and no required verification remains. Each thread must reach that status through verified corrections or disputes the reviewer accepts as resolving its findings. The coordinator’s disposition or another reviewer’s clean report cannot substitute for it.

If every initial reviewer reports no standing findings and no edits follow, no follow-up is needed. Once every thread has reached that status for the final state, no further reassurance or acknowledgment is required.

Before concluding, deliver adjacent opportunities under the global **Findings** policy. They are not standing findings and require neither implementation, another review round, nor user acknowledgment or explicit deferral for the review to finish.

If a stopping boundary or insufficient evidence prevents resolution, report the remaining findings and limitation without claiming completion.
