# Typography

Use these conventions only when no narrower user, project, surface, language, or syntax rule governs the same choice.

## Prose

These rules apply to all prose, whether atomic or connected. Prose includes natural language in documentation, source comments, help output, diagnostics, test titles, and other human-facing strings.

- **Quotation marks and apostrophes:** Use typographic “quotation marks” and apostrophes. Preserve exact punctuation where literal syntax requires it.
- **Oxford commas:** In a list of three or more items, place a comma before the final conjunction.
- **Semicolons:** Never introduce semicolons in prose or human-facing technical copy. Preserve a supplied semicolon only when the user explicitly wants it retained.
- **Pause punctuation:** Use dashes and other pause punctuation sparingly. Add a dash only when its pause or emphasis materially improves the reading unit. Never put spaces around an em dash.

## Headings and Technical Text

- **Headings:** Use title case. Prefer equally clear, natural wording that avoids a word title case would lowercase. Keep the lowercased word when no alternative preserves the meaning or the user requires it.
- **Documentation syntax:** Write named placeholders as `<lower-kebab-case>`. Use `…` only for omitted or repeatable content and ordinary ellipses. Preserve exact language, markup, regex, and quoted source syntax.
- **Code tokens:** Wrap identifiers, paths, commands, and quoted code tokens in backticks.
- **Commit references:** Write abbreviated commit hashes at 8 characters unless disambiguation or an external format requires the full object ID.
