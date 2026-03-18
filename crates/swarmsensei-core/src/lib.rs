use std::fmt;

use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

use uuid::Uuid;

fn new_id(prefix: &str) -> String {
    format!("{}-{}", prefix, Uuid::new_v4().simple())
}

macro_rules! string_id {
    ($name:ident, $prefix:literal) => {
        #[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
        pub struct $name(String);

        impl $name {
            pub fn new() -> Self {
                Self(new_id($prefix))
            }

            pub fn from_value(value: impl Into<String>) -> Self {
                Self(value.into())
            }

            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl Default for $name {
            fn default() -> Self {
                Self::new()
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str(self.as_str())
            }
        }

        impl From<&str> for $name {
            fn from(value: &str) -> Self {
                Self::from_value(value)
            }
        }

        impl From<String> for $name {
            fn from(value: String) -> Self {
                Self::from_value(value)
            }
        }
    };
}

string_id!(AgentId, "agent");
string_id!(TaskId, "task");
string_id!(PolicyId, "policy");
string_id!(MemoryCommitId, "memc");

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

    pub fn short_label(self) -> &'static str {
        match self {
            Self::Sandbox => "Sandbox",
            Self::Teaming => "Teaming",
            Self::Models => "Models",
            Self::Swarm => "Swarm",
            Self::Governance => "Governance",
            Self::HumanLoop => "Human loop",
            Self::Memory => "Memory",
            Self::Workflow => "Workflow",
            Self::Interface => "Interface",
            Self::CodingHarness => "Coding harness",
        }
    }
}

impl fmt::Display for CapabilityArea {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.short_label())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ModelProvider {
    OpenAi,
    Anthropic,
    Google,
    NvidiaNim,
    Custom,
}

impl ModelProvider {
    pub fn slug(self) -> &'static str {
        match self {
            Self::OpenAi => "openai",
            Self::Anthropic => "anthropic",
            Self::Google => "google",
            Self::NvidiaNim => "nvidia-nim",
            Self::Custom => "custom",
        }
    }
}

impl fmt::Display for ModelProvider {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.slug())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ModelCapability {
    Fast,
    DeepReasoning,
    ToolUse,
    CodeGeneration,
    Audit,
    MemorySynthesis,
}

impl ModelCapability {
    pub fn label(self) -> &'static str {
        match self {
            Self::Fast => "fast",
            Self::DeepReasoning => "deep-reasoning",
            Self::ToolUse => "tool-use",
            Self::CodeGeneration => "code-generation",
            Self::Audit => "audit",
            Self::MemorySynthesis => "memory-synthesis",
        }
    }
}

impl fmt::Display for ModelCapability {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.label())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ModelRef {
    pub provider: ModelProvider,
    pub model: String,
    pub capabilities: Vec<ModelCapability>,
}

impl ModelRef {
    pub fn new(
        provider: ModelProvider,
        model: impl Into<String>,
        capabilities: impl Into<Vec<ModelCapability>>,
    ) -> Self {
        Self {
            provider,
            model: model.into(),
            capabilities: capabilities.into(),
        }
    }
}

impl fmt::Display for ModelRef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{}", self.provider, self.model)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum AgentState {
    Idle,
    Supervising,
    Mapping,
    Coding,
    Auditing,
    Committing,
    Running,
}

impl AgentState {
    pub fn label(self) -> &'static str {
        match self {
            Self::Idle => "Idle",
            Self::Supervising => "Supervising",
            Self::Mapping => "Mapping",
            Self::Coding => "Coding",
            Self::Auditing => "Auditing",
            Self::Committing => "Committing",
            Self::Running => "Running",
        }
    }
}

impl fmt::Display for AgentState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.label())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum PolicyEffect {
    Block,
    RequireApproval,
    Protect,
    Verify,
}

impl fmt::Display for PolicyEffect {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Block => "Block",
            Self::RequireApproval => "Require approval",
            Self::Protect => "Protect",
            Self::Verify => "Verify",
        })
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum PolicyScope {
    Dlp,
    Hitl,
    Rbac,
    Workflow,
}

