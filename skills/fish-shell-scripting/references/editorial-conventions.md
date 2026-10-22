# Editorial Conventions

Apply these conventions only when no narrower user, project, surface, language, or syntax rule governs the same choice. Punctuation you see in repository text or previous submissions doesn’t override applicable writing instructions.

## Prose

These rules apply to all prose, from short labels to connected passages. Natural language in documentation, source comments, help output, diagnostics, test titles, and other human-facing strings counts as prose.

- **Quotation marks and apostrophes:** Use typographic “quotation marks” and apostrophes in prose. Preserve exact punctuation where literal syntax requires it.
- **Oxford commas:** In a list of three or more items, place a comma before the final conjunction.
- **Semicolons:** Never introduce semicolons in prose or human-facing technical copy. Preserve a supplied semicolon only when the user explicitly wants it retained.
- **Pause punctuation:** Limit dashes and other punctuation used to create a pause. Use a dash only when the added pause or emphasis materially improves the text. Never put spaces around an em dash.

## Hyphenation

These defaults apply to modifiers before nouns in all human-facing prose, including GitHub collaboration. [Prefixes](#prefixes) governs prefixed words. Other choices between hyphenated and closed spellings, predicative uses, and verbs are outside this section’s scope.

- **Noun phrases:** Keep normally open noun phrases open when they modify another noun. Their position before a noun isn’t, by itself, a reason to add a hyphen. Write `file pattern imports`, not `file-pattern imports`. Use a hyphen only to prevent a specific, plausible misreading.
- **Adverb modifiers:** Don’t use a hyphen to join `already` or an adverb ending in `-ly` to the adjective or participle it modifies. Write `already published articles`, `highly readable prose`, and `widely quoted passages`.
- **Other compound adjectives:** Otherwise retain conventional hyphenation, as in `best-known authors`, `fast-moving narratives`, `long-running columns`, `longest-running series`, and `well-defined terms`.

## Prefixes

These steps apply to prefixed words in all human-facing prose, including GitHub collaboration, in every position and part of speech. Treat a word as prefixed when you can replace its prefix with the prefix’s plain meaning without changing the word’s meaning, as with `non-empty` (“not empty”) and `rerun` (“run again”). This test applies even when a dictionary lists another form. Words that fail it keep their spelling, such as `none`, `nonsense`, `postfix`, `postpone`, `preflight`, and `prefix`.

When discussing a specific feature or defined term, keep the spelling its product or specification uses, such as the CSS `pseudo-element` or the Git `pre-commit` hook. For ordinary vocabulary, follow the steps even when sources disagree.

Reword when a prefix applies to a multi-word phrase, such as “pre-pull request checks”. Also reword if you can’t tell whether a word is prefixed, which step applies, or whether the result reads clearly. Otherwise, apply the first matching step:

1. Hyphenate `cross-`, `multi-`, `non-`, and `self-` when the rest of the word stands on its own, as in `multi-line` and `non-overlapping`.
2. Hyphenate `pre-` and `post-` when the rest of the word is a noun and the word means “before” or “after” that noun, as in `pre-implementation` and `post-review`.
3. Hyphenate a prefix ending in a vowel when the rest of the word stands on its own and starts with the same vowel, as in `pre-existing`, `re-enable`, and `semi-interactive`, except for `co-`. Also hyphenate `re-` when the rest of the word starts with `re`, as in `re-review`.
4. Hyphenate any prefix before a capitalized word, an initialism or acronym, or a numeral. Clipped words such as `subdir` follow the other steps.
5. Before closing a word, reword when no established meaning of the closed form matches the intended one. For example, to say “sourcing again”, write that phrase rather than `resourcing`, which means “supplying resources”.
6. Otherwise close the word, as in `rerun`, `preload`, `subcommand`, and `colocate`.

## Headings

Use title case, and keep peer headings grammatically parallel. Prefer equally clear, natural wording that avoids a word that title case would lowercase. Keep the lowercased word when no alternative preserves the meaning or the user requires it. Apply these conventions to prose table headers, and preserve literal field names.

## Links

Write link text that reads naturally within the sentence, even when it uses the same words as a document or section title. When naming a document or section by its title, keep the title’s exact wording and capitalization.

## Technical Text

- **Documentation syntax:** Write named placeholders as `<lower-kebab-case>`. Use `…` only to indicate omitted or repeatable content, or for ordinary ellipses. Preserve exact language, markup, regex, and quoted source syntax.
- **Code tokens:** Wrap identifiers, paths, commands, and quoted code tokens in backticks.
- **Commit references:** Use 8 characters for abbreviated commit hashes by default. Use a full object ID only when disambiguation or an external format requires it.
