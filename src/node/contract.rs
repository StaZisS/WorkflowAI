use crate::ids::{RouteName, SchemaId, StateChannelId};
use crate::node::error::NodeValidationError;
use crate::types::{ArtifactRef, Selector, TypeSpec};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::BTreeSet;

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct InputContract {
    pub ports: Vec<InputPort>,
}

impl InputContract {
    pub fn validate_basic(&self) -> Result<(), NodeValidationError> {
        let mut names = BTreeSet::new();

        for port in &self.ports {
            if port.name.trim().is_empty() {
                return Err(NodeValidationError::EmptyField);
            }

            if !names.insert(port.name.clone()) {
                return Err(NodeValidationError::DuplicateInputPort);
            }

            match &port.source {
                InputSource::State(channel) => {
                    if channel.as_str().trim().is_empty() {
                        return Err(NodeValidationError::EmptyField);
                    }
                }
                InputSource::Scope(selector) | InputSource::Param(selector) => {
                    validate_selector(selector)?
                }
                InputSource::Literal(_) | InputSource::Artifact(_) => {}
            }
        }

        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InputPort {
    pub name: String,
    pub ty: TypeSpec,
    pub source: InputSource,
    pub required: bool,
    pub default_value: Option<Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum InputSource {
    State(StateChannelId),
    Scope(Selector),
    Param(Selector),
    Literal(Value),
    Artifact(ArtifactRef),
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct OutputContract {
    pub schema: Option<SchemaId>,
    pub writes: Vec<StateWriteMapping>,
    pub artifacts: Vec<ArtifactMapping>,
    pub routes: RouteContract,
}

impl OutputContract {
    pub fn validate_basic(&self) -> Result<(), NodeValidationError> {
        self.routes.validate_basic()?;

        for write in &self.writes {
            if write.target.as_str().trim().is_empty() {
                return Err(NodeValidationError::EmptyField);
            }

            validate_selector(&write.value_from)?;
        }

        for artifact in &self.artifacts {
            validate_selector(&artifact.value_from)?;
        }

        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StateWriteMapping {
    pub target: StateChannelId,
    pub value_from: Selector,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ArtifactMapping {
    pub value_from: Selector,
    pub description: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct RouteContract {
    pub allowed_routes: Vec<RouteName>,
    pub default_route: Option<RouteName>,
}

impl RouteContract {
    pub fn validate_basic(&self) -> Result<(), NodeValidationError> {
        let mut routes = BTreeSet::new();

        for route in &self.allowed_routes {
            if !routes.insert(route.clone()) {
                return Err(NodeValidationError::DuplicateRoute);
            }
        }

        if let Some(default_route) = &self.default_route {
            if !routes.contains(default_route) {
                return Err(NodeValidationError::DefaultRouteNotAllowed);
            }
        }

        Ok(())
    }
}

fn validate_selector(selector: &Selector) -> Result<(), NodeValidationError> {
    if selector.is_empty_or_whitespace() {
        return Err(NodeValidationError::InvalidSelector);
    }

    Ok(())
}
