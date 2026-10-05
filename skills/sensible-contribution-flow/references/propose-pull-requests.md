# Propose Pull Requests

## Resolve Execution Authority

Once the repository, upstream target, and intended contribution scope are established, compare the proposed preparation effects with existing authorization. Surface known uncovered approvals without waiting for the complete PR draft, design review, provisional commit breakdown, or validation tool investigation unless that evidence is needed to settle the scope or effects being approved. Preserve the [dependency and execution boundaries](execution-boundaries.md).

When applicable governing instructions expressly delegate a continuing preparation mode, use their bounded checkpoint. Name only the effects that authority permits, including any implementation, validation, independent review, later validated corrections, local commits, provisional revisions, or unpublished-history synchronization it covers. Make its revision boundaries and lifetime explicit. Unless the user requests a narrower deliverable, do not substitute an edit-only proposal for that checkpoint. A narrower approval already given remains limited to its recorded effects.

Without that governing delegation, request only implementation authority not already supplied by the user rather than offering a new broad continuing grant. Let the resolved commit workflow establish execution authorization for commits and history updates, including any applicable one-off request.

When approval is needed, briefly reiterate the verified target, intended behavior, exclusions, proposed actions, and handoff, then explicitly ask to execute the uncovered effects. Wait for that response and retain its record before those effects. Approval of a draft or plan alone never substitutes for execution authorization. Proceed through already covered work without requiring another recap or approval. For later discoveries, ask only about the new decision or uncovered effect.

## Prepare Proposals

Identify the contribution’s central claim and the most direct appropriate validation. A regression test should distinguish the defect from the intended behavior, while a documentation correction may require checking the description against current behavior. Use the repository’s checks and the resolved commit workflow’s validation checkpoints to establish that claim rather than creating a parallel validation procedure.

Before implementation, inspect the repository’s validation commands, tool versions, and acquisition path. Establish whether validation uses available tooling, may acquire already prescribed dependencies or tools, or requires a new dependency choice. A `PATH` check alone does not settle package runner availability or download needs. Do not download tools merely to investigate availability. Resolve any newly uncovered approval before its dependent work, without reopening covered effects.

Ask the resolved commit workflow for a read-only, prospective breakdown based on the intended scope. Supply the [contribution’s packaging constraints](prepare-pull-requests.md#plan-and-implement-commits), then retain only a provisional plan until the actual diff exists.

Unless the user waives the early draft, prepare a title and body through [post content preparation](prepare-post-content.md). Use the draft to expose the intended outcome and scope, not to claim implementation or testing has already happened. Keep the wording provisional while discussing the solution.

When presenting that initial draft, add a brief source note outside the title and body. Identify the repository template followed by its repository-relative path, and cite any existing PR used as a structural model, distinguishing examples from repository requirements. If no template was found, say so and identify the fallback used. If template availability could not be verified, report that limitation instead of claiming absence or compliance. Attribute only sources actually inspected.

Unless explicitly waived, run a quick adversarial design review using the [review method](prepare-pull-requests.md#review-contributions) before implementation. Challenge the problem’s current existence, the proposed solution, the best simpler alternative, and upstream fit. Present the draft, planned validation, provisional commit breakdown, and review conclusions as a concise update. Resolve material decisions with the user, without another approval ceremony when the existing authority covers the proposal.
