use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TypeSpec {
    Any,
    String,
    Integer,
    Number,
    Boolean,
    Object,
    Array(Box<TypeSpec>),
    Custom(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Selector(String);

impl Selector {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn is_empty_or_whitespace(&self) -> bool {
        self.0.trim().is_empty()
    }
}

impl From<&str> for Selector {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

impl From<String> for Selector {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

impl fmt::Display for Selector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ArtifactRef {
    pub id: String,
    pub kind: Option<String>,
    pub metadata: Value,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct UsageMetrics {
    pub model_calls: u32,
    pub tool_calls: u32,
    pub input_tokens: u64,
    pub output_tokens: u64,
    pub cost_usd: f64,
}
