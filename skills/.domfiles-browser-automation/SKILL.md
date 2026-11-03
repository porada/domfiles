---
name: browser-automation
description: |-
    Use before selecting or invoking tools that use browser capabilities, regardless of how they’re exposed, including browser automation prescribed by another workflow. Also use before delegating work that depends on browser state.

metadata:
    internal: true
---

# Browser Automation

- **Tool choice:** For eligible browser interaction, honor the governing host’s browser selection and fallback conditions. Otherwise honor an explicit user or applicable project tool selection. Without such a selection, use Safari MCP, then Chrome MCP when Safari is unavailable or cannot perform a required step, even when a skill prescribes `agent-browser` or another driver. Tool presence alone doesn’t establish precedence. Translate interaction steps while preserving required checks and evidence, and keep repository browser test suites unchanged.
- **Capability gaps:** If the selected browser is unavailable or cannot perform a required step, choose an equivalent available fallback without asking when substitution is permitted. A user’s tool selection alone doesn’t prohibit fallback, but preserve express no-substitution requirements and browser-dependent task behavior. Among suitable alternatives, use the host’s prescribed fallback when provided, otherwise Safari MCP, then Chrome MCP, then another equivalent available tool. Stay within current authorization and the global **Retrieval failure** policy. An access or authentication denial isn’t permission to switch tools. Ask before changing a material task assumption, not merely because the requested tool is unavailable. Report any substitution and its reason. If no permitted alternative can complete the step, report the specific gap rather than silently omitting it. Tool substitution doesn’t authorize installation.
- **Authenticated research:** Keep online research that depends on authenticated or otherwise non-transferable browser state in the main thread, with a narrow source count and stopping point defined before browsing.
- **Process isolation:** For authorized independent Chrome MCP server launches that start their own browsers, use `--isolated` for temporary profiles. When persistent sessions are explicitly required, use distinct machine-local `--user-data-dir` profiles outside repositories instead. Process isolation doesn’t separate agents sharing one server.
- **Shared servers:** Assign each agent its own pages in the selected browser. When agents need independent cookies and storage, use that browser’s supported isolation. Under the default tool policy, use Chrome MCP with a unique `isolatedContext` for each agent. If explicit page targeting or required isolation is unavailable, or agents must share a page, serialize the complete browser workflow instead of substituting another tool under **Capability gaps**. If serialization can’t provide the required separation, report the limitation. Clean up only task-created pages and contexts.
- **Profile conflicts:** Stop affected browser work and report profile contention. Don’t terminate another session or delete profile locks to proceed.
