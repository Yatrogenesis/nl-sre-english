//! # BK-Tree Implementation
//!
//! Burkhard-Keller Tree for efficient fuzzy string matching.
//! Reduces spell-check from O(N*M) to O(log N * M) average case.
//!
//! Reference: Burkhard, W. A., & Keller, R. M. (1973).
//! "Some approaches to best-match file searching"

use std::collections::HashMap;

/// BK-Tree node
#[derive(Debug)]
struct BKNode {
    /// The word stored at this node
    word: String,
    /// Children indexed by distance
    children: HashMap<usize, BKNode>,
}

impl BKNode {
    fn new(word: String) -> Self {
        Self {
            word,
            children: HashMap::new(),
        }
    }
}

/// BK-Tree for efficient fuzzy string matching
///
/// A BK-Tree exploits the triangle inequality of the Levenshtein distance:
/// d(x,z) >= |d(x,y) - d(y,z)|
///
/// This allows pruning large portions of the search space.
#[derive(Debug)]
pub struct BKTree {
    root: Option<BKNode>,
    size: usize,
}

impl Default for BKTree {
    fn default() -> Self {
        Self::new()
    }
}

impl BKTree {
    /// Create a new empty BK-Tree
    pub fn new() -> Self {
        Self {
            root: None,
            size: 0,
        }
    }

    /// Insert a word into the tree
    pub fn insert(&mut self, word: String) {
        if word.is_empty() {
            return;
        }

        match &mut self.root {
            None => {
                self.root = Some(BKNode::new(word));
                self.size = 1;
            }
            Some(root) => {
                Self::insert_at(root, word);
                self.size += 1;
            }
        }
    }

    fn insert_at(node: &mut BKNode, word: String) {
        let dist = levenshtein(&node.word, &word);

        if dist == 0 {
            return; // Duplicate word
        }

        match node.children.get_mut(&dist) {
            Some(child) => Self::insert_at(child, word),
            None => {
                node.children.insert(dist, BKNode::new(word));
            }
        }
    }

    /// Find all words within max_distance of the query
    ///
    /// Returns Vec<(word, distance)> sorted by distance
    pub fn find_within(&self, query: &str, max_distance: usize) -> Vec<(String, usize)> {
        let mut results = Vec::new();

        if let Some(root) = &self.root {
            self.search_at(root, query, max_distance, &mut results);
        }

        // Sort by distance, then alphabetically
        results.sort_by(|a, b| {
            a.1.cmp(&b.1).then_with(|| a.0.cmp(&b.0))
        });

        results
    }

    #[allow(clippy::only_used_in_recursion)]
    fn search_at(
        &self,
        node: &BKNode,
        query: &str,
        max_distance: usize,
        results: &mut Vec<(String, usize)>,
    ) {
        let dist = levenshtein(&node.word, query);

        // If this node is within range, add it
        if dist <= max_distance && dist > 0 {
            results.push((node.word.clone(), dist));
        }

        // Triangle inequality pruning:
        // Only check children where distance could possibly be <= max_distance
        // Child at distance d from node can have distance to query in range [|dist-d|, dist+d]
        let min_child_dist = dist.saturating_sub(max_distance);
        let max_child_dist = dist + max_distance;

        for (&child_dist, child) in &node.children {
            if child_dist >= min_child_dist && child_dist <= max_child_dist {
                self.search_at(child, query, max_distance, results);
            }
        }
    }

    /// Number of words in the tree
    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.size
    }

    /// Check if the tree is empty
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.size == 0
    }
}

/// Levenshtein distance calculation
///
/// Optimized with early termination when distance exceeds threshold.
pub fn levenshtein(a: &str, b: &str) -> usize {
    let a_chars: Vec<char> = a.chars().collect();
    let b_chars: Vec<char> = b.chars().collect();
    let m = a_chars.len();
    let n = b_chars.len();

    // Early exit for empty strings
    if m == 0 { return n; }
    if n == 0 { return m; }

    // Use single-row optimization (O(min(m,n)) space instead of O(m*n))
    let (shorter, longer, short_chars, long_chars) = if m <= n {
        (m, n, &a_chars, &b_chars)
    } else {
        (n, m, &b_chars, &a_chars)
    };

    let mut prev_row: Vec<usize> = (0..=shorter).collect();
    let mut curr_row: Vec<usize> = vec![0; shorter + 1];

    for j in 1..=longer {
        curr_row[0] = j;

        for i in 1..=shorter {
            let cost = if short_chars[i - 1] == long_chars[j - 1] { 0 } else { 1 };
            curr_row[i] = (prev_row[i] + 1)
                .min(curr_row[i - 1] + 1)
                .min(prev_row[i - 1] + cost);
        }

        std::mem::swap(&mut prev_row, &mut curr_row);
    }

    prev_row[shorter]
}

