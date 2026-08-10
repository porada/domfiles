# Terminal permissions

Follow the shared [agent permission workflow](permissions.md) and this branch whenever terminal commands or terminal permission patterns are in scope. For Git commands or patterns, also read [Git permissions](git-permissions.md).

## Apply the terminal permission policy

- Treat ordinary package-manager workflows as intentional allowances. Continue to require confirmation for package runners that can download and execute arbitrary code.
- Allow Docker inspection operations without confirmation. Require confirmation for operations that execute workloads or create, modify, or remove Docker state.
- Set `"case_sensitive": true` on every terminal command pattern unless a verified command-specific requirement justifies case-insensitive matching. Prefer a scoped inline case-insensitive group for an exceptional token instead of making the entire pattern case-insensitive.
- Give every normalized command form one command-owner group.
    - Resolve the top-level executable after removing approved fixed assignments and wrappers from the normalized command.
    - Use that executable as the owner by default. A selected domain policy may partition one executable into root, subcommand, or compound-workflow owners when those forms require independent maintenance.
    - Keep distinct executable names in separate owner groups even when they use equivalent syntax. A domain partition never authorizes combining unrelated top-level executables.
    - Let one owner use multiple adjacent patterns when separate syntax roles keep them clearer.
    - Keep command ownership consistent across `terminal.always_allow`, `terminal.always_confirm`, and `terminal.always_deny`. An explicitly command-independent lexical guard may remain global.
