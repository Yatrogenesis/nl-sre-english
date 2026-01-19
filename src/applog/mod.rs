//! # Application Logic (APPLOG)
//!
//! Layer 2: Shared context and constraint management.

use std::collections::HashMap;

/// Shared context for semantic analysis
#[derive(Debug, Clone, Default)]
pub struct SharedContext {
    variables: HashMap<String, ContextValue>,
    metadata: HashMap<String, String>,
}

/// Context value types
#[derive(Debug, Clone)]
pub enum ContextValue {
    String(String),
    Number(f64),
    Boolean(bool),
    List(Vec<ContextValue>),
}

impl SharedContext {
    pub fn new() -> Self {
        Self::default()
    }

    /// Set a string value
    pub fn set_string(&mut self, key: &str, value: &str) {
        self.variables.insert(key.to_string(), ContextValue::String(value.to_string()));
    }

    /// Set a number value
    pub fn set_number(&mut self, key: &str, value: f64) {
        self.variables.insert(key.to_string(), ContextValue::Number(value));
    }

    /// Get a value
    pub fn get(&self, key: &str) -> Option<&ContextValue> {
        self.variables.get(key)
    }

    /// Get as string
    pub fn get_string(&self, key: &str) -> Option<&str> {
        match self.get(key) {
            Some(ContextValue::String(s)) => Some(s.as_str()),
            _ => None,
        }
    }

    /// Set metadata
    pub fn set_metadata(&mut self, key: &str, value: &str) {
        self.metadata.insert(key.to_string(), value.to_string());
    }

    /// Get metadata
    pub fn get_metadata(&self, key: &str) -> Option<&str> {
        self.metadata.get(key).map(|s| s.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shared_context() {
        let mut ctx = SharedContext::new();
        ctx.set_string("theme", "technology");
        assert_eq!(ctx.get_string("theme"), Some("technology"));
    }
}
