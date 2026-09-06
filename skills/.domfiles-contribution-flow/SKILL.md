---
name: contribution-flow
description: |-
    Use when evaluating, preparing, or reviewing a contribution to a GitHub repository the user does not own, including a discussion, issue, private vulnerability report, or pull request. Also use when incorporating approved findings before submission or preparing the next pull request in a contribution series.

    Do not use for routine work in the user’s own repositories or for post-submission maintenance alone.

metadata:
    internal: true
---

# Contribution Flow

Prepare an upstream contribution for the user’s manual submission. A useful outcome may be a focused change, a report, a conversation, or a decision not to proceed.

The workflow ends at submission readiness. External or adversarial review is optional and user-directed, not a prerequisite for that state. Post-submission monitoring and maintainer follow-up are outside this workflow.

## Compose With Peers

Load each peer when its responsibility enters scope. Keep its policy under that owner rather than reproducing it here.

| Peer | Responsibility |
| --- | --- |
| `commit` | Prospective commit planning, concrete proposals, authorized execution, unpushed-history updates, and verification. |
| `git-worktrees` | Entry into the supplied checkout, isolation, and worktree lifecycle. |
| `human-facing-writing` | General editorial guidance, including technical accuracy, templates, and typography. |
| `simple-github-cli` | Remote GitHub context and separately authorized remote operations through the appropriate interface. |

This workflow creates no exception to the global **Commit gate**, **Index preservation**, or **Git publication** policies. If a peer cannot support a required operation under those policies, stop that operation and report the limitation rather than inventing another execution path.

For a standalone review of a prepared contribution, keep the task read-only, assess it against the applicable steps below, and report only findings and material evidence limitations. Do not treat a review request as authorization to apply fixes or change Git state.

## Assess the Contribution

Identify the target repository, intended outcome, and available evidence. Use the available contribution guidance, security policy, and templates to establish repository expectations.

Consider confidentiality before public searches or choosing a submission surface. An undisclosed vulnerability belongs in the repository’s designated private reporting channel, not a public issue or pull request. Reporting a vulnerability and requesting a CVE identifier are distinct actions. Do not assume CVE eligibility or an assigned identifier. If no suitable private channel is available, or the required channel conflicts with manual browser submission, stop for the user’s decision rather than exposing the report publicly.

Through `simple-github-cli`, make bounded searches for related issues and pull requests and inspect relevant current upstream evidence. Establish what remains unresolved, distinguishing complete upstream fixes, existing reports, partial solutions, and proposed fixes. Read decisive comments to understand the disposition of earlier work rather than treating closure as rejection or approval as integration. Follow the peer’s retrieval-failure handling rather than treating unavailable evidence as an empty search.

Assess upstream fit separately from implementation size. Distinguish concerns about correctness, maintenance cost, and product direction, then address the concern that determines whether the contribution is wanted. A smaller patch does not necessarily answer an objection to the capability itself. Identify the strongest motivation for proceeding and why the best existing alternative falls short.

Choose the outcome from the problem and repository expectations:

- **Discussion:** A question, proposal, or open-ended design conversation needs input before a concrete change is appropriate.
- **Issue:** A problem should be established or reported rather than immediately addressed through a patch.
- **No contribution:** The evidence, expected benefit, or upstream fit does not justify proceeding. Explain the decisive reason and stop without manufacturing a post.
- **Pull request:** A bounded implementation or documentation change is an appropriate contribution. Follow [Prepare Pull Requests](references/prepare-pull-requests.md).
- **Security report:** A potential vulnerability requires private disclosure. Supply the report context to `human-facing-writing` for its security-report guidance.

Push back on weak assumptions and disproportionate scope. Unexpected size is a reason to reassess the premise and look for a narrower contribution, not automatically split the same unwanted change into several submissions. Confirm a material change from the user’s explicitly requested deliverable before proceeding.

For every post-producing outcome, follow [Prepare Post Content](references/prepare-post-content.md) before authoring its body.

## Incorporate Findings Before Submission

When the user supplies findings, `agent-task-relay` owns their validation and applicable fix confirmation through its existing routing. Do not add a separate findings protocol or require another review.

If validated findings undermine the contribution’s premise, return to [Assess the Contribution](#assess-the-contribution) before continuing implementation. Distinguish changes needed for the current contribution’s correctness or supporting evidence from adjacent cleanup or improvements that could become focused follow-ups. Agreement to defer adjacent work does not settle the current contribution’s correctness or evidence questions.

After fixes are approved, resume the selected preparation path. For pull requests, apply both [upstream-sync checkpoints](references/prepare-pull-requests.md#synchronize-with-upstream) to the fix round. Use [commit packaging](references/prepare-pull-requests.md#plan-and-implement-the-commits) to decide which fixes belong in existing commits and which should remain separate, then repeat the submission-readiness check.

## Hand Back the Contribution

Provide the finished title and body, the intended repository and submission surface, and material evidence or validation limitations. For a pull request, include its head branch, upstream target branch, and the exact user-run Git publication command when publication is needed. Resolve those values from the verified checkout rather than guessing destinations.

The user submits pull requests, issues, discussions, and security reports manually through the browser. Do not submit through `gh`, an API, or browser automation. Report preparation readiness, not publication or acceptance, then stop. A subsequent user request can reopen preparation without making ongoing monitoring part of this workflow.
