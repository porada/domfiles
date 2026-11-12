# Super Review

## Resolve the Invocation

Resolve the task scope, selected stages, review target, and runtime overrides from the invocation and conversation. The default sequence is the initial review stage, commit preparation, and final review stage.

Apply explicit selections directly. Do not require a particular phrase or fixed option syntax to select stages or change settings. Ask only when materially different interpretations remain.

Use the global **Review workflow requests** policy for direct invocation. When another workflow invokes this skill, retain its established authority. Preserve read-only and no-commit limits rather than silently dropping uncommitted work from the target. If those limits prevent a selected stage’s prerequisite, report the incompatibility and continue only independent covered work.

Load `subagent-review-flow` for shared coordination. Use `sensible-commit-flow` with `dom-sensible-commit-flow` for commit operations.

## Resolve Review Units

Divide the resolved review target into one unit per repository and target checkout. Apply the selected stages to each unit with separate reviewer roles, committed targets, clearance, and final-review cycle accounting in the shared review record. Comparison-only sources remain evidence rather than becoming review units unless explicitly selected as targets.

Retain the calling workflow’s repository roles, authorization, cross-repository dependencies, validation, and permitted deferrals. Per-unit clearance does not replace checks spanning repositories. Apply the shared review process’s independence criteria when scheduling work across units. A ready unit may advance to its next selected stage while independent units remain in an earlier stage. Wait for unfinished work elsewhere when it supplies a prerequisite or affects the advancing unit’s reviewed result or integration boundaries.

## Establish Review Execution

