pub mod ids;
pub mod node;
pub mod types;

pub use ids::*;
pub use node::*;
pub use types::*;

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::{BTreeMap, BTreeSet};

    fn node_id(value: &str) -> NodeId {
        NodeId::new(value).unwrap()
    }

    fn role_id(value: &str) -> RoleId {
        RoleId::new(value).unwrap()
    }

    fn agent_id(value: &str) -> AgentId {
        AgentId::new(value).unwrap()
    }

    fn tool_id(value: &str) -> ToolId {
        ToolId::new(value).unwrap()
    }

    fn model_class(value: &str) -> ModelClass {
        ModelClass::new(value).unwrap()
    }

    fn model_binding_id(value: &str) -> ModelBindingId {
        ModelBindingId::new(value).unwrap()
    }

    fn state_channel(value: &str) -> StateChannelId {
        StateChannelId::new(value).unwrap()
    }

    fn route(value: &str) -> RouteName {
        RouteName::new(value).unwrap()
    }

    fn reviewer_template() -> NodeTemplate {
        NodeTemplate {
            id: node_id("reviewer"),
            name: Some("Reviewer".to_string()),
            description: Some("Reviews a draft and chooses the next route.".to_string()),
            kind: NodeKind::Agent,
            role: Some(role_id("reviewer")),
            input: InputContract {
                ports: vec![
                    InputPort {
                        name: "task".to_string(),
                        ty: TypeSpec::String,
                        source: InputSource::State(state_channel("task")),
                        required: true,
                        default_value: None,
                    },
                    InputPort {
                        name: "plan".to_string(),
                        ty: TypeSpec::Object,
                        source: InputSource::State(state_channel("plan")),
                        required: true,
                        default_value: None,
                    },
                    InputPort {
                        name: "draft".to_string(),
                        ty: TypeSpec::String,
                        source: InputSource::State(state_channel("draft")),
                        required: true,
                        default_value: None,
                    },
                ],
            },
            output: OutputContract {
                schema: None,
                writes: vec![StateWriteMapping {
                    target: state_channel("review"),
                    value_from: Selector::new("$.result.review"),
                }],
                artifacts: vec![],
                routes: RouteContract {
                    allowed_routes: vec![route("accept"), route("revise"), route("escalate")],
                    default_route: Some(route("revise")),
                },
            },
            required_capabilities: vec![CapabilityRequirement::ModelUse(model_class(
                "critic_strong",
            ))],
            tool_exposure: ToolExposureRequest {
                requested_tools: vec![tool_id("repo_read")],
                mode: ToolExposureMode::Explicit,
            },
            execution_hints: ExecutionHints::default(),
        }
    }

    fn reviewer_context(template: &NodeTemplate) -> NodeCompilationContext {
        let allowed_capabilities = template
            .derive_capability_requirements()
            .into_iter()
            .map(CapabilityGrant::from)
            .collect();

        NodeCompilationContext {
            role_bindings: BTreeMap::from([(role_id("reviewer"), agent_id("reviewer_agent"))]),
            model_bindings: BTreeMap::from([(
                model_class("critic_strong"),
                model_binding_id("critic_strong_binding"),
            )]),
            available_tools: BTreeSet::from([tool_id("repo_read")]),
            allowed_capabilities,
            default_execution_policy: EffectiveExecutionPolicy::default(),
        }
    }

    #[test]
    fn reviewer_template_derives_capabilities() {
        let template = reviewer_template();
        let capabilities = template.derive_capability_requirements();

        assert!(capabilities.contains(&CapabilityRequirement::StateRead(state_channel("task"))));
        assert!(capabilities.contains(&CapabilityRequirement::StateRead(state_channel("plan"))));
        assert!(capabilities.contains(&CapabilityRequirement::StateRead(state_channel("draft"))));
        assert!(capabilities.contains(&CapabilityRequirement::StateWrite(state_channel("review"))));
        assert!(capabilities.contains(&CapabilityRequirement::ToolUse(tool_id("repo_read"))));
        assert!(capabilities.contains(&CapabilityRequirement::EmitRoute(route("accept"))));
        assert!(capabilities.contains(&CapabilityRequirement::EmitRoute(route("revise"))));
        assert!(capabilities.contains(&CapabilityRequirement::EmitRoute(route("escalate"))));
        assert!(
            capabilities.contains(&CapabilityRequirement::ModelUse(model_class(
                "critic_strong"
            )))
        );
    }

    #[test]
    fn compile_reviewer_success() {
        let template = reviewer_template();
        let ctx = reviewer_context(&template);

        let compiled = NodeCompiler::compile(&template, &ctx).unwrap();

        assert_eq!(compiled.id, node_id("reviewer"));
        assert_eq!(
            compiled.resolved_agent,
            Some(ResolvedAgent {
                agent_id: agent_id("reviewer_agent"),
                role_id: role_id("reviewer")
            })
        );
        assert_eq!(
            compiled.resolved_model,
            Some(ResolvedModelBinding {
                model_class: model_class("critic_strong"),
                binding_id: model_binding_id("critic_strong_binding")
            })
        );
        assert_eq!(
            compiled.exposed_tools.exposed_tools,
            vec![tool_id("repo_read")]
        );
        assert!(compiled.has_route(&route("accept")));
        assert!(compiled.has_route(&route("revise")));
        assert!(compiled.has_route(&route("escalate")));
    }

    #[test]
    fn compile_fails_when_tool_missing() {
        let template = reviewer_template();
        let mut ctx = reviewer_context(&template);
        ctx.available_tools.clear();

        let err = NodeCompiler::compile(&template, &ctx).unwrap_err();

        assert!(matches!(err, NodeCompileError::ToolNotAvailable));
    }

    #[test]
    fn compile_fails_when_capability_not_granted() {
        let template = reviewer_template();
        let mut ctx = reviewer_context(&template);
        ctx.allowed_capabilities
            .remove(&CapabilityGrant::StateWrite(state_channel("review")));

        let err = NodeCompiler::compile(&template, &ctx).unwrap_err();

        assert!(matches!(err, NodeCompileError::MissingCapabilityGrant));
    }

    #[test]
    fn validation_fails_for_default_route_outside_allowed() {
        let contract = OutputContract {
            schema: None,
            writes: vec![],
            artifacts: vec![],
            routes: RouteContract {
                allowed_routes: vec![route("accept")],
                default_route: Some(route("revise")),
            },
        };

        let err = contract.validate_basic().unwrap_err();

        assert!(matches!(err, NodeValidationError::DefaultRouteNotAllowed));
    }

    #[test]
    fn validation_fails_for_duplicate_input_ports() {
        let contract = InputContract {
            ports: vec![
                InputPort {
                    name: "task".to_string(),
                    ty: TypeSpec::String,
                    source: InputSource::State(state_channel("task")),
                    required: true,
                    default_value: None,
                },
                InputPort {
                    name: "task".to_string(),
                    ty: TypeSpec::String,
                    source: InputSource::State(state_channel("task_copy")),
                    required: true,
                    default_value: None,
                },
            ],
        };

        let err = contract.validate_basic().unwrap_err();

        assert!(matches!(err, NodeValidationError::DuplicateInputPort));
    }
}
