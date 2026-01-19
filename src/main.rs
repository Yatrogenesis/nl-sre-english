//! # NL-SRE-English CLI
//!
//! Command-line interface for the English semantic disambiguation engine.

use nl_sre_english::{SemanticDisambiguator, info};
use nl_sre_english::verbs::FunctionalCategory;

fn main() {
    println!("{}", info());
    println!();

    let disambiguator = SemanticDisambiguator::new();

    // Show verb database stats
    println!("=== Verb Database Statistics ===");
    let verb_db = disambiguator.verbs();
    println!("Total verbs: {}", verb_db.stats.total_verbs);
    println!("Irregular verbs: {}", verb_db.stats.irregular_verbs);
    println!("Regular verbs: {}", verb_db.stats.regular_verbs);
    println!("Total forms indexed: {}", verb_db.stats.total_forms);
    println!("Categories: {}", verb_db.stats.categories_used);
    println!("Groups: {}", verb_db.stats.groups_used);
    println!();

    // Show verbs by category
    println!("=== Verbs by Functional Category ===");
    for category in FunctionalCategory::all() {
        let verbs = disambiguator.verbs_by_category(*category);
        println!("{} ({} verbs): {}...",
            category.name(),
            verbs.len(),
            verbs.iter().take(5).cloned().collect::<Vec<_>>().join(", ")
        );
    }
    println!();

    // Process some example sentences
    println!("=== Processing Examples ===");
    let examples = [
        "The cat runs quickly across the room",
        "She walked to the store and bought some groceries",
        "I think we should go home now",
        "Please help me understand this concept",
        "The light shines brightly through the window",
    ];

    for sentence in &examples {
        println!("\nInput: {}", sentence);
        let result = disambiguator.process(sentence);
        println!("Confidence: {:.2}", result.confidence);

        if !result.detected_actions.is_empty() {
            println!("Actions detected:");
            for action in &result.detected_actions {
                println!("  - {} (base: {}, category: {}, group: {})",
                    action.verb,
                    action.base_form,
                    action.category.name(),
                    action.group.name()
                );
            }
        }

        if !result.corrections.is_empty() {
            println!("Corrections:");
            for corr in &result.corrections {
                println!("  - {} -> {}", corr.original, corr.corrected);
            }
        }
    }

    println!("\n=== Done ===");
}
