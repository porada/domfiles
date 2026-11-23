# Super Push

## Identify the Outgoing Commits

Start with the invoking agent’s task, including its uncommitted work. Resolve one repository and destination branch from the request and the branch’s push configuration. If the task spans repositories, establish which one the user intends to publish. Preserve review dependencies across repositories.

Verify that the push reaches one authorized endpoint. Use that endpoint for destination checks, including when the remote has a different fetch URL. If the destination is ambiguous or the remote would push to several endpoints, resolve the intended one before publication. The command advances an existing branch by fast-forward. If the branch is absent, continue independent review work and ask before creating it.

By default, select the last commit needed to deliver the invoking agent’s task. If the user selects all unpushed commits, select the current local branch tip even when it includes later work from other tasks. In either case, account for every commit the selected tip would add. This selection sets the review and publication scope. Commit regrouping stays within the invoking agent’s task.

Decide whether each required ancestor is ready to publish. In T3 Code, use relevant task and reviewer threads to identify the work, prior clearance, and any hold on publication. Reuse clearance that still covers the commits and their interactions under **Validation evidence**. An open or settled thread alone establishes neither readiness nor a hold. Include ancestors lacking applicable clearance in this review, even when they originated outside T3.

Respect explicit exclusions, outstanding findings, and work another agent is actively revising. Resolve findings within scope. When a correction needs coordination with another task’s owner, use an already authorized channel or bring the decision to the user. If a required ancestor must stay local, continue independent review work and report what still prevents the push.

## Review and Prepare the Series

Read and apply `super-review`’s entrypoint from the same installed skills root. Pass it the resolved target and the authority from **Review workflow requests**. Use its default review stages unless the user selects otherwise. An explicit read-only or local-only limit applies throughout.

When the invoking agent’s task spans at least two commits, run `prepush` as part of preparing the target for the final review stage, including a final-only review. Count commits the task already produced or still needs, including review corrections. Commits from other tasks don’t count toward that trigger. If a later correction first brings the task to two commits, run `prepush` before the next final assessment. Load `prepush`’s installed entrypoint and limit its proposal to the task’s eligible unpublished commits and pending changes. Keep independently useful changes in separate commits. Combine fixes with the change they serve when the diffs justify it.

When the user invokes `super-push`, pass the waiver and local rewrite authority from **Review workflow requests** to `prepush`. Use your judgment to apply a feasible grouping within that grant. If regrouping would require replaying another task’s commits or setting aside unrelated pending changes without authorization, keep the affected commits as they are.

Review the resulting task commits. For every other commit in the selected outgoing range, obtain any stage coverage still needed for that commit or its interactions. Reassess any coverage affected by a revised series.

## Push the Reviewed Tip

After the selected review clears, identify the exact reviewed tip containing the task’s intended changes. A review limited to uncommitted changes ends at the local result. If newer work descends from the reviewed tip, push only through that tip. If the local branch has lost the reviewed tip, establish which commits still belong to the task and refresh affected clearance.

Refresh the destination branch at the verified push endpoint. If it already contains the reviewed tip, report that outcome. For a fast-forward, recompute the outgoing range from the refreshed destination. Account for every commit and its interactions through current or still-applicable prior clearance. Return newly exposed ancestors to the readiness and review steps above. If the destination has disappeared or diverged, retain the local result.

Show the destination and the complete outgoing commit messages before pushing, reusing any messages already shown for those commits. Under **Git publication**, push the reviewed object ID to the explicit branch ref. Require the destination to still point to the commit used for the verified fast-forward. If it moves, repeat the destination and coverage checks. Limit publication to that branch even when Git is configured to push other refs, tags, or submodules. Verify the destination contains the reviewed tip before reporting publication. Apply `super-review`’s coordinating thread settlement rules at this workflow’s final handoff.
