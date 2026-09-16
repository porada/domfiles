# Prepare Security Reports

Use this security-specific drafting procedure when the entrypoint’s existing [peer choice](../SKILL.md#compose-with-peers) selects the local fallback for `human-facing-writing`. Do not rediscover the peer or substitute this procedure for its required stop. The shared [post content workflow](prepare-post-content.md) owns checklists, common prose, templates, and title/body delivery. Apply the [execution boundaries](execution-boundaries.md) to evidence gathering and any reproduction.

## Establish the Private Boundary

Identify the affected project and its designated private reporting channel from the available security policy or verified platform guidance. Establish the receiving template and whether the user can submit manually through that channel. If no suitable private channel is available, or its required process conflicts with manual browser submission, stop for the user’s decision. Do not substitute a public issue or pull request.

Keep undisclosed details out of public queries, public repository artifacts, and unrelated services. Use only legitimately available evidence within its disclosure boundary. Do not request access to private report history merely for writing examples. A report request does not authorize testing against a third party or transmitting a draft.

## Establish the Verified Mechanism

Identify the affected revision or version and relevant configuration, the intended security decision, and the boundary the observed behavior crosses. Trace what an attacker controls through the causal steps to the demonstrated consequence. Preserve only the source identifiers needed to locate important transitions. If investigation disproves the original theory, replace it with the verified mechanism rather than retaining it behind a caveat.

Separate direct observations from source-based inference and untested possibilities. Do not generalize a tested configuration into an unsupported affected-version range. State missing evidence and its effect on confidence. If a necessary claim cannot be verified safely, limit the draft to what is established and identify the required evidence before claiming readiness.

## Prepare a Safe Reproduction

Describe a complete, repeatable, minimally hazardous reproduction using an authorized isolated environment and synthetic data. Preserve the structural setup, relevant configuration, control cases, and expected and actual behavior. Make the attacker’s prerequisites explicit, including required access and control over the input. Distinguish steps actually exercised from steps proposed for the recipient to verify.

Use clearly named placeholders such as `<test-account>` or `<test-token>` instead of private values or real secrets. Preserve only the format constraints necessary to reproduce the issue. Never inspect, reproduce, or request a real secret for the report, even at the user’s direction. Do not place live credentials in artifacts, commands, or examples.

Prefer a harmless demonstration of the boundary failure over unnecessary exploitation. Do not access real user data, run destructive tests, or test an external target merely to strengthen the narrative. If safe repeatability depends on unavailable authority or an unsuitable environment, stop that execution and report the limitation rather than claiming successful reproduction.

## Draft the Security Content

Follow the receiving template’s structure. When it separates these roles, give each fact one primary home without removing unique evidence:

- **Title:** Pair a familiar mechanism with a concrete security consequence. Keep low-level terminology in the details unless needed for accuracy or distinction.
- **Summary:** State the verified mechanism, affected security boundary, and representative impact. Retain any prerequisite or limitation necessary to avoid overstating exploitability.
- **Details:** Explain the verified causal sequence and the evidence locating its important transitions.
- **Proof of concept:** Supply the safe reproduction, including its controls and observed result.
- **Impact:** Identify affected users, attacker prerequisites, consequences supported by the evidence, and limits imposed by independent security boundaries. An independent limit on harm does not restore the security decision that was bypassed.

Include remediation or mitigation only when evidence supports it. Explain what it would address and any remaining limitation. Do not invent a fix, promise that an untested measure resolves the issue, or treat report drafting as authorization to change the affected project.

## Separate Reports From CVE Requests

Reporting a vulnerability and requesting a CVE identifier are distinct actions. Establish which the user is preparing, and use only verified requirements for that receiving process. Do not infer eligibility or invent an identifier. A request is not an assignment. If eligibility or assignment cannot be established, leave it explicitly unverified.

Return the draft through [content finalization](prepare-post-content.md#finalize-content), preserving the private destination in the [handoff](../SKILL.md#hand-back-the-contribution). Required personal attestations or fields that the agent cannot supply remain for the user, not guessed values. Pause readiness for an unmet requirement or unsuitable channel. The user submits manually through the designated private channel. Do not submit through an API, browser automation, or `gh`.
