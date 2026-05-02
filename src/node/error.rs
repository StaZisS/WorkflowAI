use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum NodeValidationError {
    #[error("empty field")]
    EmptyField,

    #[error("duplicate input port")]
    DuplicateInputPort,

    #[error("duplicate route")]
    DuplicateRoute,

    #[error("default route is not in allowed routes")]
    DefaultRouteNotAllowed,

    #[error("agent node requires a role")]
    AgentNodeRequiresRole,

    #[error("end node cannot have routes")]
    EndNodeCannotHaveRoutes,

    #[error("explicit tool exposure requires at least one requested tool")]
    ExplicitToolExposureRequiresTools,

    #[error("invalid selector")]
    InvalidSelector,
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum NodeCompileError {
    #[error(transparent)]
    Validation(#[from] NodeValidationError),

    #[error("missing capability grant")]
    MissingCapabilityGrant,

    #[error("missing role binding")]
    MissingRoleBinding,

    #[error("missing model binding")]
    MissingModelBinding,

    #[error("tool is not available")]
    ToolNotAvailable,

    #[error("tool is not granted")]
    ToolNotGranted,
}