Use T3 Code top-level threads by default. Select another interface only when the default cannot meet host or workflow requirements, or the user explicitly selects an alternative. Before using that alternative, satisfy [Verify an Equivalent Interface](#verify-an-equivalent-interface). An invocation-specific interface selection remains binding.

Keep reviewer assessments read-only. The coordinator owns corrections and commit operations. Follow the applicable interface branch below before dependent review work.

### Bind T3 Reviewers

Resolve each unit’s T3 environment before looking up or registering its project. For a checkout on another machine, use the host’s linked-environment interface and establish access there. SSH access alone does not establish T3 availability. Preserve the selected remote checkout rather than substituting a local clone or registering its path on the local machine.

Verify that the resolved environment supports the selected [reviewer profiles](reviewer-defaults.md#profiles), target project and checkout binding, and required creation, continuation, observation, and cleanup operations before dependent work.

Within that environment, use T3 Code top-level threads in the project corresponding to the unit’s repository, bound to its target checkout. Resolve the project from its registered workspace and repository identity rather than inheriting the initiating thread’s project or using the skill’s source location. When no matching project exists, register the existing repository folder there automatically without creating a repository or worktree.

Retain the environment, project, and checkout binding with the unit’s review record. Select that environment and project explicitly when creating each reviewer, including replacements and repeated final reviews, and use that environment for subsequent thread operations. For an existing worktree, use the corresponding project’s host-supported worktree binding rather than its root checkout. A directory instruction in the review prompt does not establish the thread’s environment, project, or checkout.

Follow the host’s top-level-thread lifecycle, distinguishing it from delegate-task requirements. If this route cannot supply a required capability, return to [Establish Review Execution](#establish-review-execution) or report the specific limitation. Do not change the requested provider or settings.

### Verify an Equivalent Interface

An alternative must preserve the selected profiles, environment and exact checkout, independent initial assessment and prompt isolation, receiving instructions and access boundaries, retrievable review evidence, follow-up continuity, and permitted cleanup. The coordinator and user must be able to inspect reviewer status, reports, and session references, revisit retained evidence, and identify each role by its prescribed title. A matching model or a thread creation capability alone doesn’t establish equivalence. Use the shared reviewer-continuity process for interfaces that require fresh sessions, preserving independent initial reports before reconciliation of inherited findings. This route cannot substitute a delegate for a task that specifically tests top-level session behavior.

Verify those properties before dependent review work using documented host capabilities and applicable retained execution evidence. If a material property remains uncertain, use a minimal bounded capability probe only within established execution and disclosure authority. State the property, expected evidence, effects, and stopping condition before the probe. Keep its task neutral and preserve the selected provider, model, settings, environment, and checkout. Exercise only the uncertain lifecycle operations and apply [Settle Reviewer Sessions](#settle-reviewer-sessions) to every probe session. A probe does not count as review clearance. If authority or evidence is insufficient, retain the limitation instead of dispatching the review.

Record the chosen interface, supporting evidence, limitations, and applicable authority in the existing review record. Reuse that verification only while its relevant capabilities and execution conditions remain applicable. Interface selection doesn’t authorize another processing service, recipient, checkout, or disclosure. It also doesn’t bypass denied access or reset recovery limits. Follow host requirements when selecting and operating the alternative, including its project binding, continuation, observation, and cleanup mechanisms. Apply the remaining review stages to that interface without weakening their assessment, evidence, or clearance requirements.

## Run the Initial Stage

Start the selected initial reviewers for each unit independently against the same target and baseline within that unit. Include relevant cross-repository requirements and comparison evidence through the shared assignment contract. Run reviewers in parallel when supported.

Use the shared review process to collect their reports, resolve disputes, apply authorized corrections, and obtain the required clearance.

Before starting or continuing a unit’s final review, complete its required initial review and follow-ups for the whole target, including every selected initial role’s clearance and required validation. Apply [Settle Reviewer Sessions](#settle-reviewer-sessions) to that unit’s initial reviewers. A limited review does not satisfy this prerequisite. Early corrections or clearance of part of the target do not make the whole target ready. Independent units’ outstanding reports or session settlement do not delay this transition.

Later follow-ups may resume initial reviewers through the shared continuity process. Ordinary corrections during the final stage do not automatically reopen the initial stage.

### Settle Reviewer Sessions

Settle each reviewer and capability probe through the selected interface’s lifecycle when the session is no longer needed, applying global **Thread cleanup** to top-level threads. Verify completion whether automatic or through a permitted action. A verified transient cleanup failure may remain pending while subsequent review work proceeds only when the session’s assigned work is complete, its required evidence is retained and retrievable, and host state establishes that no live operation can interfere with that work. For reviewers, that includes every required assessment, follow-up, and clearance. Uncertain execution state or unfinished review obligations still pause the dependent transition.

Keep each pending cleanup action and its session reference in the existing review record. The coordinator owns bounded retries under host and governing retry limits, without starting a replacement reviewer merely to obtain cleanup. Resolve pending cleanup before final handoff when possible. If recovery is exhausted or the permitted action remains unavailable, report the affected sessions and remaining limitation without claiming settlement. Preserve conversation history, reopenability, and the distinction between completion and archival.

## Run the Final Stage

For each unit selecting the final stage, follow [Final Review](final-review.md), beginning with committed-target preparation.

## Respond to Changes

Distinguish corrections within the agreed task from changes to its target, intended behavior, or covered work.

On an authorized scope change, identify affected units, including cross-repository dependencies. Start the selected stages for newly added units. For existing units, follow up with their latest valid reviewers for the applicable stages. Refresh each initial pair when that stage participated. Do not create reviewers for a stage the invocation omitted.

For a final stage that has started, follow [Continue After Changes](final-review.md#continue-after-changes).

Reassess every unit whose reviewed result or integration boundaries changed, even without scope expansion. Unrelated concurrent changes do not automatically become part of a review unit.

## Complete the Run

Complete the run only when every participating unit has its selected stages’ required clearance, no repeat is pending, and no accepted work or required verification remains, including checks spanning repositories. For units selecting the final stage, clearance must cover the current committed target.

Apply [Settle Reviewer Sessions](#settle-reviewer-sessions) to every reviewer and capability probe session created for the run, including initial, replacement, and repeated final-review sessions. Distinguish review clearance from any remaining settlement limitation.

Report the resulting commits, permitted deferrals, and material remaining limitations under the governing reporting policy. Do not claim clearance for deferred scope.

When the completed run included the final stage, settle your own coordinating thread at the coordinating task’s final handoff only when no outstanding matters remain assigned to that thread. These include required work, verification, user questions, approvals, required human actions, promised follow-ups, and reported optional opportunities. Review clearance alone doesn’t establish this condition. Otherwise, leave the thread open and state what remains. Use the completion action permitted by global **Thread cleanup**.

An item stops preventing settlement only when completed, explicitly dismissed by the user within applicable requirements, or transferred to a verified follow-up destination with established continuation ownership and user visibility. Deferring required work or an optional opportunity in the same thread doesn’t resolve it for settlement. Don’t infer an item’s disposition from an agent’s proposal or user silence. Disclosed limitations don’t prevent settlement unless they require a decision or further action to complete the agreed endpoint.
