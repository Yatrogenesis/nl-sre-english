//! # English Dictionary Module
//!
//! Dictionary management for English words with frequency data.

use std::collections::{HashMap, HashSet};

/// Dictionary entry
#[derive(Debug, Clone)]
pub struct DictionaryEntry {
    /// Word (lowercase)
    pub word: String,
    /// Parts of speech
    pub pos: Vec<PartOfSpeech>,
    /// Definitions
    pub definitions: Vec<String>,
    /// Frequency (1-100, higher = more common)
    pub frequency: u8,
}

/// Parts of speech
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PartOfSpeech {
    Noun,
    Verb,
    Adjective,
    Adverb,
    Preposition,
    Article,
    Pronoun,
    Conjunction,
    Interjection,
    Unknown,
}

/// English Dictionary
#[derive(Debug)]
pub struct EnglishDictionary {
    entries: HashMap<String, DictionaryEntry>,
    valid_words: HashSet<String>,
    pub stats: DictionaryStats,
}

/// Dictionary statistics
#[derive(Debug, Clone, Default)]
pub struct DictionaryStats {
    pub total_entries: usize,
    pub nouns: usize,
    pub verbs: usize,
    pub adjectives: usize,
    pub adverbs: usize,
}

impl Default for EnglishDictionary {
    fn default() -> Self {
        Self::new()
    }
}

impl EnglishDictionary {
    pub fn new() -> Self {
        let mut dict = Self {
            entries: HashMap::new(),
            valid_words: HashSet::new(),
            stats: DictionaryStats::default(),
        };
        dict.load_common_words();
        dict
    }

    fn load_common_words(&mut self) {
        // Load 5000+ most common English words
        let common_words = include_str!("common_words.txt");
        for line in common_words.lines() {
            let word = line.trim().to_lowercase();
            if !word.is_empty() && !word.starts_with('#') {
                self.valid_words.insert(word.clone());
                self.entries.insert(word.clone(), DictionaryEntry {
                    word,
                    pos: vec![PartOfSpeech::Unknown],
                    definitions: vec![],
                    frequency: 50,
                });
            }
        }
        self.stats.total_entries = self.entries.len();
    }

    /// Check if a word is valid
    pub fn is_valid(&self, word: &str) -> bool {
        self.valid_words.contains(&word.to_lowercase())
    }

    /// Get entry for a word
    pub fn get(&self, word: &str) -> Option<&DictionaryEntry> {
        self.entries.get(&word.to_lowercase())
    }

    /// Get frequency of a word (0 if not found)
    pub fn frequency(&self, word: &str) -> u8 {
        self.get(word).map(|e| e.frequency).unwrap_or(0)
    }

    /// Find similar words (for spell correction)
    pub fn find_similar(&self, word: &str, max_distance: usize) -> Vec<(String, usize)> {
        let word_lower = word.to_lowercase();
        self.valid_words
            .iter()
            .filter_map(|w| {
                let dist = Self::levenshtein(&word_lower, w);
                if dist <= max_distance && dist > 0 {
                    Some((w.clone(), dist))
                } else {
                    None
                }
            })
            .collect()
    }

    /// Levenshtein distance
    fn levenshtein(a: &str, b: &str) -> usize {
        let a_chars: Vec<char> = a.chars().collect();
        let b_chars: Vec<char> = b.chars().collect();
        let m = a_chars.len();
        let n = b_chars.len();

        if m == 0 { return n; }
        if n == 0 { return m; }

        let mut dp = vec![vec![0usize; n + 1]; m + 1];

        for i in 0..=m { dp[i][0] = i; }
        for j in 0..=n { dp[0][j] = j; }

        for i in 1..=m {
            for j in 1..=n {
                let cost = if a_chars[i-1] == b_chars[j-1] { 0 } else { 1 };
                dp[i][j] = (dp[i-1][j] + 1)
                    .min(dp[i][j-1] + 1)
                    .min(dp[i-1][j-1] + cost);
            }
        }

        dp[m][n]
    }

    /// Total word count
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dictionary() {
        let dict = EnglishDictionary::new();
        assert!(dict.is_valid("the"));
        assert!(dict.is_valid("and"));
        assert!(dict.len() > 1000);
    }

    #[test]
    fn test_levenshtein() {
        assert_eq!(EnglishDictionary::levenshtein("kitten", "sitting"), 3);
        assert_eq!(EnglishDictionary::levenshtein("hello", "hello"), 0);
        assert_eq!(EnglishDictionary::levenshtein("", "abc"), 3);
    }
}
