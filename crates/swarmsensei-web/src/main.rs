mod components;

use components::agent_board::AgentBoard;
use components::decision_gate::DecisionGate;
use components::governance_panel::GovernancePanel;
use components::memory_graph::MemoryGraph;
use components::model_router::ModelRouter;
use components::task_board::TaskBoard;
use std::rc::Rc;
use swarmsensei_core::{sample_scenario, CapabilityArea, MemoryKind, Scenario, TaskStatus};
use yew::prelude::*;

#[derive(Clone, PartialEq)]
struct AppState {
    scenario: Scenario,
}

#[derive(Clone, PartialEq)]
enum AppAction {
    CreateTask(String, String, String),
    AssignTask(usize, String),
    ChangeTaskStatus(usize, TaskStatus),
    SwitchModel(String, String),
    OpenApproval(String, String, String),
    DecideApproval(usize, bool),
    AppendMemory(String, MemoryKind, String),
    StartSwarmRun,
    StopSwarmRun,
}

impl Reducible for AppState {
    type Action = AppAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let mut next = (*self).clone();
        match action {
            AppAction::CreateTask(title, owner, lane) => {
                next.scenario.create_task(title, owner, lane)
            }
            AppAction::AssignTask(id, owner) => next.scenario.assign_task(id, owner),
            AppAction::ChangeTaskStatus(id, status) => next.scenario.update_task_status(id, status),
            AppAction::SwitchModel(provider, model) => next.scenario.switch_model(provider, model),
            AppAction::OpenApproval(title, requested_by, summary) => next
                .scenario
                .open_approval_request(title, requested_by, summary),
            AppAction::DecideApproval(id, accept) => next.scenario.decide_approval(id, accept),
            AppAction::AppendMemory(branch, kind, summary) => {
                next.scenario.append_memory(branch, kind, summary)
            }
            AppAction::StartSwarmRun => next.scenario.start_swarm_run(),
            AppAction::StopSwarmRun => next.scenario.stop_swarm_run(),
        }
        Rc::new(next)
    }
}

