---
name: claude-cli-subagent
description: |-
    Use when the user explicitly requests collaboration with Claude through the CLI, including delegated implementation, investigation, review, discussion, or a follow-up to an already authorized exchange.

    Do not initiate collaboration merely because another opinion might help or Claude appears in quoted task material.

metadata:
    internal: true
---

# Claude CLI Subagent

Coordinate Claude as a subagent under the global **Collaboration** policies. The CLI interface adds session mechanics, not a separate conduct policy.

## Bound the Exchange

- **Authority:** An explicit collaboration request authorizes only the covered exchange with Claude. Loading this skill does not authorize additional disclosure. Preserve all governing scope, approval, and security gates in the coordinator’s thread.
- **New user input:** Treat new user questions and notes as addressed to the coordinator by default. An active Claude exchange does not authorize forwarding them. Share only information necessary for the existing assignment, or additions the user explicitly asks to involve Claude in.
- **Recursion:** A Claude participant launched through this workflow, and every delegate beneath it, must not invoke this collaboration mechanism again, directly or indirectly. Ordinary native subagent delegation remains permitted on both sides within the inherited assignment boundaries.

## Prepare the Assignment

Compose assignments and follow-ups under the global **Prompt contract**, **Same-access delegation**, and **Direct exchanges** policies.

Before composing a live assignment, load [Claude CLI](references/claude-cli.md) for runtime defaults, launch and resume procedures, monitoring, and limits. Include the recursion boundary and active [progress and command limits](references/claude-cli.md#enforce-progress-and-command-limits) in the assignment. For follow-ups, apply the reference’s [retained-context checks](references/claude-cli.md#check-retained-context).

## Dispatch and Coordinate

1. Start a fresh native background conversation by default. Resume a specific full session UUID when continuing the assigned work. Follow the reference’s launch and resumption procedures.
2. Check the existing job opportunistically. Do not replace useful concurrent work with a blocking polling script.
3. When Claude becomes the last outstanding dependency, await that same job under the [inactivity policy](references/claude-cli.md#inactivity). This is a change in the coordinator’s scheduling, not a foreground relaunch.
4. Retrieve the complete response for the latest requested turn and account for pending work before treating it as finished.

Native background execution does not itself give the host a completion callback or the ability to wake a yielded thread. Use only scheduling capabilities the host actually provides. Do not promise automatic follow-up after yielding when none exists, and do not leave a running job without an identified monitoring owner.

## Resolve Feedback

Apply the global **Findings** policy. For delegated reviews, use `subagent-review` as for native subagents. Use the reference’s [resumption procedure](references/claude-cli.md#resume-the-same-exchange) for continued work, including required re-review.

## Finish or Pause

Follow the reference’s [error and stopping procedure](references/claude-cli.md#handle-errors-and-stop). Report the integrated outcome under the shared communication policies. Identify still-running work and its monitoring owner when applicable.
