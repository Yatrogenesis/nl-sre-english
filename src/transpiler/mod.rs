// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Francisco Molina-Burgos, Avermex Research Division

//! # English to PIRS Transpiler
//!
//! Converts natural English text into PIRS (Prolog) rules.
//! Handles: declarative SVO, questions, commands (imperative),
//! negation, adjective modifiers, prepositional phrases.
//!
//! ## Examples
//!
//! ```
//! use nl_sre_english::transpiler::Transpiler;
//!
//! let transpiler = Transpiler::new();
//! let rules = transpiler.to_pirs("The cat eats fish.");
//! assert_eq!(rules.len(), 1);
//! assert_eq!(rules[0].to_prolog(), "eat(cat, fish).");
//! ```

use crate::grammar::EnglishGrammar;
use crate::verbs::VerbDatabase;

/// Sentence type detected from surface form
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SentenceType {
    /// Statement: "The cat eats fish."
    Declarative,
    /// Question: "Does the cat eat fish?"
    Question,
    /// Command: "Eat the fish!"
    Command,
}

/// A PIRS (Prolog) rule generated from English text
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PirsRule {
    /// The rule head (predicate name, typically a verb lemma)
    pub head: String,
    /// Arguments (subject, objects)
    pub args: Vec<String>,
    /// Body conditions (prepositional phrases, sentence type metadata) - empty for facts
    pub body: Vec<String>,
}

impl PirsRule {
    /// Convert to Prolog-style string representation
    pub fn to_prolog(&self) -> String {
        let head = if self.args.is_empty() {
            self.head.clone()
        } else {
            format!("{}({})", self.head, self.args.join(", "))
        };

        if self.body.is_empty() {
            format!("{}.", head)
        } else {
            format!("{} :- {}.", head, self.body.join(", "))
        }
    }
}

impl std::fmt::Display for PirsRule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_prolog())
    }
}

/// A parsed token with POS and metadata
#[derive(Debug, Clone)]
struct Token {
    lemma: String,
    pos: TokenPOS,
    is_negation: bool,
    is_preposition: bool,
}

/// Simplified POS tags for transpiler use
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TokenPOS {
    Noun,
    Verb,
    Adjective,
    Adverb,
    Pronoun,
    Article,
    Preposition,
    Auxiliary,
    Conjunction,
}

/// A noun phrase with optional adjective modifiers
#[derive(Debug, Clone)]
struct NounPhrase {
    head: String,
    modifiers: Vec<String>,
}

impl NounPhrase {
    /// Convert to PIRS term representation
    fn to_pirs_term(&self) -> String {
        if self.modifiers.is_empty() {
            self.head.clone()
        } else {
            format!("{}_{}", self.head, self.modifiers.join("_"))
        }
    }
}

/// A prepositional phrase
#[derive(Debug, Clone)]
struct PrepPhrase {
    prep: String,
    object: NounPhrase,
}

impl PrepPhrase {
    /// Convert to PIRS body condition
    fn to_pirs_term(&self) -> String {
        format!("{}({})", self.prep, self.object.to_pirs_term())
    }
}

/// English negation words
const NEGATION_WORDS: &[&str] = &[
    "not", "never", "no", "neither", "nor", "nowhere", "nothing",
    "nobody", "none", "hardly", "barely", "scarcely",
];

/// Common English adjectives for POS detection
const COMMON_ADJECTIVES: &[&str] = &[
    "big", "small", "large", "little", "tall", "short", "long", "wide", "narrow",
    "thick", "thin", "heavy", "light", "fast", "slow", "quick", "hot", "cold",
    "warm", "cool", "wet", "dry", "hard", "soft", "rough", "smooth", "sharp",
    "dull", "bright", "dark", "loud", "quiet", "clean", "dirty", "full", "empty",
    "rich", "poor", "strong", "weak", "young", "old", "new", "fresh", "stale",
    "good", "bad", "great", "nice", "fine", "beautiful", "ugly", "pretty",
    "handsome", "happy", "sad", "angry", "afraid", "brave", "calm", "nervous",
    "tired", "busy", "free", "safe", "dangerous", "easy", "difficult", "hard",
    "simple", "complex", "clear", "confusing", "right", "wrong", "true", "false",
    "real", "fake", "important", "serious", "funny", "strange", "weird", "normal",
    "special", "common", "rare", "popular", "famous", "unknown", "obvious",
    "red", "blue", "green", "yellow", "white", "black", "brown", "pink", "purple",
    "orange", "gray", "golden", "silver", "entire", "whole", "complete", "partial",
    "main", "major", "minor", "primary", "secondary", "final", "initial", "next",
    "previous", "current", "modern", "ancient", "recent", "early", "late",
];

