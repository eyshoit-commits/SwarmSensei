use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

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

    pub fn can_transition_to(self, next: Self) -> bool {
        if self == next {
            return true;
        }

        matches!(
            (self, next),
            (Self::Todo, Self::InProgress)
                | (Self::Todo, Self::Blocked)
                | (Self::InProgress, Self::Review)
                | (Self::InProgress, Self::Blocked)
                | (Self::Review, Self::InProgress)
                | (Self::Review, Self::Done)
                | (Self::Review, Self::Blocked)
                | (Self::Blocked, Self::Todo)
                | (Self::Blocked, Self::InProgress)
        )
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

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ModelDescriptor {
    pub provider: &'static str,
    pub canonical: &'static str,
    pub aliases: &'static [&'static str],
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SelectedModel {
    pub provider: &'static str,
    pub canonical: &'static str,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ModelError {
    UnknownAlias,
    ProviderUnavailable,
}

pub struct ModelRegistry {
    models: &'static [ModelDescriptor],
    active_provider: &'static str,
}

impl ModelRegistry {
    pub fn new(models: &'static [ModelDescriptor], active_provider: &'static str) -> Self {
        Self {
            models,
            active_provider,
        }
    }

    pub fn active_provider(&self) -> &'static str {
        self.active_provider
    }

    pub fn resolve_alias(&self, alias: &str) -> Option<SelectedModel> {
        self.models.iter().find_map(|model| {
            let matches_alias = model.canonical == alias || model.aliases.contains(&alias);
            matches_alias.then_some(SelectedModel {
                provider: model.provider,
                canonical: model.canonical,
            })
        })
    }

    pub fn switch_provider(&mut self, provider: &'static str) -> Result<(), ModelError> {
        if self.models.iter().any(|model| model.provider == provider) {
            self.active_provider = provider;
            Ok(())
        } else {
            Err(ModelError::ProviderUnavailable)
        }
    }

    pub fn select(&self, alias: &str) -> Result<SelectedModel, ModelError> {
        let resolved = self.resolve_alias(alias).ok_or(ModelError::UnknownAlias)?;
        if resolved.provider == self.active_provider {
            Ok(resolved)
        } else {
            Err(ModelError::ProviderUnavailable)
        }
    }
}

pub const CORE_MODELS: &[ModelDescriptor] = &[
    ModelDescriptor {
        provider: "openai",
        canonical: "openai/gpt-5",
        aliases: &["gpt-5", "lead", "planner"],
    },
    ModelDescriptor {
        provider: "google",
        canonical: "google/gemini-2.5-flash",
        aliases: &["gemini-fast", "scout", "operator"],
    },
    ModelDescriptor {
        provider: "nvidia-nim",
        canonical: "nvidia-nim/deepseek-ai/deepseek-v3.2",
        aliases: &["deepseek", "nim-deep", "reasoning"],
    },
];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ActionKind {
    View,
    Edit,
    DeleteFile,
    GitPush,
    Release,
}

