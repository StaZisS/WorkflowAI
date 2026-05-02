use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ExecutionHints {
    pub timeout_ms: Option<u64>,
    pub max_attempts: Option<u32>,
    pub max_model_calls: Option<u32>,
    pub max_tool_calls: Option<u32>,
    pub max_cost_usd: Option<f64>,
    pub checkpoint_after_node: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EffectiveExecutionPolicy {
    pub timeout_ms: u64,
    pub max_attempts: u32,
    pub max_model_calls: u32,
    pub max_tool_calls: u32,
    pub max_cost_usd: Option<f64>,
    pub checkpoint_after_node: bool,
}

impl Default for EffectiveExecutionPolicy {
    fn default() -> Self {
        Self {
            timeout_ms: 60_000,
            max_attempts: 1,
            max_model_calls: 4,
            max_tool_calls: 8,
            max_cost_usd: None,
            checkpoint_after_node: true,
        }
    }
}

impl EffectiveExecutionPolicy {
    pub fn from_hints(defaults: &Self, hints: &ExecutionHints) -> Self {
        Self {
            timeout_ms: hints.timeout_ms.unwrap_or(defaults.timeout_ms),
            max_attempts: hints.max_attempts.unwrap_or(defaults.max_attempts),
            max_model_calls: hints.max_model_calls.unwrap_or(defaults.max_model_calls),
            max_tool_calls: hints.max_tool_calls.unwrap_or(defaults.max_tool_calls),
            max_cost_usd: hints.max_cost_usd.or(defaults.max_cost_usd),
            checkpoint_after_node: hints
                .checkpoint_after_node
                .unwrap_or(defaults.checkpoint_after_node),
        }
    }
}