/// WH-question words
const WH_WORDS: &[&str] = &[
    "who", "what", "where", "when", "why", "how", "which", "whom", "whose",
];

/// English to PIRS Transpiler
pub struct Transpiler {
    grammar: EnglishGrammar,
    verbs: VerbDatabase,
    adjective_set: std::collections::HashSet<String>,
}

impl Transpiler {
    /// Create a new transpiler
    pub fn new() -> Self {
        let mut adjective_set = std::collections::HashSet::new();
        for adj in COMMON_ADJECTIVES {
            adjective_set.insert(adj.to_string());
        }
        Self {
            grammar: EnglishGrammar::new(),
            verbs: VerbDatabase::with_builtin(),
            adjective_set,
        }
    }

    /// Transpile English text to PIRS rules
    ///
    /// Splits the text into sentences and converts each one.
    pub fn to_pirs(&self, text: &str) -> Vec<PirsRule> {
        let mut rules = Vec::new();

        let mut start = 0;
        let chars: Vec<char> = text.chars().collect();
        let mut i = 0;

        while i < chars.len() {
            let c = chars[i];
            if c == '.' || c == '?' || c == '!' {
                let byte_start = text.char_indices().nth(start).map(|(idx, _)| idx).unwrap_or(0);
                let byte_end = text.char_indices().nth(i).map(|(idx, _)| idx).unwrap_or(text.len());
                let sentence = text[byte_start..byte_end].trim();
                if !sentence.is_empty() {
                    let stype = match c {
                        '?' => SentenceType::Question,
                        '!' => SentenceType::Command,
                        _ => SentenceType::Declarative,
                    };
                    if let Some(rule) = self.transpile_sentence(sentence, stype) {
                        rules.push(rule);
                    }
                }
                start = i + 1;
            }
            i += 1;
        }

        // Handle trailing sentence without punctuation
        if start < chars.len() {
            let byte_start = text.char_indices().nth(start).map(|(idx, _)| idx).unwrap_or(0);
            let remaining = text[byte_start..].trim();
            if !remaining.is_empty() {
                let stype = self.detect_sentence_type(remaining);
                if let Some(rule) = self.transpile_sentence(remaining, stype) {
                    rules.push(rule);
                }
            }
        }

        rules
    }

    /// Transpile a single English sentence into a PIRS rule
    pub fn transpile_sentence(&self, sentence: &str, stype: SentenceType) -> Option<PirsRule> {
        let tokens = self.tokenize(sentence);
        if tokens.is_empty() {
            return None;
        }
        self.parse_tokens(&tokens, stype)
    }

    /// Detect sentence type from surface form (for sentences without punctuation)
    fn detect_sentence_type(&self, sentence: &str) -> SentenceType {
        let lower = sentence.to_lowercase();
        let first_word = lower.split_whitespace().next().unwrap_or("");

        // WH-questions
        if WH_WORDS.contains(&first_word) {
            return SentenceType::Question;
        }

        // Auxiliary-initial questions: "Do you...", "Is he...", "Can we...", etc.
        let aux_question_starters = [
            "do", "does", "did", "is", "are", "was", "were", "am",
            "will", "would", "shall", "should", "can", "could", "may", "might",
            "has", "have", "had",
        ];
        if aux_question_starters.contains(&first_word) {
            // Heuristic: if first word is aux and there's a pronoun/noun after it, likely question
            let words: Vec<&str> = lower.split_whitespace().collect();
            if words.len() >= 2 {
                let second = words[1];
                if self.grammar.is_pronoun(second) || !self.verbs.is_verb(second) {
                    return SentenceType::Question;
                }
            }
        }

        // Imperative: starts with a verb (no subject before it)
        if self.verbs.is_verb(first_word) && !self.grammar.is_auxiliary(first_word) {
            return SentenceType::Command;
        }

        // Special imperative starters
        if first_word == "please" || first_word == "let" {
            return SentenceType::Command;
        }

        SentenceType::Declarative
    }

