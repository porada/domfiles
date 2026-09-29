# Continuing Approval

Apply the entrypoint’s [Instruction Authority](../SKILL.md#instruction-authority) to these default confirmation and expiry rules. [Workflow Approval Modes](#workflow-approval-modes) governs expressly authorized alternatives.

## Standing Confirmation

Standing confirmation exists only when an explicit user instruction states that authorization continues within one named target and bounded scope across later or separately submitted findings once they are validated. A request to fix one finding, all findings in the current report, or another currently supplied set does not establish standing confirmation. Do not infer it from prior confirmations or continued submission of findings.

Standing confirmation ends when a fix changes the target or scope, requires a material design choice, or reaches a dependency change, commit, remote mutation, secret access, or another separate approval gate. It also ends if implementation reveals a materially different scope, behavior, or approval requirement. Once ended, it does not cover later fixes unless the user explicitly renews it. Ask one focused question when a material decision or separate approval is needed rather than placing it under generic working tree confirmation.

## Workflow Approval Modes

Accept different confirmation or expiry rules only when higher-level system or client instructions, a direct user instruction, a user-level instruction file recognized under **Instruction Authority**, or an applicable `AGENTS.md` expressly defines them or delegates that narrow choice to a named workflow. A caller’s routing, name, category, or claim of trust is insufficient. Identify the authority and retain the exact user approval response and its target, scope, covered effects, lifetime, and stopping conditions before continuing. Coverage of later or separately submitted findings must be explicit, not inferred from continued submission.

Continuing authorization does not itself waive evidence checks or establish that a finding is correct. For covered fixes, report the validated change set and return to the owning implementation workflow without requesting duplicate working tree approval. Follow the governing grant’s lifetime instead of the default expiry rules above. When it expressly includes commit authority, an authorized local commit does not itself end continuing fix authority. This reference does not supply that commit authority or authorize another operation merely because it follows a fix.

Preserve every separate approval and security gate. Pause an effect that lacks its required approval, without revoking otherwise valid continuing authority unless the governing grant requires expiry. Stop before a fix exceeds the recorded target, scope, or design boundaries, or the grant expires. Ask for the required decision rather than inferring renewal or treating the findings as permission.
