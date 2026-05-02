use crate::ids::{ActivationId, NodeId, RunId};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NodeActivation {
    pub activation_id: ActivationId,
    pub run_id: RunId,
    pub node_id: NodeId,
    pub parent_activation_id: Option<ActivationId>,
    pub scope: Value,
    pub attempt: u32,
    pub status: NodeActivationStatus,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum NodeActivationStatus {
    Pending,
    Scheduled,
    Running,
    WaitingForTool,
    WaitingForHuman,
    Succeeded,
    Failed,
    Cancelled,
    Skipped,
    Retrying,
}
