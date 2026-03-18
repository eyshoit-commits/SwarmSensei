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
            Self::Sandbox => "Secure sandboxes",
            Self::Teaming => "Team coordination",
            Self::Models => "Model routing",
            Self::Swarm => "Ant-colony swarms",
            Self::Governance => "Governance & audit",
            Self::HumanLoop => "Ask-user decisions",
            Self::Memory => "Versioned memory",
            Self::Workflow => "Workflow enforcement",
            Self::Interface => "Rich web workspace",
            Self::CodingHarness => "Core coding harness",
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
        summary: "A mono Rust/WASM control room that merges sandbox execution, multi-agent teaming, adaptive model switching, governance, human approvals, memory, and a rich web cockpit.",
        active_model: "nvidia-nim/deepseek-ai/deepseek-v3.2",
        team_mode: "Parallel swarm with human approval checkpoints",
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
                specialty: "Quality & governance",
                model: "nvidia-nim/z-ai/glm5",
                thinking: ThinkingLevel::Medium,
                status: "Auditing",
            },
            AgentProfile {
                name: "Archivist",
                specialty: "Memory synthesis",
                model: "openai/gpt-5-mini",
                thinking: ThinkingLevel::Medium,
                status: "Committing",
            },
            AgentProfile {
                name: "Operator",
                specialty: "Sandbox execution",
                model: "google/gemini-2.5-flash",
                thinking: ThinkingLevel::Off,
                status: "Running",
            },
        ],
        tasks: vec![
            WorkItem { title: "Spin up isolated Rust/WASM workspace", owner: "Operator", status: TaskStatus::Done, lane: "Sandbox" },
            WorkItem { title: "Route heavy tasks to premium model", owner: "Lead", status: TaskStatus::InProgress, lane: "Models" },
            WorkItem { title: "Scout parallel refactor candidates", owner: "Scout", status: TaskStatus::Review, lane: "Swarm" },
            WorkItem { title: "Review DLP hits before export", owner: "Reviewer", status: TaskStatus::Blocked, lane: "Governance" },
            WorkItem { title: "Checkpoint architecture decisions", owner: "Archivist", status: TaskStatus::Done, lane: "Memory" },
        ],
        rules: vec![
            PolicyRule { name: "Block secret exfiltration", effect: "Mask tokens and deny suspicious outbound content", scope: "DLP" },
            PolicyRule { name: "Require approval on destructive commands", effect: "Human review before file deletion or git push", scope: "HITL" },
            PolicyRule { name: "Protect governance config", effect: "Agents cannot mutate policy files", scope: "RBAC" },
            PolicyRule { name: "Verification gate", effect: "Commits require passing checks", scope: "Workflow" },
        ],
        memory: vec![
            MemoryEntry { branch: "main", kind: "commit", summary: "Initialized workspace and captured upstream feature map." },
            MemoryEntry { branch: "research/model-routing", kind: "branch", summary: "Explored provider-specific model switching and NIM thinking modes." },
            MemoryEntry { branch: "main", kind: "merge", summary: "Merged governance, human-loop, and ant-colony orchestration into the shared control plane." },
        ],
        sources: source_features(),
    }
}

pub fn source_features() -> Vec<SourceFeature> {
    vec![
        SourceFeature {
            source: "agentkernel",
            area: CapabilityArea::Sandbox,
            feature: "MicroVM-style isolated command execution",
            outcome: "Safe task runs with receipts and runtime auto-detection",
        },
        SourceFeature {
            source: "pi-teams",
            area: CapabilityArea::Teaming,
            feature: "Parallel specialist agents with a shared task board",
            outcome: "Lead + teammate coordination in one workspace",
        },
        SourceFeature {
            source: "pi-model-switch",
            area: CapabilityArea::Models,
            feature: "Autonomous model search and switching",
            outcome: "Dynamic routing between cheap, fast, and deep models",
        },
        SourceFeature {
            source: "oh-pi-ant-colony",
            area: CapabilityArea::Swarm,
            feature: "Pheromone-based adaptive concurrency",
            outcome: "Scouting, worker execution, and review waves",
        },
        SourceFeature {
            source: "pi-nvidia-nim",
            area: CapabilityArea::Models,
            feature: "Custom NVIDIA NIM provider with reasoning controls",
            outcome: "NIM catalog surfaced as first-class model options",
        },
        SourceFeature {
            source: "pi-governance",
            area: CapabilityArea::Governance,
            feature: "RBAC, DLP, audit logging, and HITL",
            outcome: "Policy-aware actions with approval checkpoints",
        },
        SourceFeature {
            source: "pi-ask-user",
            area: CapabilityArea::HumanLoop,
            feature: "Interactive structured decisions",
            outcome: "User approval prompts for ambiguous or risky steps",
        },
        SourceFeature {
            source: "pi-brain",
            area: CapabilityArea::Memory,
            feature: "Versioned memory branches and merges",
            outcome: "Persistent context and milestone snapshots",
        },
        SourceFeature {
            source: "pi-superpowers-plus",
            area: CapabilityArea::Workflow,
            feature: "Workflow/TDD enforcement and subagent support",
            outcome: "Guided execution phases and verification gates",
        },
        SourceFeature {
            source: "opencode-chamber",
            area: CapabilityArea::Interface,
            feature: "Web/desktop coding workspace",
            outcome: "Single browser cockpit for chat, diffs, plans, and tasks",
        },
        SourceFeature {
            source: "pi coding agent",
            area: CapabilityArea::CodingHarness,
            feature: "Minimal extensible coding harness",
            outcome: "Composable tool foundation for the whole mono app",
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
