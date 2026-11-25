---
name: super-flow
description: |-
    Coordinate a team of agents to carry out an agreed plan.

disable-model-invocation: true
metadata:
    internal: true
---

# Super Flow

Run this workflow only inside T3 Code. In any other client, tell the user it requires T3 Code and stop.

When I invoke `super-flow` in T3 Code, I explicitly request separate top-level threads for the team once I authorize the flow to start. Reuse the member threads we agree on. Open threads as needed for engineers, including later additions within the approved ceiling, a security engineer when needed, and an editor when someone other than the coordinator serves as editor. The agent in my invoking thread coordinates the flow.

When I authorize the flow to start, I also explicitly request a separate top-level thread by default for each peer reviewer of each flow unit and one for the cold reviewer of the consolidated series. This includes new threads for required reviewer replacements and repeated cold reviews.

Follow [Super Flow](references/super-flow.md). Start a new flow with the scope discussion. Before selecting team settings or creating member threads, read [Team Defaults](references/team-defaults.md). For a correction after final handoff, go directly to [Handle a Correction After Handoff](references/super-flow.md#handle-a-correction-after-handoff).
