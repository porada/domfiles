# Prepare Post Content

## Gather the Writing Context

Before authoring any contribution body, complete this sequence through `simple-github-cli` where remote context is needed:

1. Identify the required repository template or form for the selected pull request, issue, discussion, or private security report. Resolve a materially ambiguous template choice rather than inventing a requirement.
2. Look for a bounded, relevant sample of the user’s previous submissions of the same type in that repository. Use ordinary non-disclosing author-filter operations or an author already established by the user, not direct authentication-identity inspection. Do not search other repositories for writing examples.
3. [Select references](#select-references) from the [contribution assessment](../SKILL.md#assess-the-contribution). Pass the applicable template and useful prior examples to `human-facing-writing`. Include verified contribution facts that establish the strongest motivation and any decisive limitation of existing alternatives. Supply the selected references with their relationships and state remaining evidence or validation gaps.

A successful lookup with no useful examples is different from a retrieval failure. In the former case, use the repository template and `human-facing-writing` without inventing an established style. For a retrieval failure, follow `simple-github-cli`’s boundary handling rather than treating the failed lookup as an empty history.

For private security reports, use only legitimately available examples suitable for that disclosure. Do not request access to private disclosure history merely to match writing style, and do not move undisclosed report content into public posts or repository artifacts.

## Select References

Choose references for the relationship they establish, not to accumulate links. Verify that the relevant content supports the claim. Briefly identify how the contribution continues or corrects prior work. For a narrower contribution derived from a closed proposal, identify what it carries forward. Put supporting references beside the constraint or motivation they substantiate, unless the required template places them elsewhere. Select a particular comment rather than its parent thread when the decisive context lives there.

Use closing language only when full resolution or another intended reason for closure is established. Reference partial solutions or related context without implying closure. Default to `Fixes` for bug reports, `Resolves` for feature requests, and `Closes` for other issues, typically when they have been invalidated. These are preferences rather than a rigid taxonomy, so choose a more appropriate verb when the context warrants it. Keep `Fixes #<number>.` in its own paragraph by default, with its position determined by the body’s structure or required template.

Supply `human-facing-writing` with the user’s preference for `<owner>/<repository>#<number>` shorthand for unambiguous cross-repository GitHub issue and pull-request references. Leave other reference formatting to that peer. Let references carry background rather than repeating titles or entire reports, while keeping the body’s purpose understandable. Do not default to a link-only body or add mandatory issue links or a references section beyond the repository’s requirements.

## Compose the Content

Use supplied examples of the user’s previous GitHub submissions to identify recurring level of detail, structure, terminology, and tone. Preserve useful patterns without copying incidental wording or overriding the repository’s current template. Apply the mandated template and those patterns through `human-facing-writing`.

Normalize the template’s ordinary prose to the applicable typography conventions through `human-facing-writing`. Preserve code, exact required wording, machine-readable markers, and other literal syntax wherever their spelling is part of the contract.

Keep the body as short as it can be while satisfying the template and giving maintainers enough evidence to understand and act. Remove repetition and unnecessary explanation rather than required context, reproduction details, or material limitations.

For every pull-request body, explain the intended outcome and strongest verified reason to pursue it. When the limitation of an existing alternative is decisive, make that limitation clear. Never narrate the code changes or paraphrase the diff. Apply this rule within the mandated template rather than replacing its fields. If an explicit repository requirement cannot be satisfied without a change inventory, ask the user to resolve that conflict.

When the repository has no template, use these defaults:

- **Issues and discussions:** Keep useful in-repository writing patterns and let `human-facing-writing` choose the shortest appropriate structure.
- **Pull requests:** Use one short prose paragraph outlining the purpose. Add a second succinct paragraph only when necessary. This limit governs the fallback, not mandated template fields.

Do not present a fallback as a repository requirement. Ask for missing context only when it materially prevents a useful draft.

For vulnerability reports and CVE preparation, provide a title and body by default. Supply additional fields only when the user requests them, and flag any required form fields that still need their input.

Review the completed title and body against the selected surface, current template, supplied examples, and verified outcome before returning them to the entrypoint’s [handoff](../SKILL.md#hand-back-the-contribution).
