# Validation Tools

## ShellCheck Read Scope

1. Inspect the selected checks and diagnostics without running them. Identify every project wrapper, configuration file, `SHELLCHECK_OPTS` value, or command line option that may invoke ShellCheck or allow it to read sourced files not explicitly supplied as inputs.
2. Resolve ShellCheck’s read scope, including every sourced file it may read in the validation scope. If you can’t establish or constrain the read scope, skip each affected check and report the validation limitation.
3. Run a selected check only after every ShellCheck path it can reach has passed the scope gate. A check that can’t invoke ShellCheck doesn’t require that gate.

When no project ShellCheck configuration is established, pass the complete resolved source set explicitly to `shellcheck --norc --shell=sh -- <path>…` only after the gate passes and only when ShellCheck is already available. In any invocation, allow ShellCheck to read sourced files not explicitly supplied as inputs only after limiting the source files it can read to that resolved set.

If ShellCheck remains unavailable for an authorized check, report that validation limitation.

## Validator Dependencies

Require explicit user approval for an agent-selected validator or a change to its prescribed features, source, or version, even for temporary acquisition through a package runner. Authorization to run an established project check includes acquiring the validators it already prescribes through configuration, lockfiles, manifests, or scripts, using its normal acquisition mechanism. Don’t ask again solely because a prescribed validator is absent locally or downloaded on demand. An agent can’t create that authorization by adding its own declaration or acquisition step. Preserve explicit task restrictions and applicable execution, lifecycle script, permission, and trust boundaries.
