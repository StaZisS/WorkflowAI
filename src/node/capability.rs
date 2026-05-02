use crate::ids::{ModelClass, RouteName, StateChannelId, ToolId, WorkflowId};
use crate::node::error::NodeCompileError;
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum CapabilityRequirement {
    StateRead(StateChannelId),
    StateWrite(StateChannelId),
    ToolUse(ToolId),
    ArtifactRead,
    ArtifactWrite,
    MemoryRead,
    MemoryWrite,
    ModelUse(ModelClass),
    EmitRoute(RouteName),
    RequestReplan,
    RequestHumanApproval,
    RunSubworkflow(WorkflowId),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum CapabilityGrant {
    StateRead(StateChannelId),
    StateWrite(StateChannelId),
    ToolUse(ToolId),
    ArtifactRead,
    ArtifactWrite,
    MemoryRead,
    MemoryWrite,
    ModelUse(ModelClass),
    EmitRoute(RouteName),
    RequestReplan,
    RequestHumanApproval,
    RunSubworkflow(WorkflowId),
}

impl From<CapabilityRequirement> for CapabilityGrant {
    fn from(requirement: CapabilityRequirement) -> Self {
        match requirement {
            CapabilityRequirement::StateRead(channel) => Self::StateRead(channel),
            CapabilityRequirement::StateWrite(channel) => Self::StateWrite(channel),
            CapabilityRequirement::ToolUse(tool) => Self::ToolUse(tool),
            CapabilityRequirement::ArtifactRead => Self::ArtifactRead,
            CapabilityRequirement::ArtifactWrite => Self::ArtifactWrite,
            CapabilityRequirement::MemoryRead => Self::MemoryRead,
            CapabilityRequirement::MemoryWrite => Self::MemoryWrite,
            CapabilityRequirement::ModelUse(model_class) => Self::ModelUse(model_class),
            CapabilityRequirement::EmitRoute(route) => Self::EmitRoute(route),
            CapabilityRequirement::RequestReplan => Self::RequestReplan,
            CapabilityRequirement::RequestHumanApproval => Self::RequestHumanApproval,
            CapabilityRequirement::RunSubworkflow(workflow_id) => Self::RunSubworkflow(workflow_id),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct EffectiveNodeGrants {
    pub grants: BTreeSet<CapabilityGrant>,
}

impl EffectiveNodeGrants {
    pub fn contains(&self, grant: &CapabilityGrant) -> bool {
        self.grants.contains(grant)
    }

    pub fn require(&self, grant: &CapabilityGrant) -> Result<(), NodeCompileError> {
        if self.contains(grant) {
            Ok(())
        } else {
            Err(NodeCompileError::MissingCapabilityGrant)
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ToolExposureRequest {
    pub requested_tools: Vec<ToolId>,
    pub mode: ToolExposureMode,
}

impl Default for ToolExposureRequest {
    fn default() -> Self {
        Self {
            requested_tools: Vec::new(),
            mode: ToolExposureMode::None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum ToolExposureMode {
    None,
    Explicit,
    Auto,
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct CompiledToolExposure {
    pub exposed_tools: Vec<ToolId>,
}
