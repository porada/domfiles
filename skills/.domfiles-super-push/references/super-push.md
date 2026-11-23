# Super Push

## Identify the Outgoing Commits

Start with the invoking agent’s task, including its uncommitted work. Resolve one repository and destination branch from the request and the branch’s push configuration. If the task spans repositories, establish which one the user intends to publish. Preserve review dependencies across repositories.

Verify that the push reaches one authorized endpoint. Use that endpoint for destination checks, including when the remote has a different fetch URL. If the destination is ambiguous or the remote would push to several endpoints, resolve the intended one before publication. The command advances an existing branch by fast-forward. If the branch is absent, continue independent review work and ask before creating it.

By default, select the last commit needed to deliver the invoking agent’s task. If the user selects all unpushed commits, select the current local branch tip even when it includes later work from other tasks. The outgoing range includes every commit the selected tip would add to the destination.

Check publication readiness for every outgoing commit, including the selected tip. In T3 Code, use relevant task and reviewer threads to identify the work, prior clearance, and any hold on publication. Reuse clearance that still covers the commits and their interactions under **Validation evidence**. An open or settled thread alone establishes neither readiness nor a hold. Include commits lacking applicable clearance in this review, even when they originated outside T3.

Respect explicit exclusions, outstanding findings, and work another agent is actively revising. Resolve findings within scope. When a correction needs coordination with another task’s owner, use an already authorized channel or bring the decision to the user. If an outgoing commit must stay local, continue independent review work and report what still prevents the push.

## Review and Prepare the Series

Read and apply `super-review`’s entrypoint from the same installed skills root. Pass it the resolved target and the authority from **Review workflow requests**. Use its default review stages unless the user selects otherwise. An explicit read-only or local-only limit applies throughout.

Run `prepush` from its installed entrypoint when preparing commits for final review, including a final-only review. If publication is in scope and the user omits final review, run `prepush` before the selected review starts. Inspect the diff and message of every outgoing commit, including commits from other tasks, along with the invoking task’s eligible pending changes. Reuse applicable assessments under **Validation evidence** and reassess any grouping or message affected by later corrections.

Pass the proposal waiver and local rewrite authority from **Push preparation grant** to `prepush`. Limit proposed changes to the invoking task’s eligible unpublished commits and pending changes. Keep affected commits as they are wherever a rewrite would require replaying another task’s commits or setting aside unrelated changes without authority.

Hold the push while any outgoing commit’s subject starts with an autosquash marker (`amend!`, `fixup!`, or `squash!` followed by a space), even if that commit has review clearance. Fold it into its verified target when the rewrite is authorized. If the verified target is already published and an ordinary follow-up commit preserves the intended correction, record it that way when authorized. For each remaining marker, report the commit, its intended target or the ambiguity, and the decision or coordination needed to resolve it.

Review the resulting task commits. For every other commit in the selected outgoing range, obtain any stage coverage still needed for that commit or its interactions. Reassess any coverage affected by a revised series.

## Push the Reviewed Tip

After the selected review clears, identify the exact reviewed tip containing the task’s intended changes. A review limited to uncommitted changes ends at the local result. If newer work descends from the reviewed tip, push only through that tip. If the local branch has lost the reviewed tip, establish which commits still belong to the task and refresh affected clearance.

Refresh the destination branch at the verified push endpoint. If it already contains the reviewed tip, report that outcome. For a fast-forward, recompute the outgoing range from the refreshed destination. Return newly exposed commits to the readiness, `prepush`, and review steps above. Verify that the exact outgoing range is free of autosquash markers and that current or still-applicable prior clearance covers every commit and its interactions. If the destination has disappeared or diverged, retain the local result.

Show the destination and the complete outgoing commit messages before pushing, reusing any messages already shown for those commits. Under **Git publication**, push the reviewed object ID to the explicit branch ref. Require the destination to still point to the commit used for the verified fast-forward. If it moves, repeat the destination and coverage checks. Limit publication to that branch even when Git is configured to push other refs, tags, or submodules. Verify the destination contains the reviewed tip before reporting publication. Apply `super-review`’s coordinating thread settlement rules at this workflow’s final handoff.
