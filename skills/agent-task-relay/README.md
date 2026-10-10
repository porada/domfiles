[![](https://skills.sh/b/porada/domfiles)](https://www.skills.sh/porada/domfiles/agent-task-relay)

# agent-task-relay

Moving work between agent threads shouldn’t mean losing context or inadvertently changing the agent’s authority.

This skill prepares outgoing prompts and validates incoming findings. It preserves the task’s context, evidence, authorization, and limits while leaving execution with the owning workflow.

## Install

```sh
npx skills add porada/domfiles --skill agent-task-relay
```

```sh
gh skill install porada/domfiles agent-task-relay
```

## License

MIT © [Dom Porada](https://dom.engineering)
