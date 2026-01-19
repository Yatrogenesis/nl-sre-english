//! # English Grammar Module
//!
//! Handles English grammar validation, POS tagging, and sentence structure.
//!
//! ## Optimizations (v0.1.1)
//!
//! - **Contraction expansion**: Handles 50+ common English contractions
//! - **Improved tokenization**: Better handling of punctuation and special cases

use std::collections::{HashSet, HashMap};

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
    /// Contraction expansions
    contractions: HashMap<String, Vec<String>>,
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
            contractions: Self::load_contractions(),
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

    fn load_contractions() -> HashMap<String, Vec<String>> {
        let mut map = HashMap::new();

        // Negative contractions
        map.insert("don't".to_string(), vec!["do".to_string(), "not".to_string()]);
        map.insert("doesn't".to_string(), vec!["does".to_string(), "not".to_string()]);
        map.insert("didn't".to_string(), vec!["did".to_string(), "not".to_string()]);
        map.insert("won't".to_string(), vec!["will".to_string(), "not".to_string()]);
        map.insert("wouldn't".to_string(), vec!["would".to_string(), "not".to_string()]);
        map.insert("can't".to_string(), vec!["can".to_string(), "not".to_string()]);
        map.insert("couldn't".to_string(), vec!["could".to_string(), "not".to_string()]);
        map.insert("shouldn't".to_string(), vec!["should".to_string(), "not".to_string()]);
        map.insert("wasn't".to_string(), vec!["was".to_string(), "not".to_string()]);
        map.insert("weren't".to_string(), vec!["were".to_string(), "not".to_string()]);
        map.insert("isn't".to_string(), vec!["is".to_string(), "not".to_string()]);
        map.insert("aren't".to_string(), vec!["are".to_string(), "not".to_string()]);
        map.insert("haven't".to_string(), vec!["have".to_string(), "not".to_string()]);
        map.insert("hasn't".to_string(), vec!["has".to_string(), "not".to_string()]);
        map.insert("hadn't".to_string(), vec!["had".to_string(), "not".to_string()]);
        map.insert("mustn't".to_string(), vec!["must".to_string(), "not".to_string()]);
        map.insert("mightn't".to_string(), vec!["might".to_string(), "not".to_string()]);
        map.insert("needn't".to_string(), vec!["need".to_string(), "not".to_string()]);
        map.insert("shan't".to_string(), vec!["shall".to_string(), "not".to_string()]);

        // Pronoun + be contractions
        map.insert("i'm".to_string(), vec!["i".to_string(), "am".to_string()]);
        map.insert("you're".to_string(), vec!["you".to_string(), "are".to_string()]);
        map.insert("he's".to_string(), vec!["he".to_string(), "is".to_string()]);
        map.insert("she's".to_string(), vec!["she".to_string(), "is".to_string()]);
        map.insert("it's".to_string(), vec!["it".to_string(), "is".to_string()]);
        map.insert("we're".to_string(), vec!["we".to_string(), "are".to_string()]);
        map.insert("they're".to_string(), vec!["they".to_string(), "are".to_string()]);
        map.insert("that's".to_string(), vec!["that".to_string(), "is".to_string()]);
        map.insert("what's".to_string(), vec!["what".to_string(), "is".to_string()]);
        map.insert("who's".to_string(), vec!["who".to_string(), "is".to_string()]);
        map.insert("where's".to_string(), vec!["where".to_string(), "is".to_string()]);
        map.insert("there's".to_string(), vec!["there".to_string(), "is".to_string()]);
        map.insert("here's".to_string(), vec!["here".to_string(), "is".to_string()]);

        // Pronoun + have contractions
        map.insert("i've".to_string(), vec!["i".to_string(), "have".to_string()]);
        map.insert("you've".to_string(), vec!["you".to_string(), "have".to_string()]);
        map.insert("we've".to_string(), vec!["we".to_string(), "have".to_string()]);
        map.insert("they've".to_string(), vec!["they".to_string(), "have".to_string()]);
        map.insert("could've".to_string(), vec!["could".to_string(), "have".to_string()]);
        map.insert("would've".to_string(), vec!["would".to_string(), "have".to_string()]);
        map.insert("should've".to_string(), vec!["should".to_string(), "have".to_string()]);
        map.insert("might've".to_string(), vec!["might".to_string(), "have".to_string()]);
        map.insert("must've".to_string(), vec!["must".to_string(), "have".to_string()]);

        // Pronoun + will contractions
        map.insert("i'll".to_string(), vec!["i".to_string(), "will".to_string()]);
        map.insert("you'll".to_string(), vec!["you".to_string(), "will".to_string()]);
        map.insert("he'll".to_string(), vec!["he".to_string(), "will".to_string()]);
        map.insert("she'll".to_string(), vec!["she".to_string(), "will".to_string()]);
        map.insert("it'll".to_string(), vec!["it".to_string(), "will".to_string()]);
        map.insert("we'll".to_string(), vec!["we".to_string(), "will".to_string()]);
        map.insert("they'll".to_string(), vec!["they".to_string(), "will".to_string()]);
        map.insert("that'll".to_string(), vec!["that".to_string(), "will".to_string()]);

        // Pronoun + would contractions
        map.insert("i'd".to_string(), vec!["i".to_string(), "would".to_string()]);
        map.insert("you'd".to_string(), vec!["you".to_string(), "would".to_string()]);
        map.insert("he'd".to_string(), vec!["he".to_string(), "would".to_string()]);
        map.insert("she'd".to_string(), vec!["she".to_string(), "would".to_string()]);
        map.insert("it'd".to_string(), vec!["it".to_string(), "would".to_string()]);
        map.insert("we'd".to_string(), vec!["we".to_string(), "would".to_string()]);
        map.insert("they'd".to_string(), vec!["they".to_string(), "would".to_string()]);

        // Pronoun + had contractions (same form as 'd for would)
        // Note: Context determines whether 'd = would or had

        // Other common contractions
        map.insert("let's".to_string(), vec!["let".to_string(), "us".to_string()]);
        map.insert("ain't".to_string(), vec!["am".to_string(), "not".to_string()]); // informal

        map
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

    /// Tokenize a sentence with contraction expansion
    ///
    /// Expands contractions like "don't" -> ["do", "not"], "I'm" -> ["i", "am"]
    pub fn tokenize(&self, sentence: &str) -> Vec<String> {
        let mut tokens = Vec::new();

        for word in sentence.split(|c: char| c.is_whitespace() || c == ',' || c == '.' || c == '!' || c == '?') {
            if word.is_empty() {
                continue;
            }

            let lower = word.to_lowercase();

            // Check if it's a known contraction
            if let Some(expansion) = self.contractions.get(&lower) {
                tokens.extend(expansion.iter().cloned());
            } else {
                tokens.push(lower);
            }
        }

        tokens
    }

    /// Tokenize without expanding contractions (for cases where you need raw tokens)
    pub fn tokenize_raw(&self, sentence: &str) -> Vec<String> {
        sentence
            .split(|c: char| c.is_whitespace() || c == ',' || c == '.' || c == '!' || c == '?')
            .filter(|s| !s.is_empty())
            .map(|s| s.to_lowercase())
            .collect()
    }

    /// Expand a single contraction, returns None if not a contraction
    pub fn expand_contraction(&self, word: &str) -> Option<Vec<String>> {
        self.contractions.get(&word.to_lowercase()).cloned()
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

    #[test]
    fn test_contractions() {
        let grammar = EnglishGrammar::new();

        // Test negative contractions
        let tokens = grammar.tokenize("I don't know");
        assert_eq!(tokens, vec!["i", "do", "not", "know"]);

        // Test pronoun + be contractions
        let tokens = grammar.tokenize("I'm walking");
        assert_eq!(tokens, vec!["i", "am", "walking"]);

        // Test pronoun + have contractions
        let tokens = grammar.tokenize("They've arrived");
        assert_eq!(tokens, vec!["they", "have", "arrived"]);

        // Test pronoun + will contractions
        let tokens = grammar.tokenize("We'll see");
        assert_eq!(tokens, vec!["we", "will", "see"]);

        // Test won't (irregular)
        let tokens = grammar.tokenize("I won't go");
        assert_eq!(tokens, vec!["i", "will", "not", "go"]);

        // Test let's
        let tokens = grammar.tokenize("Let's go");
        assert_eq!(tokens, vec!["let", "us", "go"]);
    }

    #[test]
    fn test_tokenize_raw() {
        let grammar = EnglishGrammar::new();
        let tokens = grammar.tokenize_raw("I don't know");
        assert_eq!(tokens, vec!["i", "don't", "know"]);
    }
}
