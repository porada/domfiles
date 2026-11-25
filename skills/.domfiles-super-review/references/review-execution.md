# Review Execution

Use T3 Code top-level threads by default. Select another interface only when the default cannot meet host or workflow requirements, or the user explicitly selects an alternative. Before using that alternative, satisfy [Verify an Equivalent Interface](#verify-an-equivalent-interface). An invocation-specific interface selection remains binding.

Keep reviewer assessments read-only. The coordinator owns corrections and commit operations. Reviewers may delegate bounded read-only work under the shared review process, within the established authorization and host limits. Follow the applicable interface branch below before dependent review work.

## Bind T3 Reviewers

Resolve each unit’s T3 environment before looking up or registering its project. For a checkout on another machine, use the host’s linked-environment interface and establish access there. SSH access alone does not establish T3 availability. Preserve the selected remote checkout rather than substituting a local clone or registering its path on the local machine.

Verify that the resolved environment supports the selected [reviewer profiles](reviewer-defaults.md#profiles), target project and checkout binding, and required creation, continuation, observation, and cleanup operations before dependent work.

Within that environment, use T3 Code top-level threads in the project corresponding to the unit’s repository, bound to its target checkout. Resolve the project from its registered workspace and repository identity rather than inheriting the initiating thread’s project or using the skill’s source location. When no matching project exists, register the existing repository folder there automatically without creating a repository or worktree.

Retain the environment, project, and checkout binding with the unit’s review record. Select that environment and project explicitly when creating each reviewer, including replacements and repeated cold reviews, and use that environment for subsequent thread operations. For an existing worktree, use the corresponding project’s host-supported worktree binding rather than its root checkout. A directory instruction in the review prompt does not establish the thread’s environment, project, or checkout.

Follow the host’s top-level-thread lifecycle, distinguishing it from delegate-task requirements. If this route cannot supply a required capability, return to [Review Execution](#review-execution) or report the specific limitation. Do not change the requested provider or settings.

## Verify an Equivalent Interface

An alternative must preserve the selected profiles, environment and exact checkout, independent initial assessment and prompt isolation, receiving instructions and access boundaries, retrievable review evidence, follow-up continuity, and permitted cleanup. The coordinator and user must be able to inspect reviewer status, reports, and session references, revisit retained evidence, and identify each role by its prescribed title. A matching model or a thread creation capability alone doesn’t establish equivalence. Use the shared reviewer-continuity process for interfaces that require fresh sessions, preserving independent initial reports before reconciliation of inherited findings. This route cannot substitute a delegate for a task that specifically tests top-level session behavior.

Verify those properties before dependent review work using documented host capabilities and applicable retained execution evidence. If a material property remains uncertain, use a minimal bounded capability probe only within established execution and disclosure authority. State the property, expected evidence, effects, and stopping condition before the probe. Keep its task neutral and preserve the selected provider, model, settings, environment, and checkout. Exercise only the uncertain lifecycle operations and apply [Settle Reviewer Sessions](#settle-reviewer-sessions) to every probe session. A probe does not count as review clearance. If authority or evidence is insufficient, retain the limitation instead of dispatching the review.

Record the chosen interface, supporting evidence, limitations, and applicable authority in the existing review record. Reuse that verification only while its relevant capabilities and execution conditions remain applicable. Interface selection doesn’t authorize another processing service, recipient, checkout, or disclosure. It also doesn’t bypass denied access or reset recovery limits. Follow host requirements when selecting and operating the alternative, including its project binding, continuation, observation, and cleanup mechanisms. Apply the remaining review stages to that interface without weakening their assessment, evidence, or clearance requirements.

## Settle Reviewer Sessions

Settle each reviewer and capability probe through the selected interface’s lifecycle when the session is no longer needed, applying global **Thread cleanup** to top-level threads. Verify completion whether automatic or through a permitted action. A verified transient cleanup failure may remain pending while subsequent review work proceeds only when the session’s assigned work is complete, its required evidence is retained and retrievable, and host state establishes that no live operation can interfere with that work. For reviewers, that includes every required assessment, follow-up, and clearance. Uncertain execution state or unfinished review obligations still pause the dependent transition.

Keep each pending cleanup action and its session reference in the existing review record. The coordinator owns bounded retries under host and governing retry limits, without starting a replacement reviewer merely to obtain cleanup. Resolve pending cleanup before final handoff when possible. If recovery is exhausted or the permitted action remains unavailable, report the affected sessions and remaining limitation without claiming settlement. Preserve conversation history, reopenability, and the distinction between completion and archival.
