use serde::{Deserialize, Serialize};

pub mod governance;
pub mod human_loop;
pub mod memory;
pub mod models;
pub mod sandbox;
pub mod swarm;
pub mod team;
pub mod workflow;

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
    pub source: String,
    pub area: CapabilityArea,
    pub feature: String,
    pub outcome: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AgentProfile {
    pub name: String,
    pub specialty: String,
    pub model: String,
    pub thinking: ThinkingLevel,
    pub status: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkItem {
    pub title: String,
    pub owner: String,
    pub status: TaskStatus,
    pub lane: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PolicyRule {
    pub name: String,
    pub effect: String,
    pub scope: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemoryEntry {
    pub branch: String,
    pub summary: String,
    pub kind: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Scenario {
    pub headline: String,
    pub summary: String,
    pub active_model: String,
    pub team_mode: String,
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

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeState {
    pub sandbox: sandbox::SandboxState,
    pub team: team::TeamState,
    pub models: models::ModelState,
    pub swarm: swarm::SwarmState,
    pub governance: governance::GovernanceState,
    pub human_loop: human_loop::HumanLoopState,
    pub memory: memory::MemoryState,
    pub workflow: workflow::WorkflowState,
}

impl RuntimeState {
    pub fn assemble() -> Self {
        let models = models::ModelState::demo();
        let sandbox = sandbox::SandboxState::demo();
        let governance = governance::GovernanceState::demo();
        let human_loop = human_loop::HumanLoopState::demo();
        let memory = memory::MemoryState::demo();
        let workflow = workflow::WorkflowState::demo();
        let swarm = swarm::SwarmState::demo();
        let team = team::TeamState::demo(&models.active_model);

        Self {
            sandbox,
            team,
            models,
            swarm,
            governance,
            human_loop,
            memory,
            workflow,
        }
    }

    pub fn sources(&self) -> Vec<SourceFeature> {
        let mut features = Vec::new();
        features.extend(self.sandbox.capabilities());
        features.extend(self.team.capabilities());
        features.extend(self.models.capabilities());
        features.extend(self.swarm.capabilities());
        features.extend(self.governance.capabilities());
        features.extend(self.human_loop.capabilities());
        features.extend(self.memory.capabilities());
        features.extend(self.workflow.capabilities());
        features
    }

    pub fn scenario(&self) -> Scenario {
        Scenario {
            headline: "SwarmSensei Control Room".to_string(),
            summary: "A mono Rust/WASM control room that merges sandbox execution, multi-agent teaming, adaptive model switching, governance, human approvals, memory, and workflow gates.".to_string(),
            active_model: self.models.active_model.clone(),
            team_mode: self.swarm.execution_mode(&self.workflow),
            agents: self.team.agent_profiles(),
            tasks: self.team.task_board_items(),
            rules: self.governance.policy_rules(&self.workflow),
            memory: self.memory.timeline(),
            sources: self.sources(),
        }
    }
}

pub fn runtime_state() -> RuntimeState {
    RuntimeState::assemble()
}

pub fn sample_scenario() -> Scenario {
    runtime_state().scenario()
}

pub fn source_features() -> Vec<SourceFeature> {
    runtime_state().sources()
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
    fn runtime_state_covers_all_retained_capabilities() {
        let features = source_features();
        for area in [
            CapabilityArea::Sandbox,
            CapabilityArea::Teaming,
            CapabilityArea::Models,
            CapabilityArea::Swarm,
            CapabilityArea::Governance,
            CapabilityArea::HumanLoop,
            CapabilityArea::Memory,
            CapabilityArea::Workflow,
        ] {
            assert!(features.iter().any(|feature| feature.area == area));
        }
    }

    #[test]
    fn assembled_runtime_state_links_team_and_models() {
        let state = runtime_state();
        assert!(state
            .team
            .roster
            .agents
            .iter()
            .any(|agent| agent.model == state.models.active_model));
        assert!(state
            .team
            .board
            .items
            .iter()
            .any(|task| task.lane == "Governance" && task.status == TaskStatus::Blocked));
    }
}
