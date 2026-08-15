# Terminal permissions

This branch specializes the shared [agent permission workflow](permissions.md) for terminal commands.

## Apply the terminal permission policy

- For `corepack`, `npm`, `npx`, `pnpm`, `pnpx`, `pnx`, `yarn`, or a delegated Node package binary, follow [Node package manager permissions](node-package-manager-permissions.md).
- Allow Docker inspection operations without confirmation. Require confirmation for operations that execute workloads or create, modify, or remove Docker state.
- Preserve `agent.tool_permissions.tools.terminal.default` as `confirm`.
- Classify configured prompt behavior independently from agent invocation policy, following the shared [permission and policy layering rule](permissions.md#apply-the-shared-permission-policy). A policy prohibition does not by itself justify confirmation or denial, while an explicit permission-layer invariant remains authoritative for the configured result.
- Set `"case_sensitive": true` on every terminal command pattern unless a verified command-specific requirement justifies case-insensitive matching. Prefer a scoped inline case-insensitive group for an exceptional token instead of making the entire pattern case-insensitive.
- Give every normalized command form one command-owner group.
    - Resolve the top-level executable after removing approved fixed assignments and wrappers from the normalized command.
    - Use that executable as the owner by default. A selected domain policy may partition one executable into root, subcommand, or compound-workflow owners when those forms require independent maintenance.
    - Keep distinct executable names in separate owner groups even when they use equivalent syntax. A domain partition never authorizes combining unrelated top-level executables.
    - Let one owner use multiple adjacent patterns when separate syntax roles keep them clearer.
    - When splitting a pooled pattern, place each resulting owner group at its position in the complete bucket order. Do not insert every split object at the pooled object’s former index when the resulting owners belong on opposite sides of another group.
    - Keep command ownership consistent across `terminal.always_allow`, `terminal.always_confirm`, and `terminal.always_deny`. An explicitly command-independent lexical guard may remain global.
- Let each command-owner group own its discovery, direct, and wrapped forms.
    - Do not maintain general allowances or shared discovery patterns.
    - Keep every discovery form exact and end-anchored. Include `--help`, `--version`, or another discovery spelling only after positive command-specific evidence establishes that the exact invocation exits without entering an interactive mode, mutating state, reading input, or starting normal execution. For a discovery option placed after another token, require parser-order evidence that it short-circuits before processing every preceding option or operand in that exact form. Never generalize one verified trailing-help sequence to arbitrary prefixes. Omit every form that crosses this boundary. A verified unsupported form that terminates without prompting qualifies. See [Zed command discovery defaults](../../../PROJECT.md#zed-command-discovery-defaults) for rationale.
    - For every inventoried subcommand, include its exact `--help` form in `terminal.always_allow` whenever that invocation meets this boundary, even when the subcommand’s execution forms require confirmation or denial. Scope broader `terminal.always_confirm` and `terminal.always_deny` patterns so they do not match the verified-safe help input.
    - Keep distinct command-owned discovery grammars separate but adjacent when that is clearer than consolidation. Never combine them merely because their total decoded length fits one pattern.
    - Treat `nohup`, `xargs`, and comparable wrappers as per-command forms. `xargs` owns only its own root and discovery forms, while each child command owns its bounded `xargs … <command>` form. Do not maintain a pooled child-command inventory. See [Zed xargs command ownership](../../../PROJECT.md#zed-xargs-command-ownership) for rationale.
        - Limit `xargs`’s own options to bounded, noninteractive argument splitting and batching controls.
        - Require confirmation for the complete nested child-command owner group whenever standard input could activate a code-execution hook, file-writing option, destructive operation, or other hazardous form. Do not narrow that override to individual nested syntax branches.
    - Keep each repeated wrapper grammar canonical and byte-identical across the command-owned patterns that use it.
- Keep every decoded regex pattern under `1,000` Unicode scalar values, measuring the parsed `.pattern` value rather than its JSON-escaped source representation.
    - Factor a shared prefix for finite explicit alternatives when that preserves the exact accepted language and every constrained operand role.
    - Split a pattern when the applicable ownership policy assigns its branches to different groups or when it combines unrelated syntax roles.
    - Do not split one coherent syntax role merely to pursue a smaller arbitrary threshold.
- Treat command-pattern ordering as part of every change, not as follow-up cleanup. After adding, splitting, moving, or replacing a pattern, verify the complete touched permission bucket and place every changed entry at its final ordered position instead of appending it or retaining a stale source index.
    - Order command-owner groups alphabetically by semantic owner unless a selected domain policy defines an explicit section order.
    - Within one owner group, place discovery entries first, direct entries next, and wrapped entries last. Alphabetize entries within each role by their stable semantic command or form key rather than by raw escaped regex text.
    - Alphabetize alternatives when their grammar permits.
- When another serial task owns a shared or pooled pattern, keep that object byte-identical, record the exact alternatives deferred to its owner in task evidence, and do not report the pooled cleanup as resolved.
- Keep command-specific prefixes and wrappers within the owning command group. Account for them in every participating allowance, confirmation override, and denial.
    - A security denial may intentionally recognize a broader prefix operand than the corresponding allowance when narrower matching would let the protected form fall through to confirmation. Record that asymmetry in task evidence rather than narrowing the denial for textual symmetry.
    - Apply optional `HOMEBREW_NO_*` prefixes with the fixed value `1` to every Homebrew terminal pattern.
    - Apply optional repeated `MANPAGER=cat` and `PAGER=cat` prefixes only within command-owned forms that support them. A pager prefix must not create shared command ownership.
- Prefer literal spaces over whitespace character classes.
- Prefer explicit alternatives over optional fragments when one executable has distinct accepted forms.
- Treat signaling explicit numeric process IDs as an intentional allowance for polling and stopping processes associated with the current task. Do not extend this allowance to process names or patterns.
- For every command-allowance investigation, evaluate whether a separate project-relative task-owned `.agent-<name>` family is an applicable path-scoped variant of the requested operation under the [agent-directory permission policy](agent-repository-permissions.md#apply-the-agent-directory-allowance-policy). When that policy makes the variant eligible, include it in the resulting proposal and, when the task authorizes mutation, in the implementation without requiring a separate scope selection. Do not introduce unrelated command families.
- For mixed-purpose utilities and interpreters, prefer positive allowlists of non-mutating forms. Use a broad allowance with `terminal.always_confirm` only when every hazardous form can be matched reliably. Otherwise, preserve default confirmation.
    - A fixed-expression parse-only validator may qualify only after classifying its environment and startup inputs, expression, input count and source, output, path roles, potential to wait for input, and separator placement. Keep arbitrary expressions, inputs, and paths outside that family.
- Use `terminal.always_confirm` to override broader `terminal.always_allow` entries for hazardous argument forms, including code-execution hooks, package runners, destructive operations, force flags, and commands that uninstall the invoked tool itself. Account for global options, combined short flags, and accepted long-option abbreviations.
    - When a requested safe form already belongs to the language of an existing confirmation pattern, adding an allowance alone cannot change the configured decision. First replace the broad confirmation branch with a positive hazardous grammar that continues to match every unsafe form, then retain an existing allowance that covers the safe form or add a narrow one. Leave the existing allowance unchanged unless its own language is independently unsafe. When the hazardous confirmation language cannot be expressed reliably, a coordinated owner-level decomposition of both buckets may be proposed, but a broader allowance refactor requires explicit user selection when it exceeds the requested exemption. Otherwise, leave the form confirmable.
- Do not report overlaps between `terminal.always_allow` and `terminal.always_confirm` when `terminal.always_confirm` acts as a safety override.

## Interpret commands that prompted for confirmation

- When the user supplies commands because they prompted for confirmation and asks to make them automatic without explicitly limiting the request to literal invocations, treat each command as evidence of the intended operation, not as acceptance of an exact exception or a broader grammar.
- Preserve each supplied raw shell command in task evidence and map it to every normalized permission input Zed evaluates, including nested commands and separately evaluated redirections. Keep the raw command, normalized inputs, configured pattern decisions, aggregate effective decision, observed runtime result, and user acceptance as distinct evidence levels.
- Classify every supplied operation before editing. Derive the broadest currently established safe grammar as a proposal, including only evidence-supported variable operand roles, verified aliases and equivalent forms, accepted option placement, and applicable wrappers. “Broadest” does not include deprecated or experimental APIs unless explicitly requested, unknown future options, unrestricted operands, speculative aliases, or other unverified behavior unless a selected domain policy records an explicit user-approved namespace allowance. Keep such an exception bounded to that namespace’s established syntax and preserve higher-precedence hazard overrides.
- Before candidate capture, require explicit user selection whenever the proposed grammar would automatically allow normalized forms beyond the supplied invocations or an already established user-approved family. Never use an exact allowance as an escape hatch when the natural operation family remains legitimately hazardous. If any supplied command should remain confirmable or denied, or cannot be generalized safely, stop before mutation, identify the concrete boundary and any defensible automatic family, then wait for the user to choose whether to keep confirmation or proceed with that boundary. Do not add a literal exception, implement only the agreeable subset, or defer the unsafe explanation until after a partial edit. An explicit request for one literal form does not override its safety classification.

## Investigate terminal behavior

1. Select the evidence route before investigating behavior.
    - When the current request explicitly identifies supplied classifications as verified or settled, let that evidence own behavior classifications and required cases. Do not rerun executable help, manuals, or online research to challenge them. Let the current skill and settings own pattern structure unless the request explicitly overrides it, then derive normalized inputs, translate the classifications, and validate the resulting regexes.
2. Resolve each user-supplied command under the [confirmation-prompt interpretation policy](#interpret-commands-that-prompted-for-confirmation), recording its normalized required cases and variable operand roles before translating it into a pattern.
3. When behavior classifications are not already settled, inspect the locally installed executable’s help or manual to resolve the complete bounded command family.
    - Record documented aliases and every accepted option or operand placement and ordering that preserves the operation and safety classification. Prefer a repeated grammar for order-independent tokens rather than enumerating permutations.
    - Record the forms that execute code, write data, alter state, or remove resources.
    - Record forms that block indefinitely, monitor continuously, produce unbounded output, or return a terminating snapshot. Treat duration and output boundedness as classification dimensions rather than assuming every non-mutating inspection form is equivalent.
    - For a categorical allowance whose unknown future members must remain confirmable, record the complete finite set of currently allowed names required by the [positive-branch policy](#translate-terminal-behavior-into-regex). A selected domain policy may instead define an explicit user-approved namespace exception.
    - Run each local help or manual inspection with a short, bounded timeout. Prefer `MANPAGER=cat PAGER=cat man <command> | col -b` when a manual is available. When the manual is long, save its plain-text output under the task-specific temporary directory and read only targeted option sections.
    - If the executable is unavailable or local help remains interactive, consult current official documentation or source.
4. When settled evidence supplies a semantic category without its finite current names, leave unenumerated members confirmable and report that stricter boundary. Do not research beyond the evidence when the request forbids it.
5. Before reading terminal pattern bodies, follow the [inventory-first workflow](permission-evaluator.md#inventory-terminal-patterns). Keep complete long patterns in files rather than command output or conversation context.
6. For a blocked shell line, determine the permission input that Zed evaluated. Shell operators, redirections, assignments, and wrappers can produce several independently checked segments.
7. Consult current official Zed documentation or source when parsing, regex support, or permission precedence is unclear.
8. Use `rg --no-config` for bounded operational ripgrep investigations unless ripgrep configuration behavior or an exact exit-only discovery form is itself under test.
9. Ignore repository entrypoints, custom Git aliases, and repository-specific helpers unless the user explicitly includes them in scope.

## Close the parser and effect model

Before candidate capture and during every read-only audit, diagnosis, or review of existing or proposed terminal patterns:

1. Resolve parser consumption before classifying syntax. Record each accepted attached, equals-bound, or separated option value, how value-taking short options consume the rest of a cluster or the next token, and which later `--` token is an actual terminator rather than a consumed value. Treat hazardous-looking characters according to their parsed role rather than character presence.
2. Resolve the current Zed normalization boundary. Ordinary argument quotes are removed, but current parsing can preserve quote syntax when it is data needed to represent an assignment value. Record every applicable preserved form from current evidence. When quote state is unavailable, leave brace, command-substitution, glob, list-variable, or other expansion-bearing syntax confirmable unless a positive grammar proves that expansion cannot change argument values or cardinality. Include line-break-bearing inputs when normalized commands can contain them.
3. Resolve behavior that is not explicit in the visible command. Include aliases that launch external helpers, input-controlled output paths or links, pre-dispatch configuration and startup files, shell scripts that delegate operands to another executable, stdin-injected child arguments, and other hidden effects. Classify the resulting behavior rather than the top-level command description.
4. Bind every behavior-bearing path role independently. Include configuration, destination, input, module, output, positional, rule, and source paths. Apply the [agent-directory policy](agent-repository-permissions.md#apply-the-agent-directory-allowance-policy) only when every residual effect and path role is visible and containable.
5. Distinguish complete finite inventories from combinatorial grammars. Test every member of a claimed finite set. For a combinatorial grammar, cover each boundary, feature, hazardous role, interaction, and near miss without describing the representative corpus as formal completeness.

## Translate terminal behavior into regex

- Anchor a pattern with `^`. Add `$` when trailing arguments would change the safety classification. In an allowance pattern that accepts trailing arguments, end each executable, subcommand, or option token with `(?: |$)`. The weaker `\b` is a lexical boundary, not a shell token boundary. Use it only when lexical matching is intentional, such as in a conservative confirmation override.
- Use syntax supported by Zed’s Rust-compatible regex engine. It does not support lookarounds or backreferences.
- Build positive branches from finite accepted grammar instead of trying to subtract cases with unsupported regex features. Keep unknown future names outside the allowance unless a selected domain policy explicitly defines a user-approved bounded namespace wildcard.
- Test against the normalized permission inputs established by the parser and effect model, not merely the original shell line.
- When an allowance’s safety depends on a lexical path namespace, require every behavior-bearing path role in the normalized command to establish that namespace independently, including every accepted path-bearing option operand and each positional operand, whether it is a destination or source. One namespace-bounded path does not constrain another path role. Leave a form confirmable when any path role remains unconstrained or its namespace cannot be established from the normalized input.
- Encode discovery and wrapper forms according to the command-owner policy above.
- Do not execute a destructive command merely to test a permission pattern.

## Validate terminal patterns

Validate every in-scope pattern against:

- Every user-supplied command intended to become automatic, with each derived normalized input represented in configured-decision cases and the aggregate operation resolved through the effective-permission workflow. Include a trailing-argument or materially reordered near miss for each requested form.
- Each other required matching example in normalized form, boundary values for every generalized operand role, and representative permutations that establish the accepted option and operand placement grammar.
- Every member of a claimed finite alias, delimiter, option-spelling, or prefix set. For combinatorial grammars, cover each spelling and role interaction without enumerating every product unless the grammar is explicitly finite.
- Every verified-safe exact subcommand `--help` form, including those whose execution forms are confirmable or denied, plus configured-precedence cases proving that broader confirmation or denial patterns do not capture it.
- Representative blocking, continuous-monitoring, terminating-snapshot, and unbounded-output forms when duration or output boundedness affects the classification.
- Hazardous forms that must match a confirmation or denial override or remain unmatched by an allowance.
- Near misses involving accepted long-option abbreviations, assignments, combined short flags, consumed option values, global options, real and apparent terminators, shell expansion, trailing operands, or wrappers that cross the intended boundary. Use file-backed cases for line-break-bearing inputs.
- Every accepted spelling that can match an allowance and a higher-precedence confirmation or denial, proving the expected bucket states and final decision.
- For namespace-bounded path grammar, each positional and option-bearing path role independently, including absolute paths, parent traversal, permitted-looking prefixes that escape the namespace, and supported permutations. Treat the check as lexical and do not infer the command’s working directory or resolve symlinks.
- The command-owner, ordering, discovery, wrapper, finite-inventory, and decoded-length invariants above before promoting a candidate.
