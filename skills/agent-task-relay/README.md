[![](https://skills.sh/b/porada/domfiles)](https://www.skills.sh/porada/domfiles/agent-task-relay)

# agent-task-relay

Moving work between agent threads shouldn’t mean losing context or inadvertently changing the agent’s authority.

This skill checks incoming findings, separates assignments from evidence-only handoffs, and confirms external assignments with the user before drafting their prompts. It preserves each handoff’s limits on access, approvals, changes, and scope.

## Install

```sh
npx skills add porada/domfiles --skill agent-task-relay
```

```sh
gh skill install porada/domfiles agent-task-relay
```

## License

MIT © [Dom Porada](https://dom.engineering)
