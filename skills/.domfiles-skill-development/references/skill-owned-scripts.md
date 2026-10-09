# Skill-Owned Scripts

A skill-owned script is a small program that gathers recurring repository evidence, produces declared artifacts, or does both for one project-authored skill. Use one when it makes the workflow more deterministic, efficient, or repeatable than a sequence of manual operations. Do not turn a narrow skill need into a general repository utility.

Apply the additional [portable skill script contract](portable-skill-scripts.md) before resolving an interface when the owning skill supports use across target projects or project policy designates the script as a portable command. A command’s portability does not change its owning skill’s installation category.

## Choose Contract Pass

For a script contract review or audit, inspect implementation and adjacent tests only as bounded evidence for a specific observable contract, then stop once the claim is established. Do not assess algorithms, internal structure, language idioms, performance, dead code, duplication, or general test quality unless the user explicitly includes implementation. Evaluation criteria such as security, maintainability, or project values apply within the resolved scope and do not expand it. Keep standalone reviews and audits read-only, following an applicable model-invocable project audit workflow when one exists. Otherwise, begin an audit with Git-tracked paths, add only explicitly named untracked material when local policy permits it, inspect the resolved scope, report findings, and stop without formatting or mutation.

When the resolved scope explicitly includes implementation, follow applicable project, domain, and language implementation and validation workflows for internal concerns. Keep the script contract pass focused on contract consequences, and update agent documentation only when the contract, routing, or documented invocation changes. Documentation work rejoins the shared `agent-documentation` lifecycle.

## Design the Smallest Sufficient Contract

The optimal contract is the least complex one that completely serves its named consumers within the declared operating model. For a new script or material expansion, follow [Script Contract Design](script-contract-design.md) before implementation, and use it as criteria when reviewing or auditing that design. A material expansion adds a dependency, durable artifact, input schema, mutation-authorizing decision, observable failure or status behavior, operation mode, or side effect. A fix reuses the accepted contract without reopening design when it only restores conformance to that contract and adds none of those material expansion elements.

Implementation and review verify conformance to the accepted contract. Reopen design only when new evidence invalidates an accepted assumption, using the same design procedure. A hypothetical case outside the declared operating model does not expand the contract.

## Define Observable Interface

Treat these elements as the script’s observable interface:

- Accepted arguments, defaults, documented environment inputs, required arguments, and target selection rules.
- Authorization requirements, credential sources, network access, and read and write effects.
- Compatibility guarantees, input manifests, machine-readable schemas, and produced artifacts.
- Executable and operation names.
- Exit status meanings and standard error and standard output behavior.

Document the complete interface in the owning skill at the decision that invokes the script. Keep command help consistent with that documentation, and cover the contract with adjacent tests.

Once documented, every interface element is compatibility-sensitive. A change is breaking when it:

- **Arguments:** Changes a default, documented environment input, or target selection rule, makes an optional argument required, or removes or renames an accepted argument.
- **Artifacts:** Alters an artifact destination, compatibility guarantee, input manifest schema, or machine-readable schema meaning.
- **Effects:** Expands or changes authorization requirements, credential sources, network access, or read or write effects.
- **Executables and operations:** Removes or renames an executable or operation.
- **Streams and statuses:** Changes an exit status meaning, machine-consumed output, standard error assignment, or standard output assignment.

A new optional argument or operation is compatible only when existing invocations retain their behavior. Human-facing wording may evolve without compatibility treatment unless the owning skill or an exact-string consumer declares it stable.

For a breaking change, update every repository-owned consumer, contract test, invocation, and migration requirement in the same authorized task. Internal functions and source filenames are not part of the observable interface unless a documented consumer or invocation addresses them directly.

## Choose Operation Route

- Use a read route to gather and report evidence without creating or updating repository artifacts.
- Use a write route to produce or update declared artifacts when the current task permits that mutation.
- Combine reading and writing in one bounded invocation when that avoids duplicate collection work.
- Let the agent choose the least expensive permitted route. Do not require a separate evidence pass before every write.

A script with both read and write modes must require an explicit write selection such as `--write` or `--output`. A writer-only generator does not need a redundant mode flag when its name and help text make the mutation clear.

Generating a declared artifact is not a repair. When evidence indicates that authoritative source, configuration, or documentation should change, the owning skill must define a separate repair branch and enter it only when the current request authorizes those changes.

## Keep Ownership Clear