impl fmt::Display for PolicyScope {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Dlp => "DLP",
            Self::Hitl => "HITL",
            Self::Rbac => "RBAC",
            Self::Workflow => "Workflow",
        })
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ApprovalAction {
    FileDeletion,
    GitPush,
    Export,
    PolicyChange,
    MemoryMerge,
    CommandExecution,
}

impl fmt::Display for ApprovalAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::FileDeletion => "File deletion",
            Self::GitPush => "Git push",
            Self::Export => "Export",
            Self::PolicyChange => "Policy change",
            Self::MemoryMerge => "Memory merge",
            Self::CommandExecution => "Command execution",
        })
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ApprovalStatus {
    Pending,
    Approved,
    Rejected,
    Cancelled,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum AuditEventKind {
    RunCreated,
    TaskCreated,
    TaskStatusChanged,
    ApprovalRequested,
    ApprovalResolved,
    MemoryCommitted,
    MemoryMerged,
    PheromoneUpdated,
}

impl AuditEventKind {
    pub fn label(self) -> &'static str {
        match self {
            Self::RunCreated => "Run created",
            Self::TaskCreated => "Task created",
            Self::TaskStatusChanged => "Task status changed",
            Self::ApprovalRequested => "Approval requested",
            Self::ApprovalResolved => "Approval resolved",
            Self::MemoryCommitted => "Memory committed",
            Self::MemoryMerged => "Memory merged",
            Self::PheromoneUpdated => "Pheromone updated",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum RunStatus {
    Draft,
    Active,
    AwaitingApproval,
    Completed,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SourceFeature {
    pub source: String,
    pub area: CapabilityArea,
    pub feature: String,
    pub outcome: String,
}

impl SourceFeature {
    pub fn new(
        source: impl Into<String>,
        area: CapabilityArea,
        feature: impl Into<String>,
        outcome: impl Into<String>,
    ) -> Self {
        Self {
            source: source.into(),
            area,
            feature: feature.into(),
            outcome: outcome.into(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AgentProfile {
    pub id: AgentId,
    pub name: String,
    pub specialty: String,
    pub model: ModelRef,
    pub thinking: ThinkingLevel,
    pub state: AgentState,
}

impl AgentProfile {
    pub fn new(
        name: impl Into<String>,
        specialty: impl Into<String>,
        model: ModelRef,
        thinking: ThinkingLevel,
        state: AgentState,
    ) -> Self {
        Self {
            id: AgentId::new(),
            name: name.into(),
            specialty: specialty.into(),
            model,
            thinking,
            state,
        }
    }

    pub fn set_state(&mut self, state: AgentState) {
        self.state = state;
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SwarmTask {
    pub id: TaskId,
    pub title: String,
    pub owner: AgentId,
    pub status: TaskStatus,
    pub lane: CapabilityArea,
    pub approval_request: Option<ApprovalRequest>,
    pub notes: Vec<String>,
}

impl SwarmTask {
    pub fn new(title: impl Into<String>, owner: AgentId, lane: CapabilityArea) -> Self {
        Self {
            id: TaskId::new(),
            title: title.into(),
            owner,
            status: TaskStatus::Todo,
            lane,
            approval_request: None,
            notes: Vec::new(),
        }
    }

    pub fn with_status(mut self, status: TaskStatus) -> Self {
        self.status = status;
        self
    }

    pub fn add_note(&mut self, note: impl Into<String>) {
        self.notes.push(note.into());
    }

    pub fn assign_owner(&mut self, owner: AgentId) {
        self.owner = owner;
    }

    pub fn start(&mut self) {
        self.status = TaskStatus::InProgress;
    }

    pub fn move_to_review(&mut self) {
        self.status = TaskStatus::Review;
    }

    pub fn block(&mut self, reason: impl Into<String>) {
        self.status = TaskStatus::Blocked;
        self.add_note(reason);
    }

    pub fn complete(&mut self) {
        self.status = TaskStatus::Done;
    }

    pub fn attach_approval(&mut self, request: ApprovalRequest) {
        self.approval_request = Some(request);
        self.status = TaskStatus::Blocked;
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PolicyRule {
    pub id: PolicyId,
    pub name: String,
    pub effect: PolicyEffect,
    pub scope: PolicyScope,
    pub detail: String,
}

impl PolicyRule {
    pub fn new(
        name: impl Into<String>,
        effect: PolicyEffect,
        scope: PolicyScope,
        detail: impl Into<String>,
    ) -> Self {
        Self {
            id: PolicyId::new(),
            name: name.into(),
            effect,
            scope,
            detail: detail.into(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ApprovalRequest {
    pub id: String,
    pub action: ApprovalAction,
    pub task_id: Option<TaskId>,
    pub policy_id: Option<PolicyId>,
    pub requested_by: AgentId,
    pub reason: String,
    pub status: ApprovalStatus,
    pub resolved_by: Option<String>,
}

impl ApprovalRequest {
    pub fn new(action: ApprovalAction, requested_by: AgentId, reason: impl Into<String>) -> Self {
        Self {
            id: new_id("approval"),
            action,
            task_id: None,
            policy_id: None,
            requested_by,
            reason: reason.into(),
            status: ApprovalStatus::Pending,
            resolved_by: None,
        }
    }

    pub fn for_task(mut self, task_id: TaskId) -> Self {
        self.task_id = Some(task_id);
        self
    }

    pub fn governed_by(mut self, policy_id: PolicyId) -> Self {
        self.policy_id = Some(policy_id);
        self
    }

    pub fn approve(&mut self, resolver: impl Into<String>) {
        self.status = ApprovalStatus::Approved;
        self.resolved_by = Some(resolver.into());
    }

    pub fn reject(&mut self, resolver: impl Into<String>) {
        self.status = ApprovalStatus::Rejected;
        self.resolved_by = Some(resolver.into());
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuditEvent {
    pub id: String,
    pub kind: AuditEventKind,
    pub actor: Option<AgentId>,
    pub summary: String,
    pub task_id: Option<TaskId>,
    pub memory_commit_id: Option<MemoryCommitId>,
}

impl AuditEvent {
    pub fn new(kind: AuditEventKind, summary: impl Into<String>) -> Self {
        Self {
            id: new_id("audit"),
            kind,
            actor: None,
            summary: summary.into(),
            task_id: None,
            memory_commit_id: None,
        }
    }

    pub fn with_actor(mut self, actor: AgentId) -> Self {
        self.actor = Some(actor);
        self
    }

    pub fn with_task(mut self, task_id: TaskId) -> Self {
        self.task_id = Some(task_id);
        self
    }

    pub fn with_memory_commit(mut self, commit_id: MemoryCommitId) -> Self {
        self.memory_commit_id = Some(commit_id);
        self
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemoryBranch {
    pub name: String,
    pub head: Option<MemoryCommitId>,
    pub summary: String,
}

impl MemoryBranch {
    pub fn new(name: impl Into<String>, summary: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            head: None,
            summary: summary.into(),
        }
    }

    pub fn point_to(&mut self, commit_id: MemoryCommitId) {
        self.head = Some(commit_id);
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemoryCommit {
    pub id: MemoryCommitId,
    pub branch: String,
    pub summary: String,
    pub author: AgentId,
    pub parent: Option<MemoryCommitId>,
}

impl MemoryCommit {
    pub fn new(
        branch: impl Into<String>,
        summary: impl Into<String>,
        author: AgentId,
        parent: Option<MemoryCommitId>,
    ) -> Self {
        Self {
            id: MemoryCommitId::new(),
            branch: branch.into(),
            summary: summary.into(),
            author,
            parent,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemoryMerge {
    pub id: String,
    pub source_branch: String,
    pub target_branch: String,
    pub commit_id: MemoryCommitId,
    pub summary: String,
}

impl MemoryMerge {
    pub fn new(
        source_branch: impl Into<String>,
        target_branch: impl Into<String>,
        commit_id: MemoryCommitId,
        summary: impl Into<String>,
    ) -> Self {
        Self {
            id: new_id("merge"),
            source_branch: source_branch.into(),
            target_branch: target_branch.into(),
            commit_id,
            summary: summary.into(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PheromoneSignal {
    pub lane: CapabilityArea,
    pub intensity: f32,
    pub reason: String,
}

impl PheromoneSignal {
    pub fn new(lane: CapabilityArea, intensity: f32, reason: impl Into<String>) -> Self {
        Self {
            lane,
            intensity: intensity.clamp(0.0, 1.0),
            reason: reason.into(),
        }
    }

    pub fn reinforce(&mut self, delta: f32) {
        self.intensity = (self.intensity + delta).clamp(0.0, 1.0);
    }

    pub fn decay(&mut self, delta: f32) {
        self.intensity = (self.intensity - delta).clamp(0.0, 1.0);
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum MemoryEvent {
    Branch(MemoryBranch),
    Commit(MemoryCommit),
    Merge(MemoryMerge),
}

impl MemoryEvent {
    pub fn kind_label(&self) -> &'static str {
        match self {
            Self::Branch(_) => "branch",
            Self::Commit(_) => "commit",
            Self::Merge(_) => "merge",
        }
    }

    pub fn branch_label(&self) -> &str {
        match self {
            Self::Branch(branch) => branch.name.as_str(),
            Self::Commit(commit) => commit.branch.as_str(),
            Self::Merge(merge) => merge.target_branch.as_str(),
        }
    }

    pub fn summary(&self) -> &str {
        match self {
            Self::Branch(branch) => branch.summary.as_str(),
            Self::Commit(commit) => commit.summary.as_str(),
            Self::Merge(merge) => merge.summary.as_str(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SwarmRun {
    pub id: String,
    pub headline: String,
    pub summary: String,
    pub active_model: ModelRef,
    pub team_mode: String,
    pub status: RunStatus,
    pub agents: Vec<AgentProfile>,
    pub tasks: Vec<SwarmTask>,
    pub rules: Vec<PolicyRule>,
    pub approvals: Vec<ApprovalRequest>,
    pub audit_log: Vec<AuditEvent>,
    pub memory_branches: Vec<MemoryBranch>,
    pub memory_commits: Vec<MemoryCommit>,
    pub memory_merges: Vec<MemoryMerge>,
    pub pheromones: Vec<PheromoneSignal>,
    pub sources: Vec<SourceFeature>,
}

pub type Scenario = SwarmRun;

impl SwarmRun {
    pub fn new(
        headline: impl Into<String>,
        summary: impl Into<String>,
        active_model: ModelRef,
        team_mode: impl Into<String>,
    ) -> Self {
        let mut run = Self {
            id: new_id("run"),
            headline: headline.into(),
            summary: summary.into(),
            active_model,
            team_mode: team_mode.into(),
            status: RunStatus::Draft,
            agents: Vec::new(),
            tasks: Vec::new(),
            rules: Vec::new(),
            approvals: Vec::new(),
            audit_log: Vec::new(),
            memory_branches: Vec::new(),
            memory_commits: Vec::new(),
            memory_merges: Vec::new(),
            pheromones: Vec::new(),
            sources: Vec::new(),
        };
        run.record_event(AuditEvent::new(
            AuditEventKind::RunCreated,
            "Initialized swarm run state",
        ));
        run
    }

    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }

    pub fn activate(&mut self) {
        self.status = RunStatus::Active;
    }

    pub fn set_active_model(&mut self, model: ModelRef) {
        self.active_model = model;
    }

    pub fn add_agent(&mut self, agent: AgentProfile) {
        self.agents.push(agent);
    }

    pub fn add_rule(&mut self, rule: PolicyRule) {
        self.rules.push(rule);
    }

    pub fn add_source(&mut self, source: SourceFeature) {
        self.sources.push(source);
    }

    pub fn add_task(&mut self, task: SwarmTask) {
        self.record_event(
            AuditEvent::new(
                AuditEventKind::TaskCreated,
                format!("Created task '{}'", task.title),
            )
            .with_task(task.id.clone())
            .with_actor(task.owner.clone()),
        );
        self.tasks.push(task);
    }

    pub fn transition_task(
        &mut self,
        task_id: &TaskId,
        status: TaskStatus,
        note: Option<&str>,
    ) -> bool {
        if let Some(task) = self.tasks.iter_mut().find(|task| &task.id == task_id) {
            task.status = status;
            if let Some(note) = note {
                task.add_note(note);
            }
            let event = AuditEvent::new(
                AuditEventKind::TaskStatusChanged,
                format!("Task '{}' moved to {}", task.title, status.label()),
            )
            .with_task(task.id.clone())
            .with_actor(task.owner.clone());
            self.record_event(event);
            true
        } else {
            false
        }
    }

    pub fn request_approval(&mut self, request: ApprovalRequest) {
        self.status = RunStatus::AwaitingApproval;
        self.record_event(
            AuditEvent::new(
                AuditEventKind::ApprovalRequested,
                format!("Approval requested for {}", request.action),
            )
            .with_actor(request.requested_by.clone()),
        );
        self.approvals.push(request);
    }

    pub fn resolve_approval(&mut self, request_id: &str, approve: bool, resolver: &str) -> bool {
        if let Some(request) = self
            .approvals
            .iter_mut()
            .find(|request| request.id == request_id)
        {
            if approve {
                request.approve(resolver.to_string());
                self.status = RunStatus::Active;
            } else {
                request.reject(resolver.to_string());
            }
            let event = AuditEvent::new(
                AuditEventKind::ApprovalResolved,
                format!("Approval {} was {}", request.id, request.status_label()),
            )
            .with_actor(request.requested_by.clone());
            self.record_event(event);
            true
        } else {
            false
        }
    }

    pub fn ensure_branch(&mut self, branch_name: &str, summary: &str) {
        if self
            .memory_branches
            .iter()
            .all(|branch| branch.name != branch_name)
        {
            self.memory_branches.push(MemoryBranch::new(
                branch_name.to_string(),
                summary.to_string(),
            ));
        }
    }

    pub fn commit_memory(
        &mut self,
        branch_name: &str,
        author: AgentId,
        summary: impl Into<String>,
    ) -> MemoryCommitId {
        self.ensure_branch(branch_name, "Runtime memory branch");
        let parent = self
            .memory_branches
            .iter()
            .find(|branch| branch.name == branch_name)
            .and_then(|branch| branch.head.clone());
        let commit = MemoryCommit::new(
            branch_name.to_string(),
            summary.into(),
            author.clone(),
            parent,
        );
        let commit_id = commit.id.clone();
        if let Some(branch) = self
            .memory_branches
            .iter_mut()
            .find(|branch| branch.name == branch_name)
        {
            branch.point_to(commit_id.clone());
        }
        self.record_event(
            AuditEvent::new(
                AuditEventKind::MemoryCommitted,
                format!("Committed memory to {branch_name}"),
            )
            .with_actor(author)
            .with_memory_commit(commit_id.clone()),
        );
        self.memory_commits.push(commit);
        commit_id
    }

    pub fn merge_memory(
        &mut self,
        source_branch: &str,
        target_branch: &str,
        summary: impl Into<String>,
    ) -> Option<MemoryMerge> {
        let commit_id = self
            .memory_branches
            .iter()
            .find(|branch| branch.name == source_branch)
            .and_then(|branch| branch.head.clone())?;
        self.ensure_branch(target_branch, "Runtime memory branch");
        if let Some(target) = self
            .memory_branches
            .iter_mut()
            .find(|branch| branch.name == target_branch)
        {
            target.point_to(commit_id.clone());
        }
        let merge = MemoryMerge::new(source_branch, target_branch, commit_id.clone(), summary);
        self.record_event(
            AuditEvent::new(
                AuditEventKind::MemoryMerged,
                format!("Merged memory {source_branch} -> {target_branch}"),
            )
            .with_memory_commit(commit_id),
        );
        self.memory_merges.push(merge.clone());
        Some(merge)
    }

    pub fn reinforce_pheromone(
        &mut self,
        lane: CapabilityArea,
        delta: f32,
        reason: impl Into<String>,
    ) {
        let reason = reason.into();
        if let Some(signal) = self
            .pheromones
            .iter_mut()
            .find(|signal| signal.lane == lane)
        {
            signal.reinforce(delta);
            signal.reason = reason.clone();
        } else {
            self.pheromones
                .push(PheromoneSignal::new(lane, delta.max(0.0), reason.clone()));
        }
        self.record_event(AuditEvent::new(
            AuditEventKind::PheromoneUpdated,
            format!("Updated {} pheromone: {reason}", lane.short_label()),
        ));
    }

    pub fn completed_tasks(&self) -> usize {
        self.tasks
            .iter()
            .filter(|task| task.status == TaskStatus::Done)
            .count()
    }

    pub fn active_agents(&self) -> usize {
        self.agents
            .iter()
            .filter(|agent| agent.state != AgentState::Idle)
            .count()
    }

    pub fn governance_rules(&self) -> usize {
        self.rules.len()
    }

    pub fn agent_name(&self, agent_id: &AgentId) -> &str {
        self.agents
            .iter()
            .find(|agent| &agent.id == agent_id)
            .map(|agent| agent.name.as_str())
            .unwrap_or("Unknown")
    }

    pub fn memory_timeline(&self) -> Vec<MemoryEvent> {
        let mut events = self
            .memory_branches
            .iter()
            .cloned()
            .map(MemoryEvent::Branch)
            .collect::<Vec<_>>();
        events.extend(self.memory_commits.iter().cloned().map(MemoryEvent::Commit));
        events.extend(self.memory_merges.iter().cloned().map(MemoryEvent::Merge));
        events
    }

    fn record_event(&mut self, event: AuditEvent) {
        self.audit_log.push(event);
    }
}

impl ApprovalRequest {
    pub fn status_label(&self) -> &'static str {
        match self.status {
            ApprovalStatus::Pending => "pending",
            ApprovalStatus::Approved => "approved",
            ApprovalStatus::Rejected => "rejected",
            ApprovalStatus::Cancelled => "cancelled",
        }
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
    let lead = AgentProfile::new(
        "Lead",
        "Roadmapping & approvals",
        ModelRef::new(
            ModelProvider::OpenAi,
            "gpt-5",
            vec![ModelCapability::DeepReasoning, ModelCapability::ToolUse],
        ),
        ThinkingLevel::High,
        AgentState::Supervising,
    );
    let scout = AgentProfile::new(
        "Scout",
        "Discovery & dependency tracing",
        ModelRef::new(
            ModelProvider::Google,
            "gemini-2.5-flash",
            vec![ModelCapability::Fast, ModelCapability::ToolUse],
        ),
        ThinkingLevel::Low,
        AgentState::Mapping,
    );
    let builder = AgentProfile::new(
        "Builder",
        "Implementation",
        ModelRef::new(
            ModelProvider::Anthropic,
            "claude-opus-4.5",
            vec![
                ModelCapability::DeepReasoning,
                ModelCapability::CodeGeneration,
            ],
        ),
        ThinkingLevel::High,
        AgentState::Coding,
    );
    let reviewer = AgentProfile::new(
        "Reviewer",
        "Quality & governance",
        ModelRef::new(
            ModelProvider::NvidiaNim,
            "z-ai/glm5",
            vec![ModelCapability::Audit, ModelCapability::DeepReasoning],
        ),
        ThinkingLevel::Medium,
        AgentState::Auditing,
    );
    let archivist = AgentProfile::new(
        "Archivist",
        "Memory synthesis",
        ModelRef::new(
            ModelProvider::OpenAi,
            "gpt-5-mini",
            vec![ModelCapability::Fast, ModelCapability::MemorySynthesis],
        ),
        ThinkingLevel::Medium,
        AgentState::Committing,
    );
    let operator = AgentProfile::new(
        "Operator",
        "Sandbox execution",
        ModelRef::new(
            ModelProvider::Google,
            "gemini-2.5-flash",
            vec![ModelCapability::Fast, ModelCapability::ToolUse],
        ),
        ThinkingLevel::Off,
        AgentState::Running,
    );

    let mut scenario = SwarmRun::new(
        "SwarmSensei Control Room",
        "A mono Rust/WASM control room that merges sandbox execution, multi-agent teaming, adaptive model switching, governance, human approvals, memory, and a rich web cockpit.",
        ModelRef::new(
            ModelProvider::NvidiaNim,
            "deepseek-ai/deepseek-v3.2",
            vec![ModelCapability::DeepReasoning, ModelCapability::CodeGeneration],
        ),
        "Parallel swarm with human approval checkpoints",
    );
    scenario.activate();

    for agent in [&lead, &scout, &builder, &reviewer, &archivist, &operator] {
        scenario.add_agent(agent.clone());
    }

    scenario.add_task(
        SwarmTask::new(
            "Spin up isolated Rust/WASM workspace",
            operator.id.clone(),
            CapabilityArea::Sandbox,
        )
        .with_status(TaskStatus::Done),
    );
    scenario.add_task(
        SwarmTask::new(
            "Route heavy tasks to premium model",
            lead.id.clone(),
            CapabilityArea::Models,
        )
        .with_status(TaskStatus::InProgress),
    );
    scenario.add_task(
        SwarmTask::new(
            "Scout parallel refactor candidates",
            scout.id.clone(),
            CapabilityArea::Swarm,
        )
        .with_status(TaskStatus::Review),
    );
    let mut blocked = SwarmTask::new(
        "Review DLP hits before export",
        reviewer.id.clone(),
        CapabilityArea::Governance,
    )
    .with_status(TaskStatus::Blocked);
    blocked.attach_approval(
        ApprovalRequest::new(
            ApprovalAction::Export,
            reviewer.id.clone(),
            "Potentially sensitive report leaves the workspace",
        )
        .for_task(blocked.id.clone()),
    );
    scenario.add_task(blocked.clone());
    if let Some(request) = blocked.approval_request.clone() {
        scenario.request_approval(request);
    }
    scenario.add_task(
        SwarmTask::new(
            "Checkpoint architecture decisions",
            archivist.id.clone(),
            CapabilityArea::Memory,
        )
        .with_status(TaskStatus::Done),
    );

    scenario.add_rule(PolicyRule::new(
        "Block secret exfiltration",
        PolicyEffect::Block,
        PolicyScope::Dlp,
        "Mask tokens and deny suspicious outbound content",
    ));
    scenario.add_rule(PolicyRule::new(
        "Require approval on destructive commands",
        PolicyEffect::RequireApproval,
        PolicyScope::Hitl,
        "Human review before file deletion or git push",
    ));
    scenario.add_rule(PolicyRule::new(
        "Protect governance config",
        PolicyEffect::Protect,
        PolicyScope::Rbac,
        "Agents cannot mutate policy files",
    ));
    scenario.add_rule(PolicyRule::new(
        "Verification gate",
        PolicyEffect::Verify,
        PolicyScope::Workflow,
        "Commits require passing checks",
    ));

    scenario.ensure_branch("main", "Shared control plane branch");
    scenario.ensure_branch(
        "research/model-routing",
        "Experiments for provider-specific switching",
    );
    let main_commit = scenario.commit_memory(
        "main",
        archivist.id.clone(),
        "Initialized workspace and captured upstream feature map.",
    );
    let _research_commit = scenario.commit_memory(
        "research/model-routing",
        archivist.id.clone(),
        "Explored provider-specific model switching and NIM thinking modes.",
    );
    scenario.memory_merges.push(MemoryMerge::new(
        "research/model-routing",
        "main",
        main_commit,
        "Merged governance, human-loop, and ant-colony orchestration into the shared control plane.",
    ));

    scenario.reinforce_pheromone(
        CapabilityArea::Swarm,
        0.78,
        "Parallel scout/review wave found productive refactor candidates",
    );
    scenario.reinforce_pheromone(
        CapabilityArea::Governance,
        0.64,
        "Approval checkpoint increased governance pressure",
    );

    for source in source_features() {
        scenario.add_source(source);
    }

    scenario
}

pub fn source_features() -> Vec<SourceFeature> {
    vec![
        SourceFeature::new(
            "agentkernel",
            CapabilityArea::Sandbox,
            "MicroVM-style isolated command execution",
            "Safe task runs with receipts and runtime auto-detection",
        ),
        SourceFeature::new(
            "pi-teams",
            CapabilityArea::Teaming,
            "Parallel specialist agents with a shared task board",
            "Lead + teammate coordination in one workspace",
        ),
        SourceFeature::new(
            "pi-model-switch",
            CapabilityArea::Models,
            "Autonomous model search and switching",
            "Dynamic routing between cheap, fast, and deep models",
        ),
        SourceFeature::new(
            "oh-pi-ant-colony",
            CapabilityArea::Swarm,
            "Pheromone-based adaptive concurrency",
            "Scouting, worker execution, and review waves",
        ),
        SourceFeature::new(
            "pi-nvidia-nim",
            CapabilityArea::Models,
            "Custom NVIDIA NIM provider with reasoning controls",
            "NIM catalog surfaced as first-class model options",
        ),
        SourceFeature::new(
            "pi-governance",
            CapabilityArea::Governance,
            "RBAC, DLP, audit logging, and HITL",
            "Policy-aware actions with approval checkpoints",
        ),
        SourceFeature::new(
            "pi-ask-user",
            CapabilityArea::HumanLoop,
            "Interactive structured decisions",
            "User approval prompts for ambiguous or risky steps",
        ),
        SourceFeature::new(
            "pi-brain",
            CapabilityArea::Memory,
            "Versioned memory branches and merges",
            "Persistent context and milestone snapshots",
        ),
        SourceFeature::new(
            "pi-superpowers-plus",
            CapabilityArea::Workflow,
            "Workflow/TDD enforcement and subagent support",
            "Guided execution phases and verification gates",
        ),
        SourceFeature::new(
            "opencode-chamber",
            CapabilityArea::Interface,
            "Web/desktop coding workspace",
            "Single browser cockpit for chat, diffs, plans, and tasks",
        ),
        SourceFeature::new(
            "pi coding agent",
            CapabilityArea::CodingHarness,
            "Minimal extensible coding harness",
            "Composable tool foundation for the whole mono app",
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

<<<<<<< HEAD
    #[test]
    fn scenario_metrics_are_consistent() {
        let scenario = sample_scenario();
        assert_eq!(scenario.completed_tasks(), 2);
        assert_eq!(scenario.active_agents(), 6);
        assert_eq!(scenario.governance_rules(), 4);
        assert_eq!(scenario.approvals.len(), 1);
        assert!(scenario.audit_log.len() >= 5);
=======
    mod scenario {
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
>>>>>>> origin/main
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

    #[test]
    fn run_state_round_trips_to_json() {
        let scenario = sample_scenario();
        let json = scenario.to_json().expect("scenario serializes");
        let restored = SwarmRun::from_json(&json).expect("scenario deserializes");
        assert_eq!(restored.headline, scenario.headline);
        assert_eq!(restored.tasks.len(), scenario.tasks.len());
        assert_eq!(restored.memory_commits.len(), scenario.memory_commits.len());
    }

    #[test]
    fn task_and_approval_state_transitions_are_recorded() {
        let mut run = sample_scenario();
        let task_id = run.tasks[1].id.clone();
        assert!(run.transition_task(&task_id, TaskStatus::Review, Some("Ready for review")));

        let request_id = run.approvals[0].id.clone();
        assert!(run.resolve_approval(&request_id, true, "human-operator"));
        assert_eq!(run.approvals[0].status, ApprovalStatus::Approved);
        assert_eq!(run.status, RunStatus::Active);
    }
}
