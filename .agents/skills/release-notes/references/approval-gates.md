# Approval gates

Apply these gates only when editing a user-supplied or previously approved draft. An explicit request to consolidate or restructure authorizes that operation within the requested scope. Organize and consolidate a newly inferred draft directly when evidence supports the result.

## Obtain approval

Propose and obtain approval before:

- Removing or reclassifying a supplied release-note item, including treating a supplied refactor as internal-only.
- Removing a supplied rationale or exact dependency version.
- Adding, removing, or materially changing a consumer warning or evidence link.
- Strengthening or weakening a supplied technical claim or qualifier.

## Propose a gated consolidation

Use this format when thematic consolidation requires approval:

**Before**

```text
* Added support for Prettier’s `checkIgnorePragma`, `insertPragma`, and `requirePragma` options.
* Fixed cursor positioning and partial-range formatting.
```

**After**

```text
* Improved support for Prettier’s native formatting controls, including pragma options, cursor positioning, and partial-range formatting.
```

**Evidence**

Explain the verified source or user context that connects the items.

**Approval**

Ask whether to apply the proposed wording.

## Protect approved structure

When a major structural change requires approval, show the relevant before and after, explain the benefit, and ask whether to apply it before:

- Converting bullets to prose or prose to bullets.
- Introducing a named release theme.
- Creating, renaming, removing, or materially reorganizing a heading or package section.
- Moving a change into or out of `All Packages`.
