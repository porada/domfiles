# Validation Tools

## ShellCheck Read Scope

Resolve ShellCheck’s read scope before invoking it directly or through a project entrypoint. Include every sourced file ShellCheck may read in the resolved validation scope. If the read scope cannot be established or constrained, skip each affected check and report the validation limitation.

When no project ShellCheck configuration is established, pass the complete resolved source set explicitly to `shellcheck --norc --shell=sh -- <path>…` only after the gate passes and only when ShellCheck is already available. In any invocation, allow ShellCheck to read sourced files not explicitly supplied as inputs only after limiting the source files it can read to that resolved set.

If ShellCheck remains unavailable for an authorized check, report that validation limitation.

## Validator Dependencies

Require explicit user approval for an agent-selected validator or a change to its prescribed features, source, or version, even for temporary acquisition through a package runner. Authorization to run an established project check includes acquiring the validators it already prescribes through configuration, lockfiles, manifests, or scripts, using its normal acquisition mechanism. Do not ask again solely because a prescribed validator is absent locally or downloaded on demand. An agent cannot create that authorization by adding its own declaration or acquisition step. Preserve explicit task restrictions and applicable execution, lifecycle script, permission, and trust boundaries.
