use serde::{Deserialize, Serialize};

use crate::{CapabilityArea, MemoryEntry, SourceFeature};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Snapshot {
    pub branch: String,
    pub summary: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommitRecord {
    pub branch: String,
    pub summary: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct MergeRecord {
    pub branch: String,
    pub summary: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemoryState {
    pub snapshots: Vec<Snapshot>,
    pub commits: Vec<CommitRecord>,
    pub merges: Vec<MergeRecord>,
}

impl MemoryState {
    pub fn demo() -> Self {
        Self {
            snapshots: vec![Snapshot {
                branch: "research/model-routing".to_string(),
                summary: "Explored provider-specific model switching and NIM reasoning modes"
                    .to_string(),
            }],
            commits: vec![CommitRecord {
                branch: "main".to_string(),
                summary: "Initialized workspace and captured upstream feature map".to_string(),
            }],
            merges: vec![MergeRecord {
                branch: "main".to_string(),
                summary: "Merged governance, human-loop, and swarm orchestration into the shared control plane"
                    .to_string(),
            }],
        }
    }

    pub fn timeline(&self) -> Vec<MemoryEntry> {
        let mut entries = Vec::new();
        entries.extend(self.commits.iter().map(|commit| MemoryEntry {
            branch: commit.branch.clone(),
            kind: "commit".to_string(),
            summary: commit.summary.clone(),
        }));
        entries.extend(self.snapshots.iter().map(|snapshot| MemoryEntry {
            branch: snapshot.branch.clone(),
            kind: "branch".to_string(),
            summary: snapshot.summary.clone(),
        }));
        entries.extend(self.merges.iter().map(|merge| MemoryEntry {
            branch: merge.branch.clone(),
            kind: "merge".to_string(),
            summary: merge.summary.clone(),
        }));
        entries
    }

    pub fn capabilities(&self) -> Vec<SourceFeature> {
        vec![SourceFeature {
            source: "pi-brain".to_string(),
            area: CapabilityArea::Memory,
            feature: format!(
                "{} snapshots, {} commits, and {} merges",
                self.snapshots.len(),
                self.commits.len(),
                self.merges.len()
            ),
            outcome: "Branchable memory preserves architecture context and converged decisions"
                .to_string(),
        }]
    }
}
