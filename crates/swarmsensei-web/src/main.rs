use swarmsensei_core::{sample_scenario, CapabilityArea, SourceFeature, TaskStatus, ThinkingLevel};
use web_sys::HtmlSelectElement;
use yew::prelude::*;

fn badge_class(status: TaskStatus) -> &'static str {
    match status {
        TaskStatus::Todo => "badge slate",
        TaskStatus::InProgress => "badge blue",
        TaskStatus::Review => "badge amber",
        TaskStatus::Blocked => "badge red",
        TaskStatus::Done => "badge green",
    }
}

fn thinking_chip(level: ThinkingLevel) -> &'static str {
    match level {
        ThinkingLevel::Off => "chip off",
        ThinkingLevel::Low => "chip low",
        ThinkingLevel::Medium => "chip medium",
        ThinkingLevel::High => "chip high",
    }
}

#[function_component(App)]
fn app() -> Html {
    let scenario = use_state(sample_scenario);
    let lane_filter = use_state(|| "All".to_string());
    let active_area = use_state(|| CapabilityArea::Sandbox);

    let on_lane_change = {
        let lane_filter = lane_filter.clone();
        Callback::from(move |event: Event| {
            let input: HtmlSelectElement = event.target_unchecked_into();
            lane_filter.set(input.value());
        })
    };

    let filtered_tasks = scenario
        .tasks
        .iter()
        .filter(|task| lane_filter.as_str() == "All" || task.lane == lane_filter.as_str())
        .cloned()
        .collect::<Vec<_>>();

    let active_sources = scenario
        .sources
        .iter()
        .filter(|source| source.area == *active_area)
        .cloned()
        .collect::<Vec<_>>();

    html! {
        <div class="app-shell">
            <style>{CSS}</style>
            <header class="hero">
                <div>
                    <p class="eyebrow">{"Rust + WASM + shared orchestration core"}</p>
                    <h1>{scenario.headline}</h1>
                    <p class="lede">{scenario.summary}</p>
                </div>
                <div class="hero-card">
                    <p class="muted">{"Active model"}</p>
                    <strong>{scenario.active_model}</strong>
                    <p class="muted top-gap">{"Execution mode"}</p>
                    <strong>{scenario.team_mode}</strong>
                </div>
            </header>

            <section class="stats-grid">
                <StatCard title="Feature modules" value={scenario.sources.len().to_string()} detail="SwarmSensei capabilities working together" />
                <StatCard title="Active agents" value={scenario.active_agents().to_string()} detail="Lead, scout, builders, reviewers" />
                <StatCard title="Completed tasks" value={scenario.completed_tasks().to_string()} detail="Board progress from the shared orchestration core" />
                <StatCard title="Policy rules" value={scenario.governance_rules().to_string()} detail="RBAC, DLP, approvals, verification" />
            </section>

            <section class="panel-grid">
                <div class="panel tall">
                    <div class="panel-head">
                        <h2>{"Team Mesh board"}</h2>
                        <span class="muted">{"Coordinated specialists across Swarm Lanes"}</span>
                    </div>
                    <div class="agent-list">
                        {for scenario.agents.iter().map(|agent| html! {
                            <article class="agent-card">
                                <div>
                                    <h3>{agent.name}</h3>
                                    <p>{agent.specialty}</p>
                                </div>
                                <div class="agent-meta">
                                    <span class={thinking_chip(agent.thinking)}>{agent.thinking.label()}</span>
                                    <span class="model-pill">{agent.model}</span>
                                    <span class="muted">{agent.status}</span>
                                </div>
                            </article>
                        })}
                    </div>
                </div>

                <div class="panel tall">
                    <div class="panel-head split">
                        <div>
                            <h2>{"Task lanes"}</h2>
                            <span class="muted">{"Shared backlog with Policy Guard-aware statuses"}</span>
                        </div>
                        <label class="select-wrap">
                            <span>{"Lane"}</span>
                            <select onchange={on_lane_change}>
                                <option selected={lane_filter.as_str()=="All"}>{"All"}</option>
                                <option selected={lane_filter.as_str()=="Execution Kernel"}>{"Execution Kernel"}</option>
                                <option selected={lane_filter.as_str()=="Model Router"}>{"Model Router"}</option>
                                <option selected={lane_filter.as_str()=="Swarm Lanes"}>{"Swarm Lanes"}</option>
                                <option selected={lane_filter.as_str()=="Policy Guard"}>{"Policy Guard"}</option>
                                <option selected={lane_filter.as_str()=="Memory Graph"}>{"Memory Graph"}</option>
                            </select>
                        </label>
                    </div>
                    <div class="task-list">
                        {for filtered_tasks.iter().map(|task| html! {
                            <article class="task-row">
                                <div>
                                    <h3>{task.title}</h3>
                                    <p class="muted">{format!("Owner: {} · Lane: {}", task.owner, task.lane)}</p>
                                </div>
                                <span class={badge_class(task.status)}>{task.status.label()}</span>
                            </article>
                        })}
                    </div>
                </div>
            </section>

            <section class="panel-grid">
                <div class="panel">
                    <div class="panel-head">
                        <h2>{"Capability map"}</h2>
                        <span class="muted">{"Select a capability family to explore the SwarmSensei design"}</span>
                    </div>
                    <div class="cap-grid">
                        {for [
                            CapabilityArea::Sandbox,
                            CapabilityArea::Teaming,
                            CapabilityArea::Models,
                            CapabilityArea::Swarm,
                            CapabilityArea::Governance,
                            CapabilityArea::HumanLoop,
                            CapabilityArea::Memory,
                            CapabilityArea::Workflow,
                            CapabilityArea::Interface,
                            CapabilityArea::CodingHarness,
                        ].into_iter().map(|area| {
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
                </div>

                <div class="panel">
                    <div class="panel-head">
                        <h2>{"Policy Guard & Memory Graph"}</h2>
                        <span class="muted">{"Governance, approvals, and durable swarm context"}</span>
                    </div>
                    <div class="subsection">
                        <h3>{"Policy rules"}</h3>
                        {for scenario.rules.iter().map(|rule| html! {
                            <div class="rule-row">
                                <strong>{rule.name}</strong>
                                <p>{format!("{} — {}", rule.scope, rule.effect)}</p>
                            </div>
                        })}
                    </div>
                    <div class="subsection">
                        <h3>{"Memory Graph"}</h3>
                        {for scenario.memory.iter().map(|entry| html! {
                            <div class="memory-row">
                                <span class="memory-kind">{entry.kind}</span>
                                <div>
                                    <strong>{entry.branch}</strong>
                                    <p>{entry.summary}</p>
                                </div>
                            </div>
                        })}
                    </div>
                </div>
            </section>
        </div>
    }
}

fn render_source(source: &SourceFeature) -> Html {
    html! {
        <article class="source-card">
            <p class="eyebrow small">{source.source}</p>
            <h3>{source.feature}</h3>
            <p>{source.outcome}</p>
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
.hero-card, .panel, .stat-card, .source-card, .agent-card, .task-row, .rule-row, .memory-row {
  border: 1px solid rgba(129, 160, 255, 0.18);
  background: rgba(9, 18, 35, 0.78);
  backdrop-filter: blur(16px);
  border-radius: 18px;
  box-shadow: 0 16px 40px rgba(0,0,0,0.24);
}
.hero-card, .panel, .stat-card, .source-card, .agent-card, .task-row, .rule-row, .memory-row {
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
.panel-head, .split { display: flex; justify-content: space-between; gap: 16px; align-items: center; }
.agent-list, .task-list, .source-list, .subsection { display: grid; gap: 12px; }
.agent-card, .task-row, .rule-row, .memory-row { display: flex; justify-content: space-between; gap: 16px; align-items: center; }
.agent-meta { display: flex; flex-wrap: wrap; gap: 8px; justify-content: flex-end; align-items: center; }
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
select {
  background: #10192d;
  color: #e8edf7;
  border: 1px solid rgba(129, 160, 255, 0.25);
  border-radius: 10px;
  padding: 8px 12px;
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
.source-card h3, .task-row h3, .agent-card h3 { margin: 0 0 6px; }
.muted { color: #9ca9c0; }
.memory-kind { background: #18294e; color: #91c1ff; text-transform: uppercase; }
@media (max-width: 980px) {
  .hero, .stats-grid, .panel-grid { grid-template-columns: 1fr; }
}
"#;

fn main() {
    yew::Renderer::<App>::new().render();
}
