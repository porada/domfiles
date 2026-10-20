---
name: cli-subagent
description: |-
    Use when the user explicitly requests collaboration with Claude or Codex through the CLI, including delegated implementation, investigation, review, discussion, or a follow-up to an already authorized exchange.

    Do not initiate collaboration merely because another opinion might help or Claude or Codex appears in quoted task material.

metadata:
    internal: true
---

# CLI Subagent

Coordinate Claude or Codex as a subagent under the global **Collaboration** policies. The CLI interface adds session mechanics, not a separate conduct policy.

## Bound the Exchange

- **Authority:** An explicit collaboration request authorizes only the covered exchange with the requested participant. Loading this skill doesn’t authorize additional disclosure. Preserve all governing scope, approval, and security gates in the coordinator’s thread.
- **New user input:** Treat new user questions and notes as addressed to the coordinator by default. An active CLI exchange doesn’t authorize forwarding them. Share only information needed for the existing assignment, or additions the user explicitly asks to involve the participant in.
- **Recursion:** A participant launched through this workflow, and every delegate beneath it, must not start or resume another CLI subagent through `claude`, `codex`, or any other agent CLI, directly or indirectly. Ordinary native subagent delegation remains permitted on both sides within the inherited assignment boundaries.
- **One unit per session:** Assign work to a CLI session as you would to a native subagent, under the same inherited boundaries. The participant owns its native subagents and background commands, so during normal operation the coordinator neither tracks nor cancels them individually.

## Establish Readiness

Before composing a live assignment, load [Session Coordination](references/session-coordination.md) for readiness, resumption, monitoring, and limits, together with the participant’s reference for runtime defaults and its launch, resume, and stop procedures: [Claude CLI](references/claude-cli.md) or [Codex CLI](references/codex-cli.md). The shared readiness procedure determines whether to send the substantive assignment or a readiness-only exchange.

## Prepare the Assignment

Compose assignments and follow-ups under the global **Prompt contract**, **Same-access delegation**, and **Direct exchanges** policies. Include the recursion boundary and active [progress and command limits](references/session-coordination.md#enforce-progress-and-command-limits) in the assignment. For follow-ups, apply the [retained-context checks](references/session-coordination.md#check-retained-context).

## Dispatch and Coordinate

1. Start a fresh conversation by default, one per independent assignment when the work splits. Resume a specific full session UUID when continuing the assigned work, including after its readiness-only exchange. Follow the participant reference’s launch and resumption procedures.
2. Check pending sessions opportunistically, observing all of them in one call regardless of participant. Don’t replace useful concurrent work with a blocking polling script.
3. When CLI sessions become the last outstanding dependencies, await those same sessions together under the [inactivity policy](references/session-coordination.md#inactivity). This is a change in the coordinator’s scheduling, not a foreground relaunch.
4. Retrieve the complete response for each session’s latest requested turn before treating it as finished.

Background execution doesn’t itself give the host a completion callback or the ability to wake a yielded thread. Use only scheduling capabilities the host actually provides. Don’t promise automatic follow-up after yielding when none exists, and don’t leave a running session without an identified monitoring owner.

## Resolve Feedback

Apply the global **Findings** policy. For delegated reviews, use `subagent-review-flow` as for native subagents. Use the [resumption procedure](references/session-coordination.md#resume-the-same-exchange) for continued work, including required re-review.

## Finish or Pause

Follow the [error and stopping procedure](references/session-coordination.md#handle-errors-and-stop). Report the integrated outcome under the shared communication policies. Identify still-running work and its monitoring owner when applicable.
