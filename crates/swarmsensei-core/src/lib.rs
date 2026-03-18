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
    pub const ALL: [Self; 5] = [
        Self::Todo,
        Self::InProgress,
        Self::Review,
        Self::Blocked,
        Self::Done,
    ];

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
    pub const ALL: [Self; 10] = [
        Self::Sandbox,
        Self::Teaming,
        Self::Models,
        Self::Swarm,
        Self::Governance,
        Self::HumanLoop,
        Self::Memory,
        Self::Workflow,
        Self::Interface,
        Self::CodingHarness,
    ];

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

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum MemoryKind {
    Commit,
    Branch,
    Merge,
}

impl MemoryKind {
    pub const ALL: [Self; 3] = [Self::Commit, Self::Branch, Self::Merge];

    pub fn label(self) -> &'static str {
        match self {
            Self::Commit => "commit",
            Self::Branch => "branch",
            Self::Merge => "merge",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ApprovalStatus {
    Pending,
    Accepted,
    Rejected,
}

impl ApprovalStatus {
    pub fn label(self) -> &'static str {
        match self {
            Self::Pending => "Pending",
            Self::Accepted => "Accepted",
            Self::Rejected => "Rejected",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum SwarmStatus {
    Idle,
    Running,
    Stopped,
}

impl SwarmStatus {
    pub fn label(self) -> &'static str {
        match self {
            Self::Idle => "Idle",
            Self::Running => "Running",
            Self::Stopped => "Stopped",
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
    pub id: usize,
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
    pub kind: MemoryKind,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ModelOption {
    pub provider: String,
    pub model: String,
    pub capability: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ApprovalRequest {
    pub id: usize,
    pub title: String,
    pub requested_by: String,
    pub summary: String,
    pub status: ApprovalStatus,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuditEntry {
    pub actor: String,
    pub action: String,
    pub detail: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Scenario {
    pub headline: String,
    pub summary: String,
    pub active_provider: String,
    pub active_model: String,
    pub team_mode: String,
    pub swarm_status: SwarmStatus,
    pub agents: Vec<AgentProfile>,
    pub tasks: Vec<WorkItem>,
    pub rules: Vec<PolicyRule>,
    pub memory: Vec<MemoryEntry>,
    pub approvals: Vec<ApprovalRequest>,
    pub audit_log: Vec<AuditEntry>,
    pub model_catalog: Vec<ModelOption>,
    pub sources: Vec<SourceFeature>,
    next_task_id: usize,
    next_approval_id: usize,
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

    pub fn pending_approvals(&self) -> usize {
        self.approvals
            .iter()
            .filter(|approval| approval.status == ApprovalStatus::Pending)
            .count()
    }

    pub fn create_task(
        &mut self,
        title: impl Into<String>,
        owner: impl Into<String>,
        lane: impl Into<String>,
    ) {
        let title = title.into();
        let owner = owner.into();
        let lane = lane.into();
        self.tasks.push(WorkItem {
            id: self.next_task_id,
            title: title.clone(),
            owner: owner.clone(),
            status: TaskStatus::Todo,
            lane: lane.clone(),
        });
        self.next_task_id += 1;
        self.record_audit(
            "Lead",
            "Created task",
            format!("{title} assigned to {owner} in {lane}"),
        );
    }

    pub fn assign_task(&mut self, id: usize, owner: impl Into<String>) {
        let owner = owner.into();
        let mut audit_detail = None;
        if let Some(task) = self.tasks.iter_mut().find(|task| task.id == id) {
            task.owner = owner.clone();
            audit_detail = Some(format!("{} → {}", task.title, owner));
        }
        if let Some(detail) = audit_detail {
            self.record_audit("Lead", "Assigned task", detail);
        }
    }

    pub fn update_task_status(&mut self, id: usize, status: TaskStatus) {
        let mut audit_detail = None;
        if let Some(task) = self.tasks.iter_mut().find(|task| task.id == id) {
            task.status = status;
            audit_detail = Some(format!("{} is now {}", task.title, status.label()));
        }
        if let Some(detail) = audit_detail {
            self.record_audit("Builder", "Changed task status", detail);
        }
    }

    pub fn switch_model(&mut self, provider: impl Into<String>, model: impl Into<String>) {
        self.active_provider = provider.into();
        self.active_model = model.into();
        self.record_audit(
            "Router",
            "Switched active model",
            format!("{} / {}", self.active_provider, self.active_model),
        );
    }

    pub fn open_approval_request(
        &mut self,
        title: impl Into<String>,
        requested_by: impl Into<String>,
        summary: impl Into<String>,
    ) {
        let title = title.into();
        let requested_by = requested_by.into();
        let summary = summary.into();
        self.approvals.push(ApprovalRequest {
            id: self.next_approval_id,
            title: title.clone(),
            requested_by: requested_by.clone(),
            summary: summary.clone(),
            status: ApprovalStatus::Pending,
        });
        self.next_approval_id += 1;
        self.record_audit(
            requested_by,
            "Opened approval request",
            format!("{title}: {summary}"),
        );
    }

    pub fn decide_approval(&mut self, id: usize, accept: bool) {
        let mut audit_title = None;
        if let Some(approval) = self.approvals.iter_mut().find(|approval| approval.id == id) {
            approval.status = if accept {
                ApprovalStatus::Accepted
            } else {
                ApprovalStatus::Rejected
            };
            audit_title = Some(approval.title.clone());
        }
        if let Some(title) = audit_title {
            self.record_audit(
                "Human",
                if accept {
                    "Accepted approval"
                } else {
                    "Rejected approval"
                },
                title,
            );
        }
    }

    pub fn append_memory(
        &mut self,
        branch: impl Into<String>,
        kind: MemoryKind,
        summary: impl Into<String>,
    ) {
        let branch = branch.into();
        let summary = summary.into();
        self.memory.push(MemoryEntry {
            branch: branch.clone(),
            summary: summary.clone(),
            kind,
        });
        self.record_audit(
            "Archivist",
            "Appended memory event",
            format!("{} on {}: {}", kind.label(), branch, summary),
        );
    }

    pub fn start_swarm_run(&mut self) {
        self.swarm_status = SwarmStatus::Running;
        self.record_audit("Operator", "Started swarm run", self.team_mode.clone());
    }

    pub fn stop_swarm_run(&mut self) {
        self.swarm_status = SwarmStatus::Stopped;
        self.record_audit("Operator", "Stopped swarm run", self.team_mode.clone());
    }

    fn record_audit(
        &mut self,
        actor: impl Into<String>,
        action: impl Into<String>,
        detail: impl Into<String>,
    ) {
        self.audit_log.insert(
            0,
            AuditEntry {
                actor: actor.into(),
                action: action.into(),
                detail: detail.into(),
            },
        );
    }
}

pub fn sample_scenario() -> Scenario {
    Scenario {
        headline: "SwarmSensei Control Room".into(),
        summary: "A mono Rust/WASM control room that merges sandbox execution, multi-agent teaming, adaptive model switching, governance, human approvals, memory, and a rich web cockpit.".into(),
        active_provider: "nvidia-nim".into(),
        active_model: "deepseek-ai/deepseek-v3.2".into(),
        team_mode: "Parallel swarm with human approval checkpoints".into(),
        swarm_status: SwarmStatus::Running,
        agents: vec![
            AgentProfile {
                name: "Lead".into(),
                specialty: "Roadmapping & approvals".into(),
                model: "openai/gpt-5".into(),
                thinking: ThinkingLevel::High,
                status: "Supervising".into(),
            },
            AgentProfile {
                name: "Scout".into(),
                specialty: "Discovery & dependency tracing".into(),
                model: "google/gemini-2.5-flash".into(),
                thinking: ThinkingLevel::Low,
                status: "Mapping".into(),
            },
            AgentProfile {
                name: "Builder".into(),
                specialty: "Implementation".into(),
                model: "anthropic/claude-opus-4.5".into(),
                thinking: ThinkingLevel::High,
                status: "Coding".into(),
            },
            AgentProfile {
                name: "Reviewer".into(),
                specialty: "Quality & governance".into(),
                model: "nvidia-nim/z-ai/glm5".into(),
                thinking: ThinkingLevel::Medium,
                status: "Auditing".into(),
            },
            AgentProfile {
                name: "Archivist".into(),
                specialty: "Memory synthesis".into(),
                model: "openai/gpt-5-mini".into(),
                thinking: ThinkingLevel::Medium,
                status: "Committing".into(),
            },
            AgentProfile {
                name: "Operator".into(),
                specialty: "Sandbox execution".into(),
                model: "google/gemini-2.5-flash".into(),
                thinking: ThinkingLevel::Off,
                status: "Running".into(),
            },
        ],
        tasks: vec![
            WorkItem { id: 1, title: "Spin up isolated Rust/WASM workspace".into(), owner: "Operator".into(), status: TaskStatus::Done, lane: "Sandbox".into() },
            WorkItem { id: 2, title: "Route heavy tasks to premium model".into(), owner: "Lead".into(), status: TaskStatus::InProgress, lane: "Models".into() },
            WorkItem { id: 3, title: "Scout parallel refactor candidates".into(), owner: "Scout".into(), status: TaskStatus::Review, lane: "Swarm".into() },
            WorkItem { id: 4, title: "Review DLP hits before export".into(), owner: "Reviewer".into(), status: TaskStatus::Blocked, lane: "Governance".into() },
            WorkItem { id: 5, title: "Checkpoint architecture decisions".into(), owner: "Archivist".into(), status: TaskStatus::Done, lane: "Memory".into() },
        ],
        rules: vec![
            PolicyRule { name: "Block secret exfiltration".into(), effect: "Mask tokens and deny suspicious outbound content".into(), scope: "DLP".into() },
            PolicyRule { name: "Require approval on destructive commands".into(), effect: "Human review before file deletion or git push".into(), scope: "HITL".into() },
            PolicyRule { name: "Protect governance config".into(), effect: "Agents cannot mutate policy files".into(), scope: "RBAC".into() },
            PolicyRule { name: "Verification gate".into(), effect: "Commits require passing checks".into(), scope: "Workflow".into() },
        ],
        memory: vec![
            MemoryEntry { branch: "main".into(), kind: MemoryKind::Commit, summary: "Initialized workspace and captured upstream feature map.".into() },
            MemoryEntry { branch: "research/model-routing".into(), kind: MemoryKind::Branch, summary: "Explored provider-specific model switching and NIM thinking modes.".into() },
            MemoryEntry { branch: "main".into(), kind: MemoryKind::Merge, summary: "Merged governance, human-loop, and ant-colony orchestration into the shared control plane.".into() },
        ],
        approvals: vec![
            ApprovalRequest {
                id: 1,
                title: "Approve audit export".into(),
                requested_by: "Reviewer".into(),
                summary: "Share the latest governance findings with stakeholders.".into(),
                status: ApprovalStatus::Pending,
            },
        ],
        audit_log: vec![
            AuditEntry { actor: "Operator".into(), action: "Started swarm run".into(), detail: "Parallel swarm with human approval checkpoints".into() },
            AuditEntry { actor: "Router".into(), action: "Switched active model".into(), detail: "nvidia-nim / deepseek-ai/deepseek-v3.2".into() },
            AuditEntry { actor: "Reviewer".into(), action: "Opened approval request".into(), detail: "Approve audit export: Share the latest governance findings with stakeholders.".into() },
        ],
        model_catalog: vec![
            ModelOption { provider: "openai".into(), model: "gpt-5".into(), capability: "Deep planning and approvals".into() },
            ModelOption { provider: "openai".into(), model: "gpt-5-mini".into(), capability: "Cheap memory synthesis".into() },
            ModelOption { provider: "anthropic".into(), model: "claude-opus-4.5".into(), capability: "High-context implementation".into() },
            ModelOption { provider: "google".into(), model: "gemini-2.5-flash".into(), capability: "Fast scouting and execution".into() },
            ModelOption { provider: "nvidia-nim".into(), model: "deepseek-ai/deepseek-v3.2".into(), capability: "Reasoning with NIM routing".into() },
            ModelOption { provider: "nvidia-nim".into(), model: "z-ai/glm5".into(), capability: "Governance review and verification".into() },
        ],
        sources: source_features(),
        next_task_id: 6,
        next_approval_id: 2,
    }
}

pub fn source_features() -> Vec<SourceFeature> {
    vec![
        SourceFeature {
            source: "agentkernel".into(),
            area: CapabilityArea::Sandbox,
            feature: "MicroVM-style isolated command execution".into(),
            outcome: "Safe task runs with receipts and runtime auto-detection".into(),
        },
        SourceFeature {
            source: "pi-teams".into(),
            area: CapabilityArea::Teaming,
            feature: "Parallel specialist agents with a shared task board".into(),
            outcome: "Lead + teammate coordination in one workspace".into(),
        },
        SourceFeature {
            source: "pi-model-switch".into(),
            area: CapabilityArea::Models,
            feature: "Autonomous model search and switching".into(),
            outcome: "Dynamic routing between cheap, fast, and deep models".into(),
        },
        SourceFeature {
            source: "oh-pi-ant-colony".into(),
            area: CapabilityArea::Swarm,
            feature: "Pheromone-based adaptive concurrency".into(),
            outcome: "Scouting, worker execution, and review waves".into(),
        },
        SourceFeature {
            source: "pi-nvidia-nim".into(),
            area: CapabilityArea::Models,
            feature: "Custom NVIDIA NIM provider with reasoning controls".into(),
            outcome: "NIM catalog surfaced as first-class model options".into(),
        },
        SourceFeature {
            source: "pi-governance".into(),
            area: CapabilityArea::Governance,
            feature: "RBAC, DLP, audit logging, and HITL".into(),
            outcome: "Policy-aware actions with approval checkpoints".into(),
        },
        SourceFeature {
            source: "pi-ask-user".into(),
            area: CapabilityArea::HumanLoop,
            feature: "Interactive structured decisions".into(),
            outcome: "User approval prompts for ambiguous or risky steps".into(),
        },
        SourceFeature {
            source: "pi-brain".into(),
            area: CapabilityArea::Memory,
            feature: "Versioned memory branches and merges".into(),
            outcome: "Persistent context and milestone snapshots".into(),
        },
        SourceFeature {
            source: "pi-superpowers-plus".into(),
            area: CapabilityArea::Workflow,
            feature: "Workflow/TDD enforcement and subagent support".into(),
            outcome: "Guided execution phases and verification gates".into(),
        },
        SourceFeature {
            source: "opencode-chamber".into(),
            area: CapabilityArea::Interface,
            feature: "Web/desktop coding workspace".into(),
            outcome: "Single browser cockpit for chat, diffs, plans, and tasks".into(),
        },
        SourceFeature {
            source: "pi coding agent".into(),
            area: CapabilityArea::CodingHarness,
            feature: "Minimal extensible coding harness".into(),
            outcome: "Composable tool foundation for the whole mono app".into(),
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
        assert_eq!(scenario.pending_approvals(), 1);
    }

    #[test]
    fn workflow_actions_mutate_shared_state() {
        let mut scenario = sample_scenario();
        scenario.create_task("Refactor Yew app", "Builder", "Interface");
        scenario.assign_task(2, "Builder");
        scenario.update_task_status(2, TaskStatus::Review);
        scenario.switch_model("openai", "gpt-5");
        scenario.open_approval_request("Approve release", "Lead", "Ship the refactor.");
        scenario.decide_approval(1, true);
        scenario.append_memory("release/ui", MemoryKind::Commit, "Captured final UI state.");
        scenario.stop_swarm_run();

        assert_eq!(scenario.tasks.len(), 6);
        assert_eq!(scenario.tasks[1].owner, "Builder");
        assert_eq!(scenario.tasks[1].status, TaskStatus::Review);
        assert_eq!(scenario.active_provider, "openai");
        assert_eq!(scenario.active_model, "gpt-5");
        assert_eq!(scenario.approvals.len(), 2);
        assert_eq!(scenario.approvals[0].status, ApprovalStatus::Accepted);
        assert_eq!(scenario.memory.len(), 4);
        assert_eq!(scenario.swarm_status, SwarmStatus::Stopped);
        assert!(scenario.audit_log.len() >= 8);
    }
}
