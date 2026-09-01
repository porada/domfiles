# Shared helper design

## Keep responsibilities reusable

- Keep entrypoints focused on command-specific bounds and output behavior.
- Define shared defaults in the helper that owns their behavior. Callers accepting a default must omit the corresponding argument instead of repeating its value.
- When a loop computes a reusable value, prefer a named helper that returns the result over a call-site loop with a trailing here-document redirect. Retain loop redirection when POSIX scope or failure handling requires it and extraction would not improve clarity.

## Align cross-shell helper contracts

- The POSIX helper named exactly `__` uses the established Fish counterpart `__domfiles_print_and_run`. The helper named exactly `__domfiles` has no automatic Fish counterpart name. For any other `domlib` helper, replace the leading `__` with `__domfiles_`, leaving names that already begin with `__domfiles_` unchanged.
- Treat paired POSIX and Fish helpers as peer implementations of a shared contract. Neither implementation governs the other.
- Keep their accepted inputs, `stdout` and `stderr` behavior, return statuses, side effects, defaults, failure conditions, validation timing, and adjacent contract documentation semantically aligned wherever both shells can support the same behavior.
- Use shell-native implementations without weakening the shared contract. A preferred idiom does not take precedence when it requires lossy normalization, including command substitution that changes significant whitespace or record boundaries. Allow a shell-specific contract difference when native behavior has no equivalent in the other shell or a settled project decision intentionally selects different behavior. Document each accepted difference and its rationale in [cross-shell helper differences](../../../PROJECT.md#cross-shell-helper-differences), and keep the remainder of the pair’s contract aligned.
- When either helper changes, inspect its counterpart. During an authorized change, update the implementations, contract documentation, and documented differences needed to keep the pair accurate. During a review, audit, or diagnosis, report required alignment without editing either helper or its documentation.

## Port a helper between shells

1. Inventory the source helper’s direct and transitive behavior before implementing its counterpart, including wrapper dispatch, command resolution, environment variables, failure conditions, suppression, and call sites.
2. Establish a bounded comparison across the shared contract surface defined in [Align cross-shell helper contracts](#align-cross-shell-helper-contracts).
3. Resolve each mismatch as aligned behavior or an accepted shell-specific difference before implementation. Do not infer that a transitive behavior should be retained or dropped.
4. When command form changes, follow [command form and location](command-form-and-location.md), then migrate call sites and contract documentation with the implementation.
5. Exercise both counterparts with the same bounded case matrix and compare their observable contracts before considering the port complete. When applicable, cover input-shape and content boundaries such as argument cardinality, empty values, letter-case variants, leading and trailing whitespace, and embedded and trailing newlines, plus interaction and outcome boundaries such as EOF, retries, and failure propagation.

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
