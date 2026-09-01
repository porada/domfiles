# Skill-owned scripts

A skill-owned script is a small program that gathers recurring repository evidence, produces declared artifacts, or does both for one project-authored skill. Use one when it makes the workflow more deterministic, efficient, or repeatable than a sequence of manual operations. Do not turn a narrow skill need into a general repository utility.

When a script belongs to a portable skill—one installed for use across target projects rather than scoped to one repository—apply the additional [portable skill script contract](portable-skill-scripts.md) before resolving its interface.

## Design the smallest sufficient contract

The optimal contract is the least complex one that completely serves its named consumers within the declared operating model. Apply this gate before implementing a new script or materially expanding an existing script. A material expansion adds a dependency, durable artifact, input schema, mutation-authorizing decision, observable failure or status behavior, operation mode, or side effect. A fix reuses the accepted contract without reopening design when it only restores conformance to that contract and adds none of those material-expansion elements.

1. **Establish necessity.** Name the recurring consumer and the single job the script must perform. First attempt to remove the script, use an existing repository workflow, or use a bounded direct tool sequence. Do not create a script for a one-time transition or merely to encode review preferences.
2. **Draft only the observable contract.** Define its authority, concurrency and threat model, failure boundaries, inputs, non-goals, outputs, side effects, and statuses. Do not select dependencies or internal architecture yet.
3. **Run one bounded adversarial design pass.** Challenge necessity before correctness. First try to delete the script or each retained contract element. Then test the remaining contract against boundary values, concurrency inside the declared model, malformed inputs, output failures, and partial operations. Derive a requirement only from a ground allowed by the global “Proportionality” rule or from authoritative behavior, a named recurring consumer, or a script-specific standing policy. The pass must identify the smallest viable alternative and must not invent future consumers, threats, or use cases.
4. **Choose the smallest sufficient design.** Proceed only when no simpler design satisfies the established requirements. If the adversarial pass turns a small helper into a general framework, protocol, or transactional system, stop and return to the direct workflow or narrow the requirement before implementation.
5. **Freeze the accepted contract for implementation.** Implementation and review verify conformance to that contract. Reopen design only when new evidence invalidates an accepted assumption. A hypothetical case outside the declared operating model does not expand the contract.

Use one adversarial pass and, after any revision, one focused check of the changed contract. Do not begin an open-ended design-review loop. Ask the user only when two materially different designs remain viable. Otherwise choose the smallest reversible design autonomously, while following every standing approval gate.

Keep rejected alternatives and adversarial notes in task context. Document only the accepted observable contract and non-obvious rationale.

## Choose the operation route

- Use a read route to gather and report evidence without creating or updating repository artifacts.
- Use a write route to produce or update declared artifacts when the current task permits that mutation.
- Combine reading and writing in one bounded invocation when that avoids duplicate collection work.
- Let the agent choose the least expensive permitted route. Do not require a separate evidence pass before every write.

A script with both read and write modes must require an explicit write selection such as `--write` or `--output`. A writer-only generator does not need a redundant mode flag when its name and help text make the mutation clear.

Generating a declared artifact is not a repair. When evidence indicates that authoritative source, configuration, or documentation should change, the owning skill must define a separate repair branch and enter it only when the current request authorizes those changes.

## Keep ownership clear

