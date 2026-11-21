# Documentation Comments

Use the full workflow for documentation comments and docstrings. For prose in ordinary source comments, follow [Punctuation Defaults](#punctuation-defaults), [Related Comments](#related-comments), [Line Breaks](#line-breaks), and [Validation](#validation). Runtime descriptions use the punctuation defaults.

## Scope

Let project policy and the established language-specific convention determine which declarations require documentation, which syntax, tags, or sections they use, and which wrapping and formatting constraints apply. Don’t generalize family-wide coverage or JSDoc syntax from one language or project.

For declaration-level documentation, follow the technical copy workflow’s [evidence requirements](technical-copy.md#evidence) and inspect each declaration’s call sites, declared types, and surrounding comments before composing or propagating wording. For a requested module or family pass, complete the in-scope coverage before reviewing the family as a whole.

## Punctuation Defaults

Use these defaults only when no project, language, or established local convention governs the choice.

Treat a comment or docstring as formal documentation when the language or its established tooling parses it as API documentation, such as JSDoc, TSDoc, PHPDoc, Javadoc, Python docstrings, or Rust doc comments. Write formal documentation comments, including tag descriptions, with terminal punctuation, even when the text is a single sentence, a verb-first summary, or a noun phrase.

Omit the final period from ordinary source comments and runtime descriptions, such as Fish `--description` text and completion labels, even when they contain several sentences. This also applies to function comments in languages without a formal documentation comment format, such as POSIX shell and Fish, unless established tooling parses them as API documentation.

When a host tool displays descriptions alongside its own, such as option help in a command-line interface, match the punctuation of the host’s descriptions.

## Comment Content

Across languages, begin each documentation comment or docstring with a tooltip-friendly purpose, significance, compatibility boundary, or non-obvious constraint rather than an implementation description. Prefer one concise sentence.

For promises and analogous asynchronous results, describe settlement or completion behavior when it states the purpose more directly. State distinct outcomes in separate sentences.

When no project, language, or established local convention governs callable documentation, begin with a third-person present tense verb.

Treat the surrounding symbol, type, and nearby API as context the reader already has, especially in an editor tooltip. Avoid repeating an obvious name, host tool, artifact, parameter explanation, or return type. A comment may share a verb with the function when it still adds purpose or significance beyond the identifier.

Mention a parameter in prose only when needed to state an observable consequence. Add dedicated parameter or return documentation only when the user explicitly requests it or when project policy, an established language-specific convention, or an established local convention requires it. Use the language’s established tags or sections.

Add scope, defaults, fallback, precedence, exceptions, or deprecation guidance only when the reader needs them to understand or use the declaration. A deprecation should name the replacement or required action.

For an option, state the shared behavior once, then describe what each materially different choice does. Omit qualifiers the option already implies.

> Wrap comments to the configured print width. `full-line` skips trailing comments. `all` moves them onto their own lines before wrapping.

When runtime descriptions for options and their choices are also in scope, edit them in a separate pass. Use the terms readers see in the interface and follow its [punctuation conventions](#punctuation-defaults).

## Abstraction

Preserve semantic precision even when a shorter or more familiar phrase sounds smoother. Distinctions such as current versus legacy, direct versus inherited, or shared versus owned may carry the reason a helper exists.

Under the technical copy workflow’s [composition guidance](technical-copy.md#composition), include implementation mechanics only when callers rely on them as part of the contract. Omit cache keys, object identity, restoration steps, filtering mechanics, and similar details that merely restate the body.

Describe the decision a helper makes or the result it produces, including exceptions its callers need to know. Explain why the behavior matters when its purpose is easy to miss, such as preventing recursion or preserving compatibility. Leave the full rules in the code and any specification that defines them. Use neighboring comments to judge how much detail callers need here.

If shortening the comment requires a material choice that project policy, evidence, and the user’s decisions leave open, explain which contract details are at stake. Ask the user which to keep so they can decide or consult the author.

## Related Comments

Compare related comments within each project and across closely related projects in scope, including comments whose line breaks need no change. For newer, unreleased code, reuse wording that already describes the same behavior well in released code. If the released comments you’re comparing differ, prefer wording known to have come from earlier consistency work.

For a requested family edit, choose one accurate comment for equivalent behavior. Ask before propagating wording only when project policy, evidence, and the user’s decisions leave a material choice open.

Use parallel grammar and the same level of detail for helpers with equivalent roles, adapting domain-specific terms as needed. Where helpers play different roles in the larger control flow, describe what each contributes. Keep comments that already read well and fit the family.

## Presentation

When project policy and established language conventions leave formatting unspecified, write JSDoc comments as multi-line blocks with `/**` and `*/` on separate lines, including one-sentence comments.

Review precise relationship words and qualifiers together with sentence flow. Resolve ambiguity without replacing a specific operation with a broader claim.

When documentation review exposes an unnecessarily long internal name, report the resulting wrapping constraint. Propose a focused rename and explain its value only when identifier design or source changes are explicitly in scope. Don’t rename source unless the current task authorizes that code change.

### Line Breaks

When editing comments, follow project wrapping policy. If the project has a formatter, run it on the in-scope files before judging line breaks. When no project or formatter owns wrapping, wrap prose at 80 columns, including indentation. When neither specifies tab width, count each leading tab as four columns.

Read the prose in line comments, block comments, documentation comments such as JSDoc, and docstrings at its actual indentation. Read both sides of each line break. At the end of each paragraph or tag description, fix breaks that strand a lone word or detach a short phrase from what it completes. When a line ends in “a” or “an”, including “A” and “An”, keep the article with the phrase it introduces.

Where manual wrapping is accepted, first try moving the line break. If the formatter controls wrapping or the break still reads awkwardly, add or cut words while keeping the sentence natural and its meaning precise.

When a formatter wraps this comment at 63 columns, a check for one-word lines misses the detached phrase.

```text
/**
 * Stops if the first blank line would cut another HTML block
 * in two.
 */
```

“Stops if the first blank line interrupts another HTML block.” expresses the same condition in one line at this width.

After editing, rerun the formatter if configured and read the complete comment again.

## Validation

After propagating wording, check the [line breaks](#line-breaks) at each destination. Where a local wording change is needed, make the smallest edit that reads naturally and preserves the shared meaning and terminology. Review the complete family together after formatting.
