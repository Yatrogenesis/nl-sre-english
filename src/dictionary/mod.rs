//! # English Dictionary Module
//!
//! Dictionary management for English words with frequency data.
//!
//! ## Optimizations (v0.1.1)
//!
//! - **BK-Tree**: Fuzzy search reduced from O(N*M) to O(log N * M) average
//! - **Length filtering**: Pre-filter candidates by word length
//! - **Bounded Levenshtein**: Early termination when distance exceeds threshold

mod bktree;

use std::collections::{HashMap, HashSet};
use bktree::BKTree;

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
    bk_tree: BKTree,
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
            bk_tree: BKTree::new(),
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
                self.bk_tree.insert(word.clone());
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
    ///
    /// Uses BK-Tree for O(log N * M) average complexity instead of O(N * M)
    pub fn find_similar(&self, word: &str, max_distance: usize) -> Vec<(String, usize)> {
        let word_lower = word.to_lowercase();
        self.bk_tree.find_within(&word_lower, max_distance)
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
        use super::bktree::levenshtein;
        assert_eq!(levenshtein("kitten", "sitting"), 3);
        assert_eq!(levenshtein("hello", "hello"), 0);
        assert_eq!(levenshtein("", "abc"), 3);
    }
}
