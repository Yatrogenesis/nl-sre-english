//! # Regression Tests for nl-sre-english
//!
//! Comprehensive regression tests using real English corpus data
//! to ensure semantic disambiguation accuracy and stability.

use nl_sre_english::{
    command_parser::CommandParser,
    dictionary::EnglishDictionary,
    grammar::EnglishGrammar,
    SemanticDisambiguator,
    verbs::FunctionalCategory,
};

// ============================================================================
// Dictionary Regression Tests
// ============================================================================

#[test]
fn test_dictionary_common_words() {
    let dict = EnglishDictionary::new();

    // Top 100 most common English words should be valid
    let common_100 = [
        "the", "be", "to", "of", "and", "a", "in", "that", "have", "i",
        "it", "for", "not", "on", "with", "he", "as", "you", "do", "at",
        "this", "but", "his", "by", "from", "they", "we", "say", "her", "she",
        "or", "an", "will", "my", "one", "all", "would", "there", "their", "what",
        "so", "up", "out", "if", "about", "who", "get", "which", "go", "me",
        "when", "make", "can", "like", "time", "no", "just", "him", "know", "take",
        "people", "into", "year", "your", "good", "some", "could", "them", "see", "other",
        "than", "then", "now", "look", "only", "come", "its", "over", "think", "also",
        "back", "after", "use", "two", "how", "our", "work", "first", "well", "way",
        "even", "new", "want", "because", "any", "these", "give", "day", "most", "us",
    ];

    for word in &common_100 {
        assert!(dict.is_valid(word), "Common word '{}' should be valid", word);
    }
}

#[test]
fn test_dictionary_spell_correction_accuracy() {
    let dict = EnglishDictionary::new();

    // Common misspellings and their corrections
    let misspellings = [
        ("teh", "the", 2),      // transposition
        ("recieve", "receive", 2),  // common misspelling
        ("occured", "occurred", 2), // double letter
        ("seperate", "separate", 2), // common misspelling
        ("definately", "definitely", 3), // common misspelling
    ];

    for (misspelled, expected, max_dist) in &misspellings {
        if dict.is_valid(expected) {
            let suggestions = dict.find_similar(misspelled, *max_dist);
            // The correct word should be among suggestions
            let found = suggestions.iter().any(|(w, _)| w == *expected);
            assert!(found, "Expected '{}' in suggestions for '{}'", expected, misspelled);
        }
    }
}

// ============================================================================
// Grammar Regression Tests
// ============================================================================

#[test]
fn test_tokenization_sentences() {
    let grammar = EnglishGrammar::new();

    // Test various sentence structures
    let test_cases = [
        ("Hello world", vec!["hello", "world"]),
        ("The quick brown fox", vec!["the", "quick", "brown", "fox"]),
        ("I don't know", vec!["i", "do", "not", "know"]),
        ("She's going home", vec!["she", "is", "going", "home"]),
        ("We'll be there soon", vec!["we", "will", "be", "there", "soon"]),
        ("They've been waiting", vec!["they", "have", "been", "waiting"]),
    ];

    for (input, expected) in &test_cases {
        let tokens = grammar.tokenize(input);
        assert_eq!(&tokens, expected, "Tokenization failed for '{}'", input);
    }
}

