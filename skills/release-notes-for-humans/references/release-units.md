# Release Units

When repository context is available, use package paths, release configuration, tag conventions, and workspace manifests to determine whether packages release independently or as one synchronized set. Otherwise, use package names, paths, and release context supplied with the direct evidence scope. Treat each independent package and each synchronized package set as a separate release unit.

When repository metadata is available and the request does not identify a package, treat each publishable release unit as a candidate. Resolve its evidence scope before mapping changes to determine whether it contains any.

If package ownership or release grouping remains unclear, stop and ask for direction.
