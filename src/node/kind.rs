use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum NodeKind {
    Agent,
    Tool,
    Function,
    Router,
    Evaluator,
    HumanGate,
    Subworkflow,
    Start,
    End,
}

impl NodeKind {
    pub fn is_terminal(self) -> bool {
        matches!(self, Self::End)
    }

    pub fn requires_executor(self) -> bool {
        matches!(
            self,
            Self::Tool | Self::Function | Self::HumanGate | Self::Subworkflow
        )
    }

    pub fn can_use_model(self) -> bool {
        matches!(self, Self::Agent | Self::Router | Self::Evaluator)
    }

    pub fn can_use_tools(self) -> bool {
        matches!(self, Self::Agent)
    }
}
