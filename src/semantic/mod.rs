//! # Semantic Database Module
//!
//! Semantic categorization and context analysis for English.

use std::collections::HashMap;

/// Semantic database
#[derive(Debug, Clone)]
pub struct SemanticDB {
    words: HashMap<String, SemanticEntry>,
}

/// Semantic entry
#[derive(Debug, Clone)]
pub struct SemanticEntry {
    pub word: String,
    pub category: SemanticCategory,
    pub subcategory: Option<String>,
    pub tags: Vec<String>,
}

/// Semantic category
#[derive(Debug, Clone, PartialEq)]
pub enum SemanticCategory {
    Place { place_type: String },
    Person { role: Option<String> },
    Object { object_type: String },
    Emotion { valence: Valence },
    Concept { domain: Option<String> },
    Action { action_type: String },
    Time { time_type: String },
    Quantity,
    Quality,
    Unknown,
}

/// Emotional valence
#[derive(Debug, Clone, PartialEq)]
pub enum Valence {
    Positive,
    Negative,
    Neutral,
}

impl Default for SemanticDB {
    fn default() -> Self {
        Self::new()
    }
}

impl SemanticDB {
    pub fn new() -> Self {
        Self {
            words: HashMap::new(),
        }
    }

    /// Get semantic category for a word
    pub fn get_category(&self, word: &str) -> Option<&SemanticCategory> {
        self.words.get(&word.to_lowercase()).map(|e| &e.category)
    }

    /// Add word with semantic info
    pub fn add(&mut self, word: &str, category: SemanticCategory) {
        self.words.insert(word.to_lowercase(), SemanticEntry {
            word: word.to_lowercase(),
            category,
            subcategory: None,
            tags: vec![],
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_semantic_db() {
        let mut db = SemanticDB::new();
        db.add("happiness", SemanticCategory::Emotion { valence: Valence::Positive });
        assert!(db.get_category("happiness").is_some());
    }
}
