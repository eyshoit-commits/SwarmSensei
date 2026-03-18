# SwarmSensei

SwarmSensei is a mono Rust/WASM control room that synthesizes the ideas from agentkernel, pi-teams, pi-model-switch, oh-pi-ant-colony, pi-nvidia-nim, pi-governance, pi-ask-user, pi-brain, pi-superpowers-plus, OpenChamber, and the Pi coding agent into one browser-first application.

## What is included?

- **Shared Rust domain core** for agents, tasks, governance rules, source capability mapping, and memory entries.
- **Yew/WASM frontend** that renders a control room for the integrated swarm workflow.
- **Feature synthesis dashboard** showing how the upstream tools map into a single architecture.

## Workspace layout

```text
.
├── crates/
│   ├── swarmsensei-core/   # Shared orchestration models + tests
│   └── swarmsensei-web/    # Yew/WASM UI
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

## Integrated capability map

| Upstream inspiration | SwarmSensei interpretation |
| --- | --- |
| agentkernel | sandboxed execution plane |
| pi-teams | agent roster + shared task board |
| pi-model-switch | model routing controls |
| oh-pi-ant-colony | adaptive swarm lanes |
| pi-nvidia-nim | NIM-capable model catalog |
| pi-governance | RBAC + DLP + approvals |
| pi-ask-user | human decision checkpoints |
| pi-brain | branchable memory timeline |
| pi-superpowers-plus | workflow enforcement |
| OpenChamber | visual web workspace |
| Pi coding agent | extensible harness model |
