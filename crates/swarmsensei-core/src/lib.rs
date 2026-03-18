use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum TaskStatus {
    Todo,
    InProgress,
    Review,
    Blocked,
    Done,
}

impl TaskStatus {
    pub fn label(self) -> &'static str {
        match self {
            Self::Todo => "Todo",
            Self::InProgress => "In Progress",
            Self::Review => "Review",
            Self::Blocked => "Blocked",
            Self::Done => "Done",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ThinkingLevel {
    Off,
    Low,
    Medium,
    High,
}

impl ThinkingLevel {
    pub fn label(self) -> &'static str {
        match self {
            Self::Off => "Off",
            Self::Low => "Low",
            Self::Medium => "Medium",
            Self::High => "High",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum CapabilityArea {
    Sandbox,
    Teaming,
    Models,
    Swarm,
    Governance,
    HumanLoop,
    Memory,
    Workflow,
    Interface,
    CodingHarness,
}

impl CapabilityArea {
    pub fn label(self) -> &'static str {
        match self {
            Self::Sandbox => "Execution Kernel",
            Self::Teaming => "Team Mesh",
            Self::Models => "Model Router",
            Self::Swarm => "Swarm Lanes",
            Self::Governance => "Policy Guard",
            Self::HumanLoop => "Decision Gate",
            Self::Memory => "Memory Graph",
            Self::Workflow => "Workflow Engine",
            Self::Interface => "Workspace Console",
            Self::CodingHarness => "Harness SDK",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SourceFeature {
    pub source: &'static str,
    pub area: CapabilityArea,
    pub feature: &'static str,
    pub outcome: &'static str,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AgentProfile {
    pub name: &'static str,
    pub specialty: &'static str,
    pub model: &'static str,
    pub thinking: ThinkingLevel,
    pub status: &'static str,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkItem {
    pub title: &'static str,
    pub owner: &'static str,
    pub status: TaskStatus,
    pub lane: &'static str,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PolicyRule {
    pub name: &'static str,
    pub effect: &'static str,
    pub scope: &'static str,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemoryEntry {
    pub branch: &'static str,
    pub summary: &'static str,
    pub kind: &'static str,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Scenario {
    pub headline: &'static str,
    pub summary: &'static str,
    pub active_model: &'static str,
    pub team_mode: &'static str,
    pub agents: Vec<AgentProfile>,
    pub tasks: Vec<WorkItem>,
    pub rules: Vec<PolicyRule>,
    pub memory: Vec<MemoryEntry>,
    pub sources: Vec<SourceFeature>,
}

impl Scenario {
    pub fn completed_tasks(&self) -> usize {
        self.tasks
            .iter()
            .filter(|task| task.status == TaskStatus::Done)
            .count()
    }

    pub fn active_agents(&self) -> usize {
        self.agents
            .iter()
            .filter(|agent| agent.status != "Idle")
            .count()
    }

    pub fn governance_rules(&self) -> usize {
        self.rules.len()
    }
}

pub fn sample_scenario() -> Scenario {
    Scenario {
        headline: "SwarmSensei Control Room",
        summary: "A mono Rust/WASM control room that combines the Execution Kernel, Team Mesh, Model Router, Policy Guard, Decision Gate, Memory Graph, and a rich workspace console.",
        active_model: "nvidia-nim/deepseek-ai/deepseek-v3.2",
        team_mode: "Parallel Team Mesh with Decision Gate checkpoints",
        agents: vec![
            AgentProfile {
                name: "Lead",
                specialty: "Roadmapping & approvals",
                model: "openai/gpt-5",
                thinking: ThinkingLevel::High,
                status: "Supervising",
            },
            AgentProfile {
                name: "Scout",
                specialty: "Discovery & dependency tracing",
                model: "google/gemini-2.5-flash",
                thinking: ThinkingLevel::Low,
                status: "Mapping",
            },
            AgentProfile {
                name: "Builder",
                specialty: "Implementation",
                model: "anthropic/claude-opus-4.5",
                thinking: ThinkingLevel::High,
                status: "Coding",
            },
            AgentProfile {
                name: "Reviewer",
                specialty: "Quality & Policy Guard review",
                model: "nvidia-nim/z-ai/glm5",
                thinking: ThinkingLevel::Medium,
                status: "Auditing",
            },
            AgentProfile {
                name: "Archivist",
                specialty: "Memory Graph synthesis",
                model: "openai/gpt-5-mini",
                thinking: ThinkingLevel::Medium,
                status: "Committing",
            },
            AgentProfile {
                name: "Operator",
                specialty: "Execution Kernel runs",
                model: "google/gemini-2.5-flash",
                thinking: ThinkingLevel::Off,
                status: "Running",
            },
        ],
        tasks: vec![
            WorkItem {
                title: "Spin up isolated Rust/WASM workspace",
                owner: "Operator",
                status: TaskStatus::Done,
                lane: "Execution Kernel",
            },
            WorkItem {
                title: "Route heavy tasks through Model Router",
                owner: "Lead",
                status: TaskStatus::InProgress,
                lane: "Model Router",
            },
            WorkItem {
                title: "Scout parallel refactor candidates",
                owner: "Scout",
                status: TaskStatus::Review,
                lane: "Swarm Lanes",
            },
            WorkItem {
                title: "Review DLP hits before export",
                owner: "Reviewer",
                status: TaskStatus::Blocked,
                lane: "Policy Guard",
            },
            WorkItem {
                title: "Checkpoint architecture decisions",
                owner: "Archivist",
                status: TaskStatus::Done,
                lane: "Memory Graph",
            },
        ],
        rules: vec![
            PolicyRule {
                name: "Block secret exfiltration",
                effect: "Mask tokens and deny suspicious outbound content",
                scope: "DLP",
            },
            PolicyRule {
                name: "Require approval on destructive commands",
                effect: "Decision Gate review before file deletion or git push",
                scope: "Approval",
            },
            PolicyRule {
                name: "Protect Policy Guard config",
                effect: "Agents cannot mutate policy files",
                scope: "RBAC",
            },
            PolicyRule {
                name: "Verification gate",
                effect: "Commits require passing checks",
                scope: "Workflow",
            },
        ],
        memory: vec![
            MemoryEntry {
                branch: "main",
                kind: "commit",
                summary: "Initialized the workspace and recorded the feature lineage notes.",
            },
            MemoryEntry {
                branch: "research/model-router",
                kind: "branch",
                summary: "Explored provider-specific model routing and reasoning controls.",
            },
            MemoryEntry {
                branch: "main",
                kind: "merge",
                summary: "Merged Policy Guard, Decision Gate, and Swarm Lanes orchestration into the shared control plane.",
            },
        ],
        sources: source_features(),
    }
}

pub fn source_features() -> Vec<SourceFeature> {
    vec![
        SourceFeature {
            source: "Execution Kernel",
            area: CapabilityArea::Sandbox,
            feature: "Isolated command execution with runtime awareness",
            outcome: "Safe task runs with receipts and automatic environment detection",
        },
        SourceFeature {
            source: "Team Mesh",
            area: CapabilityArea::Teaming,
            feature: "Parallel specialist agents with a shared task board",
            outcome: "Lead-and-teammate coordination in one workspace",
        },
        SourceFeature {
            source: "Model Router",
            area: CapabilityArea::Models,
            feature: "Adaptive model search and handoff",
            outcome: "Dynamic routing between fast, efficient, and deep-reasoning models",
        },
        SourceFeature {
            source: "Swarm Lanes",
            area: CapabilityArea::Swarm,
            feature: "Adaptive concurrency and review waves",
            outcome: "Scouting, worker execution, and reviewer flow across the swarm",
        },
        SourceFeature {
            source: "Model Router",
            area: CapabilityArea::Models,
            feature: "Provider-aware model catalog with reasoning controls",
            outcome: "Specialized model inventories surfaced as first-class options",
        },
        SourceFeature {
            source: "Policy Guard",
            area: CapabilityArea::Governance,
            feature: "RBAC, DLP, audit logging, and approval policies",
            outcome: "Policy-aware actions with Decision Gate checkpoints",
        },
        SourceFeature {
            source: "Decision Gate",
            area: CapabilityArea::HumanLoop,
            feature: "Interactive structured decision prompts",
            outcome: "Human approval moments for ambiguous or risky steps",
        },
        SourceFeature {
            source: "Memory Graph",
            area: CapabilityArea::Memory,
            feature: "Versioned branches, merges, and context snapshots",
            outcome: "Persistent context and milestone tracking across the swarm",
        },
        SourceFeature {
            source: "Workflow Engine",
            area: CapabilityArea::Workflow,
            feature: "Execution-phase guidance, TDD discipline, and subagent support",
            outcome: "Verification gates and guided delivery flow",
        },
        SourceFeature {
            source: "Workspace Console",
            area: CapabilityArea::Interface,
            feature: "Browser workspace for chat, diffs, plans, and tasks",
            outcome: "A single cockpit for collaborative delivery",
        },
        SourceFeature {
            source: "Harness SDK",
            area: CapabilityArea::CodingHarness,
            feature: "Minimal extensible coding-agent harness",
            outcome: "Composable tool foundations for the mono app",
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scenario_metrics_are_consistent() {
        let scenario = sample_scenario();
        assert_eq!(scenario.completed_tasks(), 2);
        assert_eq!(scenario.active_agents(), 6);
        assert_eq!(scenario.governance_rules(), 4);
    }

    #[test]
    fn source_catalog_covers_all_capability_buckets() {
        let features = source_features();
        assert!(features
            .iter()
            .any(|feature| feature.area == CapabilityArea::Sandbox));
        assert!(features
            .iter()
            .any(|feature| feature.area == CapabilityArea::Teaming));
        assert!(features
            .iter()
            .any(|feature| feature.area == CapabilityArea::Governance));
        assert!(features
            .iter()
            .any(|feature| feature.area == CapabilityArea::Interface));
    }
}
