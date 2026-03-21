use serde::{Deserialize, Serialize};

use crate::{AgentProfile, CapabilityArea, SourceFeature, TaskStatus, ThinkingLevel, WorkItem};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TeamAgent {
    pub name: String,
    pub specialty: String,
    pub model: String,
    pub thinking: ThinkingLevel,
    pub status: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TeamMessage {
    pub from: String,
    pub to: String,
    pub subject: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Assignment {
    pub task_title: String,
    pub assignee: String,
    pub priority: u8,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SharedTaskBoard {
    pub items: Vec<WorkItem>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AgentRoster {
    pub mode: String,
    pub agents: Vec<TeamAgent>,
    pub messages: Vec<TeamMessage>,
    pub assignments: Vec<Assignment>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TeamState {
    pub roster: AgentRoster,
    pub board: SharedTaskBoard,
}

impl TeamState {
    pub fn demo(active_model: &str) -> Self {
        let agents = vec![
            TeamAgent {
                name: "Lead".to_string(),
                specialty: "Roadmapping & approvals".to_string(),
                model: active_model.to_string(),
                thinking: ThinkingLevel::High,
                status: "Supervising".to_string(),
            },
            TeamAgent {
                name: "Scout".to_string(),
                specialty: "Discovery & dependency tracing".to_string(),
                model: "google/gemini-2.5-flash".to_string(),
                thinking: ThinkingLevel::Low,
                status: "Mapping".to_string(),
            },
            TeamAgent {
                name: "Builder".to_string(),
                specialty: "Implementation".to_string(),
                model: "anthropic/claude-opus-4.5".to_string(),
                thinking: ThinkingLevel::High,
                status: "Coding".to_string(),
            },
            TeamAgent {
                name: "Reviewer".to_string(),
                specialty: "Quality & governance".to_string(),
                model: "nvidia-nim/z-ai/glm5".to_string(),
                thinking: ThinkingLevel::Medium,
                status: "Auditing".to_string(),
            },
            TeamAgent {
                name: "Archivist".to_string(),
                specialty: "Memory synthesis".to_string(),
                model: "openai/gpt-5-mini".to_string(),
                thinking: ThinkingLevel::Medium,
                status: "Committing".to_string(),
            },
            TeamAgent {
                name: "Operator".to_string(),
                specialty: "Sandbox execution".to_string(),
                model: "google/gemini-2.5-flash".to_string(),
                thinking: ThinkingLevel::Off,
                status: "Running".to_string(),
            },
        ];

        let board = SharedTaskBoard {
            items: vec![
                WorkItem {
                    title: "Spin up isolated Rust/WASM workspace".to_string(),
                    owner: "Operator".to_string(),
                    status: TaskStatus::Done,
                    lane: "Sandbox".to_string(),
                },
                WorkItem {
                    title: "Route heavy tasks to premium model".to_string(),
                    owner: "Lead".to_string(),
                    status: TaskStatus::InProgress,
                    lane: "Models".to_string(),
                },
                WorkItem {
                    title: "Scout parallel refactor candidates".to_string(),
                    owner: "Scout".to_string(),
                    status: TaskStatus::Review,
                    lane: "Swarm".to_string(),
                },
                WorkItem {
                    title: "Review DLP hits before export".to_string(),
                    owner: "Reviewer".to_string(),
                    status: TaskStatus::Blocked,
                    lane: "Governance".to_string(),
                },
                WorkItem {
                    title: "Checkpoint architecture decisions".to_string(),
                    owner: "Archivist".to_string(),
                    status: TaskStatus::Done,
                    lane: "Memory".to_string(),
                },
            ],
        };

        Self {
            roster: AgentRoster {
                mode: "Parallel swarm with human approval checkpoints".to_string(),
                agents,
                messages: vec![
                    TeamMessage {
                        from: "Lead".to_string(),
                        to: "Builder".to_string(),
                        subject: "Implement runtime-backed modules".to_string(),
                    },
                    TeamMessage {
                        from: "Reviewer".to_string(),
                        to: "Lead".to_string(),
                        subject: "Escalate export after DLP review".to_string(),
                    },
                ],
                assignments: vec![
                    Assignment {
                        task_title: "Route heavy tasks to premium model".to_string(),
                        assignee: "Lead".to_string(),
                        priority: 1,
                    },
                    Assignment {
                        task_title: "Scout parallel refactor candidates".to_string(),
                        assignee: "Scout".to_string(),
                        priority: 2,
                    },
                ],
            },
            board,
        }
    }

    pub fn agent_profiles(&self) -> Vec<AgentProfile> {
        self.roster
            .agents
            .iter()
            .map(|agent| AgentProfile {
                name: agent.name.clone(),
                specialty: agent.specialty.clone(),
                model: agent.model.clone(),
                thinking: agent.thinking,
                status: agent.status.clone(),
            })
            .collect()
    }

    pub fn task_board_items(&self) -> Vec<WorkItem> {
        self.board.items.clone()
    }

    pub fn capabilities(&self) -> Vec<SourceFeature> {
        vec![SourceFeature {
            source: "pi-teams".to_string(),
            area: CapabilityArea::Teaming,
            feature: format!(
                "{}-agent roster with {} active assignments",
                self.roster.agents.len(),
                self.roster.assignments.len()
            ),
            outcome: format!(
                "Shared board, {} routed messages, and {} execution mode",
                self.roster.messages.len(),
                self.roster.mode
            ),
        }]
    }
}
