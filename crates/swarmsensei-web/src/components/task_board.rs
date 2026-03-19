use swarmsensei_core::{TaskStatus, WorkItem};
use web_sys::{HtmlInputElement, HtmlSelectElement};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct TaskBoardProps {
    pub tasks: Vec<WorkItem>,
    pub owners: Vec<String>,
    pub on_create_task: Callback<(String, String, String)>,
    pub on_assign_task: Callback<(usize, String)>,
    pub on_change_status: Callback<(usize, TaskStatus)>,
}

#[function_component(TaskBoard)]
pub fn task_board(props: &TaskBoardProps) -> Html {
    let title = use_state(String::new);
    let owner = use_state(|| {
        props
            .owners
            .first()
            .cloned()
            .unwrap_or_else(|| "Lead".into())
    });
    let lane = use_state(|| "Swarm".to_string());

    let on_title_input = {
        let title = title.clone();
        Callback::from(move |event: InputEvent| {
            let input: HtmlInputElement = event.target_unchecked_into();
            title.set(input.value());
        })
    };

    let on_owner_change = {
        let owner = owner.clone();
        Callback::from(move |event: Event| {
            let input: HtmlSelectElement = event.target_unchecked_into();
            owner.set(input.value());
        })
    };

    let on_lane_change = {
        let lane = lane.clone();
        Callback::from(move |event: Event| {
            let input: HtmlSelectElement = event.target_unchecked_into();
            lane.set(input.value());
        })
    };

    let on_submit = {
        let title = title.clone();
        let owner = owner.clone();
        let lane = lane.clone();
        let on_create_task = props.on_create_task.clone();
        Callback::from(move |_| {
            let trimmed = title.trim().to_string();
            if trimmed.is_empty() {
                return;
            }
            on_create_task.emit((trimmed, (*owner).clone(), (*lane).clone()));
            title.set(String::new());
        })
    };

    html! {
        <div class="panel tall">
            <div class="panel-head">
                <div>
                    <h2>{"Task board"}</h2>
                    <span class="muted">{"Create, assign, and move work items through approval-aware states"}</span>
                </div>
            </div>
            <div class="compose-grid">
                <input class="text-input" placeholder="New task title" value={(*title).clone()} oninput={on_title_input} />
                <select onchange={on_owner_change} value={(*owner).clone()}>
                    {for props.owners.iter().map(|entry| html! { <option value={entry.clone()}>{entry.clone()}</option> })}
                </select>
                <select onchange={on_lane_change} value={(*lane).clone()}>
                    {for ["Sandbox", "Models", "Swarm", "Governance", "Memory", "Interface"].into_iter().map(|entry| html! { <option value={entry}>{entry}</option> })}
                </select>
                <button class="button" onclick={on_submit}>{"Create task"}</button>
            </div>
            <div class="task-list">
                {for props.tasks.iter().map(|task| render_task(task, &props.owners, &props.on_assign_task, &props.on_change_status))}
            </div>
        </div>
    }
}

fn render_task(
    task: &WorkItem,
    owners: &[String],
    on_assign_task: &Callback<(usize, String)>,
    on_change_status: &Callback<(usize, TaskStatus)>,
) -> Html {
    let task_id = task.id;
    let assign_callback = {
        let on_assign_task = on_assign_task.clone();
        Callback::from(move |event: Event| {
            let input: HtmlSelectElement = event.target_unchecked_into();
            on_assign_task.emit((task_id, input.value()));
        })
    };

    let status_callback = {
        let on_change_status = on_change_status.clone();
        Callback::from(move |event: Event| {
            let input: HtmlSelectElement = event.target_unchecked_into();
            let status = match input.value().as_str() {
                "Todo" => TaskStatus::Todo,
                "In Progress" => TaskStatus::InProgress,
                "Review" => TaskStatus::Review,
                "Blocked" => TaskStatus::Blocked,
                _ => TaskStatus::Done,
            };
            on_change_status.emit((task_id, status));
        })
    };

    html! {
        <article class="task-row stacked">
            <div class="task-topline">
                <div>
                    <h3>{task.title.clone()}</h3>
                    <p class="muted">{format!("Lane: {}", task.lane)}</p>
                </div>
                <span class={classes!("badge", badge_class(task.status))}>{task.status.label()}</span>
            </div>
            <div class="inline-form two-up">
                <label class="select-wrap grow">
                    <span>{"Owner"}</span>
                    <select onchange={assign_callback} value={task.owner.clone()}>
                        {for owners.iter().map(|owner| html! { <option value={owner.clone()}>{owner.clone()}</option> })}
                    </select>
                </label>
                <label class="select-wrap grow">
                    <span>{"Status"}</span>
                    <select onchange={status_callback} value={task.status.label()}>
                        {for TaskStatus::ALL.into_iter().map(|status| html! { <option value={status.label()}>{status.label()}</option> })}
                    </select>
                </label>
            </div>
        </article>
    }
}

fn badge_class(status: TaskStatus) -> &'static str {
    match status {
        TaskStatus::Todo => "slate",
        TaskStatus::InProgress => "blue",
        TaskStatus::Review => "amber",
        TaskStatus::Blocked => "red",
        TaskStatus::Done => "green",
    }
}
