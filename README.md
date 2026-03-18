# SwarmSensei

SwarmSensei is a mono Rust/WASM control room that unifies secure execution, Team Mesh coordination, Model Router orchestration, adaptive swarm lanes, Policy Guard oversight, Decision Gate approvals, Memory Graph context, and a browser-first workspace into one application.

## What is included?

- **Shared Rust domain core** for agents, tasks, Policy Guard rules, capability mapping, and Memory Graph entries.
- **Yew/WASM frontend** that renders a control room for the integrated swarm workflow.
- **Feature overview dashboard** showing how SwarmSensei capabilities fit together in a single architecture.

## Workspace layout

```text
.
├── crates/
│   ├── swarmsensei-core/   # Shared orchestration models + tests
│   └── swarmsensei-web/    # Yew/WASM UI
├── docs/                   # Design and lineage notes for maintainers
├── Cargo.toml
├── Trunk.toml
└── index.html
```

## Run locally

```bash
cargo test
cargo check
cargo install trunk
trunk serve --open
```

## Core capability pillars

| SwarmSensei feature | What it does |
| --- | --- |
| Execution Kernel | Runs tasks inside isolated execution environments |
| Team Mesh | Coordinates specialist agents on a shared task board |
| Model Router | Routes work across fast, deep, and provider-specific models |
| Swarm Lanes | Adapts concurrency and review flow across the swarm |
| Policy Guard | Applies RBAC, DLP, approvals, and audit controls |
| Decision Gate | Captures structured human checkpoints for risky choices |
| Memory Graph | Tracks branchable context, milestones, and merges |
| Workflow Engine | Enforces execution phases and verification gates |
| Workspace Console | Presents chat, plans, diffs, and task views in one cockpit |
| Harness SDK | Provides the composable coding-agent foundation |
