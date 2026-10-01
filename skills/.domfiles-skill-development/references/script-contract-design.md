# Script Contract Design

1. **Establish necessity.** Name the recurring consumer and the single job the script must perform. First attempt to remove the script, use an existing repository workflow, or use a bounded direct tool sequence. Do not create a script for a one-time transition or merely to encode review preferences.
2. **Draft only the observable contract.** Define its authority, concurrency and threat model, failure boundaries, inputs, non-goals, outputs, side effects, and statuses. Do not select dependencies or internal architecture yet.
3. **Run one bounded adversarial design pass.** Challenge necessity before correctness. First try to delete the script or each retained contract element. Then test the remaining contract against boundary values, concurrency inside the declared model, malformed inputs, output failures, and partial operations. Derive a requirement only from a ground allowed by the global **Proportionality** rule or from authoritative behavior, a named recurring consumer, or a script-specific standing policy. The pass must identify the smallest viable alternative and must not invent future consumers, threats, or use cases.
4. **Choose the smallest sufficient design.** Proceed only when no simpler design satisfies the established requirements. If the adversarial pass turns a small helper into a general framework, protocol, or transactional system, stop and return to the direct workflow or narrow the requirement before implementation.
5. **Freeze the accepted contract for implementation.**

Use one adversarial pass and, after any revision, one focused check of the changed contract. Do not begin an open-ended design review loop. Ask the user only when two materially different designs remain viable. Otherwise choose the smallest reversible design autonomously, while following every standing approval gate.

Keep rejected alternatives and adversarial notes in task context. Document only the accepted observable contract and non-obvious rationale.
