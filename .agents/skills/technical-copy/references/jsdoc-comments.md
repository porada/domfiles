# JSDoc

Apply the entrypoint’s [standard path](../SKILL.md#use-the-standard-path) before these surface-specific rules.

## Resolve the documentation scope

- Let project policy and the established language-specific convention determine which declarations require documentation, which syntax, tags, or sections they use, and which wrapping and formatting constraints apply. Apply this reference to their wording and editorial review. Do not universalize family-wide coverage or JSDoc syntax from one language or project.
- In addition to the entrypoint evidence, inspect each declaration’s call sites, declared types, and surrounding comments before composing or propagating wording.
- For a requested module or family pass, complete the in-scope coverage before reviewing the family as a whole.

## Compose the comment

- For JSDoc, follow the global [multiline block requirement](../../../../.config/zed/AGENTS.md#writing). In every language, lead with a tooltip-friendly purpose, significance, compatibility boundary, or non-obvious constraint rather than narrating the implementation.
- Prefer one concise sentence. For promises and analogous asynchronous results, use settlement or completion behavior when it states the purpose more directly. State distinct outcomes as separate sentences.
- Treat the surrounding symbol, type, and nearby API as context already available to the reader, especially in an editor tooltip. Avoid repeating an obvious name, host tool, artifact, parameter explanation, or return type. A comment may share a verb with the function when it still adds purpose or significance beyond the identifier.
- Mention a parameter in prose only when needed to state an observable consequence. Add dedicated parameter or return documentation only when explicitly requested or required by project policy, an established language-specific convention, or an established local convention, using the language’s established tags or sections.
- Add scope, defaults, fallback, precedence, exceptions, or deprecation guidance only when readers need them to understand or use the declaration. A deprecation should name the replacement or required action.

## Choose the abstraction level

- Preserve semantic precision even when a shorter or more familiar phrase sounds smoother. Distinctions such as current versus legacy, direct versus inherited, or shared versus owned may carry the reason a helper exists.
- Under the entrypoint’s [implementation-mechanics rule](../SKILL.md#use-the-standard-path), treat mechanics as JSDoc-worthy only when callers rely on them as contract. Omit cache keys, object identity, restoration steps, filtering mechanics, and similar details that merely restate the body.
- Prefer the behavioral reason behind an operation when names and types already expose its mechanics. Explain compatibility, recursion prevention, delegation, or another meaningful consequence when that is the helper’s significance.
- Keep detail proportional to neighboring comments. A technically complete comment is still editorially wrong when it makes a small helper sound uniquely complicated.

## Propagate semantically

- For an explicitly requested family pass, select and propagate one evidence-backed canonical comment directly. Ask before propagation only when a subjective choice leaves material intent unresolved by project policy, evidence, or a user decision.
- Use parallel grammar and abstraction for genuinely parallel helpers, changing only the domain-specific terms. Do not force symmetry when responsibilities differ.
- Previously acceptable local wording may require revision when it becomes inconsistent beside the propagated family.
- Give related helpers their particular role in the larger control flow rather than interchangeable purpose comments.

## Review terminology and presentation

- Check precise relationship words and qualifiers together with sentence flow. Resolve ambiguity without replacing a specific operation with a broader claim.
- Follow applicable project wrapping policy. Account for a comment or docstring’s leading indentation when measuring line length and choosing a balanced wrap, so nested declarations use less prose width than top-level declarations. When neither project policy nor formatter configuration defines tab width, count each leading tab as four columns. When manual wrapping is accepted, avoid preventable one-word orphans even before a hard limit requires a break.
- Rephrase to improve a wrap only when meaning remains exact. Never remove a semantically important word or introduce an inaccurate synonym merely to shape the lines.
- When documentation review exposes an unnecessarily long internal name, propose the focused rename and explain its value. Do not rename source unless the current task authorizes that code change.

## Preserve and validate

- Apply the entrypoint’s [decision-preservation rule](../SKILL.md#preserve-decisions-without-adding-friction). Initial propagation and differing destination constraints may change wrapping only as required by each destination’s project policy, formatter, indentation, and available width.
- After project-required formatting, review the complete comment family again because line wrapping and adjacency affect the final result.