impl ActionKind {
    pub fn is_destructive(self) -> bool {
        matches!(self, Self::DeleteFile | Self::GitPush | Self::Release)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ApprovalState {
    Approved,
    Denied,
    Missing,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GovernedAction {
    pub kind: ActionKind,
    pub approval: ApprovalState,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GovernanceDecision {
    Allowed,
    RequiresApproval,
    Denied,
}

impl GovernedAction {
    pub fn decision(self) -> GovernanceDecision {
        match (self.kind.is_destructive(), self.approval) {
            (false, _) | (true, ApprovalState::Approved) => GovernanceDecision::Allowed,
            (true, ApprovalState::Missing) => GovernanceDecision::RequiresApproval,
            (true, ApprovalState::Denied) => GovernanceDecision::Denied,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DlpDecision {
    Allow,
    Mask,
    Block,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DlpOutcome {
    pub decision: DlpDecision,
    pub sanitized: String,
}

pub fn inspect_content(payload: &str) -> DlpOutcome {
    let mut sanitized = payload.to_owned();
    let mut decision = DlpDecision::Allow;

    for secret in ["sk-prod-", "token=", "api_key="] {
        if sanitized.contains(secret) {
            decision = DlpDecision::Mask;
            sanitized = sanitized.replace(secret, "[REDACTED]");
        }
    }

    if sanitized.contains("BEGIN PRIVATE KEY") || sanitized.contains("ssn:") {
        return DlpOutcome {
            decision: DlpDecision::Block,
            sanitized: "[BLOCKED]".to_owned(),
        };
    }

    DlpOutcome {
        decision,
        sanitized,
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AssignedTask {
    pub title: &'static str,
    pub assignee: &'static str,
    pub status: TaskStatus,
}

impl AssignedTask {
    pub fn new(title: &'static str, assignee: &'static str) -> Self {
        Self {
            title,
            assignee,
            status: TaskStatus::Todo,
        }
    }

    pub fn assign(&mut self, assignee: &'static str) {
        self.assignee = assignee;
    }

    pub fn transition(&mut self, next: TaskStatus) -> Result<(), TaskStatus> {
        if self.status.can_transition_to(next) {
            self.status = next;
            Ok(())
        } else {
            Err(self.status)
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SwarmLock {
    pub resource: String,
    pub owner: String,
}

#[derive(Default)]
pub struct SwarmLockbook {
    locks: BTreeMap<String, String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LockError {
    Conflict { resource: String, owner: String },
}

impl SwarmLockbook {
    pub fn acquire(&mut self, resource: &str, owner: &str) -> Result<(), LockError> {
        match self.locks.get(resource) {
            Some(existing_owner) if existing_owner != owner => Err(LockError::Conflict {
                resource: resource.to_owned(),
                owner: existing_owner.clone(),
            }),
            _ => {
                self.locks.insert(resource.to_owned(), owner.to_owned());
                Ok(())
            }
        }
    }

    pub fn release(&mut self, resource: &str, owner: &str) -> bool {
        matches!(self.locks.get(resource), Some(existing_owner) if existing_owner == owner)
            && self.locks.remove(resource).is_some()
    }

    pub fn holder(&self, resource: &str) -> Option<&str> {
        self.locks.get(resource).map(String::as_str)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MemoryCommit {
    pub branch: String,
    pub summary: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MemoryError {
    UnknownBranch,
    DuplicateBranch,
}

pub struct MemoryRepo {
    branches: BTreeMap<String, Vec<String>>,
    current_branch: String,
}

impl Default for MemoryRepo {
    fn default() -> Self {
        let mut branches = BTreeMap::new();
        branches.insert("main".to_owned(), Vec::new());
        Self {
            branches,
            current_branch: "main".to_owned(),
        }
    }
}

impl MemoryRepo {
    pub fn current_branch(&self) -> &str {
        &self.current_branch
    }

    pub fn branch(&mut self, name: &str) -> Result<(), MemoryError> {
        if self.branches.contains_key(name) {
            return Err(MemoryError::DuplicateBranch);
        }

        let snapshot = self
            .branches
            .get(&self.current_branch)
            .cloned()
            .expect("current branch should always exist");
        self.branches.insert(name.to_owned(), snapshot);
        Ok(())
    }

    pub fn switch(&mut self, name: &str) -> Result<(), MemoryError> {
        if self.branches.contains_key(name) {
            self.current_branch = name.to_owned();
            Ok(())
        } else {
            Err(MemoryError::UnknownBranch)
        }
    }

    pub fn commit(&mut self, summary: &str) -> MemoryCommit {
        let commit = MemoryCommit {
            branch: self.current_branch.clone(),
            summary: summary.to_owned(),
        };
        self.branches
            .entry(self.current_branch.clone())
            .or_default()
            .push(commit.summary.clone());
        commit
    }

    pub fn merge_into(&mut self, source: &str, target: &str) -> Result<Vec<String>, MemoryError> {
        let source_history = self
            .branches
            .get(source)
            .cloned()
            .ok_or(MemoryError::UnknownBranch)?;
        let target_history = self
            .branches
            .get_mut(target)
            .ok_or(MemoryError::UnknownBranch)?;

        let known: BTreeSet<_> = target_history.iter().cloned().collect();
        let mut merged = Vec::new();
        for commit in source_history {
            if !known.contains(&commit) && !target_history.contains(&commit) {
                target_history.push(commit.clone());
                merged.push(commit);
            }
        }
        Ok(merged)
    }

    pub fn history(&self, branch: &str) -> Option<&[String]> {
        self.branches.get(branch).map(Vec::as_slice)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WorkflowAction {
    Commit,
    Release,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct VerificationState {
    pub tests_passed: bool,
    pub review_complete: bool,
    pub changelog_ready: bool,
}

pub fn verify_workflow(
    action: WorkflowAction,
    state: VerificationState,
) -> Result<(), &'static str> {
    match action {
        WorkflowAction::Commit if !state.tests_passed => Err("tests must pass before commit"),
        WorkflowAction::Commit if !state.review_complete => {
            Err("review must complete before commit")
        }
        WorkflowAction::Release if !state.tests_passed => Err("tests must pass before release"),
        WorkflowAction::Release if !state.review_complete => {
            Err("review must complete before release")
        }
        WorkflowAction::Release if !state.changelog_ready => {
            Err("changelog must be ready before release")
        }
        _ => Ok(()),
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

    mod model_routing {
        use super::*;

        #[test]
        fn resolves_aliases_and_switches_providers() {
            let mut registry = ModelRegistry::new(CORE_MODELS, "openai");

            let lead = registry.select("lead").unwrap();
            assert_eq!(lead.canonical, "openai/gpt-5");

            assert_eq!(
                registry.select("deepseek"),
                Err(ModelError::ProviderUnavailable)
            );
            registry.switch_provider("nvidia-nim").unwrap();
            let reasoning = registry.select("deepseek").unwrap();
            assert_eq!(reasoning.provider, "nvidia-nim");
            assert_eq!(reasoning.canonical, "nvidia-nim/deepseek-ai/deepseek-v3.2");
        }
    }

    mod governance {
        use super::*;

        #[test]
        fn gates_destructive_actions_until_approved() {
            assert_eq!(
                GovernedAction {
                    kind: ActionKind::DeleteFile,
                    approval: ApprovalState::Missing,
                }
                .decision(),
                GovernanceDecision::RequiresApproval
            );
            assert_eq!(
                GovernedAction {
                    kind: ActionKind::DeleteFile,
                    approval: ApprovalState::Denied,
                }
                .decision(),
                GovernanceDecision::Denied
            );
            assert_eq!(
                GovernedAction {
                    kind: ActionKind::Edit,
                    approval: ApprovalState::Missing,
                }
                .decision(),
                GovernanceDecision::Allowed
            );
        }
    }

    mod dlp {
        use super::*;

        #[test]
        fn masks_secrets_and_blocks_private_material() {
            let masked = inspect_content("deploy with sk-prod-123 and token=abc");
            assert_eq!(masked.decision, DlpDecision::Mask);
            assert!(!masked.sanitized.contains("sk-prod-"));
            assert!(!masked.sanitized.contains("token="));

            let blocked = inspect_content("-----BEGIN PRIVATE KEY-----");
            assert_eq!(blocked.decision, DlpDecision::Block);
            assert_eq!(blocked.sanitized, "[BLOCKED]");
        }
    }

    mod tasks {
        use super::*;

        #[test]
        fn supports_assignment_and_valid_status_transitions() {
            let mut task = AssignedTask::new("Review queue", "Scout");
            task.assign("Reviewer");
            assert_eq!(task.assignee, "Reviewer");

            task.transition(TaskStatus::InProgress).unwrap();
            task.transition(TaskStatus::Review).unwrap();
            task.transition(TaskStatus::Done).unwrap();
            assert_eq!(task.status, TaskStatus::Done);
        }

        #[test]
        fn rejects_invalid_status_transitions() {
            let mut task = AssignedTask::new("Ship release", "Lead");
            assert_eq!(task.transition(TaskStatus::Done), Err(TaskStatus::Todo));
            assert_eq!(task.status, TaskStatus::Todo);
        }
    }

    mod swarm {
        use super::*;

        #[test]
        fn prevents_conflicting_task_locks() {
            let mut locks = SwarmLockbook::default();
            locks.acquire("task:model-routing", "Scout").unwrap();
            assert_eq!(locks.holder("task:model-routing"), Some("Scout"));

            let conflict = locks.acquire("task:model-routing", "Builder").unwrap_err();
            assert_eq!(
                conflict,
                LockError::Conflict {
                    resource: "task:model-routing".to_owned(),
                    owner: "Scout".to_owned(),
                }
            );

            assert!(locks.release("task:model-routing", "Scout"));
            locks.acquire("task:model-routing", "Builder").unwrap();
            assert_eq!(locks.holder("task:model-routing"), Some("Builder"));
        }
    }

    mod memory {
        use super::*;

        #[test]
        fn supports_branch_commit_switch_and_merge_semantics() {
            let mut repo = MemoryRepo::default();
            repo.commit("initialized control room");
            repo.branch("research/model-routing").unwrap();
            repo.switch("research/model-routing").unwrap();
            repo.commit("mapped alias routing");
            assert_eq!(repo.current_branch(), "research/model-routing");

            repo.switch("main").unwrap();
            repo.commit("captured governance gates");

            let merged = repo.merge_into("research/model-routing", "main").unwrap();
            assert_eq!(merged, vec!["mapped alias routing".to_owned()]);
            assert_eq!(
                repo.history("main").unwrap(),
                [
                    "initialized control room".to_owned(),
                    "captured governance gates".to_owned(),
                    "mapped alias routing".to_owned(),
                ]
            );
        }
    }

    mod workflow {
        use super::*;

        #[test]
        fn enforces_verification_gates_before_commit_and_release() {
            let not_ready = VerificationState {
                tests_passed: true,
                review_complete: false,
                changelog_ready: false,
            };
            assert_eq!(
                verify_workflow(WorkflowAction::Commit, not_ready),
                Err("review must complete before commit")
            );
            assert_eq!(
                verify_workflow(
                    WorkflowAction::Release,
                    VerificationState {
                        tests_passed: true,
                        review_complete: true,
                        changelog_ready: false,
                    }
                ),
                Err("changelog must be ready before release")
            );
            assert_eq!(
                verify_workflow(
                    WorkflowAction::Release,
                    VerificationState {
                        tests_passed: true,
                        review_complete: true,
                        changelog_ready: true,
                    }
                ),
                Ok(())
            );
        }
}
