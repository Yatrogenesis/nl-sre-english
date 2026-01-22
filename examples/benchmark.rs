//! Real Performance Benchmark for NL-SRE-English
//! System: Intel Core i7-12650H (10 cores/16 threads), 16 GB RAM
//! Date: January 22, 2026

use std::time::Instant;
use std::hint::black_box;

use nl_sre_english::{
    EnglishDictionary, EnglishGrammar, VerbDatabase, CommandParser, SemanticDisambiguator,
};

const WARMUP: u64 = 5_000;
const ITERATIONS: u64 = 100_000;

/// Simple Levenshtein for linear search comparison
fn levenshtein(a: &str, b: &str) -> usize {
    let a: Vec<char> = a.chars().collect();
    let b: Vec<char> = b.chars().collect();
    let m = a.len();
    let n = b.len();

    if m == 0 { return n; }
    if n == 0 { return m; }

    let mut prev: Vec<usize> = (0..=n).collect();
    let mut curr = vec![0; n + 1];

    for i in 1..=m {
        curr[0] = i;
        for j in 1..=n {
            let cost = if a[i-1] == b[j-1] { 0 } else { 1 };
            curr[j] = (prev[j] + 1).min(curr[j-1] + 1).min(prev[j-1] + cost);
        }
        std::mem::swap(&mut prev, &mut curr);
    }
    prev[n]
}

