use swarmsensei_core::{MemoryEntry, MemoryKind};
use web_sys::{HtmlInputElement, HtmlSelectElement};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct MemoryGraphProps {
    pub memory: Vec<MemoryEntry>,
    pub on_append_memory: Callback<(String, MemoryKind, String)>,
}

#[function_component(MemoryGraph)]
pub fn memory_graph(props: &MemoryGraphProps) -> Html {
    let branch = use_state(|| "main".to_string());
    let kind = use_state(|| MemoryKind::Commit);
    let summary = use_state(|| "Record the latest swarm orchestration checkpoint.".to_string());

    let on_branch_input = {
        let branch = branch.clone();
        Callback::from(move |event: InputEvent| {
            let input: HtmlInputElement = event.target_unchecked_into();
            branch.set(input.value());
        })
    };

    let on_kind_change = {
        let kind = kind.clone();
        Callback::from(move |event: Event| {
            let input: HtmlSelectElement = event.target_unchecked_into();
            let next_kind = match input.value().as_str() {
                "branch" => MemoryKind::Branch,
                "merge" => MemoryKind::Merge,
                _ => MemoryKind::Commit,
            };
            kind.set(next_kind);
        })
    };

    let on_summary_input = {
        let summary = summary.clone();
        Callback::from(move |event: InputEvent| {
            let input: HtmlInputElement = event.target_unchecked_into();
            summary.set(input.value());
        })
    };

    let on_submit = {
        let branch = branch.clone();
        let kind = kind.clone();
        let summary = summary.clone();
        let on_append_memory = props.on_append_memory.clone();
        Callback::from(move |_| {
            if branch.trim().is_empty() || summary.trim().is_empty() {
                return;
            }
            on_append_memory.emit(((*branch).clone(), *kind, (*summary).clone()));
        })
    };

    html! {
        <div class="panel">
            <div class="panel-head">
                <div>
                    <h2>{"Memory graph"}</h2>
                    <span class="muted">{"Append commits, branches, and merges backed by the shared memory state"}</span>
                </div>
            </div>
            <div class="compose-grid single">
                <input class="text-input" value={(*branch).clone()} oninput={on_branch_input} placeholder="Branch name" />
                <select onchange={on_kind_change} value={kind.label()}>
                    {for MemoryKind::ALL.into_iter().map(|entry| html! { <option value={entry.label()}>{entry.label()}</option> })}
                </select>
                <input class="text-input" value={(*summary).clone()} oninput={on_summary_input} placeholder="Memory summary" />
                <button class="button" onclick={on_submit}>{"Append memory"}</button>
            </div>
            <div class="subsection">
                {for props.memory.iter().map(|entry| html! {
                    <div class="memory-row">
                        <span class="memory-kind">{entry.kind.label()}</span>
                        <div>
                            <strong>{entry.branch.clone()}</strong>
                            <p>{entry.summary.clone()}</p>
                        </div>
                    </div>
                })}
            </div>
        </div>
    }
}
