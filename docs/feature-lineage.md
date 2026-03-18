# Feature lineage

This document is for maintainers only. Keep upstream project names and compatibility lineage here, not in end-user UI copy or top-level marketing text.

## SwarmSensei feature mapping

| SwarmSensei feature | Upstream lineage | Notes |
| --- | --- | --- |
| Execution Kernel | agentkernel | Isolated task execution and runtime-aware command handling. |
| Team Mesh | pi-teams | Shared task board and specialist-agent coordination. |
| Model Router | pi-model-switch, pi-nvidia-nim | Adaptive model selection plus provider-aware catalog support. |
| Swarm Lanes | oh-pi-ant-colony | Adaptive concurrency, wave scheduling, and swarm flow control. |
| Policy Guard | pi-governance | RBAC, DLP, audit logging, and policy enforcement. |
| Decision Gate | pi-ask-user | Structured human approval and checkpoint prompts. |
| Memory Graph | pi-brain | Versioned memory branches, merges, and context snapshots. |
| Workflow Engine | pi-superpowers-plus | Workflow discipline, verification gates, and subagent support. |
| Workspace Console | OpenChamber | Browser-based workspace for chat, tasks, plans, and diffs. |
| Harness SDK | Pi coding agent | Extensible coding-agent foundation and tool harness model. |

## Guidance

- Use SwarmSensei-native names in README text, the Yew UI, and scenario seed data.
- Reserve upstream names for maintainer docs, migration notes, or implementation comments where lineage matters.
- If a new user-facing capability is added, update this table before mentioning upstream lineage anywhere else.