    /// Tokenize an English sentence into tagged tokens
    fn tokenize(&self, sentence: &str) -> Vec<Token> {
        let raw_tokens = self.grammar.tokenize(sentence);
        let mut tokens = Vec::new();

        for word in &raw_tokens {
            let lower = word.to_lowercase();

            // Skip empty
            if lower.is_empty() {
                continue;
            }

            // Negation
            if NEGATION_WORDS.contains(&lower.as_str()) {
                tokens.push(Token {
                    lemma: lower,
                    pos: TokenPOS::Adverb,
                    is_negation: true,
                    is_preposition: false,
                });
                continue;
            }

            // Classify POS
            let (pos, lemma) = self.classify_token(&lower);

            let is_prep = pos == TokenPOS::Preposition;
            tokens.push(Token {
                lemma,
                pos,
                is_negation: false,
                is_preposition: is_prep,
            });
        }

        tokens
    }

    /// Classify a token's part of speech
    fn classify_token(&self, word: &str) -> (TokenPOS, String) {
        // Article
        if self.grammar.is_article(word) {
            return (TokenPOS::Article, word.to_string());
        }

        // Preposition
        if self.grammar.is_preposition(word) {
            return (TokenPOS::Preposition, word.to_string());
        }

        // Pronoun
        if self.grammar.is_pronoun(word) {
            return (TokenPOS::Pronoun, word.to_string());
        }

        // Conjunction
        if self.grammar.is_conjunction(word) {
            return (TokenPOS::Conjunction, word.to_string());
        }

        // Auxiliary
        if self.grammar.is_auxiliary(word) {
            return (TokenPOS::Auxiliary, word.to_string());
        }

        // Verb - look up and get base form
        if let Some(base) = self.verbs.base_form(word) {
            return (TokenPOS::Verb, base.to_string());
        }

        // Adjective
        if self.adjective_set.contains(word) || word.ends_with("ly") {
            if word.ends_with("ly") && !self.adjective_set.contains(word) {
                return (TokenPOS::Adverb, word.to_string());
            }
            return (TokenPOS::Adjective, word.to_string());
        }

        // Default: noun
        (TokenPOS::Noun, word.to_string())
    }

