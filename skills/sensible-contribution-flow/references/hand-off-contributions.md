# Hand Off Contributions

Apply [Configure Git Remotes](execution-boundaries.md#configure-git-remotes) to local setup and updates. Local configuration does not supply publication authority.

## Complete Deliverables

If a required human-only checkpoint remains, report the prepared result and exact user action, including the marker’s location when recorded in a file. Do this even when no commit was made. Implementation, commits, and agent review don’t complete a human checkpoint. Distinguish preparation ready for publication approval from authorization to publish.

For a new submission, provide the [finalized title and body](prepare-post-content.md#finalize-content), intended repository, submission surface, and changes being submitted. For revisions, identify the existing pull request and summarize the resulting changes, supplying replacement title or body text only when changed within scope. Include material evidence, review, and validation limitations. For a pull request, include its source repository and branch, upstream target branch, draft or ready status, and any planned fork. Resolve these values from verified evidence rather than guessing destinations. Deliver this complete package before publication, not in response to the user reporting a push.

Complete the [recorded local endpoint](execution-boundaries.md#preserve-authorization) rather than treating an intermediate progress report as final readiness. Continue with covered preparation and corrections rather than making the user ask to resume. For initial pull request preparation, finish at the [selected publication handoff](#preserve-remote-authorization). For a working-tree-only request, hand back the scoped changes and actual review and validation status without requiring commit preparation or publication.

## Preserve Remote Authorization

Preparation, local commit permission, and local history update permission alone don’t authorize remote mutations. For initial pull request publication, follow the [user-run publication handoff](#hand-off-user-run-publication) when the user explicitly selects that endpoint. Otherwise, follow [Confirm Pull Request Publication](#confirm-pull-request-publication).

Other submissions and revisions require a direct, scoped user command covering their destination and effects. Without it, keep publication and submission user-run, handing back the prepared result rather than proposing or requesting those effects. For needed Git publication, include the [user-run publication handoff](#hand-off-user-run-publication).

Permission to fork alone authorizes neither pushing a branch nor opening a pull request, and permission to push alone doesn’t authorize pull request creation. Preserve narrower user limits and separately required approvals without asking again for effects whose applicable approval requirements are satisfied.

Immediately before Git publication or handing back its command, have the resolved commit workflow recheck the final publication destination and the destination branch’s current remote head, independently of the upstream base. If the remote head changed, inspect and reconcile the new work within current authorization, then repeat affected validation and review. For published history replacement, use its required [final destination check](update-commit-history.md#hand-back-published-updates) rather than adding a duplicate check.

Use the resolved remote-change workflow when available, or a documented available interface without installing a replacement. Published history replacement requires the commit workflow’s verified destination, explicit expected-head lease, and preservation of remote work. Credential and key operations require explicit authorization under [Secrets and Authentication](../SKILL.md#secrets-and-authentication), independently of publication authority. Report the actual prepared or submitted result and material limitations without claiming acceptance, then stop. A subsequent request may begin another scoped round, not ongoing monitoring.

## Resolve Publication Destination

Use this bounded, read-only procedure after initial setup access and fetch, before substantial implementation, then again at final readiness. The early phase surfaces required access and naming decisions. The final phase revalidates the selected destination against the actual branch and intended publication operation, reusing early evidence only while its premises remain applicable and refreshing changed or uncertain evidence. Neither phase replaces the [final publication recheck](#preserve-remote-authorization). Assessment-only and working-tree-only deliverables don’t require this procedure. It authorizes neither hosted fork creation nor pushes.

1. Preserve an established publication destination and any existing pull request’s source repository and branch. Otherwise consider the verified upstream repository as the candidate. Write access determines where the contribution branch can be published, not whether a pull request is required.
2. Use the resolved evidence workflow to verify write access for the intended operation on the contribution branch from repository permission metadata and applicable branch restrictions. Don’t infer access from ownership, fork status, or remote names, and never probe by pushing. Unavailable or inconclusive evidence is an evidence gap, not confirmed lack of access. Follow the evidence workflow’s retrieval boundaries rather than treating a lookup failure as a reason to fork.
3. If the candidate is writable, use it. When confirmed lack of access leaves a new pull request needing a fork, reuse a suitable existing fork under the authenticated user’s personal profile, regardless of its name, after verifying its upstream relationship and required branch access. Don’t silently replace an existing pull request or its source branch.
4. If a new fork is necessary, plan it under that personal profile and check availability of the upstream repository’s name. If an unrelated repository occupies that name, check `<upstream-owner>-<repo-name>`, where `<upstream-owner>` is the verified upstream repository’s owner. If an unrelated repository also occupies the fallback name, ask for a naming decision. Don’t delete, rename, or repurpose existing repositories to make room. Retain the available planned destination for the publication package, without creating it before approval.
5. Retain the decisive evidence, verified or planned destination, and any exact remaining access, naming, or capability requirement in the existing task context. Ask only for decisions the evidence and current authority can’t settle, not confirmation of a resolved choice.

During the early phase, surface those requirements before substantial implementation, pause only dependent work, and return to the calling preparation or revision phase. Continue independent authorized work without implying that deliverables are finalized or publication is ready.

At final readiness, return to [Complete Deliverables](#complete-deliverables) with the verified or planned destination. If required evidence or capabilities remain unavailable, report the prepared local result and exact remaining requirement without claiming publication readiness.

## Hand Off User-Run Publication

If the selected destination requires a new fork, ask the user to create the planned fork and supply its URL. Verify the fork’s upstream relationship and required branch access under [Resolve Publication Destination](#resolve-publication-destination) before preparing a publication command. This handoff doesn’t authorize agent-run hosted fork creation.

Once the destination exists, complete necessary authorized local setup under [Configure Git Remotes](execution-boundaries.md#configure-git-remotes). Apply the [final publication recheck](#preserve-remote-authorization), then provide the exact verified user-run command with the [completed deliverables](#complete-deliverables), rather than requesting agent publication authority. Don’t present a push command targeting a planned but uncreated repository as ready to run.

## Confirm Pull Request Publication

For agent-run publication of a new pull request, including a draft, obtain explicit publication approval after implementation, validation, review, local commits, and final pull request copy are ready under the preparation workflow’s applicable requirements and authorized procedural waivers. Complete other required human actions before this checkpoint. An initial request to prepare or open a pull request does not replace this final approval. Skip the checkpoint only when the user explicitly waives it and supplies scoped publication authority.

Present the [completed deliverables](#complete-deliverables), then ask whether to publish the prepared contribution and open the pull request, including any necessary personal-profile fork creation and local remote setup. Wait before creating the hosted fork, pushing, or opening the pull request. A clear affirmative answer to this publication question supplies approval. Agreement with an implementation plan, draft wording, local commits, or agent review does not.

This approval covers that contribution’s necessary personal-profile fork, local remote setup, branch publication, and pull request creation as one bounded operation. It does not cover merging, organization-owned fork creation, unrelated remote changes, or ongoing publication of later revisions. Carry the actual user approval, or the explicit waiver and scoped command, to the resolved execution workflows. Return for approval only if the prepared contribution or publication effects materially exceed that authority, not merely because execution advances to another covered operation.

## Publish Approved Pull Requests

After publication is authorized, complete these steps in order:

1. Create the planned fork if necessary through the resolved remote-change workflow or a documented available interface. Avoid implicit cloning or other effects outside the approved operation. If no suitable interface is available, report the limitation and ask the user to create the planned fork. Verify any created or user-supplied fork’s upstream relationship and required branch access before continuing.
2. Complete necessary local remote setup in the supplied checkout under [Configure Git Remotes](execution-boundaries.md#configure-git-remotes).
3. Apply the [final publication recheck](#preserve-remote-authorization), then publish only the approved branch and open the pull request with the approved target, title, body, and draft or ready status. Inspect existing remote state when needed to avoid duplicating completed effects after an interruption. Preserve completed work and resolve any failure within the existing authority and execution boundaries rather than restarting the whole operation.
4. Report the pull request URL and actual publication result, including any remaining limitation. End the bounded publication operation without merging or starting ongoing monitoring.