#[function_component(App)]
fn app() -> Html {
    let state = use_reducer(|| AppState {
        scenario: sample_scenario(),
    });
    let scenario = &state.scenario;
    let active_area = use_state(|| CapabilityArea::Sandbox);

    let owners = scenario
        .agents
        .iter()
        .map(|agent| agent.name.clone())
        .collect::<Vec<_>>();

    let active_sources = scenario
        .sources
        .iter()
        .filter(|source| source.area == *active_area)
        .cloned()
        .collect::<Vec<_>>();

    let on_create_task = {
        let state = state.clone();
        Callback::from(move |(title, owner, lane)| {
            state.dispatch(AppAction::CreateTask(title, owner, lane))
        })
    };
    let on_assign_task = {
        let state = state.clone();
        Callback::from(move |(id, owner)| state.dispatch(AppAction::AssignTask(id, owner)))
    };
    let on_change_status = {
        let state = state.clone();
        Callback::from(move |(id, status)| state.dispatch(AppAction::ChangeTaskStatus(id, status)))
    };
    let on_switch_model = {
        let state = state.clone();
        Callback::from(move |(provider, model)| {
            state.dispatch(AppAction::SwitchModel(provider, model))
        })
    };
    let on_open_approval = {
        let state = state.clone();
        Callback::from(move |(title, requested_by, summary)| {
            state.dispatch(AppAction::OpenApproval(title, requested_by, summary))
        })
    };
    let on_decide_approval = {
        let state = state.clone();
        Callback::from(move |(id, accept)| state.dispatch(AppAction::DecideApproval(id, accept)))
    };
    let on_append_memory = {
        let state = state.clone();
        Callback::from(move |(branch, kind, summary)| {
            state.dispatch(AppAction::AppendMemory(branch, kind, summary))
        })
    };
    let on_start_swarm = {
        let state = state.clone();
        Callback::from(move |_| state.dispatch(AppAction::StartSwarmRun))
    };
    let on_stop_swarm = {
        let state = state.clone();
        Callback::from(move |_| state.dispatch(AppAction::StopSwarmRun))
    };

    html! {
        <div class="app-shell">
            <style>{CSS}</style>
            <header class="hero">
                <div>
                    <p class="eyebrow">{"Rust + WASM + shared orchestration core"}</p>
                    <h1>{scenario.headline.clone()}</h1>
                    <p class="lede">{scenario.summary.clone()}</p>
                </div>
                <div class="hero-card">
                    <p class="muted">{"Active route"}</p>
                    <strong>{format!("{} / {}", scenario.active_provider, scenario.active_model)}</strong>
                    <p class="muted top-gap">{"Swarm mode"}</p>
                    <strong>{scenario.team_mode.clone()}</strong>
                </div>
            </header>

            <section class="stats-grid">
                <StatCard title="Feature sources" value={scenario.sources.len().to_string()} detail="Integrated upstream inspirations" />
                <StatCard title="Active agents" value={scenario.active_agents().to_string()} detail="Lead, scout, builders, reviewers" />
                <StatCard title="Completed tasks" value={scenario.completed_tasks().to_string()} detail="Live progress from shared state" />
                <StatCard title="Pending approvals" value={scenario.pending_approvals().to_string()} detail="Human checkpoints waiting for action" />
            </section>

            <section class="panel-grid">
                <AgentBoard
                    agents={scenario.agents.clone()}
                    swarm_status={scenario.swarm_status}
                    on_start={on_start_swarm}
                    on_stop={on_stop_swarm}
                />
                <TaskBoard
                    tasks={scenario.tasks.clone()}
                    owners={owners.clone()}
                    on_create_task={on_create_task}
                    on_assign_task={on_assign_task}
                    on_change_status={on_change_status}
                />
            </section>

            <section class="panel-grid">
                <ModelRouter
                    active_provider={scenario.active_provider.clone()}
                    active_model={scenario.active_model.clone()}
                    catalog={scenario.model_catalog.clone()}
                    on_switch_model={on_switch_model}
                />
                <DecisionGate approvals={scenario.approvals.clone()} on_decide={on_decide_approval} />
            </section>

            <section class="panel-grid">
                <GovernancePanel
                    rules={scenario.rules.clone()}
                    approvals={scenario.approvals.clone()}
                    audit_log={scenario.audit_log.clone()}
                    on_open_approval={on_open_approval}
                />
                <MemoryGraph memory={scenario.memory.clone()} on_append_memory={on_append_memory} />
            </section>

            <section class="panel">
                <div class="panel-head">
                    <div>
                        <h2>{"Capability map"}</h2>
                        <span class="muted">{"Select a capability family to inspect live source inspirations"}</span>
                    </div>
                </div>
                <div class="cap-grid">
                    {for CapabilityArea::ALL.into_iter().map(|area| {
                        let active_area = active_area.clone();
                        let is_active = *active_area == area;
                        let onclick_handle = active_area.clone();
                        let onclick = Callback::from(move |_| onclick_handle.set(area));
                        html! {
                            <button class={classes!("cap-button", is_active.then_some("active"))} {onclick}>
                                {area.label()}
                            </button>
                        }
                    })}
                </div>
                <div class="source-list">
                    {for active_sources.iter().map(render_source)}
                </div>
            </section>
        </div>
    }
}

fn render_source(source: &swarmsensei_core::SourceFeature) -> Html {
    html! {
        <article class="source-card">
            <p class="eyebrow small">{source.source.clone()}</p>
            <h3>{source.feature.clone()}</h3>
            <p>{source.outcome.clone()}</p>
        </article>
    }
}

#[derive(Properties, PartialEq)]
struct StatCardProps {
    title: AttrValue,
    value: String,
    detail: AttrValue,
}

#[function_component(StatCard)]
fn stat_card(props: &StatCardProps) -> Html {
    html! {
        <article class="stat-card">
            <p class="muted">{props.title.clone()}</p>
            <h2>{props.value.clone()}</h2>
            <p>{props.detail.clone()}</p>
        </article>
    }
}

