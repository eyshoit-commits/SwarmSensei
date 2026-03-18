use swarmsensei_core::{ApprovalRequest, ApprovalStatus, AuditEntry, PolicyRule};
use web_sys::{HtmlInputElement, HtmlTextAreaElement};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct GovernancePanelProps {
    pub rules: Vec<PolicyRule>,
    pub approvals: Vec<ApprovalRequest>,
    pub audit_log: Vec<AuditEntry>,
    pub on_open_approval: Callback<(String, String, String)>,
}

#[function_component(GovernancePanel)]
pub fn governance_panel(props: &GovernancePanelProps) -> Html {
    let title = use_state(|| "Approve production cutover".to_string());
    let requester = use_state(|| "Lead".to_string());
    let summary =
        use_state(|| "Promote the latest swarm plan after verification checks pass.".to_string());

    let on_title_input = {
        let title = title.clone();
        Callback::from(move |event: InputEvent| {
            let input: HtmlInputElement = event.target_unchecked_into();
            title.set(input.value());
        })
    };

    let on_requester_input = {
        let requester = requester.clone();
        Callback::from(move |event: InputEvent| {
            let input: HtmlInputElement = event.target_unchecked_into();
            requester.set(input.value());
        })
    };

    let on_summary_input = {
        let summary = summary.clone();
        Callback::from(move |event: InputEvent| {
            let input: HtmlTextAreaElement = event.target_unchecked_into();
            summary.set(input.value());
        })
    };

    let on_submit = {
        let title = title.clone();
        let requester = requester.clone();
        let summary = summary.clone();
        let on_open_approval = props.on_open_approval.clone();
        Callback::from(move |_| {
            if title.trim().is_empty() || requester.trim().is_empty() || summary.trim().is_empty() {
                return;
            }
            on_open_approval.emit(((*title).clone(), (*requester).clone(), (*summary).clone()));
        })
    };

    html! {
        <div class="panel tall">
            <div class="panel-head">
                <div>
                    <h2>{"Governance panel"}</h2>
                    <span class="muted">{"Open approvals, inspect policy rules, and review audit history"}</span>
                </div>
                <span class={classes!("badge", pending_badge(&props.approvals))}>{format!("{} pending", props.approvals.iter().filter(|item| item.status == ApprovalStatus::Pending).count())}</span>
            </div>
            <div class="subsection">
                <h3>{"Policy rules"}</h3>
                {for props.rules.iter().map(|rule| html! {
                    <div class="rule-row stacked-card">
                        <strong>{rule.name.clone()}</strong>
                        <p>{format!("{} — {}", rule.scope, rule.effect)}</p>
                    </div>
                })}
            </div>
            <div class="subsection">
                <h3>{"Open approval request"}</h3>
                <div class="compose-grid single">
                    <input class="text-input" value={(*title).clone()} oninput={on_title_input} placeholder="Approval title" />
                    <input class="text-input" value={(*requester).clone()} oninput={on_requester_input} placeholder="Requested by" />
                    <textarea class="text-area" value={(*summary).clone()} oninput={on_summary_input} placeholder="Why approval is needed" />
                    <button class="button" onclick={on_submit}>{"Open approval"}</button>
                </div>
            </div>
            <div class="subsection">
                <h3>{"Audit log"}</h3>
                <div class="audit-list">
                    {for props.audit_log.iter().map(|entry| html! {
                        <article class="memory-row stacked-card">
                            <div>
                                <strong>{format!("{} · {}", entry.actor, entry.action)}</strong>
                                <p>{entry.detail.clone()}</p>
                            </div>
                        </article>
                    })}
                </div>
            </div>
        </div>
    }
}

fn pending_badge(approvals: &[ApprovalRequest]) -> &'static str {
    if approvals
        .iter()
        .any(|approval| approval.status == ApprovalStatus::Pending)
    {
        "amber"
    } else {
        "green"
    }
}
