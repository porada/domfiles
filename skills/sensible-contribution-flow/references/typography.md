# Typography

Apply these conventions only when no narrower user, project, surface, language, or syntax rule governs the same choice. Observed punctuation in repository text or previous submissions does not override applicable writing instructions.

## Prose

These rules apply to all prose, from short labels to connected passages. Natural language in documentation, source comments, help output, diagnostics, test titles, and other human-facing strings counts as prose.

- **Quotation marks and apostrophes:** Use typographic “quotation marks” and apostrophes in prose. Preserve exact punctuation where literal syntax requires it.
- **Oxford commas:** In a list of three or more items, place a comma before the final conjunction.
- **Semicolons:** Never introduce semicolons in prose or human-facing technical copy. Preserve a supplied semicolon only when the user explicitly wants it retained.
- **Pause punctuation:** Limit dashes and other punctuation used to create a pause. Use a dash only when its additional pause or emphasis materially improves the text. Never surround an em dash with spaces.

## Hyphenation

These defaults apply to modifiers before nouns in all human-facing prose, including GitHub collaboration. [Prefixes](#prefixes) governs prefixed words. Other choices between hyphenated and closed spellings remain outside scope, as do predicative uses and verbs.

- **Noun phrases:** Keep normally open noun phrases open when they modify another noun. Their position before a noun is not, by itself, a reason to add a hyphen. Write `file pattern imports`, not `file-pattern imports`. Use a hyphen only to prevent a specific, plausible misreading.
- **Adverb modifiers:** Do not join `already` or an adverb ending in `-ly` to the adjective or participle it modifies with a hyphen. Write `already published articles`, `highly readable prose`, and `widely quoted passages`.
- **Other compound adjectives:** Otherwise retain conventional hyphenation, as in `best-known authors`, `fast-moving narratives`, `long-running columns`, `longest-running series`, and `well-defined terms`.

## Prefixes

These steps apply to prefixed words in all human-facing prose, including GitHub collaboration, in every position and part of speech. A word is prefixed when replacing the prefix with its plain meaning keeps the word’s meaning, as with `non-empty` (“not empty”) and `rerun` (“run again”), even when a dictionary lists another form. Words that fail this test keep their spelling, such as `none`, `nonsense`, `postfix`, `postpone`, `preflight`, and `prefix`. When the text discusses a specific feature or defined term, keep the spelling its product or specification uses, such as the CSS `pseudo-element` or the Git `pre-commit` hook. Ordinary vocabulary follows the steps, including when sources disagree. Reword when a prefix applies to a multi-word phrase, such as “pre-pull request checks”, or when it is unclear whether a word is prefixed, which step applies, or whether the result reads clearly. Otherwise, apply the first matching step:

1. Hyphenate `cross-`, `multi-`, `non-`, and `self-` when the rest of the word stands on its own, as in `multi-line` and `non-overlapping`.
2. Hyphenate `pre-` and `post-` when the rest of the word is a noun and the word means “before” or “after” that noun, as in `pre-implementation` and `post-review`.
3. Hyphenate a prefix ending in a vowel when the rest of the word stands on its own and starts with the same vowel, as in `pre-existing`, `re-enable`, and `semi-interactive`, except for `co-`. Also hyphenate `re-` when the rest of the word starts with `re`, as in `re-review`.
4. Hyphenate any prefix before a capitalized word, an initialism or acronym, or a numeral. Clipped words such as `subdir` follow the other steps.
5. Before closing a word, reword when no established meaning of the closed form matches the intended one. For example, to say “sourcing again”, write that phrase rather than `resourcing`, which means “supplying resources”.
6. Otherwise close the word, as in `rerun`, `preload`, `subcommand`, and `colocate`.

## Headings

Use title case, and keep peer headings grammatically parallel. Prefer equally clear, natural wording that avoids a word title case would lowercase. Keep the lowercased word when no alternative preserves the meaning or the user requires it. Apply these conventions to prose table headers, preserving literal field names.

## Links

Write link text to read naturally within the sentence, even when it uses the same words as a document or section title. When naming a document or section by its title, keep the title’s exact wording and capitalization.

## Technical Text

- **Documentation syntax:** Write named placeholders as `<lower-kebab-case>`. Use `…` only for omitted or repeatable content and ordinary ellipses. Preserve exact language, markup, regex, and quoted source syntax.
- **Code tokens:** Wrap identifiers, paths, commands, and quoted code tokens in backticks.
- **Commit references:** Write abbreviated commit hashes at 8 characters by default. Use a full object ID only when disambiguation or an external format requires it.