const CSS: &str = r#"
:root {
  color-scheme: dark;
  font-family: Inter, ui-sans-serif, system-ui, sans-serif;
  background: #08111f;
  color: #e8edf7;
}
body {
  margin: 0;
  background: radial-gradient(circle at top, #132846, #08111f 55%);
}
button, input, select, textarea {
  font: inherit;
}
.app-shell {
  max-width: 1280px;
  margin: 0 auto;
  padding: 32px;
}
.hero, .panel-grid, .stats-grid {
  display: grid;
  gap: 20px;
}
.hero {
  grid-template-columns: 2fr 1fr;
  align-items: stretch;
  margin-bottom: 24px;
}
.hero h1 { font-size: 3rem; margin: 0 0 12px; }
.lede { font-size: 1.1rem; max-width: 70ch; color: #b6c3d9; }
.eyebrow { text-transform: uppercase; letter-spacing: 0.12em; color: #7ad7ff; font-size: 0.78rem; }
.eyebrow.small { color: #96f0c4; }
.hero-card, .panel, .stat-card, .source-card, .agent-card, .task-row, .rule-row, .memory-row, .decision-card {
  border: 1px solid rgba(129, 160, 255, 0.18);
  background: rgba(9, 18, 35, 0.78);
  backdrop-filter: blur(16px);
  border-radius: 18px;
  box-shadow: 0 16px 40px rgba(0,0,0,0.24);
  padding: 18px;
}
.top-gap { margin-top: 18px; }
.stats-grid {
  grid-template-columns: repeat(4, minmax(0, 1fr));
  margin-bottom: 24px;
}
.stat-card h2 { font-size: 2rem; margin: 8px 0; }
.panel-grid { grid-template-columns: repeat(2, minmax(0, 1fr)); margin-bottom: 24px; }
.panel.tall { min-height: 360px; }
.panel-head, .split, .inline-actions, .task-topline {
  display: flex;
  justify-content: space-between;
  gap: 16px;
  align-items: center;
}
.inline-actions { flex-wrap: wrap; }
.agent-list, .task-list, .source-list, .subsection, .audit-list {
  display: grid;
  gap: 12px;
}
.agent-card, .memory-row {
  display: flex;
  justify-content: space-between;
  gap: 16px;
  align-items: center;
}
.agent-meta, .inline-form {
  display: flex;
  flex-wrap: wrap;
  gap: 8px;
  justify-content: flex-end;
  align-items: center;
}
.inline-form.two-up > * { flex: 1 1 220px; }
.compose-grid {
  display: grid;
  grid-template-columns: 2fr 1fr 1fr auto;
  gap: 12px;
  margin-bottom: 16px;
}
.compose-grid.single {
  grid-template-columns: 1fr;
}
.model-pill, .memory-kind, .chip, .badge {
  border-radius: 999px;
  padding: 6px 10px;
  font-size: 0.78rem;
  font-weight: 600;
}
.model-pill { background: #202f57; color: #bed1ff; }
.chip.off { background: #333d50; }
.chip.low { background: #1f5a78; }
.chip.medium { background: #426b28; }
.chip.high { background: #6b3f85; }
.badge.slate { background: #3f4758; }
.badge.blue { background: #244c92; }
.badge.amber { background: #7b5a17; }
.badge.red { background: #7d2830; }
.badge.green { background: #256443; }
.select-wrap { display: grid; gap: 6px; font-size: 0.88rem; }
select, .text-input, .text-area {
  background: #10192d;
  color: #e8edf7;
  border: 1px solid rgba(129, 160, 255, 0.25);
  border-radius: 10px;
  padding: 10px 12px;
}
.text-area {
  min-height: 88px;
  resize: vertical;
}
.button {
  border: 0;
  border-radius: 10px;
  background: linear-gradient(135deg, #1565c0, #26a69a);
  color: white;
  padding: 10px 14px;
  cursor: pointer;
}
.button.subtle {
  background: #22314f;
}
.cap-grid {
  display: flex;
  flex-wrap: wrap;
  gap: 10px;
  margin-bottom: 16px;
}
.cap-button {
  border: 1px solid rgba(129, 160, 255, 0.18);
  background: #10192d;
  color: #dce7fa;
  border-radius: 999px;
  padding: 10px 14px;
  cursor: pointer;
}
.cap-button.active {
  background: linear-gradient(135deg, #1565c0, #26a69a);
  border-color: transparent;
}
.source-card h3, .task-row h3, .agent-card h3, .decision-card h3 { margin: 0 0 6px; }
.source-card.compact .badge { margin-top: 8px; display: inline-flex; }
.stacked, .stacked-card {
  display: grid;
  gap: 12px;
}
.grow { width: 100%; }
.muted { color: #9ca9c0; }
.memory-kind { background: #18294e; color: #91c1ff; text-transform: uppercase; }
@media (max-width: 980px) {
  .hero, .stats-grid, .panel-grid, .compose-grid { grid-template-columns: 1fr; }
}
"#;

fn main() {
    yew::Renderer::<App>::new().render();
}
