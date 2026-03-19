use serde::{Deserialize, Serialize};

use crate::{CapabilityArea, SourceFeature};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Phase {
    Discover,
    Build,
    Verify,
}

impl Phase {
    pub fn label(self) -> &'static str {
        match self {
            Self::Discover => "discovery",
            Self::Build => "build",
            Self::Verify => "verification",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct VerificationGate {
    pub name: String,
    pub required: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkflowState {
    pub phases: Vec<Phase>,
    pub current_phase: Phase,
    pub gates: Vec<VerificationGate>,
}

impl WorkflowState {
    pub fn demo() -> Self {
        Self {
            phases: vec![Phase::Discover, Phase::Build, Phase::Verify],
            current_phase: Phase::Verify,
            gates: vec![
                VerificationGate {
                    name: "cargo fmt".to_string(),
                    required: true,
                },
                VerificationGate {
                    name: "cargo test".to_string(),
                    required: true,
                },
                VerificationGate {
                    name: "review approval".to_string(),
                    required: true,
                },
            ],
        }
    }

    pub fn capabilities(&self) -> Vec<SourceFeature> {
        vec![SourceFeature {
            source: "pi-superpowers-plus".to_string(),
            area: CapabilityArea::Workflow,
            feature: format!(
                "{} workflow phases with {} verification gates",
                self.phases.len(),
                self.gates.len()
            ),
            outcome: format!(
                "Execution is currently pinned to {}",
                self.current_phase.label()
            ),
        }]
    }
}
