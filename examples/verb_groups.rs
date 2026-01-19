//! # Verb Groups Example
//!
//! Demonstrates the functional verb grouping system.

use nl_sre_english::verbs::{VerbDatabase, FunctionalCategory, VerbGroup};

fn main() {
    println!("=== NL-SRE-English: Functional Verb Groups ===\n");

    let db = VerbDatabase::with_builtin();

    println!("Database Statistics:");
    println!("  Total verbs: {}", db.stats.total_verbs);
    println!("  Irregular: {}", db.stats.irregular_verbs);
    println!("  Regular: {}", db.stats.regular_verbs);
    println!("  Forms indexed: {}", db.stats.total_forms);
    println!();

    // Show all categories
    println!("=== Functional Categories ===\n");
    for category in FunctionalCategory::all() {
        let verbs = db.by_category(*category);
        println!("{}:", category.name());
        println!("  Description: {}", category.description());
        println!("  Count: {} verbs", verbs.len());

        // Show first 10 verbs
        let samples: Vec<_> = verbs.iter().take(10).collect();
        print!("  Examples: ");
        for (i, v) in samples.iter().enumerate() {
            if i > 0 { print!(", "); }
            print!("{}", v.base);
        }
        println!("\n");
    }

    // Demonstrate verb lookup
    println!("=== Verb Lookup Examples ===\n");
    let test_words = ["running", "went", "thought", "eaten", "swam", "written"];

    for word in &test_words {
        if let Some(entry) = db.lookup(word) {
            println!("'{}' -> base: '{}', category: {}, group: {}, irregular: {}",
                word,
                entry.base,
                entry.category.name(),
                entry.group.name(),
                entry.irregular
            );
            println!("  Forms: {} / {} / {} / {}",
                entry.base, entry.past, entry.past_participle, entry.present_participle);
            if !entry.synonyms.is_empty() {
                println!("  Synonyms: {}", entry.synonyms.join(", "));
            }
        } else {
            println!("'{}' -> not found", word);
        }
        println!();
    }

    // Show specific groups
    println!("=== Movement Verb Groups ===\n");
    let movement_groups = [
        VerbGroup::Walk, VerbGroup::Run, VerbGroup::Jump,
        VerbGroup::Fly, VerbGroup::Swim, VerbGroup::Climb
    ];

    for group in &movement_groups {
        let verbs = db.by_group(*group);
        print!("{}: ", group.name());
        for (i, v) in verbs.iter().enumerate() {
            if i > 0 { print!(", "); }
            print!("{}", v.base);
        }
        println!();
    }

    println!("\n=== Communication Verb Groups ===\n");
    let comm_groups = [
        VerbGroup::Speak, VerbGroup::Ask, VerbGroup::Answer,
        VerbGroup::Explain, VerbGroup::Argue, VerbGroup::Warn
    ];

    for group in &comm_groups {
        let verbs = db.by_group(*group);
        print!("{}: ", group.name());
        for (i, v) in verbs.iter().enumerate() {
            if i > 0 { print!(", "); }
            print!("{}", v.base);
        }
        println!();
    }

    println!("\n=== Emotion Verb Groups ===\n");
    let emotion_groups = [
        VerbGroup::Love, VerbGroup::Hate, VerbGroup::Fear,
        VerbGroup::Hope, VerbGroup::Enjoy, VerbGroup::Suffer
    ];

    for group in &emotion_groups {
        let verbs = db.by_group(*group);
        print!("{}: ", group.name());
        for (i, v) in verbs.iter().enumerate() {
            if i > 0 { print!(", "); }
            print!("{}", v.base);
        }
        println!();
    }

    println!("\n=== Done ===");
}
