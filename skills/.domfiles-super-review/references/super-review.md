# Super Review

## Resolve the Invocation

Resolve the task scope, selected stages, review target, and runtime overrides from the invocation and conversation. The default sequence is the initial review stage, commit preparation, and final review stage.

Apply explicit selections directly. Do not require a particular phrase or fixed option syntax to select stages or change settings. Ask only when materially different interpretations remain.

Use the global **Review workflow requests** policy for direct invocation. When another workflow invokes this skill, retain its established authority. Preserve read-only and no-commit limits rather than silently dropping uncommitted work from the target. If those limits prevent a selected stage’s prerequisite, report the incompatibility and continue only independent covered work.

Load `subagent-review-flow` for shared coordination. Use `sensible-commit-flow` with `dom-sensible-commit-flow` for commit operations.

## Resolve Review Units

Divide the resolved review target into one unit per repository and target checkout. Apply the selected stages to each unit with separate reviewer roles, committed targets, clearance, and final-review cycle accounting in the shared review record. Comparison-only sources remain evidence rather than becoming review units unless explicitly selected as targets.

Retain the calling workflow’s repository roles, authorization, cross-repository dependencies, validation, and permitted deferrals. Per-unit clearance does not replace checks spanning repositories. Apply the shared review process’s independence criteria when scheduling work across units. A ready unit may advance to its next selected stage while independent units remain in an earlier stage. Wait for unfinished work elsewhere when it supplies a prerequisite or affects the advancing unit’s reviewed result or integration boundaries.

## Establish T3 Execution

Resolve each unit’s T3 environment before looking up or registering its project. For a checkout on another machine, use the host’s linked-environment interface and establish access there. SSH access alone does not establish T3 availability. Preserve the selected remote checkout rather than substituting a local clone or registering its path on the local machine.

Verify that the resolved environment supports the selected [reviewer profiles](reviewer-defaults.md#profiles), target project and checkout binding, and required creation, continuation, observation, and cleanup operations before dependent work.

Within that environment, use T3 Code top-level threads in the project corresponding to the unit’s repository, bound to its target checkout. Resolve the project from its registered workspace and repository identity rather than inheriting the initiating thread’s project or using the skill’s source location. When no matching project exists, register the existing repository folder there automatically without creating a repository or worktree.

Retain the environment, project, and checkout binding with the unit’s review record. Select that environment and project explicitly when creating each reviewer, including replacements and repeated final reviews, and use that environment for subsequent thread operations. For an existing worktree, use the corresponding project’s host-supported worktree binding rather than its root checkout. A directory instruction in the review prompt does not establish the thread’s environment, project, or checkout.

Follow the host’s top-level-thread lifecycle, distinguishing it from delegate-task requirements. Do not substitute another execution interface when a required capability is unavailable. Report the specific limitation without changing the requested provider or settings.

Keep reviewer assessments read-only. The coordinator owns corrections and commit operations.

## Run the Initial Stage

Start the selected initial reviewers for each unit independently against the same target and baseline within that unit. Include relevant cross-repository requirements and comparison evidence through the shared assignment contract. Run reviewers in parallel when supported.

Use the shared review process to collect their reports, resolve disputes, apply authorized corrections, and obtain the required clearance.

Before starting or continuing a unit’s final review, complete its required initial review and follow-ups for the whole target, including every selected initial role’s clearance and required validation. Settle that unit’s initial reviewer threads using the completion action permitted by global **Thread cleanup**. A limited review does not satisfy this prerequisite. Early corrections or clearance of part of the target do not make the whole target ready. Independent units’ outstanding reports or thread settlement do not delay this transition.

Later follow-ups may reopen initial reviewer threads through the shared continuity process. Ordinary corrections during the final stage do not automatically reopen the initial stage.

## Run the Final Stage

For each unit selecting the final stage, follow [Final Review](final-review.md), beginning with committed-target preparation.

## Respond to Changes

Distinguish corrections within the agreed task from changes to its target, intended behavior, or covered work.

On an authorized scope change, identify affected units, including cross-repository dependencies. Start the selected stages for newly added units. For existing units, follow up with their latest valid reviewers for the applicable stages. Refresh each initial pair when that stage participated. Do not create reviewers for a stage the invocation omitted.

For a final stage that has started, follow [Continue After Changes](final-review.md#continue-after-changes).

Reassess every unit whose reviewed result or integration boundaries changed, even without scope expansion. Unrelated concurrent changes do not automatically become part of a review unit.

## Complete the Run

Complete the run only when every participating unit has its selected stages’ required clearance, no repeat is pending, and no accepted work or required verification remains, including checks spanning repositories. For units selecting the final stage, clearance must cover the current committed target.

Ensure global **Thread cleanup** has been applied to every reviewer thread created for the run, including initial, replacement, and repeated final-review threads.

Report the resulting commits, permitted deferrals, and material remaining limitations under the governing reporting policy. Do not claim clearance for deferred scope.
