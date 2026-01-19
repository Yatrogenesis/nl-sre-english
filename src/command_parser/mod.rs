//! # Command Parser Module
//!
//! Parse natural language into structured commands.

use crate::verbs::{VerbDatabase, FunctionalCategory, VerbGroup};
use crate::grammar::EnglishGrammar;

/// Parsed command structure
#[derive(Debug, Clone)]
pub struct ParsedCommand {
    /// Main action verb
    pub action: String,
    /// Action category
    pub category: FunctionalCategory,
    /// Action group
    pub group: VerbGroup,
    /// Subject (who/what performs)
    pub subject: Option<String>,
    /// Object (who/what receives)
    pub object: Option<String>,
    /// Modifiers
    pub modifiers: Vec<String>,
    /// Confidence score
    pub confidence: f64,
    /// Original input
    pub original: String,
}

/// Parser statistics
#[derive(Debug, Clone, Default)]
pub struct ParserStats {
    pub commands_parsed: usize,
    pub verbs_detected: usize,
    pub avg_confidence: f64,
}

/// Natural language command parser
pub struct CommandParser {
    verbs: VerbDatabase,
    grammar: EnglishGrammar,
    stats: ParserStats,
}

impl Default for CommandParser {
    fn default() -> Self {
        Self::new()
    }
}

impl CommandParser {
    pub fn new() -> Self {
        Self {
            verbs: VerbDatabase::with_builtin(),
            grammar: EnglishGrammar::new(),
            stats: ParserStats::default(),
        }
    }

    /// Parse a command from natural language
    pub fn parse(&mut self, input: &str) -> Option<ParsedCommand> {
        let tokens = self.grammar.tokenize(input);
        if tokens.is_empty() { return None; }

        // Find the main verb
        let mut action_idx = None;
        let mut action_entry = None;

        for (i, token) in tokens.iter().enumerate() {
            if let Some(entry) = self.verbs.lookup(token) {
                action_idx = Some(i);
                action_entry = Some(entry.clone());
                break;
            }
        }

        let entry = action_entry?;
        let idx = action_idx?;

        // Extract subject (before verb)
        let subject = if idx > 0 {
            Some(tokens[..idx].join(" "))
        } else {
            None
        };

        // Extract object (after verb)
        let object = if idx + 1 < tokens.len() {
            Some(tokens[idx + 1..].join(" "))
        } else {
            None
        };

        // Update stats
        self.stats.commands_parsed += 1;
        self.stats.verbs_detected += 1;

        Some(ParsedCommand {
            action: entry.base.clone(),
            category: entry.category,
            group: entry.group,
            subject,
            object,
            modifiers: vec![],
            confidence: 0.85,
            original: input.to_string(),
        })
    }

    /// Parse multiple commands from text
    pub fn parse_all(&mut self, text: &str) -> Vec<ParsedCommand> {
        // Split by common sentence delimiters
        text.split(|c: char| c == '.' || c == '!' || c == '?' || c == ';')
            .filter(|s| !s.trim().is_empty())
            .filter_map(|s| self.parse(s.trim()))
            .collect()
    }

    /// Get parser statistics
    pub fn stats(&self) -> &ParserStats {
        &self.stats
    }

    /// Get verbs for a specific category (for autocompletion)
    pub fn suggest_verbs(&self, category: FunctionalCategory, limit: usize) -> Vec<String> {
        self.verbs.by_category(category)
            .iter()
            .take(limit)
            .map(|e| e.base.clone())
            .collect()
    }

    /// Check if word is a known verb
    pub fn is_action(&self, word: &str) -> bool {
        self.verbs.is_verb(word)
    }

    /// Get action category
    pub fn get_action_category(&self, word: &str) -> Option<FunctionalCategory> {
        self.verbs.get_category(word)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_command() {
        let mut parser = CommandParser::new();
        // Note: avoid "please" as first word since it's a registered verb
        let cmd = parser.parse("walk to the store").unwrap();
        assert_eq!(cmd.action, "walk");
        assert_eq!(cmd.category, FunctionalCategory::Movement);
    }

    #[test]
    fn test_parse_multiple() {
        let mut parser = CommandParser::new();
        let cmds = parser.parse_all("Run to the store. Buy some milk. Come back home.");
        assert_eq!(cmds.len(), 3);
    }

    #[test]
    fn test_suggest_verbs() {
        let parser = CommandParser::new();
        let movement = parser.suggest_verbs(FunctionalCategory::Movement, 5);
        assert!(!movement.is_empty());
    }
}
