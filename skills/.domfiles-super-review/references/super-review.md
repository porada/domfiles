# Super Review

## Resolve the Invocation

Resolve the task scope, selected stages, review target, and runtime overrides from the invocation and conversation. By default, run peer reviews of the changes followed by a cold review of the committed target.

Apply explicit selections directly. Do not require a particular phrase or fixed option syntax to select stages or change settings. Ask only when materially different interpretations remain.

When the user invokes `super-review` directly, first show them the complete commit messages for the resolved review target. Quote existing messages verbatim and label drafts for uncommitted changes as proposed. Then start the selected stages. The global **Reviews** rule governs the subsequent assessments and findings reports.

Use the global **Review workflow requests** policy for direct invocation. When another workflow invokes this skill, retain its established authority. Preserve read-only and no-commit limits rather than silently dropping uncommitted work from the target. If those limits prevent a selected stage’s prerequisite, report the incompatibility and continue only independent covered work.

Load `subagent-review-flow` for shared coordination. Use `sensible-commit-flow` with `dom-sensible-commit-flow` for commit operations.

## Resolve Review Units

Divide the resolved review target into one unit per repository and target checkout. Each unit owns its execution binding and selected stages. Comparison-only sources remain evidence rather than becoming review units unless explicitly selected as targets.

Retain the calling workflow’s repository roles, authorization, cross-repository dependencies, validation, and permitted deferrals. Per-unit clearance does not replace checks spanning repositories.

### Choose Review Groups

Within each unit, choose review groups separately for peer reviews and cold review. A group is the target assigned to the selected peer reviewers or one cold reviewer. Default to one group per selected stage, and split substantial work when separable areas can be reviewed concurrently with full coverage and less elapsed time. Honor explicit user selections. Use the shared review process’s independence and scheduling criteria.

Peer review groups may cover separate areas of the changes. Each cold review group contains one or more whole commits. Several peer review groups may feed one cold review, or one peer review group may feed several cold reviews. Commits can receive separate cold reviews or one combined cold review. Commit boundaries alone do not establish independence. Keep changes together when separate assessments would miss their combined behavior.

Before dispatch, account for every task-owned change in the unit’s review target, including uncommitted work, and its integration boundaries across the groups. Assign shared contracts and interactions to groups with enough context to assess them. Retain overlapping evidence where necessary. If a boundary cannot be reviewed adequately within separate groups, combine the affected work. When splitting groups, preserve each selected peer reviewer profile’s coverage of the full target and cold review coverage of the complete committed target. Do not change commit structure merely to match review groups.

Keep group scopes, dependencies, reviewer roles, clearance, and each cold review group’s committed target and cycle accounting in the shared review record. Map each cold review group to the peer review groups and validation it depends on. A ready group may advance while independent groups or units remain in an earlier stage. Wait for unfinished work that supplies a prerequisite or affects its reviewed result or integration boundaries.

## Run Peer Reviews

Start the selected peer reviewers for each group independently against that group’s target and the unit’s common starting baseline. Run each group’s reviewers concurrently by default, subject to host capacity. Include relevant integration requirements and comparison evidence through the shared assignment contract. Each selected reviewer assesses the complete group. Do not divide that coverage between the profiles.

Use the shared review process to collect their reports, resolve disputes, apply authorized corrections, and obtain the required clearance.

Before starting or continuing a cold review, complete the peer reviews and validation its group depends on. Each selected peer reviewer role must clear its complete assigned target. Apply [Settle Reviewer Sessions](review-execution.md#settle-reviewer-sessions) to those peer reviewers. A limited review or partial clearance does not satisfy this prerequisite. Independent groups’ outstanding reports or session settlement do not delay this transition. If the invocation omits peer reviews, complete the required validation before cold review.

Later follow-ups may resume peer reviewers through the shared continuity process. Ordinary cold review corrections do not automatically reopen peer reviews.

## Run Cold Review

Read and apply the installed `cold-review` entrypoint for each selected cold review group. Continue in the invoking thread with the established review record, target groups, dependencies, settings, and authority. Schedule independent groups through the shared review process.

## Respond to Changes

Distinguish corrections within the agreed task from changes to its target, intended behavior, or covered work.

On an authorized scope change, identify affected units and groups, including cross-repository dependencies. Start the selected stages for newly added groups. For existing groups, follow up with their latest valid reviewers for the applicable stages. For peer reviews, refresh clearance for each selected reviewer role in the affected groups. Do not create reviewers for a stage the invocation omitted.

If the peer review groups change after their reviews start, retain standing findings and required follow-ups with the affected work. Reuse clearance only for coverage whose premises still hold, and obtain the remaining assessments through the shared continuity process without resetting recovery limits.

Once cold review has started, complete any affected peer reviews and validation before resuming the affected cold review groups.

Reassess every group whose reviewed result or integration boundaries changed, even without scope expansion. Unrelated concurrent changes do not automatically become part of a review unit.

## Complete the Run

Complete the run only when every group in every participating unit has its selected stage’s required clearance, no repeat is pending, and no accepted work or required verification remains, including checks spanning groups and repositories. For each unit that includes cold review, verify that every task-owned change in its review target has been committed and that the cleared cold review groups cover those commits and their integration boundaries. Account explicitly for any deferrals the calling workflow permits.

Apply [Settle Reviewer Sessions](review-execution.md#settle-reviewer-sessions) to every reviewer and capability probe session created for the run, including peer reviewers, cold reviewers, their replacements, and any repeated cold reviews. Distinguish review clearance from any remaining settlement limitation.

Report the resulting commits, permitted deferrals, and material remaining limitations under the governing reporting policy. Do not claim clearance for deferred scope.

## Settle the Coordinating Thread

Apply these rules when `super-review` completes cold review or a calling workflow expressly routes here after cold review. Settle the coordinating thread only at the outermost task’s final handoff, when no outstanding matters remain assigned to it. Review clearance alone doesn’t establish that condition. Outstanding matters include required work, verification, user questions, approvals, required human actions, promised follow-ups, and reported optional opportunities. Return from a routed review to its caller for any remaining work. Keep the thread open on explicit request or while an item remains outstanding, and identify any remaining work at handoff. Use the completion action permitted by global **Thread cleanup**.

An item stops preventing settlement only when completed, explicitly dismissed by the user within applicable requirements, or transferred to a verified follow-up destination with established continuation ownership and user visibility. Deferring required work or an optional opportunity in the same thread doesn’t resolve it for settlement. Don’t infer an item’s disposition from an agent’s proposal or user silence. Disclosed limitations don’t prevent settlement unless they require a decision or further action to complete the agreed endpoint.
