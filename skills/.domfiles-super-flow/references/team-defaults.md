# Team Defaults

## Profiles

Keep your own model and effort as coordinator. By default, use Claude Opus for engineers, Codex Astra for the editor, and Codex Daybreak for a security engineer. Daybreak is costly for what it delivers, so use that role sparingly. Use `xhigh` effort for every team thread you create.

Resolve models against the current catalog in the invoking thread’s T3 environment. Honor the user’s model and effort overrides. For an unpinned model, select the newest version in its family, searching the catalog if the listing is partial. If a selected model or effort is unavailable, bring that choice to the user. Act as editor yourself only if you run the selected editor model. Otherwise create a separate editor thread.

## Names and Titles

Give every member, including the coordinator, a short name unique within the team. Use the invoking thread’s title as the flow title. Title each new member thread `⤷ <Role> <Name>: <flow title>`.
