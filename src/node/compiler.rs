use crate::ids::{AgentId, ModelBindingId, ModelClass, RoleId, ToolId};
use crate::node::capability::{
    CapabilityGrant, CapabilityRequirement, CompiledToolExposure, EffectiveNodeGrants,
    ToolExposureMode,
};
use crate::node::compiled::{
    CompiledInputPlan, CompiledNode, CompiledOutputPlan, ResolvedAgent, ResolvedModelBinding,
};
use crate::node::error::NodeCompileError;
use crate::node::kind::NodeKind;
use crate::node::policy::EffectiveExecutionPolicy;
use crate::node::template::NodeTemplate;
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Clone, PartialEq)]
pub struct NodeCompilationContext {
    pub role_bindings: BTreeMap<RoleId, AgentId>,
    pub model_bindings: BTreeMap<ModelClass, ModelBindingId>,
    pub available_tools: BTreeSet<ToolId>,
    pub allowed_capabilities: BTreeSet<CapabilityGrant>,
    pub default_execution_policy: EffectiveExecutionPolicy,
}

impl Default for NodeCompilationContext {
    fn default() -> Self {
        Self {
            role_bindings: BTreeMap::new(),
            model_bindings: BTreeMap::new(),
            available_tools: BTreeSet::new(),
            allowed_capabilities: BTreeSet::new(),
            default_execution_policy: EffectiveExecutionPolicy::default(),
        }
    }
}

pub struct NodeCompiler;

impl NodeCompiler {
    pub fn compile(
        template: &NodeTemplate,
        ctx: &NodeCompilationContext,
    ) -> Result<CompiledNode, NodeCompileError> {
        template.validate_basic()?;

        let required_capabilities = template.derive_capability_requirements();
        let required_grants = required_capabilities
            .iter()
            .cloned()
            .map(CapabilityGrant::from)
            .collect::<BTreeSet<_>>();

        for grant in &required_grants {
            if !ctx.allowed_capabilities.contains(grant) {
                return Err(NodeCompileError::MissingCapabilityGrant);
            }
        }

        let grants = EffectiveNodeGrants {
            grants: required_grants,
        };

        let resolved_agent = if template.kind == NodeKind::Agent {
            let role = template.role.clone().ok_or(NodeCompileError::Validation(
                crate::node::error::NodeValidationError::AgentNodeRequiresRole,
            ))?;

            let agent_id = ctx
                .role_bindings
                .get(&role)
                .cloned()
                .ok_or(NodeCompileError::MissingRoleBinding)?;

            Some(ResolvedAgent {
                agent_id,
                role_id: role,
            })
        } else {
            None
        };

        let mut resolved_model = None;
        for requirement in &required_capabilities {
            if let CapabilityRequirement::ModelUse(model_class) = requirement {
                let binding_id = ctx
                    .model_bindings
                    .get(model_class)
                    .cloned()
                    .ok_or(NodeCompileError::MissingModelBinding)?;

                if resolved_model.is_none() {
                    resolved_model = Some(ResolvedModelBinding {
                        model_class: model_class.clone(),
                        binding_id,
                    });
                }
            }
        }

        let exposed_tools = match template.tool_exposure.mode {
            ToolExposureMode::Explicit => {
                for tool in &template.tool_exposure.requested_tools {
                    if !ctx.available_tools.contains(tool) {
                        return Err(NodeCompileError::ToolNotAvailable);
                    }

                    let grant = CapabilityGrant::ToolUse(tool.clone());
                    if !grants.contains(&grant) {
                        return Err(NodeCompileError::ToolNotGranted);
                    }
                }

                CompiledToolExposure {
                    exposed_tools: template.tool_exposure.requested_tools.clone(),
                }
            }
            ToolExposureMode::None => CompiledToolExposure::default(),
            ToolExposureMode::Auto => {
                let exposed_tools = grants
                    .grants
                    .iter()
                    .filter_map(|grant| match grant {
                        CapabilityGrant::ToolUse(tool) if ctx.available_tools.contains(tool) => {
                            Some(tool.clone())
                        }
                        _ => None,
                    })
                    .collect();

                CompiledToolExposure { exposed_tools }
            }
        };

        let execution_policy = EffectiveExecutionPolicy::from_hints(
            &ctx.default_execution_policy,
            &template.execution_hints,
        );

        Ok(CompiledNode {
            id: template.id.clone(),
            name: template.name.clone(),
            description: template.description.clone(),
            kind: template.kind,
            role: template.role.clone(),
            resolved_agent,
            resolved_model,
            input_plan: CompiledInputPlan {
                ports: template.input.ports.clone(),
            },
            output_plan: CompiledOutputPlan {
                schema: template.output.schema.clone(),
                writes: template.output.writes.clone(),
                artifacts: template.output.artifacts.clone(),
            },
            grants,
            exposed_tools,
            routes: template.output.routes.clone(),
            execution_policy,
        })
    }
}
