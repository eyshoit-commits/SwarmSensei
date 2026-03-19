use swarmsensei_core::ModelOption;
use web_sys::HtmlSelectElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ModelRouterProps {
    pub active_provider: String,
    pub active_model: String,
    pub catalog: Vec<ModelOption>,
    pub on_switch_model: Callback<(String, String)>,
}

#[function_component(ModelRouter)]
pub fn model_router(props: &ModelRouterProps) -> Html {
    let selected = use_state(|| format!("{}::{}", props.active_provider, props.active_model));

    let on_change = {
        let selected = selected.clone();
        let on_switch_model = props.on_switch_model.clone();
        Callback::from(move |event: Event| {
            let input: HtmlSelectElement = event.target_unchecked_into();
            let value = input.value();
            selected.set(value.clone());
            if let Some((provider, model)) = value.split_once("::") {
                on_switch_model.emit((provider.to_string(), model.to_string()));
            }
        })
    };

    html! {
        <div class="panel">
            <div class="panel-head">
                <div>
                    <h2>{"Model router"}</h2>
                    <span class="muted">{"Switch provider/model pairs from the shared core catalog"}</span>
                </div>
                <span class="model-pill">{format!("{} / {}", props.active_provider, props.active_model)}</span>
            </div>
            <label class="select-wrap">
                <span>{"Active route"}</span>
                <select onchange={on_change} value={(*selected).clone()}>
                    {for props.catalog.iter().map(|option| {
                        let key = format!("{}::{}", option.provider, option.model);
                        html! {
                            <option value={key.clone()}>{format!("{} / {} — {}", option.provider, option.model, option.capability)}</option>
                        }
                    })}
                </select>
            </label>
            <div class="source-list top-gap">
                {for props.catalog.iter().map(|option| html! {
                    <article class="source-card compact">
                        <p class="eyebrow small">{option.provider.clone()}</p>
                        <h3>{option.model.clone()}</h3>
                        <p>{option.capability.clone()}</p>
                    </article>
                })}
            </div>
        </div>
    }
}
