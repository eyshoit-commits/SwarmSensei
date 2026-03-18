use swarmsensei_core::{AgentProfile, SwarmStatus, ThinkingLevel};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct AgentBoardProps {
    pub agents: Vec<AgentProfile>,
    pub swarm_status: SwarmStatus,
    pub on_start: Callback<MouseEvent>,
    pub on_stop: Callback<MouseEvent>,
}

#[function_component(AgentBoard)]
pub fn agent_board(props: &AgentBoardProps) -> Html {
    html! {
        <div class="panel tall">
            <div class="panel-head split">
                <div>
                    <h2>{"Agent board"}</h2>
                    <span class="muted">{"Swarm operators, specialists, and execution controls"}</span>
                </div>
                <div class="inline-actions">
                    <span class={classes!("badge", swarm_badge(props.swarm_status))}>{props.swarm_status.label()}</span>
                    <button class="button" onclick={props.on_start.clone()}>{"Start run"}</button>
                    <button class="button subtle" onclick={props.on_stop.clone()}>{"Stop run"}</button>
                </div>
            </div>
            <div class="agent-list">
                {for props.agents.iter().map(render_agent)}
            </div>
        </div>
    }
}

fn render_agent(agent: &AgentProfile) -> Html {
    html! {
        <article class="agent-card">
            <div>
                <h3>{agent.name.clone()}</h3>
                <p>{agent.specialty.clone()}</p>
            </div>
            <div class="agent-meta">
                <span class={thinking_chip(agent.thinking)}>{agent.thinking.label()}</span>
                <span class="model-pill">{agent.model.clone()}</span>
                <span class="muted">{agent.status.clone()}</span>
            </div>
        </article>
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

fn swarm_badge(status: SwarmStatus) -> &'static str {
    match status {
        SwarmStatus::Idle => "slate",
        SwarmStatus::Running => "green",
        SwarmStatus::Stopped => "red",
    }
}
