# Prepare Post Content

## Gather Writing Context

Before authoring any contribution body, complete this sequence through the entrypoint’s resolved [evidence and writing workflows](../SKILL.md#compose-with-peers). Consume their existing peer choices rather than resolving them again:

1. Identify the required repository template or form for the selected pull request, issue, discussion, or private security report. Resolve a materially ambiguous template choice rather than inventing a requirement.
2. Look for a bounded, relevant sample of the user’s previous submissions of the same type in that repository. Use ordinary non-disclosing author filter operations or an author already established by the user, not direct authentication identity inspection. Do not search other repositories for writing examples.
3. [Select references](#select-references) from the [contribution assessment](../SKILL.md#assess-the-contribution). Supply the applicable template and useful prior examples to the selected writing workflow. Include verified contribution facts that establish the strongest motivation and any decisive limitation of existing alternatives. Supply the selected references with their relationships and state remaining evidence or validation gaps.

A successful lookup with no useful examples is different from a retrieval failure. In the former case, use the repository template without inventing an established style. For a retrieval failure, follow the evidence workflow’s boundary handling rather than treating the failed lookup as an empty history. An unknown template status permits a qualified draft from available facts, not a claim of template compliance. Ask the user when a known required template is unavailable or the choice materially changes the result.

For private security reports, use only legitimately available examples suitable for that disclosure. Do not request access to private disclosure history merely to match writing style, and do not move undisclosed report content into public posts or repository artifacts.

## Select References

Choose references for the relationship they establish, not to accumulate links. Verify that the relevant content supports the claim. Briefly identify how the contribution continues or corrects prior work. For a narrower contribution derived from a closed proposal, identify what it carries forward. Put supporting references beside the constraint or motivation they substantiate, unless the required template places them elsewhere. Select a particular comment rather than its parent thread when the decisive context lives there.

Use closing language only when full resolution or another intended reason for closure is established. Reference partial solutions or related context without implying closure. Default to `Fixes` for bug reports, `Resolves` for feature requests, and `Closes` for other issues, typically when they have been invalidated. These are preferences rather than a rigid taxonomy, so choose a more appropriate verb when the context warrants it. Keep `Fixes #<number>.` in its own paragraph by default, with its position determined by the body’s structure or required template.

Prefer `<owner>/<repository>#<number>` shorthand for unambiguous cross-repository GitHub issue and pull request references. Give that constraint to the writing peer when selected. Without it, use direct links for comments, discussions, and other destinations the shorthand cannot identify. Let references carry background rather than repeating titles or entire reports, while keeping the body’s purpose understandable. For a follow-up whose rationale is already established by the reference, state what this contribution completes or what now makes it appropriate without arguing the original case again. Do not default to a link-only body or add mandatory issue links or a references section beyond the repository’s requirements.

## Compose Content

The following procedure remains complete when the writing peer is absent. When it is available, supply these contribution-specific constraints without replacing its general editorial guidance.

Choose a clear, compact title that names the problem or intended outcome. Preserve the user’s meaning, voice, exact technical tokens, and settled wording. Keep claims proportional to verified evidence, distinguish inference from observation, and never invent a motivation or result to improve the prose.

Use supplied examples of the user’s previous GitHub submissions to identify recurring level of detail, structure, terminology, and tone. Preserve useful patterns without copying incidental wording or overriding the repository’s current template. Apply the mandated template and those patterns to the selected post.

Preserve the template’s checklist statements, field and section order, headings, and separators unless the user or template authorizes a change. Fill placeholders and remove authoring instructions rather than submitting them as prose. Omit optional sections only when permitted and appropriate. Checklist applicability determines completion under [finalization](#finalize-content), not permission to paraphrase the item.

Normalize the template’s ordinary prose under the applicable [typography conventions](typography.md). Preserve code, exact required wording, machine-readable markers, and other literal syntax wherever their spelling is part of the contract.

Write for a maintainer scanning with limited attention. Keep the body as short as it can be while satisfying the template and making the purpose, relevant context, and material limitations clear. Keep validation evidence accessible in the agent thread, including tool results, with task artifacts for supplementary detail or long logs. Do not reproduce that execution record in the PR. Remove repetition and unnecessary explanation, not required reproduction details or decisive evidence.

For every pull request body, explain the intended outcome and strongest verified reason to pursue it. When the limitation of an existing alternative is decisive, make that limitation clear. Never narrate the code changes or paraphrase the diff. Apply this rule within the mandated template rather than replacing its fields. If an explicit repository requirement cannot be satisfied without a change inventory, ask the user to resolve that conflict.

When the repository has no template, use these defaults:

- **Issues and discussions:** Keep useful in-repository writing patterns and use the shortest structure that establishes the purpose, decisive evidence, material limitations, and any requested action.
- **Pull requests:** Use one short prose paragraph outlining the purpose. Add a second succinct paragraph only when necessary. This limit governs the fallback, not mandated template fields.

Do not present a fallback as a repository requirement. Ask for missing context only when it materially prevents a useful draft.

For vulnerability reports and CVE preparation, use the writing peer’s security-report guidance when selected. Otherwise follow [Prepare Security Reports](prepare-security-reports.md) before drafting. Provide a title and body by default. Supply additional fields only when the user requests them, and flag any required form fields that still need their input.

## Describe Testing

Use a short, truthful reassurance about what was tested or not tested. State the relevant result and any limitation that materially affects confidence in the contribution. Do not turn Testing into a catalogue of irrelevant checks not run, command transcript, diagnostic report, or inventory of checks. Include such detail only when a repository requirement or the contribution’s reproducibility makes it necessary.

In an early draft, describe planned validation as planned. Before final delivery, replace plans with actual results. Do not imply that a check ran, passed, or covered behavior beyond the available evidence.

## Finalize Content

Review the title and body against the selected surface, current template, supplied examples, and verified outcome. Keep only useful optional sections, and do not add boilerplate beyond the repository’s requirements.

Resolve required checklist items before final delivery. Mark applicable items whose completion is established. Obtain the user’s confirmation for personal attestations that the agent cannot make on their behalf, and resolve unmet requirements or permitted not-applicable treatment rather than checking boxes speculatively. Do not present a body with unresolved required items as ready to submit.

Use blockquotes when discussing the draft. For the final copy-ready handoff, provide the title in its own plain-text code block and the complete body in a separate `markdown` code block, using a longer outer fence when the body contains code fences. Keep labels, publication instructions, and any thread-only validation detail outside those blocks. Return the complete content to the entrypoint’s [handoff](../SKILL.md#hand-back-the-contribution), not fragments the user must assemble.
