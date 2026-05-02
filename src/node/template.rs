use crate::ids::{NodeId, RoleId};
use crate::node::capability::{CapabilityRequirement, ToolExposureMode, ToolExposureRequest};
use crate::node::contract::{InputContract, InputSource, OutputContract};
use crate::node::error::NodeValidationError;
use crate::node::kind::NodeKind;
use crate::node::policy::ExecutionHints;
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NodeTemplate {
    pub id: NodeId,
    pub name: Option<String>,
    pub description: Option<String>,
    pub kind: NodeKind,
    pub role: Option<RoleId>,
    pub input: InputContract,
    pub output: OutputContract,
    pub required_capabilities: Vec<CapabilityRequirement>,
    pub tool_exposure: ToolExposureRequest,
    pub execution_hints: ExecutionHints,
}

impl NodeTemplate {
    pub fn validate_basic(&self) -> Result<(), NodeValidationError> {
        self.input.validate_basic()?;
        self.output.validate_basic()?;

        if self.kind == NodeKind::Agent && self.role.is_none() {
            return Err(NodeValidationError::AgentNodeRequiresRole);
        }

        if self.kind == NodeKind::End
            && (!self.output.routes.allowed_routes.is_empty()
                || self.output.routes.default_route.is_some())
        {
            return Err(NodeValidationError::EndNodeCannotHaveRoutes);
        }

        if self.tool_exposure.mode == ToolExposureMode::Explicit
            && self.tool_exposure.requested_tools.is_empty()
        {
            return Err(NodeValidationError::ExplicitToolExposureRequiresTools);
        }

        let derived = self.derive_capability_requirements();

        for tool in &self.tool_exposure.requested_tools {
            debug_assert!(derived.contains(&CapabilityRequirement::ToolUse(tool.clone())));
        }

        for route in &self.output.routes.allowed_routes {
            debug_assert!(derived.contains(&CapabilityRequirement::EmitRoute(route.clone())));
        }

        Ok(())
    }

    pub fn derive_capability_requirements(&self) -> BTreeSet<CapabilityRequirement> {
        let mut capabilities: BTreeSet<CapabilityRequirement> =
            self.required_capabilities.iter().cloned().collect();

        for port in &self.input.ports {
            if let InputSource::State(channel) = &port.source {
                capabilities.insert(CapabilityRequirement::StateRead(channel.clone()));
            }
        }

        for write in &self.output.writes {
            capabilities.insert(CapabilityRequirement::StateWrite(write.target.clone()));
        }

        for tool in &self.tool_exposure.requested_tools {
            capabilities.insert(CapabilityRequirement::ToolUse(tool.clone()));
        }

        for route in &self.output.routes.allowed_routes {
            capabilities.insert(CapabilityRequirement::EmitRoute(route.clone()));
        }

        capabilities
    }
}
