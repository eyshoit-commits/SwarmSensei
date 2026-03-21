use serde::{Deserialize, Serialize};

use crate::{CapabilityArea, SourceFeature};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum RuntimeKind {
    Wasm,
    Container,
    Native,
}

impl RuntimeKind {
    fn label(self) -> &'static str {
        match self {
            Self::Wasm => "WASM",
            Self::Container => "Container",
            Self::Native => "Native",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum JobStatus {
    Queued,
    Running,
    Completed,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommandSpec {
    pub program: String,
    pub args: Vec<String>,
    pub cwd: String,
    pub timeout_secs: u32,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct JobReceipt {
    pub id: String,
    pub runtime: RuntimeKind,
    pub status: JobStatus,
    pub command: CommandSpec,
    pub artifact_count: usize,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SandboxState {
    pub runtimes: Vec<RuntimeKind>,
    pub jobs: Vec<JobReceipt>,
    pub receipt_store: String,
}

impl SandboxState {
    pub fn demo() -> Self {
        Self {
            runtimes: vec![
                RuntimeKind::Wasm,
                RuntimeKind::Container,
                RuntimeKind::Native,
            ],
            jobs: vec![JobReceipt {
                id: "job-operator-compile".to_string(),
                runtime: RuntimeKind::Wasm,
                status: JobStatus::Completed,
                command: CommandSpec {
                    program: "cargo".to_string(),
                    args: vec![
                        "check".to_string(),
                        "-p".to_string(),
                        "swarmsensei-core".to_string(),
                    ],
                    cwd: "/workspace/SwarmSensei".to_string(),
                    timeout_secs: 120,
                },
                artifact_count: 3,
            }],
            receipt_store: "receipts://sandbox/jobs".to_string(),
        }
    }

    pub fn capabilities(&self) -> Vec<SourceFeature> {
        vec![SourceFeature {
            source: "agentkernel".to_string(),
            area: CapabilityArea::Sandbox,
            feature: format!(
                "Isolated jobs across {} runtimes with persistent receipts",
                self.runtimes.len()
            ),
            outcome: format!(
                "{} command execution with {} stored receipt stream",
                self.jobs
                    .first()
                    .map(|job| job.runtime.label())
                    .unwrap_or("Unknown"),
                self.receipt_store
            ),
        }]
    }
}