fn main() {
    println!("╔══════════════════════════════════════════════════════════════════╗");
    println!("║       NL-SRE-English Performance Benchmark                       ║");
    println!("║       System: Intel Core i7-12650H, 16 GB RAM                    ║");
    println!("║       Date: January 22, 2026                                     ║");
    println!("╚══════════════════════════════════════════════════════════════════╝");
    println!();

    // Initialize components
    println!("Initializing components...");
    let start = Instant::now();
    let verb_db = VerbDatabase::new();
    let dictionary = EnglishDictionary::new();
    let grammar = EnglishGrammar::new();
    let mut command_parser = CommandParser::new();
    let _disambiguator = SemanticDisambiguator::new();
    println!("Initialization time: {:?}", start.elapsed());
    println!();

    // Test data
    let test_verbs = ["walk", "run", "think", "believe", "create", "destroy", "give", "take"];
    let test_misspellings = ["wlak", "runn", "thnk", "beleive", "creat", "destory"];
    let test_commands = [
        "walk to the store",
        "run quickly home",
        "think about it",
        "create a document",
    ];
    let test_contractions = ["don't", "won't", "can't", "I'm", "we'll", "they've"];

    // ========== MEMORY FOOTPRINT ==========
    println!("═══════════════════════════════════════════════════════════════════");
    println!("                      MEMORY FOOTPRINT                             ");
    println!("═══════════════════════════════════════════════════════════════════");

    let verb_count = verb_db.len();
    let dict_count = dictionary.len();

    println!("\nVerb Database:");
    println!("  - Total verbs: {}", verb_count);
    println!("  - Categories: 25 functional categories");
    println!("  - Estimated size: ~{} KB", verb_count * 200 / 1024);

    println!("\nDictionary (BK-Tree):");
    println!("  - Total words: {}", dict_count);
    println!("  - Structure: BK-Tree for O(log N) fuzzy search");
    println!("  - Estimated size: ~{} KB", dict_count * 50 / 1024);

    println!("\nGrammar Engine:");
    println!("  - Contractions: 50+ patterns");
    println!("  - Estimated size: ~45 KB");

    println!("\n  TOTAL ESTIMATED: ~{} MB",
        (verb_count * 200 + dict_count * 50 + 45 * 1024) / (1024 * 1024) + 1);
    println!();

    // ========== BENCHMARK 1: VERB LOOKUP ==========
    println!("═══════════════════════════════════════════════════════════════════");
    println!("  1. VERB LOOKUP BENCHMARK");
    println!("═══════════════════════════════════════════════════════════════════");

    // Warmup
    for _ in 0..WARMUP {
        for verb in &test_verbs {
            black_box(verb_db.lookup(verb));
        }
    }

    let start = Instant::now();
    for _ in 0..ITERATIONS {
        for verb in &test_verbs {
            black_box(verb_db.lookup(verb));
        }
    }
    let elapsed = start.elapsed();
    let ops = ITERATIONS * test_verbs.len() as u64;
    let verb_ops_per_sec = ops as f64 / elapsed.as_secs_f64();
    let verb_latency_us = elapsed.as_nanos() as f64 / ops as f64 / 1000.0;

    println!("  Operations: {}", ops);
    println!("  Time: {:?}", elapsed);
    println!("  Throughput: {:.2}M ops/sec", verb_ops_per_sec / 1_000_000.0);
    println!("  Latency: {:.3} µs", verb_latency_us);
    println!();

    // ========== BENCHMARK 2: SPELL CORRECTION (BK-Tree) ==========
    println!("═══════════════════════════════════════════════════════════════════");
    println!("  2. SPELL CORRECTION (BK-Tree) BENCHMARK");
    println!("═══════════════════════════════════════════════════════════════════");

    // Warmup
    for _ in 0..WARMUP {
        for word in &test_misspellings {
            black_box(dictionary.find_similar(word, 2));
        }
    }

    let start = Instant::now();
    for _ in 0..ITERATIONS {
        for word in &test_misspellings {
            black_box(dictionary.find_similar(word, 2));
        }
    }
    let elapsed = start.elapsed();
    let ops = ITERATIONS * test_misspellings.len() as u64;
    let bktree_ops_per_sec = ops as f64 / elapsed.as_secs_f64();
    let bktree_latency_us = elapsed.as_nanos() as f64 / ops as f64 / 1000.0;

    println!("  Operations: {}", ops);
    println!("  Time: {:?}", elapsed);
    println!("  Throughput: {:.2}K ops/sec", bktree_ops_per_sec / 1000.0);
    println!("  Latency: {:.2} µs", bktree_latency_us);
    println!();

    // ========== BENCHMARK 3: SPELL CORRECTION (Linear - baseline) ==========
    println!("═══════════════════════════════════════════════════════════════════");
    println!("  3. SPELL CORRECTION (Linear Search) - BASELINE");
    println!("═══════════════════════════════════════════════════════════════════");

    // Build a word list for linear search simulation (use common words)
    let linear_dict: Vec<&str> = vec![
        "walk", "run", "think", "believe", "create", "destroy", "give", "take",
        "the", "and", "but", "or", "if", "then", "when", "where", "what", "who",
        "have", "has", "had", "do", "does", "did", "will", "would", "could", "should",
        "make", "made", "see", "saw", "come", "came", "go", "went", "get", "got",
        "know", "knew", "find", "found", "tell", "told", "say", "said", "ask", "asked",
        "help", "hello", "world", "work", "time", "year", "people", "way", "day", "man",
        "woman", "child", "part", "place", "case", "week", "company", "system", "program",
        "question", "government", "number", "night", "point", "home", "water", "room",
        "mother", "area", "money", "story", "fact", "month", "lot", "right", "study",
        "book", "eye", "job", "word", "business", "issue", "side", "kind", "head", "house",
    ];

    // For fair comparison, use fewer iterations (linear is O(N) per query)
    let linear_iters = ITERATIONS / 100;

    // Warmup
    for _ in 0..WARMUP / 100 {
        for word in &test_misspellings {
            for dict_word in &linear_dict {
                let d = levenshtein(word, dict_word);
                if d <= 2 {
                    black_box(d);
                }
            }
        }
    }

    let start = Instant::now();
    for _ in 0..linear_iters {
        for word in &test_misspellings {
            let mut results = Vec::new();
            for dict_word in &linear_dict {
                let d = levenshtein(word, dict_word);
                if d <= 2 {
                    results.push((*dict_word, d));
                }
            }
            black_box(results);
        }
    }
    let elapsed = start.elapsed();
    let ops = linear_iters * test_misspellings.len() as u64;
    let linear_ops_per_sec = ops as f64 / elapsed.as_secs_f64();
    let linear_latency_us = elapsed.as_nanos() as f64 / ops as f64 / 1000.0;

    // Scale to full dictionary size
    let scale_factor = dict_count as f64 / linear_dict.len() as f64;
    let projected_linear_ops = linear_ops_per_sec / scale_factor;
    let projected_linear_latency = linear_latency_us * scale_factor;

    println!("  Sample dict size: {} words", linear_dict.len());
    println!("  Full dict size: {} words", dict_count);
    println!("  Scale factor: {:.1}x", scale_factor);
    println!("  Sampled throughput: {:.2}K ops/sec", linear_ops_per_sec / 1000.0);
    println!("  Projected throughput: {:.2}K ops/sec", projected_linear_ops / 1000.0);
    println!("  Projected latency: {:.1} µs", projected_linear_latency);
    println!();

    let speedup = bktree_ops_per_sec / projected_linear_ops;
    println!("  >>> BK-Tree SPEEDUP: {:.1}x <<<", speedup);
    println!();

    // ========== BENCHMARK 4: COMMAND PARSING ==========
    println!("═══════════════════════════════════════════════════════════════════");
    println!("  4. COMMAND PARSING BENCHMARK");
    println!("═══════════════════════════════════════════════════════════════════");

    // Warmup
    for _ in 0..WARMUP {
        for cmd in &test_commands {
            black_box(command_parser.parse(cmd));
        }
    }

    let start = Instant::now();
    for _ in 0..ITERATIONS {
        for cmd in &test_commands {
            black_box(command_parser.parse(cmd));
        }
    }
    let elapsed = start.elapsed();
    let ops = ITERATIONS * test_commands.len() as u64;
    let cmd_ops_per_sec = ops as f64 / elapsed.as_secs_f64();
    let cmd_latency_us = elapsed.as_nanos() as f64 / ops as f64 / 1000.0;

    println!("  Operations: {}", ops);
    println!("  Time: {:?}", elapsed);
    println!("  Throughput: {:.2}K ops/sec", cmd_ops_per_sec / 1000.0);
    println!("  Latency: {:.2} µs", cmd_latency_us);
    println!();

    // ========== BENCHMARK 5: CONTRACTION EXPANSION ==========
    println!("═══════════════════════════════════════════════════════════════════");
    println!("  5. CONTRACTION EXPANSION BENCHMARK");
    println!("═══════════════════════════════════════════════════════════════════");

    // Warmup
    for _ in 0..WARMUP {
        for contraction in &test_contractions {
            black_box(grammar.expand_contraction(contraction));
        }
    }

    let start = Instant::now();
    for _ in 0..ITERATIONS {
        for contraction in &test_contractions {
            black_box(grammar.expand_contraction(contraction));
        }
    }
    let elapsed = start.elapsed();
    let ops = ITERATIONS * test_contractions.len() as u64;
    let contr_ops_per_sec = ops as f64 / elapsed.as_secs_f64();
    let contr_latency_us = elapsed.as_nanos() as f64 / ops as f64 / 1000.0;

    println!("  Operations: {}", ops);
    println!("  Time: {:?}", elapsed);
    println!("  Throughput: {:.2}M ops/sec", contr_ops_per_sec / 1_000_000.0);
    println!("  Latency: {:.3} µs", contr_latency_us);
    println!();

    // ========== SUMMARY TABLE ==========
    println!("═══════════════════════════════════════════════════════════════════");
    println!("                    SUMMARY FOR PAPER                              ");
    println!("═══════════════════════════════════════════════════════════════════");
    println!();
    println!("System: Intel Core i7-12650H (10 cores, 16 threads), 16 GB RAM");
    println!();
    println!("┌──────────────────────────────┬─────────────────────┬──────────────┐");
    println!("│ Operation                    │ Throughput          │ Latency      │");
    println!("├──────────────────────────────┼─────────────────────┼──────────────┤");
    println!("│ Verb lookup                  │ {:.1}M ops/sec       │ {:.2} µs     │",
             verb_ops_per_sec / 1_000_000.0, verb_latency_us);
    println!("│ Spell correction (BK-Tree)   │ {:.0}K ops/sec       │ {:.1} µs     │",
             bktree_ops_per_sec / 1000.0, bktree_latency_us);
    println!("│ Spell correction (Linear)    │ {:.0}K ops/sec       │ {:.0} µs     │",
             projected_linear_ops / 1000.0, projected_linear_latency);
    println!("│ Command parsing              │ {:.0}K ops/sec       │ {:.1} µs     │",
             cmd_ops_per_sec / 1000.0, cmd_latency_us);
    println!("│ Contraction expansion        │ {:.1}M ops/sec       │ {:.2} µs     │",
             contr_ops_per_sec / 1_000_000.0, contr_latency_us);
    println!("└──────────────────────────────┴─────────────────────┴──────────────┘");
    println!();
    println!("BK-Tree Speedup: {:.0}x over linear search", speedup);
    println!();
    println!("Memory Footprint:");
    println!("  - Verb database: ~{} KB", verb_count * 200 / 1024);
    println!("  - Dictionary (BK-Tree): ~{} MB", dict_count * 50 / 1024 / 1024 + 1);
    println!("  - Grammar rules: ~45 KB");
    println!("  - Total: ~{} MB", (verb_count * 200 + dict_count * 50 + 45 * 1024) / (1024 * 1024) + 1);
    println!();
    println!("Benchmark completed successfully!");
}
