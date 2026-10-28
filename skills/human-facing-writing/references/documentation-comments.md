# Documentation Comments

Use technical copy evidence and reader-facing terminology to shape declaration-level documentation and choose punctuation for source comments and runtime descriptions.

## Scope

Let project policy and the established language-specific convention determine which declarations require documentation, which syntax, tags, or sections they use, and which wrapping and formatting constraints apply. This reference governs declaration-level wording and editorial review, plus punctuation defaults for ordinary source comments and runtime descriptions. Don’t generalize family-wide coverage or JSDoc syntax from one language or project.

For declaration-level documentation, follow the technical copy workflow’s [evidence requirements](technical-copy.md#evidence) and inspect each declaration’s call sites, declared types, and surrounding comments before composing or propagating wording. For a requested module or family pass, complete the in-scope coverage before reviewing the family as a whole.

## Punctuation Defaults

Use these defaults only when no project, language, or established local convention governs the choice.

Treat a comment or docstring as formal documentation when the language or its established tooling parses it as API documentation, such as JSDoc, TSDoc, PHPDoc, Javadoc, Python docstrings, or Rust doc comments. Write formal documentation comments, including tag descriptions, with terminal punctuation, even when the text is a single sentence, a verb-first summary, or a noun phrase.

Omit the final period from ordinary source comments and runtime descriptions, such as Fish `--description` text and completion labels, even when they contain several sentences. This also applies to function comments in languages without a formal documentation comment format, such as POSIX shell and Fish, unless established tooling parses them as API documentation.

When a host tool displays descriptions alongside its own, such as option help in a command-line interface, match the punctuation of the host’s descriptions.

## Comment Content

When project policy and established language conventions leave formatting unspecified, write JSDoc comments as multi-line blocks with `/**` and `*/` on separate lines, including one-sentence comments. Across languages, begin each documentation comment or docstring with a tooltip-friendly purpose, significance, compatibility boundary, or non-obvious constraint rather than an implementation description. Prefer one concise sentence.

For promises and analogous asynchronous results, describe settlement or completion behavior when it states the purpose more directly. State distinct outcomes in separate sentences.

When no project, language, or established local convention governs callable documentation, begin with a third-person present tense verb.

Treat the surrounding symbol, type, and nearby API as context the reader already has, especially in an editor tooltip. Avoid repeating an obvious name, host tool, artifact, parameter explanation, or return type. A comment may share a verb with the function when it still adds purpose or significance beyond the identifier.

Mention a parameter in prose only when needed to state an observable consequence. Add dedicated parameter or return documentation only when the user explicitly requests it or when project policy, an established language-specific convention, or an established local convention requires it. Use the language’s established tags or sections.

Add scope, defaults, fallback, precedence, exceptions, or deprecation guidance only when the reader needs them to understand or use the declaration. A deprecation should name the replacement or required action.

## Abstraction

Preserve semantic precision even when a shorter or more familiar phrase sounds smoother. Distinctions such as current versus legacy, direct versus inherited, or shared versus owned may carry the reason a helper exists.

Under the technical copy workflow’s [composition guidance](technical-copy.md#composition), include implementation mechanics only when callers rely on them as part of the contract. Omit cache keys, object identity, restoration steps, filtering mechanics, and similar details that merely restate the body.

Prefer the behavioral reason for an operation when names and types already expose its mechanics. Explain compatibility, recursion prevention, delegation, or another meaningful consequence when that’s the helper’s significance. Keep detail proportional to neighboring comments. A technically complete comment is still editorially wrong when it makes a small helper sound uniquely complicated.

## Related Comments

For an explicitly requested family pass, select one evidence-backed canonical comment and propagate it directly. Ask before propagation only when a subjective choice leaves material intent unresolved by project policy, evidence, or a user decision.

Use parallel grammar and abstraction for genuinely parallel helpers, changing only the domain-specific terms. Don’t force symmetry when responsibilities differ.

Previously acceptable local wording may require revision when it becomes inconsistent beside the propagated family. Describe each related helper’s particular role in the larger control flow rather than giving it an interchangeable purpose comment.

## Presentation

Review precise relationship words and qualifiers together with sentence flow. Resolve ambiguity without replacing a specific operation with a broader claim.

Follow applicable project wrapping policy. When no project or formatter owns wrapping, wrap prose at 80 columns, including indentation. Account for a comment or docstring’s leading indentation when measuring line length and choosing a balanced wrap, so nested declarations use less prose width than top-level declarations.

When neither project policy nor formatter configuration defines tab width, count each leading tab as four columns. When manual wrapping is accepted, avoid preventable one-word orphans even before a hard limit requires a break.

Rephrase to improve a wrap only when the meaning remains exact. Never remove a semantically important word or introduce an inaccurate synonym merely to shape the lines.

When documentation review exposes an unnecessarily long internal name, report the resulting wrapping constraint. Propose a focused rename and explain its value only when identifier design or source changes are explicitly in scope. Don’t rename source unless the current task authorizes that code change.

## Validation

When propagating a comment, change its wrapping only as required by each destination’s project policy, formatter, indentation, or available width. After project-required formatting, review the complete comment family again because line wrapping and adjacency affect the final result.
