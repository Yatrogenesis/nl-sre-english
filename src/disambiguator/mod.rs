//! # Semantic Disambiguator
//!
//! Main engine for semantic disambiguation.

use crate::{Config, ProcessedSentence, Correction, CorrectionExplanation, DetectedAction};
use crate::verbs::{VerbDatabase, FunctionalCategory, VerbGroup};
use crate::grammar::EnglishGrammar;
use crate::dictionary::EnglishDictionary;

/// Main semantic disambiguator
pub struct SemanticDisambiguator {
    #[allow(dead_code)]
    config: Config,
    verbs: VerbDatabase,
    grammar: EnglishGrammar,
    dictionary: EnglishDictionary,
}

impl Default for SemanticDisambiguator {
    fn default() -> Self {
        Self::new()
    }
}

impl SemanticDisambiguator {
    pub fn new() -> Self {
        Self {
            config: Config::default(),
            verbs: VerbDatabase::with_builtin(),
            grammar: EnglishGrammar::new(),
            dictionary: EnglishDictionary::new(),
        }
    }

    pub fn with_config(config: Config) -> Self {
        Self {
            config,
            verbs: VerbDatabase::with_builtin(),
            grammar: EnglishGrammar::new(),
            dictionary: EnglishDictionary::new(),
        }
    }

    /// Process a sentence
    pub fn process(&self, sentence: &str) -> ProcessedSentence {
        let tokens = self.grammar.tokenize(sentence);
        let mut corrections = Vec::new();
        let mut detected_actions = Vec::new();
        let mut corrected_tokens = tokens.clone();

        for (i, token) in tokens.iter().enumerate() {
            // Check for verb actions
            if let Some(entry) = self.verbs.lookup(token) {
                detected_actions.push(DetectedAction {
                    verb: token.clone(),
                    base_form: entry.base.clone(),
                    category: entry.category,
                    group: entry.group,
                    confidence: 0.95,
                    position: i,
                });
            }

            // Check for spelling errors
            if !self.dictionary.is_valid(token) && !self.verbs.is_verb(token) {
                if let Some((corrected, _)) = self.suggest_correction(token) {
                    corrections.push(Correction {
                        position: i,
                        original: token.clone(),
                        corrected: corrected.clone(),
                        confidence: 0.8,
                        explanation: CorrectionExplanation {
                            char_score: 0.8,
                            grammar_score: 0.7,
                            context_score: 0.75,
                            candidates: vec![(corrected.clone(), 0.8)],
                            reason: format!("Spelling correction: {} -> {}", token, corrected),
                        },
                    });
                    corrected_tokens[i] = corrected;
                }
            }
        }

        let confidence = if corrections.is_empty() { 1.0 } else { 0.85 };

        ProcessedSentence {
            original: sentence.to_string(),
            corrected: corrected_tokens.join(" "),
            confidence,
            corrections,
            detected_actions,
        }
    }

    /// Suggest a spelling correction
    fn suggest_correction(&self, word: &str) -> Option<(String, f64)> {
        let similar = self.dictionary.find_similar(word, 2);
        similar.into_iter()
            .min_by_key(|(_, dist)| *dist)
            .map(|(w, dist)| (w, 1.0 - (dist as f64 * 0.2)))
    }

    /// Get verb database reference
    pub fn verbs(&self) -> &VerbDatabase {
        &self.verbs
    }

    /// Get grammar reference
    pub fn grammar(&self) -> &EnglishGrammar {
        &self.grammar
    }

    /// Get dictionary reference
    pub fn dictionary(&self) -> &EnglishDictionary {
        &self.dictionary
    }

    /// Detect actions in a sentence
    pub fn detect_actions(&self, sentence: &str) -> Vec<DetectedAction> {
        let tokens = self.grammar.tokenize(sentence);
        let mut actions = Vec::new();

        for (i, token) in tokens.iter().enumerate() {
            if let Some(entry) = self.verbs.lookup(token) {
                actions.push(DetectedAction {
                    verb: token.clone(),
                    base_form: entry.base.clone(),
                    category: entry.category,
                    group: entry.group,
                    confidence: 0.95,
                    position: i,
                });
            }
        }

        actions
    }

    /// Get all verbs by category
    pub fn verbs_by_category(&self, category: FunctionalCategory) -> Vec<String> {
        self.verbs.by_category(category)
            .iter()
            .map(|e| e.base.clone())
            .collect()
    }

    /// Get all verbs by group
    pub fn verbs_by_group(&self, group: VerbGroup) -> Vec<String> {
        self.verbs.by_group(group)
            .iter()
            .map(|e| e.base.clone())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_disambiguator() {
        let dis = SemanticDisambiguator::new();
        let result = dis.process("The cat runs quickly");
        assert!(!result.detected_actions.is_empty());
        assert_eq!(result.detected_actions[0].base_form, "run");
    }

    #[test]
    fn test_action_detection() {
        let dis = SemanticDisambiguator::new();
        let actions = dis.detect_actions("She walked to the store and bought some food");
        assert!(actions.len() >= 2);
    }

    #[test]
    fn test_category_lookup() {
        let dis = SemanticDisambiguator::new();
        let movement = dis.verbs_by_category(FunctionalCategory::Movement);
        assert!(movement.contains(&"walk".to_string()));
        assert!(movement.contains(&"run".to_string()));
    }
}