    /// Parse tokens into a PIRS rule
    fn parse_tokens(&self, tokens: &[Token], stype: SentenceType) -> Option<PirsRule> {
        // Detect negation
        let negated = tokens.iter().any(|t| t.is_negation);

        // Filter out negation markers, articles, auxiliaries, conjunctions, adverbs
        let content_tokens: Vec<&Token> = tokens.iter()
            .filter(|t| {
                !t.is_negation
                    && t.pos != TokenPOS::Article
                    && t.pos != TokenPOS::Auxiliary
                    && t.pos != TokenPOS::Conjunction
                    && t.pos != TokenPOS::Adverb
            })
            .collect();

        if content_tokens.is_empty() {
            return None;
        }

        // Find verb
        let verb_idx = content_tokens.iter().position(|t| t.pos == TokenPOS::Verb);

        match verb_idx {
            Some(idx) => {
                let verb = content_tokens[idx];

                // Collect subject tokens (before verb)
                let subject_tokens = &content_tokens[..idx];
                // Collect object tokens (after verb)
                let object_tokens = &content_tokens[idx + 1..];

                let subject_phrases = self.extract_noun_phrases(subject_tokens);
                let (object_phrases, prep_phrases) = self.extract_objects_and_preps(object_tokens);

                // Build head predicate
                let head = if negated {
                    format!("not_{}", verb.lemma)
                } else {
                    verb.lemma.clone()
                };

                // Build arguments
                let mut args: Vec<String> = Vec::new();
                for np in &subject_phrases {
                    args.push(np.to_pirs_term());
                }
                for np in &object_phrases {
                    args.push(np.to_pirs_term());
                }

                // Build body from prepositional phrases + sentence type
                let mut body: Vec<String> = Vec::new();
                for pp in &prep_phrases {
                    body.push(pp.to_pirs_term());
                }

                match stype {
                    SentenceType::Question => {
                        body.push("type(question)".to_string());
                    }
                    SentenceType::Command => {
                        body.push("type(imperative)".to_string());
                    }
                    SentenceType::Declarative => {}
                }

                Some(PirsRule { head, args, body })
            }
            None => {
                // No verb found: create a property/fact from noun phrases
                let noun_phrases = self.extract_noun_phrases(&content_tokens);
                if noun_phrases.is_empty() {
                    return None;
                }

                let head = noun_phrases[0].head.clone();
                let args: Vec<String> = noun_phrases[1..].iter()
                    .map(|np| np.to_pirs_term())
                    .collect();

                let mut body = Vec::new();
                for adj in &noun_phrases[0].modifiers {
                    body.push(format!("{}({})", adj, head));
                }

                if stype == SentenceType::Question {
                    body.push("type(question)".to_string());
                }

                Some(PirsRule { head, args, body })
            }
        }
    }

    /// Extract noun phrases with adjective modifiers from a token slice
    fn extract_noun_phrases(&self, tokens: &[&Token]) -> Vec<NounPhrase> {
        let mut phrases = Vec::new();
        let mut i = 0;

        while i < tokens.len() {
            if tokens[i].pos == TokenPOS::Noun || tokens[i].pos == TokenPOS::Pronoun {
                let mut np = NounPhrase {
                    head: tokens[i].lemma.clone(),
                    modifiers: Vec::new(),
                };

                // Look ahead for adjective modifiers (English: adj before noun is more common,
                // but we also handle post-nominal for generality)
                let mut j = i + 1;
                while j < tokens.len() && tokens[j].pos == TokenPOS::Adjective {
                    np.modifiers.push(tokens[j].lemma.clone());
                    j += 1;
                }

                // Check for preceding adjectives
                if i > 0 && tokens[i - 1].pos == TokenPOS::Adjective {
                    let already_consumed = phrases.iter().any(|p: &NounPhrase|
                        p.modifiers.contains(&tokens[i - 1].lemma)
                    );
                    if !already_consumed {
                        np.modifiers.insert(0, tokens[i - 1].lemma.clone());
                    }
                }

                phrases.push(np);
                i = j;
            } else {
                i += 1;
            }
        }

        phrases
    }

    /// Extract objects and prepositional phrases from post-verb tokens
    fn extract_objects_and_preps(&self, tokens: &[&Token]) -> (Vec<NounPhrase>, Vec<PrepPhrase>) {
        let mut objects = Vec::new();
        let mut preps = Vec::new();
        let mut i = 0;

        while i < tokens.len() {
            if tokens[i].is_preposition {
                let prep = tokens[i].lemma.clone();
                i += 1;

                // Skip articles after preposition
                while i < tokens.len() && tokens[i].pos == TokenPOS::Article {
                    i += 1;
                }

                // Collect the NP after the preposition
                let mut np_tokens = Vec::new();
                while i < tokens.len() && !tokens[i].is_preposition && tokens[i].pos != TokenPOS::Verb {
                    np_tokens.push(tokens[i]);
                    i += 1;
                }

                let noun_phrases = self.extract_noun_phrases(&np_tokens);
                if let Some(np) = noun_phrases.into_iter().next() {
                    preps.push(PrepPhrase { prep, object: np });
                }
            } else if tokens[i].pos == TokenPOS::Noun || tokens[i].pos == TokenPOS::Pronoun {
                let mut np = NounPhrase {
                    head: tokens[i].lemma.clone(),
                    modifiers: Vec::new(),
                };
                i += 1;
                while i < tokens.len() && tokens[i].pos == TokenPOS::Adjective {
                    np.modifiers.push(tokens[i].lemma.clone());
                    i += 1;
                }
                objects.push(np);
            } else {
                i += 1;
            }
        }

        (objects, preps)
    }
}

