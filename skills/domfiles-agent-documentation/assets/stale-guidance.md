### Stale Guidance

Classify each part of this skill’s guidance used by the selected workflow as required, optional, or supporting. Treat missing local targets, malformed destinations, and HTTP responses that report a resource as missing or permanently unavailable as broken references. Broken references and verified conflicts with the current interface or behavior mean the guidance is stale. Use any failure response the guidance defines. Otherwise, report the stale guidance and evidence, recommend updating this skill, and follow the appropriate recovery below.

When required guidance is stale, stop only the affected branch and use any complete fallback provided by the available guidance. Without one, ask whether to continue. The choice applies only to this conversation and to work independent of the stale guidance. Stale optional or supporting guidance does not stop the workflow.

Access restrictions, authentication problems, network failures, and HTTP server errors are not evidence of staleness. Use any relevant access or retrieval guidance. If none applies, stop retrieving the resource and report the resource, attempted method, exact error, and smallest corrective action.

Never infer missing content. Never substitute an unverified location. Never weaken scope, approval, mutation, or security boundaries.
