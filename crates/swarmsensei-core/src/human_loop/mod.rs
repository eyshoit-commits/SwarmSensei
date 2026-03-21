use serde::{Deserialize, Serialize};

use crate::{CapabilityArea, SourceFeature};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PromptOption {
    pub label: String,
    pub impact: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DecisionPrompt {
    pub title: String,
    pub question: String,
    pub options: Vec<PromptOption>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserResponse {
    pub prompt_title: String,
    pub selected: String,
    pub rationale: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct HumanLoopState {
    pub prompts: Vec<DecisionPrompt>,
    pub responses: Vec<UserResponse>,
}

impl HumanLoopState {
    pub fn demo() -> Self {
        Self {
            prompts: vec![DecisionPrompt {
                title: "Approve risky export".to_string(),
                question: "Should the reviewer release the blocked export after DLP review?"
                    .to_string(),
                options: vec![
                    PromptOption {
                        label: "Approve".to_string(),
                        impact: "Unblocks publication and records the approval".to_string(),
                    },
                    PromptOption {
                        label: "Deny".to_string(),
                        impact: "Keeps the export blocked and requests remediation".to_string(),
                    },
                ],
            }],
            responses: vec![UserResponse {
                prompt_title: "Approve risky export".to_string(),
                selected: "Approve".to_string(),
                rationale: "Reviewer confirmed the payload is scrubbed and traceable".to_string(),
            }],
        }
    }

    pub fn capabilities(&self) -> Vec<SourceFeature> {
        vec![SourceFeature {
            source: "pi-ask-user".to_string(),
            area: CapabilityArea::HumanLoop,
            feature: format!(
                "{} structured prompts with {} recorded user responses",
                self.prompts.len(),
                self.responses.len()
            ),
            outcome: "Risky or ambiguous actions flow through explicit human decisions".to_string(),
        }]
    }
}
