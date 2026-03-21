use serde::{Deserialize, Serialize};

use crate::{workflow::WorkflowState, CapabilityArea, SourceFeature};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ParallelWave {
    pub name: String,
    pub objective: String,
    pub concurrency: u8,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PheromoneTrail {
    pub lane: String,
    pub intensity: u8,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TaskLock {
    pub task_title: String,
    pub owner: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SwarmState {
    pub waves: Vec<ParallelWave>,
    pub pheromones: Vec<PheromoneTrail>,
    pub task_locks: Vec<TaskLock>,
}

impl SwarmState {
    pub fn demo() -> Self {
        Self {
            waves: vec![
                ParallelWave {
                    name: "Scout wave".to_string(),
                    objective: "Map modules and isolate ownership boundaries".to_string(),
                    concurrency: 2,
                },
                ParallelWave {
                    name: "Worker wave".to_string(),
                    objective: "Implement retained capability families in parallel".to_string(),
                    concurrency: 3,
                },
                ParallelWave {
                    name: "Review wave".to_string(),
                    objective: "Converge on workflow and governance gates".to_string(),
                    concurrency: 1,
                },
            ],
            pheromones: vec![
                PheromoneTrail {
                    lane: "Models".to_string(),
                    intensity: 8,
                },
                PheromoneTrail {
                    lane: "Governance".to_string(),
                    intensity: 9,
                },
                PheromoneTrail {
                    lane: "Memory".to_string(),
                    intensity: 6,
                },
            ],
            task_locks: vec![
                TaskLock {
                    task_title: "Route heavy tasks to premium model".to_string(),
                    owner: "Lead".to_string(),
                },
                TaskLock {
                    task_title: "Review DLP hits before export".to_string(),
                    owner: "Reviewer".to_string(),
                },
            ],
        }
    }

    pub fn execution_mode(&self, workflow: &WorkflowState) -> String {
        format!(
            "{} adaptive waves with {} verification gates in {}",
            self.waves.len(),
            workflow.gates.len(),
            workflow.current_phase.label()
        )
    }

    pub fn capabilities(&self) -> Vec<SourceFeature> {
        vec![SourceFeature {
            source: "oh-pi-ant-colony".to_string(),
            area: CapabilityArea::Swarm,
            feature: format!(
                "{} task waves coordinated with {} pheromone trails",
                self.waves.len(),
                self.pheromones.len()
            ),
            outcome: format!(
                "{} active task locks steer review-safe parallelism",
                self.task_locks.len()
            ),
        }]
    }
}
