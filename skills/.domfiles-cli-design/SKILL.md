---
name: cli-design
description: |-
    Use when designing, revising, or reviewing what a command-line tool prints or asks during a run. This includes the order and visual hierarchy of results, status labels, progress, confirmation prompts, verdicts, and commands the reader needs to run.

    Use it for an individual message when its wording affects the state it reports, the scope of consent, or the action the reader should take. Skip tasks confined to shell mechanics.

metadata:
    internal: true
---

# CLI Design

A tool earns trust when the person running it can see what they control. Before approving a change, they should understand what their answer will authorize. Afterward, even if the run stopped partway, they should be able to account for what happened. Make it easy to tell at a glance which tool is speaking and whose turn it is to act. An agent reading the run through a pipe needs the same facts in the same order.

Use `human-facing-writing` for the wording of messages. Apply the relevant scripting guidance when the task also involves shell code. For a review alone, keep the work read-only. Identify the line or transition that could hide a fact, suggest the wrong action, or leave the reader wondering why the presentation changed.

## Write the Transcript

Identify the tool at the start of a run report and in any view that can be displayed on its own. Keep prompts, progress, and narration off a stream whose output is data. Preserve documented quiet behavior. For a long step, a short line naming the work as it begins explains the wait. Append results as they become available so progress stays legible live and in the log without spinners or redrawing.

Start with the plain text a reader would see in a log. Default to plain text for pipes and logs. Use it whenever the reader disables styling or the destination cannot render it. Add styling to make the same facts quicker to find. If a symbol becomes ambiguous once its color or weight disappears, spell out its meaning in the plain form. Put details that never vary in help when they inform the choice of command. Reserve space during the run for facts about this invocation.

## Give Attention to What Matters

Keep enough detail to account for the work, even when results arrive quickly. In a uniformly dim report, a failure can pass unnoticed. Giving every line the same strong treatment makes each routine check compete with the failure for attention.

Choose the emphasis of each line by what it asks of the reader. Findings that need attention, changes awaiting consent, the verdict, and the next action should stand out among routine details. In a design that uses contrast for emphasis, reserve full contrast for those lines. Where all text needs the same contrast for legibility, use weight, spacing, or explicit labels to establish that priority. Leave space between groups so a finding stays close to its supporting lines. Keep the finding prominent and its cause quieter. Preserve the writing conventions within an error or warning message. Place separate supporting evidence beneath the finding, including a command the tool ran when showing it helps explain the result and its text is safe to display.

Give comparable results a consistent alignment across sections. Use columns when readers compare values down a list. If long names crowd those values at the available width, stack the details under each result with a common left edge. An unexplained shift suggests a difference between otherwise similar results. Consistency makes a deliberate break recognizable when the reader’s role changes.

Use color consistently to convey state. Give colored labels enough weight to stay legible on both light and dark backgrounds. Keep routine states recognizable at a glance while giving exceptions the greater emphasis. Repeated passing checks can take a compact mark when its meaning is clear. Name warnings and failures in words so readers know which rows to read. Give failure an additional cue strong enough to outrank a warning on both light and dark themes. Distinguish a successful check from a successful change. In a file list, for example, “Up to date” tells the reader what a check found. “Updated” tells them this run changed the file.

## Make Consent Specific

Before asking for a decision, show what was checked, the exact change proposed, and which items or destinations it affects. Make the question the focal point. Keep the input hint and any visible typed response secondary. If another program will ask the next question, introduce it by name and say what it needs. Leave a blank line between its exchange and the tool’s resumed output so the reader can tell who is speaking.

Use the command’s authorization rules to decide whether confirmation is required. When someone confirms a displayed proposal, bind the answer to those effects and targets. If an agent can apply it later, give the agent a way to identify that approved proposal. Honor existing authorization within its scope. When proposed effects exceed it, show the difference and obtain a new decision. A run without a terminal supplies no consent. On refusal or when required approval is unavailable, skip the change and say why.

## Close the Run and Hand Over the Next Step

End each human-facing run report with a verdict that accounts for what happened, what did not happen, and what remains. After a failure, distinguish completed changes from items known to be untouched. For example, a photo organizer might report “Moved 8 photos. The ninth move failed. The last 3 photos are still in the inbox.” Claim an item’s final state only when the tool can establish it. Describe the overall outcome consistently with the command’s documented exit status.

Make the verdict easy to pick out even when the next sentence adds detail. In a closing list, lead with entries that have consequences for the reader and give those entries the stronger treatment. Let entries kept for the record recede.

When the reader has work to do, give the instruction a place outside the surrounding results so the handoff is visible. A change in alignment, a blank line, or an explicit action label can mark that boundary, depending on the layout. Address the reader directly, put steps in execution order, and open each with the action. Give commands meant to be copied a treatment distinct from headings and from commands the tool ran. Print each on a line the reader can select in full and paste as written. Keep prompt signs, indentation, and enclosing quotation marks off that line. Preserve quoting the command itself needs, and use names and paths that resolve where the reader will paste it.

## Try the Design in the Reader’s Terminal

Use realistic wording and simulated outcomes to exercise the affected paths without changing real state. For a new design, cover success, warning, failure, confirmation, and refusal wherever the tool supports them. Read the plain transcript in order. Find the first item that needs attention and identify the scope of any proposed change.

Check line breaks at the reader’s usual width and again in a narrow window. At either width, aim for even lines that keep related words together. If a short explanation leaves one word stranded on the next line, try wording that fits on one line.

For a new or substantially revised layout or consent flow, try the affected scenes in the reader’s terminal. Inspect both light and dark themes and select a command as if about to paste it. If the terminal is unavailable, distinguish the transcript checks you completed from any visual or selection checks that still need a reader.

When a reader hesitates, ask what they thought the line meant. When they understand it but say it feels misplaced, ask what they expected to see there. The answer may reveal a pattern the design broke by accident, such as a result shifted out of alignment with its peers. A command mistaken for a heading points to a distinction the design needs wherever it offers a command. Revise the treatment and try the affected scenes again.
