use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

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
}
