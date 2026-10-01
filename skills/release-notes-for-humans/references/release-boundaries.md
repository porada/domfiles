# Release Boundaries

Require a target ref and derive one shared relevant tag for a synchronized package set and a separate package-relevant tag for each independent package. For a release version, select the preceding relevant tag, which must be an ancestor of and strictly older than the target. For `Unreleased`, select the newest relevant tag that is an ancestor of and no newer than the target. Use each selected tag through the target ref as that release unit’s default range.

When the `Unreleased` boundary tag equals the target, record that the release unit has no unreleased changes. Keep the range empty rather than falling back to an earlier tag.

If no relevant default boundary can be located for a release unit, stop and ask for a release boundary or confirmation that it is an initial release. Do not infer an initial release from a missing tag.
