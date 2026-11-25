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

When I invoke `super-flow`, I explicitly request separate top-level T3 Code threads for the engineers and, when needed, the editor and a security engineer. This includes engineers added within the team I approve. The agent in my invoking thread coordinates the flow.

Follow [Super Flow](references/super-flow.md). Start a new flow with the scope discussion. Before selecting team settings or creating member threads, read [Team Defaults](references/team-defaults.md). For a correction after final handoff, go directly to [Handle a Correction After Handoff](references/super-flow.md#handle-a-correction-after-handoff).
