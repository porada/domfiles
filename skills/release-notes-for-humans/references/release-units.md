# Release Units

When repository metadata is available, use package paths, release configuration, tag conventions, and workspace manifests to determine whether packages release independently or as one synchronized package set. Otherwise, use package names, paths, and release context supplied with the direct evidence scope.

When repository metadata is available and the request doesn’t identify a package, treat each publishable release unit as a candidate. Before mapping changes, resolve each candidate’s evidence scope to determine whether it contains any changes.

If package ownership or release grouping remains unclear, stop and ask for direction.
