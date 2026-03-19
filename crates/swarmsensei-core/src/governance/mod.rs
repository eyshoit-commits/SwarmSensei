use serde::{Deserialize, Serialize};

use crate::{workflow::WorkflowState, CapabilityArea, PolicyRule, SourceFeature};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RoleBinding {
    pub actor: String,
    pub role: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DlpCheck {
    pub name: String,
    pub action: String,
    pub triggered: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ApprovalGate {
    pub action: String,
    pub required_by: String,
    pub status: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuditRecord {
    pub actor: String,
    pub event: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct GovernanceState {
    pub rbac: Vec<RoleBinding>,
    pub dlp_checks: Vec<DlpCheck>,
    pub approval_gates: Vec<ApprovalGate>,
    pub audit_log: Vec<AuditRecord>,
}

impl GovernanceState {
    pub fn demo() -> Self {
        Self {
            rbac: vec![
                RoleBinding {
                    actor: "Lead".to_string(),
                    role: "approver".to_string(),
                },
                RoleBinding {
                    actor: "Reviewer".to_string(),
                    role: "governance-admin".to_string(),
                },
                RoleBinding {
                    actor: "Builder".to_string(),
                    role: "implementer".to_string(),
                },
            ],
            dlp_checks: vec![
                DlpCheck {
                    name: "secret exfiltration".to_string(),
                    action: "mask tokens and deny outbound payload".to_string(),
                    triggered: true,
                },
                DlpCheck {
                    name: "pii export".to_string(),
                    action: "require manual review".to_string(),
                    triggered: false,
                },
            ],
            approval_gates: vec![
                ApprovalGate {
                    action: "delete files".to_string(),
                    required_by: "human approver".to_string(),
                    status: "pending".to_string(),
                },
                ApprovalGate {
                    action: "publish artifacts".to_string(),
                    required_by: "review board".to_string(),
                    status: "waiting for DLP clear".to_string(),
                },
            ],
            audit_log: vec![
                AuditRecord {
                    actor: "Operator".to_string(),
                    event: "recorded cargo check receipt".to_string(),
                },
                AuditRecord {
                    actor: "Reviewer".to_string(),
                    event: "flagged outbound export for approval".to_string(),
                },
            ],
        }
    }

    pub fn policy_rules(&self, workflow: &WorkflowState) -> Vec<PolicyRule> {
        vec![
            PolicyRule {
                name: "Block secret exfiltration".to_string(),
                effect: self.dlp_checks[0].action.clone(),
                scope: "DLP".to_string(),
            },
            PolicyRule {
                name: "Require approval on destructive commands".to_string(),
                effect: format!(
                    "{} before {}",
                    self.approval_gates[0].required_by, self.approval_gates[0].action
                ),
                scope: "HITL".to_string(),
            },
            PolicyRule {
                name: "Protect governance config".to_string(),
                effect: "Only governance-admin bindings may mutate policy files".to_string(),
                scope: "RBAC".to_string(),
            },
            PolicyRule {
                name: "Verification gate".to_string(),
                effect: format!(
                    "{} checks must pass before commit promotion",
                    workflow.gates.len()
                ),
                scope: "Workflow".to_string(),
            },
        ]
    }

    pub fn capabilities(&self) -> Vec<SourceFeature> {
        vec![SourceFeature {
            source: "pi-governance".to_string(),
            area: CapabilityArea::Governance,
            feature: format!(
                "{} RBAC bindings, {} DLP checks, and {} approval gates",
                self.rbac.len(),
                self.dlp_checks.len(),
                self.approval_gates.len()
            ),
            outcome: format!(
                "{} audit records keep decisions reviewable",
                self.audit_log.len()
            ),
        }]
    }
}