/// Levenshtein with early termination threshold
///
/// Returns None if distance exceeds threshold (faster for pruning)
#[allow(dead_code)]
pub fn levenshtein_bounded(a: &str, b: &str, threshold: usize) -> Option<usize> {
    let a_chars: Vec<char> = a.chars().collect();
    let b_chars: Vec<char> = b.chars().collect();
    let m = a_chars.len();
    let n = b_chars.len();

    // Early exit: length difference exceeds threshold
    let len_diff = m.abs_diff(n);
    if len_diff > threshold {
        return None;
    }

    if m == 0 { return if n <= threshold { Some(n) } else { None }; }
    if n == 0 { return if m <= threshold { Some(m) } else { None }; }

    let mut prev_row: Vec<usize> = (0..=n).collect();
    let mut curr_row: Vec<usize> = vec![0; n + 1];

    for i in 1..=m {
        curr_row[0] = i;
        let mut row_min = i;

        for j in 1..=n {
            let cost = if a_chars[i - 1] == b_chars[j - 1] { 0 } else { 1 };
            curr_row[j] = (prev_row[j] + 1)
                .min(curr_row[j - 1] + 1)
                .min(prev_row[j - 1] + cost);
            row_min = row_min.min(curr_row[j]);
        }

        // Early termination: if minimum in row exceeds threshold, no need to continue
        if row_min > threshold {
            return None;
        }

        std::mem::swap(&mut prev_row, &mut curr_row);
    }

    let result = prev_row[n];
    if result <= threshold {
        Some(result)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_levenshtein() {
        assert_eq!(levenshtein("kitten", "sitting"), 3);
        assert_eq!(levenshtein("hello", "hello"), 0);
        assert_eq!(levenshtein("", "abc"), 3);
        assert_eq!(levenshtein("abc", ""), 3);
        assert_eq!(levenshtein("book", "back"), 2);
    }

    #[test]
    fn test_levenshtein_bounded() {
        assert_eq!(levenshtein_bounded("kitten", "sitting", 3), Some(3));
        assert_eq!(levenshtein_bounded("kitten", "sitting", 2), None);
        assert_eq!(levenshtein_bounded("hello", "hello", 0), Some(0));
    }

    #[test]
    fn test_bktree_basic() {
        let mut tree = BKTree::new();
        tree.insert("hello".to_string());
        tree.insert("help".to_string());
        tree.insert("hell".to_string());
        tree.insert("world".to_string());

        assert_eq!(tree.len(), 4);

        // Find words within distance 1 of "hello"
        let results = tree.find_within("hello", 1);
        assert!(results.iter().any(|(w, d)| w == "hell" && *d == 1));
    }

    #[test]
    fn test_bktree_spell_correction() {
        let mut tree = BKTree::new();
        let words = vec![
            "the", "there", "their", "they", "them", "then",
            "this", "that", "these", "those",
            "help", "hello", "hell", "heal",
        ];

        for word in words {
            tree.insert(word.to_string());
        }

        // "teh" should find "the" at distance 2 (transposition = 2 edits in Levenshtein)
        let results = tree.find_within("teh", 2);
        assert!(results.iter().any(|(w, _)| w == "the"));

        // "helo" should find "hello" and "help" within distance 2
        let results = tree.find_within("helo", 2);
        assert!(results.iter().any(|(w, _)| w == "hello"));
        assert!(results.iter().any(|(w, _)| w == "help"));
    }

    #[test]
    fn test_bktree_no_results() {
        let mut tree = BKTree::new();
        tree.insert("apple".to_string());
        tree.insert("banana".to_string());

        // "xyz" should have no matches within distance 1
        let results = tree.find_within("xyz", 1);
        assert!(results.is_empty());
    }
}
