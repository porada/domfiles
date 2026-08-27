# Shared helper design

## Keep responsibilities reusable

- Keep entrypoints focused on command-specific bounds and output behavior.
- Define shared defaults in the helper that owns their behavior. Callers accepting a default must omit the corresponding argument instead of repeating its value.
- When a loop computes a reusable value, prefer a named helper that returns the result over a call-site loop with a trailing here-document redirect. Retain loop redirection when POSIX scope or failure handling requires it and extraction would not improve clarity.

## Align cross-shell helper contracts

- The helpers named exactly `__` and `__domfiles` have no automatic Fish counterpart names. For any other `domlib` helper, replace the leading `__` with `__domfiles_`, leaving names that already begin with `__domfiles_` unchanged.
- Treat paired POSIX and Fish helpers as peer implementations of a shared contract. Neither implementation governs the other.
- Keep their accepted inputs, stdout and stderr behavior, return statuses, side effects, defaults, failure conditions, and adjacent contract documentation semantically aligned wherever both shells can support the same behavior.
- Use shell-native implementations. Allow a shell-specific contract difference when native behavior has no equivalent in the other shell. Document each accepted difference and its rationale in [cross-shell helper differences](../../../PROJECT.md#cross-shell-helper-differences), and keep the remainder of the pair’s contract aligned.
- When either helper changes, inspect its counterpart. During an authorized change, update the implementations, contract documentation, and documented differences needed to keep the pair accurate. During a review, audit, or diagnosis, report required alignment without editing either helper or its documentation.

## Document helper contracts

- Give every `domlib` function one adjacent natural-language contract comment. Do not use `@param` or `@returns` tags.
- Describe the semantic contract rather than narrating implementation. Include non-obvious inputs, outputs, or fallback behavior only when omitting them would mislead, and repeat a source-owned value only when the contract would not make sense without it.
- Begin standard-output contracts with `Prints …`, predicates with `Returns success …`, and side-effect contracts with a direct action verb.
- Let command-shaped wrappers rely on ordinary command semantics implied by their names rather than restating them.
- Name `$1`, `$@`, and other positional parameters only when their positions clarify the contract.
- Wrap comment prose at 80 columns while preserving ordinary sentence flow across continuation lines. Use internal punctuation between sentences and omit terminal punctuation. Follow [domlib helper documentation](../../../PROJECT.md#domlib-helper-documentation) for project terminology and intentional exceptions.

## Review difficult contracts

- Treat a helper that is difficult to explain in a concise contract comment as a design-review signal. Flag comments that need unrelated clauses, counterintuitive parameter positions, or implementation walkthroughs rather than broadening the comment.
- Review option-bearing helpers for options before operands. Across related helper families, review subject and object positions for consistency.
- Record refactor signals separately from rename signals. When both are within the resolved scope, resolve refactors before renaming. Do not force a refactor for an intentional overload documented in [domlib helper documentation](../../../PROJECT.md#domlib-helper-documentation).
