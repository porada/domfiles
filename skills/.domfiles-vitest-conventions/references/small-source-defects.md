# Small Source Defects

A source defect qualifies for correction under the global **Test defect corrections** policy only when all of these conditions hold:

- **Clear defect:** The behavior contradicts the code’s evident intent, the established contract expressed by its test title, or platform requirements. Resolve an ambiguous contract with the user rather than treating a title alone as proof that the source is wrong.
- **Local correction:** The fix stays within the modules covered by the tests in progress and requires no broader refactor. It doesn’t change a public API, configuration, dependencies, persisted data formats, or behavior beyond correcting the defect.
- **Demonstrating test:** A changed existing test or a new test permitted by the project fails because of the defect’s actual effect and passes after the correction. A failure caused only by an incidental effect doesn’t establish the fix.

Before treating a branch as dead or redundant, check attributes and props supplied by the framework.

Apply qualifying corrections and report the source fix and its demonstrating test in the handoff. Honor project constraints, such as restoration fidelity to archived behavior, and explicit read-only limits.

Ask the user before a larger or ambiguous change, a change to behavior beyond the defect, or applying the same fix outside the modules under test. If project policy prevents the required correction or demonstrating test, report the defect rather than treating it as covered by this permission.
