use swarmsensei_core::{ApprovalRequest, ApprovalStatus};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct DecisionGateProps {
    pub approvals: Vec<ApprovalRequest>,
    pub on_decide: Callback<(usize, bool)>,
}

#[function_component(DecisionGate)]
pub fn decision_gate(props: &DecisionGateProps) -> Html {
    let pending = props
        .approvals
        .iter()
        .find(|approval| approval.status == ApprovalStatus::Pending)
        .cloned();

    html! {
        <div class="panel">
            <div class="panel-head">
                <div>
                    <h2>{"Decision gate"}</h2>
                    <span class="muted">{"Human-in-the-loop approvals for risky or ambiguous actions"}</span>
                </div>
            </div>
            {
                if let Some(approval) = pending {
                    render_pending(&approval, &props.on_decide)
                } else {
                    html! { <p class="muted">{"No pending approvals. Governance is clear to proceed."}</p> }
                }
            }
            <div class="subsection">
                <h3>{"Recent decisions"}</h3>
                <div class="audit-list">
                    {for props.approvals.iter().map(|approval| html! {
                        <article class="source-card compact">
                            <p class="eyebrow small">{approval.requested_by.clone()}</p>
                            <h3>{approval.title.clone()}</h3>
                            <p>{approval.summary.clone()}</p>
                            <span class={classes!("badge", status_badge(approval.status))}>{approval.status.label()}</span>
                        </article>
                    })}
                </div>
            </div>
        </div>
    }
}

fn render_pending(approval: &ApprovalRequest, on_decide: &Callback<(usize, bool)>) -> Html {
    let accept = {
        let on_decide = on_decide.clone();
        let id = approval.id;
        Callback::from(move |_| on_decide.emit((id, true)))
    };

    let reject = {
        let on_decide = on_decide.clone();
        let id = approval.id;
        Callback::from(move |_| on_decide.emit((id, false)))
    };

    html! {
        <div class="decision-card">
            <p class="eyebrow">{approval.requested_by.clone()}</p>
            <h3>{approval.title.clone()}</h3>
            <p>{approval.summary.clone()}</p>
            <div class="inline-actions">
                <button class="button" onclick={accept}>{"Accept"}</button>
                <button class="button subtle" onclick={reject}>{"Reject"}</button>
            </div>
        </div>
    }
}

fn status_badge(status: ApprovalStatus) -> &'static str {
    match status {
        ApprovalStatus::Pending => "amber",
        ApprovalStatus::Accepted => "green",
        ApprovalStatus::Rejected => "red",
    }
}
