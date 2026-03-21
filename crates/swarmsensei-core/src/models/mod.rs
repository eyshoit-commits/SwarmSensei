use serde::{Deserialize, Serialize};

use crate::{CapabilityArea, SourceFeature};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Provider {
    pub name: String,
    pub family: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ModelDescriptor {
    pub id: String,
    pub provider: String,
    pub strengths: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ModelAlias {
    pub alias: String,
    pub target: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SwitchRule {
    pub trigger: String,
    pub from: String,
    pub to: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ModelState {
    pub providers: Vec<Provider>,
    pub discovered_models: Vec<ModelDescriptor>,
    pub aliases: Vec<ModelAlias>,
    pub switch_rules: Vec<SwitchRule>,
    pub active_model: String,
}

impl ModelState {
    pub fn demo() -> Self {
        Self {
            providers: vec![
                Provider {
                    name: "OpenAI".to_string(),
                    family: "premium reasoning".to_string(),
                },
                Provider {
                    name: "Google".to_string(),
                    family: "fast exploration".to_string(),
                },
                Provider {
                    name: "Anthropic".to_string(),
                    family: "deep implementation".to_string(),
                },
                Provider {
                    name: "NVIDIA NIM".to_string(),
                    family: "self-hosted catalog".to_string(),
                },
            ],
            discovered_models: vec![
                ModelDescriptor {
                    id: "openai/gpt-5".to_string(),
                    provider: "OpenAI".to_string(),
                    strengths: vec!["planning".to_string(), "approvals".to_string()],
                },
                ModelDescriptor {
                    id: "google/gemini-2.5-flash".to_string(),
                    provider: "Google".to_string(),
                    strengths: vec!["discovery".to_string(), "execution".to_string()],
                },
                ModelDescriptor {
                    id: "anthropic/claude-opus-4.5".to_string(),
                    provider: "Anthropic".to_string(),
                    strengths: vec!["implementation".to_string(), "refactors".to_string()],
                },
                ModelDescriptor {
                    id: "nvidia-nim/deepseek-ai/deepseek-v3.2".to_string(),
                    provider: "NVIDIA NIM".to_string(),
                    strengths: vec!["reasoning".to_string(), "tool use".to_string()],
                },
            ],
            aliases: vec![
                ModelAlias {
                    alias: "lead-default".to_string(),
                    target: "openai/gpt-5".to_string(),
                },
                ModelAlias {
                    alias: "deep-review".to_string(),
                    target: "nvidia-nim/deepseek-ai/deepseek-v3.2".to_string(),
                },
            ],
            switch_rules: vec![
                SwitchRule {
                    trigger: "large architectural reasoning".to_string(),
                    from: "google/gemini-2.5-flash".to_string(),
                    to: "nvidia-nim/deepseek-ai/deepseek-v3.2".to_string(),
                },
                SwitchRule {
                    trigger: "implementation spike".to_string(),
                    from: "openai/gpt-5".to_string(),
                    to: "anthropic/claude-opus-4.5".to_string(),
                },
            ],
            active_model: "openai/gpt-5".to_string(),
        }
    }

    pub fn capabilities(&self) -> Vec<SourceFeature> {
        vec![
            SourceFeature {
                source: "pi-model-switch".to_string(),
                area: CapabilityArea::Models,
                feature: format!(
                    "{} providers, {} discovered models, and {} switch rules",
                    self.providers.len(),
                    self.discovered_models.len(),
                    self.switch_rules.len()
                ),
                outcome: format!("Dynamic routing keeps {} active for lead work", self.active_model),
            },
            SourceFeature {
                source: "pi-nvidia-nim".to_string(),
                area: CapabilityArea::Models,
                feature: format!(
                    "{} aliases expose specialized routing targets",
                    self.aliases.len()
                ),
                outcome: "NVIDIA NIM catalog remains a first-class model family with swap-ready descriptors"
                    .to_string(),
            },
        ]
    }
}
