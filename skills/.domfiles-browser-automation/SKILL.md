---
name: browser-automation
description: |-
    Use before selecting or invoking tools that use browser capabilities, regardless of how they’re exposed, including browser automation prescribed by another workflow. Also use before delegating work that depends on browser state.

metadata:
    internal: true
---

# Browser Automation

- **Tool choice:** For eligible browser interaction, use Safari MCP even when a skill prescribes `agent-browser` or another browser driver. Translate those interaction steps while preserving required checks and evidence. Use Chrome MCP when Safari MCP is unavailable or cannot perform a required step. Honor an explicit user or applicable project selection of another tool, and keep repository browser test suites unchanged.
- **Capability gaps:** If neither Safari MCP nor Chrome MCP can perform a required step, use an equivalent available tool only within current authorization and the global **Retrieval failure** policy. Honor an explicit user tool selection, and ask before changing a material task assumption. If no permitted alternative can complete the step, report the specific gap rather than silently omitting it. Tool substitution doesn’t authorize installation.
- **Authenticated research:** Keep online research that depends on authenticated or otherwise non-transferable browser state in the main thread, with a narrow source count and stopping point defined before browsing.
- **Process isolation:** For authorized independent Chrome MCP server launches that start their own browsers, use `--isolated` for temporary profiles. When persistent sessions are explicitly required, use distinct machine-local `--user-data-dir` profiles outside repositories instead. Process isolation doesn’t separate agents sharing one server.
- **Shared servers:** Assign each agent its own pages. When agents need independent cookies and storage, use Chrome MCP with a unique `isolatedContext` for each agent, unless an explicit user or applicable project selection of another tool applies. If explicit page targeting or required isolation is unavailable, or agents must share a page, serialize the complete browser workflow instead of substituting another tool under **Capability gaps**. Clean up only task-created pages and contexts.
- **Profile conflicts:** Stop affected browser work and report profile contention. Don’t terminate another session or delete profile locks to proceed.