impl Default for Transpiler {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_svo() {
        let t = Transpiler::new();
        let rules = t.to_pirs("The cat eats fish.");
        assert_eq!(rules.len(), 1);
        assert_eq!(rules[0].head, "eat");
        assert_eq!(rules[0].to_prolog(), "eat(cat, fish).");
    }

    #[test]
    fn test_negation() {
        let t = Transpiler::new();
        let rules = t.to_pirs("The cat does not eat fish.");
        assert_eq!(rules.len(), 1);
        assert_eq!(rules[0].head, "not_eat");
        assert!(rules[0].to_prolog().starts_with("not_eat("));
    }

    #[test]
    fn test_question() {
        let t = Transpiler::new();
        let rules = t.to_pirs("Does the cat eat fish?");
        assert_eq!(rules.len(), 1);
        assert!(rules[0].body.contains(&"type(question)".to_string()));
    }

    #[test]
    fn test_command() {
        let t = Transpiler::new();
        let rules = t.to_pirs("Eat the fish!");
        assert_eq!(rules.len(), 1);
        assert_eq!(rules[0].head, "eat");
        assert!(rules[0].body.contains(&"type(imperative)".to_string()));
    }

    #[test]
    fn test_prepositional_phrase() {
        let t = Transpiler::new();
        let rules = t.to_pirs("The cat eats in the house.");
        assert_eq!(rules.len(), 1);
        assert_eq!(rules[0].head, "eat");
        assert!(rules[0].body.iter().any(|b| b == "in(house)"));
    }

    #[test]
    fn test_adjective_modifier() {
        let t = Transpiler::new();
        let rules = t.to_pirs("The big cat eats fish.");
        assert_eq!(rules.len(), 1);
        assert!(rules[0].args[0].contains("big"));
        assert!(rules[0].args[0].contains("cat"));
    }

    #[test]
    fn test_multiple_sentences() {
        let t = Transpiler::new();
        let rules = t.to_pirs("The cat eats. The dog runs.");
        assert_eq!(rules.len(), 2);
    }

    #[test]
    fn test_no_punctuation() {
        let t = Transpiler::new();
        let rules = t.to_pirs("The cat eats fish");
        assert_eq!(rules.len(), 1);
        assert_eq!(rules[0].head, "eat");
    }

    #[test]
    fn test_contraction_negation() {
        let t = Transpiler::new();
        // "don't" expands to "do not" via grammar tokenizer
        let rules = t.to_pirs("I don't like fish.");
        assert_eq!(rules.len(), 1);
        assert_eq!(rules[0].head, "not_like");
    }

    #[test]
    fn test_wh_question() {
        let t = Transpiler::new();
        let rules = t.to_pirs("What does the cat eat");
        assert_eq!(rules.len(), 1);
        assert!(rules[0].body.contains(&"type(question)".to_string()));
    }

    #[test]
    fn test_pirs_rule_display() {
        let rule = PirsRule {
            head: "eat".to_string(),
            args: vec!["cat".to_string(), "fish".to_string()],
            body: vec![],
        };
        assert_eq!(format!("{}", rule), "eat(cat, fish).");
    }

    #[test]
    fn test_pirs_rule_with_body() {
        let rule = PirsRule {
            head: "eat".to_string(),
            args: vec!["cat".to_string(), "fish".to_string()],
            body: vec!["in(house)".to_string()],
        };
        assert_eq!(rule.to_prolog(), "eat(cat, fish) :- in(house).");
    }

    #[test]
    fn test_imperative_detection() {
        let t = Transpiler::new();
        // Without punctuation, "Run to the store" should be detected as command
        let rules = t.to_pirs("Run to the store");
        assert_eq!(rules.len(), 1);
        assert!(rules[0].body.contains(&"type(imperative)".to_string()));
    }
}