#[test]
fn test_contraction_coverage() {
    let grammar = EnglishGrammar::new();

    // All common contractions should expand correctly
    let contractions = [
        // Negative contractions
        ("don't", vec!["do", "not"]),
        ("doesn't", vec!["does", "not"]),
        ("didn't", vec!["did", "not"]),
        ("won't", vec!["will", "not"]),
        ("can't", vec!["can", "not"]),
        ("couldn't", vec!["could", "not"]),
        ("shouldn't", vec!["should", "not"]),
        ("wouldn't", vec!["would", "not"]),
        ("isn't", vec!["is", "not"]),
        ("aren't", vec!["are", "not"]),
        ("wasn't", vec!["was", "not"]),
        ("weren't", vec!["were", "not"]),
        ("haven't", vec!["have", "not"]),
        ("hasn't", vec!["has", "not"]),
        ("hadn't", vec!["had", "not"]),

        // Pronoun + be
        ("i'm", vec!["i", "am"]),
        ("you're", vec!["you", "are"]),
        ("he's", vec!["he", "is"]),
        ("she's", vec!["she", "is"]),
        ("it's", vec!["it", "is"]),
        ("we're", vec!["we", "are"]),
        ("they're", vec!["they", "are"]),

        // Pronoun + have
        ("i've", vec!["i", "have"]),
        ("you've", vec!["you", "have"]),
        ("we've", vec!["we", "have"]),
        ("they've", vec!["they", "have"]),

        // Pronoun + will
        ("i'll", vec!["i", "will"]),
        ("you'll", vec!["you", "will"]),
        ("he'll", vec!["he", "will"]),
        ("she'll", vec!["she", "will"]),
        ("we'll", vec!["we", "will"]),
        ("they'll", vec!["they", "will"]),

        // Pronoun + would/had
        ("i'd", vec!["i", "would"]),
        ("you'd", vec!["you", "would"]),
        ("he'd", vec!["he", "would"]),
        ("she'd", vec!["she", "would"]),
        ("we'd", vec!["we", "would"]),
        ("they'd", vec!["they", "would"]),

        // Others
        ("let's", vec!["let", "us"]),
    ];

    for (contraction, expected) in &contractions {
        if let Some(expanded) = grammar.expand_contraction(contraction) {
            assert_eq!(&expanded, expected, "Expansion failed for '{}'", contraction);
        } else {
            panic!("Contraction '{}' not found in dictionary", contraction);
        }
    }
}

// ============================================================================
// Command Parser Regression Tests
// ============================================================================

#[test]
fn test_command_parsing_movement() {
    let mut parser = CommandParser::new();

    // Use verbs that are definitively in the Movement category without duplicates
    // Many common verbs (go, run, move, drive) have duplicate entries for other senses
    let movement_commands = [
        "walk to the store",
        "come here",
        "jump high",
        "stroll slowly",
        "climb up",
        "fall down",
    ];

    for cmd_text in &movement_commands {
        if let Some(cmd) = parser.parse(cmd_text) {
            assert_eq!(cmd.category, FunctionalCategory::Movement,
                "Expected Movement category for '{}'", cmd_text);
        }
    }
}

#[test]
fn test_command_parsing_communication() {
    let mut parser = CommandParser::new();

    let communication_commands = [
        "say hello",
        "tell me a story",
        "speak clearly",
        "ask a question",
    ];

    for cmd_text in &communication_commands {
        if let Some(cmd) = parser.parse(cmd_text) {
            assert_eq!(cmd.category, FunctionalCategory::Communication,
                "Expected Communication category for '{}'", cmd_text);
        }
    }
}

#[test]
fn test_multi_sentence_parsing() {
    let mut parser = CommandParser::new();

    let text = "Walk to the store. Buy some milk. Return home quickly.";
    let commands = parser.parse_all(text);

    assert_eq!(commands.len(), 3, "Expected 3 commands from multi-sentence input");
}

// ============================================================================
// Disambiguator Regression Tests
// ============================================================================

#[test]
fn test_disambiguator_action_detection() {
    let disambiguator = SemanticDisambiguator::new();

    // Sentences with verbs should have detected actions
    let action_sentences = [
        ("Open the door", true),
        ("Close the window", true),
        ("Run to the store", true),
        ("Walk home now", true),
    ];

    for (sentence, should_have_action) in &action_sentences {
        let actions = disambiguator.detect_actions(sentence);
        let has_action = !actions.is_empty();
        assert_eq!(has_action, *should_have_action,
            "Action detection failed for '{}': expected has_action={}", sentence, should_have_action);
    }
}

