# Typography

Apply these conventions only when no narrower user, project, surface, language, or syntax rule governs the same choice.

## Prose

These rules apply to all prose, whether atomic or connected. Natural language in documentation, source comments, help output, diagnostics, test titles, and other human-facing strings counts as prose.

- **Quotation marks and apostrophes:** Use typographic “quotation marks” and apostrophes in prose. Preserve exact punctuation where literal syntax requires it.
- **Oxford commas:** In a list of three or more items, place a comma before the final conjunction.
- **Semicolons:** Never introduce semicolons in prose or human-facing technical copy. Preserve a supplied semicolon only when the user explicitly wants it retained.
- **Pause punctuation:** Limit dashes and other punctuation used to create a pause. Use a dash only when its additional pause or emphasis materially improves the reading unit. Never surround an em dash with spaces.

## Headings

Use title case, and keep peer headings grammatically parallel. Prefer equally clear, natural wording that avoids a word title case would lowercase. Keep the lowercased word when no alternative preserves the meaning or the user requires it.

## Technical Text

- **Documentation syntax:** Write named placeholders as `<lower-kebab-case>`. Use `…` only for omitted or repeatable content and ordinary ellipses. Preserve exact language, markup, regex, and quoted source syntax.
- **Code tokens:** Wrap identifiers, paths, commands, and quoted code tokens in backticks.
- **Commit references:** Write abbreviated commit hashes at 8 characters unless disambiguation or an external format requires the full object ID.
