//! # Unification Kernel (UNIFORM)
//!
//! Layer 4: Unification of semantic analysis results.

use std::collections::HashMap;

/// Unification context
#[derive(Debug, Clone, Default)]
pub struct UnifyContext {
    bindings: HashMap<String, String>,
    constraints: Vec<Constraint>,
}

/// A constraint in unification
#[derive(Debug, Clone)]
pub struct Constraint {
    pub var1: String,
    pub var2: String,
    pub relation: ConstraintRelation,
}

/// Constraint relation types
#[derive(Debug, Clone, PartialEq)]
pub enum ConstraintRelation {
    Equal,
    NotEqual,
    Compatible,
    Incompatible,
}

impl UnifyContext {
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a binding
    pub fn bind(&mut self, var: &str, value: &str) {
        self.bindings.insert(var.to_string(), value.to_string());
    }

    /// Get a binding
    pub fn get(&self, var: &str) -> Option<&String> {
        self.bindings.get(var)
    }

    /// Add a constraint
    pub fn add_constraint(&mut self, constraint: Constraint) {
        self.constraints.push(constraint);
    }

    /// Check if all constraints are satisfied
    pub fn is_consistent(&self) -> bool {
        for c in &self.constraints {
            match c.relation {
                ConstraintRelation::Equal => {
                    if let (Some(v1), Some(v2)) = (self.get(&c.var1), self.get(&c.var2)) {
                        if v1 != v2 { return false; }
                    }
                }
                ConstraintRelation::NotEqual => {
                    if let (Some(v1), Some(v2)) = (self.get(&c.var1), self.get(&c.var2)) {
                        if v1 == v2 { return false; }
                    }
                }
                _ => {}
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unify_context() {
        let mut ctx = UnifyContext::new();
        ctx.bind("X", "hello");
        assert_eq!(ctx.get("X"), Some(&"hello".to_string()));
    }
}
