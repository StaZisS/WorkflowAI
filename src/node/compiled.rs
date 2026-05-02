use crate::ids::{AgentId, ModelBindingId, ModelClass, NodeId, RoleId, RouteName, SchemaId};
use crate::node::capability::{CompiledToolExposure, EffectiveNodeGrants};
use crate::node::contract::{ArtifactMapping, InputPort, RouteContract, StateWriteMapping};
use crate::node::kind::NodeKind;
use crate::node::policy::EffectiveExecutionPolicy;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CompiledNode {
    pub id: NodeId,
    pub name: Option<String>,
    pub description: Option<String>,
    pub kind: NodeKind,
    pub role: Option<RoleId>,
    pub resolved_agent: Option<ResolvedAgent>,
    pub resolved_model: Option<ResolvedModelBinding>,
    pub input_plan: CompiledInputPlan,
    pub output_plan: CompiledOutputPlan,
    pub grants: EffectiveNodeGrants,
    pub exposed_tools: CompiledToolExposure,
    pub routes: RouteContract,
    pub execution_policy: EffectiveExecutionPolicy,
}

impl CompiledNode {
    pub fn has_route(&self, route: &RouteName) -> bool {
        self.routes.allowed_routes.contains(route)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResolvedAgent {
    pub agent_id: AgentId,
    pub role_id: RoleId,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResolvedModelBinding {
    pub model_class: ModelClass,
    pub binding_id: ModelBindingId,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CompiledInputPlan {
    pub ports: Vec<InputPort>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CompiledOutputPlan {
    pub schema: Option<SchemaId>,
    pub writes: Vec<StateWriteMapping>,
    pub artifacts: Vec<ArtifactMapping>,
}
