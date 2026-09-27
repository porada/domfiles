# Approval Gates

These gates protect the user’s authorship when a release note draft already exists. Apply them only to text the user supplied or previously approved. Preserve its wording, bullet order, headings, and structure outside the requested revision.

An explicit request authorizes the revisions it covers, including consolidation or restructuring, under [Instruction Authority](../SKILL.md#instruction-authority). A newly inferred draft has no preserved authorial baseline, so organize and consolidate it directly when the evidence supports the result.

## Approval Requirement

Before making a listed change that the user has not already authorized, show the relevant before and after, explain the supporting evidence or material effect, and ask whether to apply it. Approval covers only the proposed change. Without that approval, retain the affected supplied text.

- Removing or reclassifying a supplied release note item, including treating a supplied refactor as internal-only.
- Removing a supplied rationale or exact dependency version.
- Adding, removing, or materially changing a consumer warning or evidence link.
- Strengthening or weakening a supplied technical claim or qualifier.
- Converting bullets to prose or prose to bullets.
- Introducing a named release theme.
- Creating, renaming, removing, or materially reorganizing a heading or package section.
- Moving a change into or out of `All Packages`.

## Consolidation Proposal

Use this format when several supplied items may describe one consumer outcome:

**Before**

```text
- Added support for Prettier’s `checkIgnorePragma`, `insertPragma`, and `requirePragma` options.
- Fixed cursor positioning and partial-range formatting.
```

**After**

```text
- Improved support for Prettier’s native formatting controls, including pragma options, cursor positioning, and partial-range formatting.
```

**Evidence**

Name the verified source or user context that establishes the connection between the items.

**Approval**

Ask whether to apply the proposed wording.