- Store each executable script and its adjacent contract test directly under `<skill>/scripts`. Store shared implementation helpers and their adjacent tests under `<skill>/scripts/helpers`. Do not create a per-script directory for a single script-and-test pair or put executable entrypoints in `helpers`. Follow the [filename contract](#resolve-file-names) for every pair.
- Let the skill own the source, tests, purpose, invocation, operation routes, artifact contract, and repair workflow.
- Let the repository root own toolchain configuration, dependencies and host-language type packages, static validation, optional Cargo integration, and repository build-output policy.
- Do not give the scripts directory or its `helpers` directory a separate package, crate, manifest, TypeScript configuration, lockfile, or workspace membership.

Root ownership assumes the script runs inside its canonical repository. A skill installed for use outside that repository keeps that ownership by running from its host rather than from the installed path.

## Change protected scripts

Before changing a skill-owned script within a protected skill tree, follow the [protected skill mutation policy](protected-skill-mutation.md), which defines those trees and selects the applicable route. For a staged change, keep scripts, helpers, adjacent tests, and fixtures under `<staging>/editable/<skill>/scripts`, and promote only the reviewed staging unit. Scripts elsewhere under root `skills` use the ordinary direct-edit workflow. Do not apply protected-skill staging to them merely because `domfiles sync` exposes them through global symlinks.

## Apply human-facing copy policy

- Treat every project-authored string that a skill-owned script or its adjacent tests can present to a person—including CLI help, runtime output, prompts, generated human-readable artifacts, test titles, and failure-only assertion diagnostics—as human-facing implementation copy rather than agent documentation. Apply the `human-facing-writing` skill when implementation, review, or maintenance creates, changes, or evaluates that wording.
- Keep related human-facing terminology and adjacent exact-string tests aligned in the same task. Agent-documentation policy owns script behavior, interfaces, schemas, and machine contracts, while `human-facing-writing`’s technical-copy workflow owns human-facing implementation copy.
- Do not rewrite machine-readable protocol records, serialized field names, exact syntax tokens, fixture payloads, or preserved upstream text merely for style. Review any surrounding project-authored implementation copy through `human-facing-writing`.

## Make the interface discoverable

- Support `--help` and describe the script’s purpose, modes, accepted inputs, output behavior, destination selection, overwrite policy, and relevant exit behavior.
- Treat source and configuration as the authority for implemented CLI behavior and serialized schemas. Treat adjacent contract tests as corroborating evidence, and treat `--help`, proposals, runtime argument diagnostics, and workflow documentation as projections. A stale test or projection never authorizes adding or broadening behavior without explicit user authorization.
- Before changing a CLI projection, inspect the exact implementation and relevant adjacent tests for every affected combination, mode, option, or schema. Keep help-only and documentation-only tasks behavior-neutral. When an authorized task changes behavior, align help, source and configuration, tests, and workflow documentation before treating the change as complete.
- Cover help-to-parser agreement with a contract test rather than review alone. Assert for each mode or standalone operation that the options documented for that route exactly match the options its parser accepts. Performing the alignment without a test leaves the next change free to reintroduce the drift.
- Keep each help option list alphabetized within its section. Ordering there is order-independent, so a reviewer cannot distinguish a deliberate order from drift.
- Keep each operation bounded and deterministic for the same repository state and inputs.
- When a named consumer requires repeated records, prefer one batch or suite invocation over repeated one-record processes. Do not introduce batch input without that consumer.
- When the accepted contract requires cooperating modes or scripts to share artifacts, define one explicit artifact graph and keep each artifact’s authority, integrity boundary, and mutation contract clear. Do not introduce a manifest solely to connect operations that can remain one invocation.
- Aggregate the complete result while retaining only the bounded details that can be reported. State exact total and omitted counts without storing or emitting every failure body.
- Choose human-readable or structured output according to the consumer’s needs. Do not require JSON without a consumer that needs it.
- Do not invent an output filename in the current directory. Use a declared repository destination or require the caller to supply one.

## Bound artifact locations

Every filesystem write must remain within one of these authorized categories:

- A declared repository artifact at its canonical path, whether intentionally tracked or stored in an established repository-owned output location.
- An explicit file or directory supplied by the caller.
- A short-lived sibling used exclusively to atomically replace an otherwise authorized destination.

Do not repurpose a location merely because it is ignored. Never modify `.gitignore` while running the script, and do not add an ignore rule solely to accommodate script-specific output. If repository-wide toolchain output exposes a missing ignore policy, handle that as a separate repository configuration change under the current task’s authorization.

For ephemeral artifacts, follow the global “Temporary files” policy. Accept the resolved task-specific destination from the caller instead of establishing a separate temporary-output convention.

Before writing:

- Confirm that the resolved destination remains within the authorized location. Reject traversal or symlink redirection outside it.
- Do not write Git metadata or files unrelated to the declared artifact contract.
- Apply the global “Concurrent work” preservation rule to repository destinations.
- Replace an existing path only when it is a declared generated artifact or the current request explicitly authorizes overwriting it.
- Leave byte-identical output unchanged.

## Write artifacts safely

- When atomic replacement requires a same-filesystem temporary file, use a short-lived sibling as an internal implementation detail and remove it after success or a handled failure.
- Remove stale paths only when they belong to a declared script-owned output set and exact synchronization is part of the documented contract.
- Do not provide a generic `--force` or `--fix` escape hatch that bypasses ownership or destination checks.
- Report every artifact created, updated, unchanged, or removed.

## Test the contracts

- Test applicable read and write modes, destination resolution, overwrite refusal, unchanged output, cleanup, and failure behavior.
- Cover every distinct externally observable refusal and every routine that authorizes a mutation on a correctness claim, such as an accounting, containment, or equivalence proof. For a refusal shared by multiple external routes, keep the detailed refusal cases on the shared path and add one lightweight wiring assertion for each route proving that it reaches that path. Add route-specific detailed cases only when the route changes behavior or a caller relies on that distinction. Assert the refusal a caller would rely on, not only that the operation failed.
- Keep durable repository-owned fixture inputs narrow and deterministic under `<skill>/scripts`. Contain runtime-created fixture outputs, repositories, and scratch state through the [ephemeral-artifact rule](#bound-artifact-locations).
- Run focused tests during implementation and after each behaviorally relevant correction. Run the repository’s root static validation once after the consolidated change batch, then rerun it only when a later correction changes an input or configuration that it covers. Direct execution and focused tests do not replace root typechecking or compilation.

Document focused script and test commands in the owning skill or its repair reference.

## Choose dependencies before implementation

- Prefer an existing repository dependency or standard-library capability when it directly provides the required behavior and keeps the implementation small, complete, and maintainable.
- Do not reimplement a mature general-purpose capability merely to avoid adding a dependency or requesting approval. This includes cryptography and hashing, shell parsing, structured-data parsing and serialization, Unicode processing, URL handling, and other standards-heavy behavior.
- When the best implementation requires a dependency change whose addition or update needs approval, follow the approval gate in the global “Dependencies” policy before implementation or mutating delegation. Before asking for approval, tell the user:
    - Which dependency or smallest dependency set is proposed
    - What each dependency provides
    - Where it will be declared and which code will use it
    - Why existing dependencies or the standard library are insufficient
    - Why a custom implementation would be less correct, maintainable, proportionate, or secure
    - Any material feature, licensing, runtime, supply-chain, or version implications
- If approval is declined, propose the strongest constrained alternative and explain its limitations.
- Enable only the dependency features required by the approved design.

## Integrate root validation

When a script reveals that root validation omits a source category, extend the root contract for that complete category rather than hardcoding one skill path. For example, add `.agents/**` to a TypeScript repository’s root include patterns when they do not cover skill-owned sources. Ensure the root check covers both the script and its test. Do not broaden configuration prospectively before a real script establishes the need.

For standard-library-only Rust scripts:

- Compile the script directly with stable `rustc` and compile its adjacent test with `rustc --test`.
- Pass the repository’s supported Rust edition explicitly. Pass an explicit valid crate name when the repository’s filename pattern contains characters that Rust crate names do not accept.
- Resolve compiled binaries and other transient output through the [ephemeral-artifact rule](#bound-artifact-locations).
- Include both compile commands in root static validation. Do not register Cargo targets merely because the repository otherwise uses Cargo.

When a Rust script requires a non-standard-library dependency:

- Use Cargo through an existing repository-owned tooling package or, when the current task authorizes it, one shared tooling package for skill-owned scripts.
- Let a root package own the targets in a package-root workspace. In a virtual workspace, let a package member own them because the virtual manifest cannot define targets.
- Use repository-unique, skill-qualified target names and the repository’s normal shared Cargo target directory. Do not override `CARGO_TARGET_DIR` merely to isolate a script.
- Keep target registration and applicable root manifests committed, follow the repository’s lockfile policy, and keep generated `target/` contents ignored and uncommitted.

## Resolve file names

Give every script and adjacent test the same filename stem, adding only the resolved test suffix, and rename the pair together so their stems never diverge. Resolve the stem and suffix independently before considering language-native naming:

1. Preserve an explicit user-selected path or applicable project instruction for the current pair. Treat it as a broader convention only when the user or project policy says so.
2. Follow an existing script-and-test pair in the same skill unless it is documented as exceptional.
3. Treat a pattern shared by at least two project-authored skills as cross-skill precedent.
4. Derive the stem style from repository-owned standalone scripts and executable helpers. Derive test-suffix placement from sidecar tests, including tests written in another language. Treat a single pair from another skill as supporting evidence rather than an automatic winner. When applicable patterns conflict, prefer files with the same role and closest scope, then ask only when equally applicable evidence remains unresolved.
5. When no repository pattern governs the pair, use the portable fallback `script-name.<extension>` and `script-name.test.<extension>`.

Treat skill-owned scripts as repository tooling rather than ordinary language modules. Do not infer language-native filenames merely from a manifest, package, or file extension. Use a language-native alternative only when existing repository files, explicit project policy, or a tooling constraint requires it. The presence of Cargo alone does not establish Rust-native filename conventions.