- Store each script’s implementation and its adjacent contract test directly under `<skill>/scripts`. Store shared implementation helpers and their adjacent tests under `<skill>/scripts/helpers`. Do not create a per-script directory for a single script-and-test pair or put executable entrypoints in `helpers`. Follow the [filename contract](#resolve-filenames) for every pair. Place thin launchers at the repository’s prescribed command surface without relocating or duplicating the implementation.
- Let the skill own the source, tests, purpose, invocation, operation routes, artifact contract, and repair workflow.
- Let the repository root own toolchain configuration, dependencies and host language type packages, static validation, optional Cargo integration, and repository build output policy.
- Do not give the scripts directory or its `helpers` directory a separate package, crate, manifest, TypeScript configuration, lockfile, or workspace membership.

Build compiled scripts in their canonical repository’s build context. The runtime working directory follows the documented command contract and may differ from the build directory.

## Change Protected Scripts

Before changing a skill-owned script within a protected skill tree, follow the [protected skill mutation policy](protected-skill-mutation.md), which defines those trees and selects the applicable route. For a staged change, keep scripts, helpers, adjacent tests, and fixtures under `<staging>/editable/<skill>/scripts`, and promote only the reviewed staging unit. Scripts elsewhere under root `skills` use the ordinary direct edit workflow. Do not apply protected skill staging to them merely because `domfiles sync` exposes them through global symlinks.

## Make Interfaces Discoverable

- Support `--help` and describe the script’s purpose, modes, accepted inputs, output behavior, destination selection, overwrite policy, and relevant exit behavior.
- Treat source and configuration as the authority for implemented CLI behavior and serialized schemas. Treat adjacent contract tests as corroborating evidence, and treat `--help`, proposals, runtime argument diagnostics, and workflow documentation as projections. A stale test or projection never authorizes adding or broadening behavior without explicit user authorization.
- Before changing a CLI projection, inspect the exact implementation and relevant adjacent tests for every affected combination, mode, option, or schema. Keep help-only and documentation-only tasks behavior-neutral. When an authorized task changes behavior, align help, source and configuration, tests, and workflow documentation before treating the change as complete.
- Cover help-to-parser agreement with a contract test rather than review alone. Assert for each mode or standalone operation that the options documented for that route exactly match the options its parser accepts. Performing the alignment without a test leaves the next change free to reintroduce the drift.
- Keep each help option list alphabetized within its section so new options have a predictable position.

## Bound Execution and Output

- Aggregate the complete result while retaining only the bounded details that can be reported. State exact total and omitted counts without storing or emitting every failure body.
- Choose human-readable or structured output according to the consumer’s needs. Do not require JSON without a consumer that needs it.
- Do not invent an output filename in the current directory. Use a declared repository destination or require the caller to supply one.
- Keep each operation bounded and deterministic for the same repository state and inputs.
- When a named consumer requires repeated records, prefer one batch or suite invocation over repeated one-record processes. Do not introduce batch input without that consumer.
- When the accepted contract requires cooperating modes or scripts to share artifacts, define one explicit artifact graph and keep each artifact’s authority, integrity boundary, and mutation contract clear. Do not introduce a manifest solely to connect operations that can remain one invocation.

## Select Artifact Guidance

For scripts or tests with filesystem writes, including fixture setup and temporary outputs, follow [Script Artifact Boundaries](script-artifact-boundaries.md) before planning, reviewing, or performing those writes.

## Test Contracts

- Cover every distinct externally observable refusal and every routine that authorizes a mutation on a correctness claim, such as an accounting, containment, or equivalence proof. For a refusal shared by multiple external routes, keep the detailed refusal cases on the shared path and add one lightweight wiring assertion for each route proving that it reaches that path. Add route-specific detailed cases only when the route changes behavior or a caller relies on that distinction. Assert the refusal a caller would rely on, not only that the operation failed.
- Keep durable repository-owned fixture inputs narrow and deterministic under `<skill>/scripts`. Contain runtime-created fixture outputs, repositories, and scratch state through the [ephemeral artifact rule](script-artifact-boundaries.md#bound-artifact-locations).
- Run focused tests during implementation and after each behaviorally relevant correction. Run the repository’s root static validation once after the consolidated change batch, then rerun it only when a later correction changes an input or configuration that it covers. Direct execution and focused tests do not replace root typechecking or compilation.
- Test applicable read and write modes, destination resolution, overwrite refusal, unchanged output, cleanup, and failure behavior.

Tests may initialize declared disposable fixture repositories and populate their indexes only when the current task authorizes that setup and the caller has established isolated storage under the [ephemeral artifact rule](script-artifact-boundaries.md#bound-artifact-locations). Keep these writes inside the fixture repositories. Obtain any required sandbox grants separately. Tests that create commits, including fixture commits, still require explicit user authorization under the global **Commit gate**.

Document focused script and test commands in the owning skill or its repair reference.

## Choose Dependencies Before Implementation

Apply `intentional-dependency-choice` before choosing a bespoke implementation of a mature general-purpose capability. Such capabilities include cryptography and hashing, shell parsing, structured-data parsing and serialization, Unicode processing, URL handling, and other standards-heavy behavior. Reuse established decisions and approvals. The global **Dependencies** policy still governs implementation and mutating delegation.

## Integrate Root Validation

When a script reveals that root validation omits a source category, extend the root contract for that complete category rather than hardcoding one skill path. For example, add `.agents/**` to a TypeScript repository’s root include patterns when they do not cover skill-owned sources. Ensure the root check covers both the script and its test. Do not broaden configuration prospectively before a real script establishes the need.

For Rust scripts, follow [Rust Script Integration](rust-script-integration.md) to select the build route under the repository’s policy.

## Resolve Filenames

Give every script and adjacent test the same filename stem, adding only the resolved test suffix, and rename the pair together so their stems never diverge. Resolve the stem and suffix independently before considering language-native naming. For each component, use the first applicable rule in the following precedence order:

1. Preserve an explicit user-selected path or applicable project instruction for the current pair. Treat it as a broader convention only when the user or project policy says so.
2. Follow an existing script-and-test pair in the same skill unless it is documented as exceptional.
3. Treat a pattern shared by at least two project-authored skills as cross-skill precedent.
4. Derive the stem style from repository-owned standalone scripts and executable helpers. Derive test suffix placement from sidecar tests, including tests written in another language. Treat a single pair from another skill as supporting evidence rather than an automatic winner. When applicable patterns conflict, prefer files with the same role and closest scope, then ask only when equally applicable evidence remains unresolved.
5. When no repository pattern governs the pair, use the portable fallback `script-name.<extension>` and `script-name.test.<extension>`.

Treat skill-owned scripts as repository tooling rather than ordinary language modules. Do not infer language-native filenames merely from a manifest, package, or file extension. Use a language-native alternative only when existing repository files, explicit project policy, or a tooling constraint requires it. The presence of Cargo alone does not establish Rust-native filename conventions.
