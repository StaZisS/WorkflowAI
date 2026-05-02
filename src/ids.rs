use serde::{Deserialize, Serialize};
use std::fmt;
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum IdError {
    #[error("id value must not be empty or whitespace")]
    Empty,
}

macro_rules! define_id {
    ($name:ident) => {
        #[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
        pub struct $name(String);

        impl $name {
            pub fn new(value: impl Into<String>) -> Result<Self, IdError> {
                let value = value.into();
                if value.trim().is_empty() {
                    return Err(IdError::Empty);
                }

                Ok(Self(value))
            }

            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str(self.as_str())
            }
        }
    };
}

define_id!(NodeId);
define_id!(RoleId);
define_id!(AgentId);
define_id!(ToolId);
define_id!(ModelClass);
define_id!(ModelBindingId);
define_id!(StateChannelId);
define_id!(SchemaId);
define_id!(RouteName);
define_id!(WorkflowId);
define_id!(ActivationId);
define_id!(RunId);
