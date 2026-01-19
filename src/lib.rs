//! # NL-SRE-English
//!
//! Probabilistic Semantic Disambiguation Engine for English
//!
//! ## Architecture (4 Layers)
//!
//! ```text
//! LAYER 4: UNIFORM (Unification Kernel)
//! LAYER 3: TAO (Encapsulation + Messages)
//! LAYER 2: APPLOG (Shared Variables + Constraints)
//! LAYER 1: Base Engines (Grammar + Semantic)
//! ```
//!
//! ## Key Features
//!
//! - **Functional Verb Groups**: 25+ semantic categories for verbs
//! - **Full Dictionary**: 100K+ words with frequency data
//! - **Zero Dependencies**: Pure Rust, no external crates
//! - **Command Parser**: Natural language to structured commands
//!
//! ## Author
//! Francisco Molina-Burgos, Avermex Research Division
//!
//! ## Date
//! January 2026

pub mod uniform;
pub mod applog;
pub mod tao;
pub mod grammar;
pub mod semantic;
pub mod disambiguator;
pub mod chars;
pub mod dictionary;
pub mod verbs;
pub mod command_parser;
pub mod domain;
pub mod quantitative;

// Main re-exports
pub use disambiguator::SemanticDisambiguator;
pub use uniform::UnifyContext;
pub use applog::SharedContext;
pub use grammar::EnglishGrammar;
pub use semantic::{SemanticDB, SemanticCategory};
pub use dictionary::{EnglishDictionary, DictionaryEntry};
pub use verbs::{VerbDatabase, VerbEntry, VerbGroup, FunctionalCategory};
pub use command_parser::{CommandParser, ParsedCommand, ParserStats};

// Domain plugin exports (for NL-SRE-Domains integration)
pub use domain::{DomainPlugin, DomainRegistry, DomainConstraint, ValidatedQuantity, GenericSIDomain};

// Quantitative tokenizer exports
pub use quantitative::{QuantitativeTokenizer, QuantitativeToken, TokenKind, TokenizerConfig};

/// Processed sentence result
#[derive(Debug, Clone)]
pub struct ProcessedSentence {
    /// Original sentence
    pub original: String,
    /// Corrected sentence
    pub corrected: String,
    /// Global confidence (0.0 - 1.0)
    pub confidence: f64,
    /// Individual corrections applied
    pub corrections: Vec<Correction>,
    /// Detected verb actions
    pub detected_actions: Vec<DetectedAction>,
}

/// An individual correction
#[derive(Debug, Clone)]
pub struct Correction {
    /// Position in sentence (token index)
    pub position: usize,
    /// Original word (possibly erroneous)
    pub original: String,
    /// Corrected word
    pub corrected: String,
    /// Confidence of this correction
    pub confidence: f64,
    /// Explanation of why this correction was chosen
    pub explanation: CorrectionExplanation,
}

/// Detected action from verb analysis
#[derive(Debug, Clone)]
pub struct DetectedAction {
    /// The verb detected
    pub verb: String,
    /// Base form (infinitive)
    pub base_form: String,
    /// Functional category
    pub category: FunctionalCategory,
    /// Specific verb group
    pub group: VerbGroup,
    /// Confidence score
    pub confidence: f64,
    /// Position in sentence
    pub position: usize,
}

/// Detailed explanation of a correction
#[derive(Debug, Clone)]
pub struct CorrectionExplanation {
    /// Character similarity score
    pub char_score: f64,
    /// Grammar score
    pub grammar_score: f64,
    /// Semantic context score
    pub context_score: f64,
    /// Candidates considered with their scores
    pub candidates: Vec<(String, f64)>,
    /// Reason in readable text
    pub reason: String,
}

/// Engine configuration
#[derive(Debug, Clone)]
pub struct Config {
    /// Weight for character similarity (α)
    pub alpha: f64,
    /// Weight for grammar validation (β)
    pub beta: f64,
    /// Weight for semantic context (γ)
    pub gamma: f64,
    /// Minimum confidence threshold to accept correction
    pub min_confidence: f64,
    /// Maximum candidates to consider
    pub max_candidates: usize,
    /// Enable verb action detection
    pub detect_actions: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            alpha: 0.30,  // 30% weight to characters
            beta: 0.30,   // 30% weight to grammar
            gamma: 0.40,  // 40% weight to semantic context
            min_confidence: 0.60,
            max_candidates: 10,
            detect_actions: true,
        }
    }
}

/// Engine version
pub const VERSION: &str = "0.1.2";

/// Engine information
pub fn info() -> String {
    format!(
        "NL-SRE-English v{}\n\
         Probabilistic Semantic Disambiguation Engine\n\
         Author: Francisco Molina-Burgos, Avermex Research Division\n\
         Zero dependencies - Pure Rust\n\
         Features: 25+ functional verb categories, 100K+ dictionary",
        VERSION
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let cfg = Config::default();
        assert!((cfg.alpha + cfg.beta + cfg.gamma - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_info() {
        let info = info();
        assert!(info.contains("NL-SRE-English"));
        assert!(info.contains("Francisco Molina-Burgos"));
    }
}
