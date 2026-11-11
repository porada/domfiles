# Validation Tools

## ShellCheck Read Scope

1. Inspect the selected checks and diagnostics without running them. Identify every project wrapper, configuration file, `SHELLCHECK_OPTS` value, or command line option that may invoke ShellCheck or allow it to read sourced files not explicitly supplied as inputs.
2. Resolve ShellCheck’s read scope, including every sourced file it may read in the validation scope. If you can’t establish or constrain the read scope, skip each affected check and report the validation limitation.
3. Run a selected check only after every ShellCheck path it can reach has passed the scope gate. A check that can’t invoke ShellCheck doesn’t require that gate.

When no project ShellCheck configuration is established, pass the complete resolved source set explicitly to `shellcheck --norc --shell=sh -- <path>…` only after the gate passes and only when ShellCheck is already available. In any invocation, allow ShellCheck to read sourced files not explicitly supplied as inputs only after limiting the source files it can read to that resolved set.

If ShellCheck remains unavailable for an authorized check, report that validation limitation.

## Validator Dependencies

An authorized check includes installing or restoring its project-prescribed validators and package manager version through the established workflow, including configured lifecycle steps. Don’t ask merely because a prescribed validator is absent or needs downloading. Preserve explicit task restrictions and native build approvals, integrity checks, and trust controls. A new validator choice or a change to prescribed features, source, or version needs explicit user approval or an applicable governing grant covering that choice and its acquisition and execution effects. Verify the grant’s conditions and lifetime before use, including for temporary package runner acquisitions. Ask for uncovered choices or effects rather than treating temporary use as permission.