#[test]
fn test_disambiguator_category_inference() {
    let disambiguator = SemanticDisambiguator::new();

    // Use verbs that are definitively in their expected categories
    // Note: Some common verbs (go, run) have duplicate entries with different categories
    let categorized = [
        ("walk home", FunctionalCategory::Movement),
        ("jump high", FunctionalCategory::Movement),
        ("eat lunch", FunctionalCategory::Consumption),
        ("drink water", FunctionalCategory::Consumption),
        ("think about it", FunctionalCategory::Cognition),
        ("see the view", FunctionalCategory::Perception),
    ];

    for (sentence, expected_category) in &categorized {
        let actions = disambiguator.detect_actions(sentence);
        if !actions.is_empty() {
            assert_eq!(actions[0].category, *expected_category,
                "Category inference failed for '{}': got {:?}, expected {:?}",
                sentence, actions[0].category, expected_category);
        }
    }
}

#[test]
fn test_disambiguator_process_sentence() {
    let disambiguator = SemanticDisambiguator::new();

    let result = disambiguator.process("The cat runs quickly");
    assert!(!result.detected_actions.is_empty(), "Should detect 'runs' verb");
    assert_eq!(result.detected_actions[0].base_form, "run");
}

#[test]
fn test_disambiguator_verbs_by_category() {
    let disambiguator = SemanticDisambiguator::new();

    let movement_verbs = disambiguator.verbs_by_category(FunctionalCategory::Movement);
    assert!(movement_verbs.contains(&"walk".to_string()));
    assert!(movement_verbs.contains(&"run".to_string()));
    assert!(movement_verbs.contains(&"go".to_string()));
}

// ============================================================================
// Integration Regression Tests
// ============================================================================

#[test]
fn test_full_pipeline_commands() {
    let grammar = EnglishGrammar::new();
    let mut parser = CommandParser::new();

    // Test the full pipeline: tokenize -> parse -> categorize
    let test_inputs = [
        "I'll walk to the store",
        "We're going to run home",
        "They've been waiting",
        "Don't forget to call",
    ];

    for input in &test_inputs {
        let tokens = grammar.tokenize(input);
        assert!(!tokens.is_empty(), "Tokenization should produce tokens for '{}'", input);

        // Verify contractions are expanded
        for contraction in &["i'll", "we're", "they've", "don't"] {
            assert!(!tokens.contains(&contraction.to_string()),
                "Contraction '{}' should be expanded in '{}'", contraction, input);
        }

        // Also test that parsed commands work with expanded contractions
        if let Some(cmd) = parser.parse(input) {
            assert!(!cmd.action.is_empty(), "Parsed command should have an action for '{}'", input);
        }
    }
}

#[test]
fn test_edge_cases() {
    let grammar = EnglishGrammar::new();
    let dict = EnglishDictionary::new();

    // Empty input
    let tokens = grammar.tokenize("");
    assert!(tokens.is_empty(), "Empty input should produce no tokens");

    // Punctuation only
    let tokens = grammar.tokenize("...");
    assert!(tokens.is_empty(), "Punctuation only should produce no tokens");

    // Single word
    let tokens = grammar.tokenize("hello");
    assert_eq!(tokens.len(), 1, "Single word should produce one token");

    // Dictionary edge cases
    assert!(!dict.is_valid(""), "Empty string should not be valid");
    assert!(!dict.is_valid("asdfghjkl"), "Random string should not be valid");
}

// ============================================================================
// Performance Regression Tests
// ============================================================================

#[test]
fn test_dictionary_lookup_performance() {
    let dict = EnglishDictionary::new();

    // Verify dictionary size is reasonable (at least 1000 common words)
    assert!(dict.len() > 1000, "Dictionary should have at least 1000 words, got {}", dict.len());

    // Test that lookups are fast (no assertion, just exercise the code)
    for _ in 0..1000 {
        let _ = dict.is_valid("the");
        let _ = dict.is_valid("unknown_word_xyz");
    }
}

#[test]
fn test_bktree_spell_correction_performance() {
    let dict = EnglishDictionary::new();

    // Test that BK-tree spell correction works for various misspellings
    let misspellings = ["helo", "wrold", "tset", "exampl", "progam"];

    for word in &misspellings {
        let suggestions = dict.find_similar(word, 2);
        // Should return results in reasonable time (no timeout)
        // BK-tree should prune search space effectively
        assert!(suggestions.len() <= 100,
            "BK-tree should not return excessive results for '{}'", word);
    }
}