- Let each command-owner group own its discovery, direct, and wrapped forms.
    - Do not maintain general allowances or shared discovery patterns.
    - Include exact, end-anchored `-h`, `-help`, `--help`, `-v`, `-version`, and `--version` forms by default, including unsupported forms that exit without prompting. Omit a form only when the exact invocation is known to cross the discovery boundary by starting normal execution, reading input, mutating state, or entering an interactive mode. See [Zed command discovery defaults](../../../PROJECT.md#zed-command-discovery-defaults) for rationale.
    - Treat `nohup`, `xargs`, and comparable wrappers as per-command forms. `xargs` owns only its own root and discovery forms, while each child command owns its bounded `xargs … <command>` form. Do not maintain a pooled child-command inventory. See [Zed xargs command ownership](../../../PROJECT.md#zed-xargs-command-ownership) for rationale.
        - Limit `xargs`’s own options to bounded, noninteractive argument splitting and batching controls.
        - Require confirmation for the complete nested child-command owner group whenever standard input could activate a code-execution hook, file-writing option, destructive operation, or other hazardous form. Do not narrow that override to individual nested syntax branches.
    - Keep each repeated wrapper grammar canonical and byte-identical across the command-owned patterns that use it.
- Keep every decoded regex pattern under `1,000` characters, measuring the parsed `.pattern` value rather than its JSON-escaped source representation.
    - Split a pattern when it spans multiple top-level executables, multiple direct Git subcommands, multiple `xargs` child commands, or unrelated syntax roles.
    - Do not split one coherent syntax role merely to pursue a smaller arbitrary threshold.
- Group command-owned patterns alphabetically within each permission bucket.
    - Within one owner group, place discovery forms first, direct forms next, and wrapped forms alphabetically.
    - Alphabetize alternatives when their grammar permits.
- Keep command-specific prefixes and wrappers within the owning command group. Account for them in applicable allowances and confirmation overrides.
    - Apply optional `HOMEBREW_NO_*` prefixes with the fixed value `1` to every Homebrew terminal pattern.
    - Apply optional repeated `MANPAGER=cat` and `PAGER=cat` prefixes only within command-owned forms that support them. A pager prefix must not create shared command ownership.
- Prefer literal spaces over whitespace character classes.
- Prefer explicit alternatives over optional fragments when one executable has distinct accepted forms.
- Treat signaling explicit numeric process IDs as an intentional allowance for polling and stopping processes associated with the current task. Do not extend this allowance to process names or patterns.
- For mixed-purpose utilities and interpreters, prefer positive allowlists of non-mutating forms. Use a broad allowance with `terminal.always_confirm` only when every hazardous form can be matched reliably. Otherwise, preserve default confirmation.
- Use `terminal.always_confirm` to override broader `terminal.always_allow` entries for hazardous argument forms, including code-execution hooks, package runners, destructive operations, force flags, and commands that uninstall the invoked tool itself. Account for global options, combined short flags, and accepted long-option abbreviations.
- Do not report overlaps between `terminal.always_allow` and `terminal.always_confirm` when `terminal.always_confirm` acts as a safety override.

## Investigate terminal behavior

1. Select the evidence route before investigating behavior.
    - When the current request explicitly identifies supplied classifications as verified or settled, treat them as authoritative semantic evidence. Do not rerun executable help, manuals, or online research to challenge them. Continue to inspect current settings, derive normalized inputs, translate the classifications, and validate the resulting regexes.
    - Let supplied evidence own behavior classifications and required cases. Let the current skill and settings own pattern structure unless the request explicitly overrides that structure.
2. Resolve each user-supplied command before translating it into a pattern.
    - Treat a cited command that the request intends to run without confirmation as a required matching case. If its normalized permission input cannot be allowed within the requested safety boundary, leave it confirmable and report the limitation instead of silently substituting a stricter invocation.
    - Unless the user explicitly limits the allowance to the exact invocation, treat the command as evidence of the intended operation rather than a literal pattern template. Separate invariant operation and safety tokens from variable paths, identifiers, and similar operands, then generalize only those variable roles within the requested boundary.
3. When behavior classifications are not already settled, inspect the locally installed executable’s help or manual to resolve the complete bounded command family.
    - Record documented aliases and every accepted option or operand placement and ordering that preserves the operation and safety classification. Prefer a repeated grammar for order-independent tokens rather than enumerating permutations.
    - Record the forms that execute code, write data, alter state, or remove resources.
    - For a categorical allowance whose unknown future members must remain confirmable, record the complete finite set of currently allowed names. Never translate an open-ended semantic category into an open-ended allowance.
    - Run each local help or manual inspection with a short, bounded timeout. Prefer `MANPAGER=cat PAGER=cat man <command> | col -b` when a manual is available.
    - If the executable is unavailable or local help remains interactive, consult current official documentation or source.
4. When settled evidence supplies a semantic category without its finite current names, leave unenumerated members confirmable and report that stricter boundary. Do not research beyond the evidence when the request forbids it.
5. Before reading terminal pattern bodies, follow the [inventory-first workflow](permission-evaluator.md#inventory-terminal-patterns). Keep complete long patterns in files rather than command output or conversation context.
6. For a blocked shell line, determine the permission input that Zed evaluated. Shell operators, redirections, assignments, and wrappers can produce several independently checked segments.
7. Consult current official Zed documentation or source when parsing, regex support, or permission precedence is unclear.
8. Ignore repository entrypoints, custom Git aliases, and repository-specific helpers unless the user explicitly includes them in scope.

## Translate terminal behavior into regex

- Anchor a pattern with `^`. Add `$` when trailing arguments would change the safety classification. In an allowance pattern that accepts trailing arguments, end each executable, subcommand, or option token with `(?: |$)`. The weaker `\b` is a lexical boundary, not a shell token boundary. Use it only when lexical matching is intentional, such as in a conservative confirmation override.
- Use syntax supported by Zed’s Rust-compatible regex engine. It does not support lookarounds or backreferences.
- Zed matches permission patterns case-insensitively by default. Follow the explicit case-sensitivity policy above for terminal commands, including patterns whose current tokens happen to be unambiguous.
- Build positive branches from finite accepted grammar instead of trying to subtract cases with unsupported regex features. Keep unknown future names outside the allowance.
- Test against Zed’s normalized permission input, not merely the original shell line.
    - Ordinary shell quotes do not remain in the normalized input. Preserve their enclosed text and normalized spaces, and encode quote characters only when current Zed parsing treats them as data rather than shell syntax.
- Encode discovery and wrapper forms according to the command-owner policy above.
- Do not execute a destructive command merely to test a permission pattern.

## Validate terminal patterns

Validate every in-scope pattern against:

- Each required matching example in normalized form, boundary values for every generalized operand role, and representative permutations that establish the accepted option and operand placement grammar.
- Hazardous forms that must match a confirmation or denial override or remain unmatched by an allowance.
- Near misses involving global options, wrappers, assignments, combined short flags, accepted long-option abbreviations, or trailing operands that cross the intended boundary.
- The command-owner, ordering, discovery, wrapper, finite-inventory, and decoded-length invariants above before promoting a candidate.
