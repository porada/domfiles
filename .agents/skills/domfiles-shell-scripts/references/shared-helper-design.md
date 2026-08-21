# Shared helper design

Apply this policy whenever the design or contract of a reusable `domlib` helper is in scope.

## Keep responsibilities reusable

- Keep entrypoints focused on command-specific bounds and output behavior.
- Define shared defaults in the helper that owns their behavior. Callers accepting a default must omit the corresponding argument instead of repeating its value.
- When a loop computes a reusable value, prefer a named helper that returns the result over a call-site loop with a trailing here-document redirect. Retain loop redirection when POSIX scope or failure handling requires it and extraction would not improve clarity.

## Document helper contracts

- For shared helpers whose contract is not obvious, incorporate non-obvious inputs, defaults, outputs, and fallback behavior into a concise natural-language comment. Do not use `@param` or `@returns` tags.
