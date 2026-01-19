//! # English Grammar Module
//!
//! Handles English grammar validation, POS tagging, and sentence structure.

use std::collections::HashSet;

/// English grammar analyzer
#[derive(Debug)]
pub struct EnglishGrammar {
    /// Common irregular verbs (for validation)
    #[allow(dead_code)]
    irregular_verbs: HashSet<String>,
    /// Articles
    articles: HashSet<String>,
    /// Prepositions
    prepositions: HashSet<String>,
    /// Pronouns
    pronouns: HashSet<String>,
    /// Conjunctions
    conjunctions: HashSet<String>,
    /// Auxiliary verbs
    auxiliaries: HashSet<String>,
}

impl Default for EnglishGrammar {
    fn default() -> Self {
        Self::new()
    }
}

impl EnglishGrammar {
    pub fn new() -> Self {
        Self {
            irregular_verbs: Self::load_irregular_verbs(),
            articles: Self::load_articles(),
            prepositions: Self::load_prepositions(),
            pronouns: Self::load_pronouns(),
            conjunctions: Self::load_conjunctions(),
            auxiliaries: Self::load_auxiliaries(),
        }
    }

    fn load_irregular_verbs() -> HashSet<String> {
        ["be", "have", "do", "say", "go", "get", "make", "know", "think", "take",
         "see", "come", "want", "look", "use", "find", "give", "tell", "work",
         "may", "can", "will", "shall", "must", "should", "could", "would", "might"]
            .iter().map(|s| s.to_string()).collect()
    }

    fn load_articles() -> HashSet<String> {
        ["a", "an", "the"].iter().map(|s| s.to_string()).collect()
    }

    fn load_prepositions() -> HashSet<String> {
        ["in", "on", "at", "to", "for", "with", "by", "from", "up", "about",
         "into", "over", "after", "beneath", "under", "above", "below",
         "between", "through", "during", "before", "behind", "beyond",
         "without", "within", "along", "among", "around", "against", "across"]
            .iter().map(|s| s.to_string()).collect()
    }

    fn load_pronouns() -> HashSet<String> {
        ["i", "me", "my", "mine", "myself",
         "you", "your", "yours", "yourself", "yourselves",
         "he", "him", "his", "himself",
         "she", "her", "hers", "herself",
         "it", "its", "itself",
         "we", "us", "our", "ours", "ourselves",
         "they", "them", "their", "theirs", "themselves",
         "who", "whom", "whose", "which", "what", "that",
         "this", "these", "those", "all", "each", "every",
         "both", "few", "more", "most", "other", "some", "any",
         "no", "none", "one", "everyone", "someone", "anyone",
         "nobody", "everybody", "somebody", "anybody", "nothing",
         "everything", "something", "anything"]
            .iter().map(|s| s.to_string()).collect()
    }

    fn load_conjunctions() -> HashSet<String> {
        ["and", "but", "or", "nor", "for", "yet", "so",
         "although", "because", "since", "unless", "while",
         "if", "when", "where", "whether", "whereas", "however",
         "therefore", "moreover", "furthermore", "nevertheless",
         "consequently", "meanwhile", "otherwise", "thus", "hence"]
            .iter().map(|s| s.to_string()).collect()
    }

    fn load_auxiliaries() -> HashSet<String> {
        ["be", "am", "is", "are", "was", "were", "been", "being",
         "have", "has", "had", "having",
         "do", "does", "did", "doing",
         "will", "would", "shall", "should",
         "may", "might", "must", "can", "could",
         "need", "dare", "ought", "used"]
            .iter().map(|s| s.to_string()).collect()
    }

    /// Check if word is an article
    pub fn is_article(&self, word: &str) -> bool {
        self.articles.contains(&word.to_lowercase())
    }

    /// Check if word is a preposition
    pub fn is_preposition(&self, word: &str) -> bool {
        self.prepositions.contains(&word.to_lowercase())
    }

    /// Check if word is a pronoun
    pub fn is_pronoun(&self, word: &str) -> bool {
        self.pronouns.contains(&word.to_lowercase())
    }

    /// Check if word is a conjunction
    pub fn is_conjunction(&self, word: &str) -> bool {
        self.conjunctions.contains(&word.to_lowercase())
    }

    /// Check if word is an auxiliary verb
    pub fn is_auxiliary(&self, word: &str) -> bool {
        self.auxiliaries.contains(&word.to_lowercase())
    }

    /// Tokenize a sentence
    pub fn tokenize(&self, sentence: &str) -> Vec<String> {
        sentence
            .split(|c: char| c.is_whitespace() || c == ',' || c == '.' || c == '!' || c == '?')
            .filter(|s| !s.is_empty())
            .map(|s| s.to_lowercase())
            .collect()
    }

    /// Basic POS tagging
    pub fn tag_pos(&self, tokens: &[String]) -> Vec<(String, POS)> {
        tokens.iter().map(|token| {
            let pos = if self.is_article(token) {
                POS::Article
            } else if self.is_preposition(token) {
                POS::Preposition
            } else if self.is_pronoun(token) {
                POS::Pronoun
            } else if self.is_conjunction(token) {
                POS::Conjunction
            } else if self.is_auxiliary(token) {
                POS::Auxiliary
            } else {
                POS::Unknown
            };
            (token.clone(), pos)
        }).collect()
    }
}

/// Part of Speech tags
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum POS {
    Noun,
    Verb,
    Adjective,
    Adverb,
    Article,
    Preposition,
    Pronoun,
    Conjunction,
    Auxiliary,
    Interjection,
    Number,
    Unknown,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grammar_basic() {
        let grammar = EnglishGrammar::new();
        assert!(grammar.is_article("the"));
        assert!(grammar.is_preposition("in"));
        assert!(grammar.is_pronoun("they"));
        assert!(grammar.is_conjunction("and"));
        assert!(grammar.is_auxiliary("will"));
    }

    #[test]
    fn test_tokenize() {
        let grammar = EnglishGrammar::new();
        let tokens = grammar.tokenize("The quick brown fox jumps.");
        assert_eq!(tokens.len(), 5);
        assert_eq!(tokens[0], "the");
    }
}
