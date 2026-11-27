# Maintain Global Instructions

## Choose the Owner and Section

Use **Shared agent instructions** to decide whether a rule belongs in domfiles’ `.agents/GLOBAL.md`. Define each grant that isn’t specific to one project, with its limits, in the global source, even when only one skill uses it or it changes a public skill’s behavior. Put a project-specific grant in that project’s `AGENTS.md`. A command-only skill may state in its entrypoint what the user asks the host to do by invoking it.

Extend the rule that already governs the decision. Add a rule only for a subject the existing rules don’t cover. When several sections fit the topic, place the rule by the decision it changes. For example, permission to disclose material in an agent message belongs under Gates. How to deliver the message belongs under Working With Other Agents. Define the file’s own policy vocabulary, such as **grant**, in its opening paragraph.

| Section | What Belongs There |
| --- | --- |
| Authority | Whose instructions and approvals the agent can act on. |
| Task Conduct | How the agent scopes, carries out, and validates work. |
| Effects | Which actions a request authorizes and on what terms. Put prerequisites in Gates and permissions already given for specified circumstances in Standing Grants. Use a more specific subsection where one exists, such as Contribution Preparation Authorization for grants to prepare a contribution. |
| Security | How the agent protects secrets, access controls, and code execution boundaries. |
| Working With Other Agents | How agents coordinate work, communicate, and hand it off. |
| Reporting | How the agent interprets requests and reports results, reviews, and findings. Input Conventions holds definitions of the user’s recurring terms and the defaults a particular kind of request implies, such as which checkouts a batch request includes. |
| Defaults | Choices that apply across tasks and outputs, such as wording in both replies and artifacts, editing conventions, and tool use. |

Place related rules beside one another. If a rule only qualifies another, nest it under that rule. Start each rule, including a definition of a user term, with a bullet and a label unique in the file, in the form `**Label:**`. Label a nested bullet when it states a permission, condition, or default that can be cited on its own. Leave supporting detail that only explains or applies the parent rule unlabeled. Follow the [composition rules for titled bullets and policy citations](compose-documentation.md).

## Write for Every Supported Agent

Cite the rule that owns a limit. Keep a safeguard for data, credentials, or publication explicit beside the action it constrains, even when another rule states the same limit. For example, **History update authorization** keeps the publication limit from **Git publication** beside the permission to rewrite a published branch locally.

When a requirement depends on a particular client or model family, name it and make clear how the rule applies elsewhere. Include version details only when the decision depends on them.

## Check the Change

When rewording, merging, or moving a rule, identify the sentence that carries each removed sentence’s meaning. Check conditions, exceptions, and limits supplied by the old label, parent, and section as well as by the text itself.

Search tracked files, including hidden directories, for citations by label, links to the containing section, and skills that rely on the rule. Keep a cited label or update every citation. When moving a rule between sections, update links intended to reach it through its old section anchor. Update any skill instructions the change makes inaccurate. For public copies, consult `.agents/project/shared-skill-contracts.md` in the domfiles checkout and follow `skill-development`’s Mirror Alignment guidance in `skills/.domfiles-skill-development/references/public-skill-portability.md`.

Report the byte count of `.agents/GLOBAL.md` before and after the change. If the file grows, explain why the added text needs to load in every session.

A change to the opening paragraph of definitions or to a rule under Authority, Effects, or Security must clear independent model review before it is pushed or otherwise published. Count the original section as well as any destination when moving or removing a rule. For each contributing model provider, at least one independent reviewer from another provider must assess the complete change. For a change written entirely by a human, any independent model reviewer qualifies. Apply **Agent communication** to the exchange. Other changes follow the task’s usual review.

For the required review in T3 Code, read and apply `super-review`’s entrypoint from the same installed skills root or from `skills/.domfiles-super-review/SKILL.md` in the domfiles checkout. Select only stages the task authorizes. Verify that the reviewers assessing the complete change to `.agents/GLOBAL.md` meet the provider requirement above. Obtain any needed permission for a provider exchange or commit preparation before starting the affected stage.
