# Documentation boundary checks

Use these checks when an agent-documentation change crosses routed or layered documentation surfaces. Apply the entrypoint’s canonical-definition and secondary-role principle, then inspect only the applicable boundaries:

- **Description and body:** A skill body must not merely paraphrase its description’s trigger or exclusion.
- **Entrypoint and reference:** A reference must not merely paraphrase its parent entrypoint’s applicability or routing.
- **Generic and specialized workflows:** A specialized workflow must begin at its domain-specific divergence and route back to the generic lifecycle rather than repeating that lifecycle.
- **Global policy and skill:** A skill may name an always-loaded global policy but must not paraphrase it unless a distinct surface-specific application is required. For a public skill that may be installed without that policy, apply the [public skill portability contract](public-skill-portability.md) to any standalone mirror.
- **Output contract and template:** A template must not restate directives already established by its output contract.
