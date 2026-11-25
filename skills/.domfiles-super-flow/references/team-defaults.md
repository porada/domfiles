# Team Defaults

## Profiles

Keep your own model and effort as coordinator. By default, use Claude Opus for engineers, Codex Astra for the editor, and Codex Daybreak for a security engineer. Daybreak is costly for what it delivers, so use that role sparingly. Apply global **Thread effort** to new team threads.

Resolve models against the current catalog in the invoking thread’s T3 environment. Honor the user’s model and effort overrides. For an unpinned model, select the newest version in its family, searching the catalog if the listing is partial. If a selected model or effort is unavailable, bring that choice to the user. Act as editor yourself only if you run the selected editor model. Otherwise use a separate editor thread.

## Names and Titles

The proposed flow title defaults to the invoking thread’s title. Apply global **Thread titles** when selecting it.

Give every member, including the coordinator, a short name unique within the team. Title each new member thread `⤷ <Role> <Name>: <flow title>` using the title in the flow record. Keep a reused member thread’s title unless the user requests a change.
