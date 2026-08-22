# Typography and technical tokens

Within `human-facing-writing`, apply these conventions only when no narrower user, project, surface, language, or syntax rule governs the same decision.

The Typography, Semicolons, and Pause punctuation rules apply to all prose. For these rules, natural language in documentation, source comments, help output, diagnostics, test titles, and other human-facing strings counts as prose whether it is atomic or connected.

- **Typography:** Use typographic “quotation marks” and apostrophes in prose. Preserve exact punctuation where literal syntax requires it.
- **Semicolons:** Never introduce semicolons in prose or human-facing technical copy. Preserve a supplied semicolon only when the user explicitly wants it retained.
- **Pause punctuation:** Limit dashes and other punctuation used to create a pause. Use a dash only when its additional pause or emphasis materially improves the reading unit. Never surround em dashes with spaces.
- **Documentation syntax:** Write named placeholders as `<lower-kebab-case>`. Use `…` only for omitted or repeatable content and ordinary ellipses. Preserve exact language, markup, regex, and quoted source syntax.
- **Code tokens:** Wrap identifiers, paths, commands, and quoted code tokens in backticks.
- **Commit references:** Write abbreviated commit hashes at 8 characters by default. Use a full object ID only when disambiguation or an external format requires it.
