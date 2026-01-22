//! Full benchmark suite for NL-SRE-English
//! Measures all metrics reported in the academic paper

use std::time::{Duration, Instant};
use std::hint::black_box;

// Import the library
use nl_sre_english::verbs::{VerbDatabase, FunctionalCategory};
use nl_sre_english::dictionary::{Dictionary, BKTree};
use nl_sre_english::grammar::Grammar;
use nl_sre_english::command_parser::CommandParser;
use nl_sre_english::SemanticDisambiguator;

const ITERATIONS: u64 = 100_000;
const WARMUP_ITERATIONS: u64 = 10_000;

fn main() {
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║     NL-SRE-English Full Benchmark Suite                      ║");
    println!("║     System: Intel Core i7-12650H, 16 GB RAM                  ║");
    println!("╚══════════════════════════════════════════════════════════════╝\n");

    // Initialize components
    println!("Initializing components...");
    let verb_db = VerbDatabase::new();
    let dictionary = Dictionary::new();
    let grammar = Grammar::new();
    let mut command_parser = CommandParser::new();
    let disambiguator = SemanticDisambiguator::new();
    println!("Components initialized.\n");

    // ===== MEMORY FOOTPRINT =====
    println!("═══════════════════════════════════════════════════════════════");
    println!("                    MEMORY FOOTPRINT");
    println!("═══════════════════════════════════════════════════════════════");

    let verb_db_size = std::mem::size_of_val(&verb_db);
    let dict_size = std::mem::size_of_val(&dictionary);
    let grammar_size = std::mem::size_of_val(&grammar);

    // Estimate actual data size
    let verb_count = verb_db.len();
    let dict_word_count = dictionary.len();

    println!("Verb Database:");
    println!("  - Verb count: {}", verb_count);
    println!("  - Struct size: {} bytes", verb_db_size);

    println!("\nDictionary (BK-Tree):");
    println!("  - Word count: {}", dict_word_count);
    println!("  - Struct size: {} bytes", dict_size);

    println!("\nGrammar Rules:");
    println!("  - Struct size: {} bytes", grammar_size);

    println!("\n");

    // ===== PERFORMANCE BENCHMARKS =====
    println!("═══════════════════════════════════════════════════════════════");
    println!("                 PERFORMANCE BENCHMARKS");
    println!("           ({} iterations after {} warmup)", ITERATIONS, WARMUP_ITERATIONS);
    println!("═══════════════════════════════════════════════════════════════\n");

    // Test data
    let test_verbs = vec!["walk", "run", "think", "believe", "create", "destroy", "give", "take"];
    let test_misspellings = vec!["wlak", "runn", "thnk", "beleive", "creat", "destory"];
    let test_commands = vec![
        "walk to the store",
        "run quickly to the park",
        "think about the problem",
        "create a new document",
    ];
    let test_contractions = vec!["don't", "won't", "can't", "I'm", "we'll", "they've"];

    // 1. Verb Lookup
    println!("1. VERB LOOKUP");
    println!("   Warming up...");
    for _ in 0..WARMUP_ITERATIONS {
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
    let ops_per_sec = ops as f64 / elapsed.as_secs_f64();
    let latency_ns = elapsed.as_nanos() as f64 / ops as f64;
    println!("   Total operations: {}", ops);
    println!("   Total time: {:?}", elapsed);
    println!("   Throughput: {:.2} ops/sec ({:.2}M ops/sec)", ops_per_sec, ops_per_sec / 1_000_000.0);
    println!("   Latency: {:.2} ns ({:.2} µs)", latency_ns, latency_ns / 1000.0);
    println!();

    // 2. Spell Correction (BK-Tree)
    println!("2. SPELL CORRECTION (BK-Tree)");
    println!("   Warming up...");
    for _ in 0..WARMUP_ITERATIONS {
        for word in &test_misspellings {
            black_box(dictionary.suggest(word, 2));
        }
    }

    let start = Instant::now();
    for _ in 0..ITERATIONS {
        for word in &test_misspellings {
            black_box(dictionary.suggest(word, 2));
        }
    }
    let elapsed = start.elapsed();
    let ops = ITERATIONS * test_misspellings.len() as u64;
    let ops_per_sec_bktree = ops as f64 / elapsed.as_secs_f64();
    let latency_ns = elapsed.as_nanos() as f64 / ops as f64;
    println!("   Total operations: {}", ops);
    println!("   Total time: {:?}", elapsed);
    println!("   Throughput: {:.2} ops/sec ({:.2}K ops/sec)", ops_per_sec_bktree, ops_per_sec_bktree / 1000.0);
    println!("   Latency: {:.2} ns ({:.2} µs)", latency_ns, latency_ns / 1000.0);
    println!();

    // 3. Spell Correction (Linear Search for comparison)
    println!("3. SPELL CORRECTION (Linear Search - baseline)");
    println!("   Warming up...");
    let linear_iterations = ITERATIONS / 10; // Fewer iterations since it's slower
    for _ in 0..WARMUP_ITERATIONS / 10 {
        for word in &test_misspellings {
            black_box(dictionary.suggest_linear(word, 2));
        }
    }

    let start = Instant::now();
    for _ in 0..linear_iterations {
        for word in &test_misspellings {
            black_box(dictionary.suggest_linear(word, 2));
        }
    }
    let elapsed = start.elapsed();
    let ops = linear_iterations * test_misspellings.len() as u64;
    let ops_per_sec_linear = ops as f64 / elapsed.as_secs_f64();
    let latency_ns = elapsed.as_nanos() as f64 / ops as f64;
    let speedup = ops_per_sec_bktree / ops_per_sec_linear;
    println!("   Total operations: {}", ops);
    println!("   Total time: {:?}", elapsed);
    println!("   Throughput: {:.2} ops/sec ({:.2}K ops/sec)", ops_per_sec_linear, ops_per_sec_linear / 1000.0);
    println!("   Latency: {:.2} ns ({:.2} µs)", latency_ns, latency_ns / 1000.0);
    println!("   BK-Tree SPEEDUP: {:.1}x", speedup);
    println!();

    // 4. Command Parsing
    println!("4. COMMAND PARSING");
    println!("   Warming up...");
    for _ in 0..WARMUP_ITERATIONS {
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
    let ops_per_sec = ops as f64 / elapsed.as_secs_f64();
    let latency_ns = elapsed.as_nanos() as f64 / ops as f64;
    println!("   Total operations: {}", ops);
    println!("   Total time: {:?}", elapsed);
    println!("   Throughput: {:.2} ops/sec ({:.2}K ops/sec)", ops_per_sec, ops_per_sec / 1000.0);
    println!("   Latency: {:.2} ns ({:.2} µs)", latency_ns, latency_ns / 1000.0);
    println!();

    // 5. Contraction Expansion
    println!("5. CONTRACTION EXPANSION");
    println!("   Warming up...");
    for _ in 0..WARMUP_ITERATIONS {
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
    let ops_per_sec = ops as f64 / elapsed.as_secs_f64();
    let latency_ns = elapsed.as_nanos() as f64 / ops as f64;
    println!("   Total operations: {}", ops);
    println!("   Total time: {:?}", elapsed);
    println!("   Throughput: {:.2} ops/sec ({:.2}M ops/sec)", ops_per_sec, ops_per_sec / 1_000_000.0);
    println!("   Latency: {:.2} ns ({:.2} µs)", latency_ns, latency_ns / 1000.0);
    println!();

    // 6. Full Semantic Disambiguation
    println!("6. FULL SEMANTIC DISAMBIGUATION");
    println!("   Warming up...");
    for _ in 0..WARMUP_ITERATIONS / 10 {
        for cmd in &test_commands {
            black_box(disambiguator.process(cmd));
        }
    }

    let start = Instant::now();
    for _ in 0..ITERATIONS / 10 {
        for cmd in &test_commands {
            black_box(disambiguator.process(cmd));
        }
    }
    let elapsed = start.elapsed();
    let ops = (ITERATIONS / 10) * test_commands.len() as u64;
    let ops_per_sec = ops as f64 / elapsed.as_secs_f64();
    let latency_ns = elapsed.as_nanos() as f64 / ops as f64;
    println!("   Total operations: {}", ops);
    println!("   Total time: {:?}", elapsed);
    println!("   Throughput: {:.2} ops/sec ({:.2}K ops/sec)", ops_per_sec, ops_per_sec / 1000.0);
    println!("   Latency: {:.2} ns ({:.2} µs)", latency_ns, latency_ns / 1000.0);
    println!();

    // ===== SUMMARY TABLE =====
    println!("═══════════════════════════════════════════════════════════════");
    println!("                     SUMMARY TABLE");
    println!("         (For paper Table 2 - Performance Benchmarks)");
    println!("═══════════════════════════════════════════════════════════════");
    println!("| Operation                | Throughput      | Latency     |");
    println!("|--------------------------|-----------------|-------------|");
    // Note: Values will be filled by actual benchmark output
    println!();
    println!("BK-Tree Speedup over Linear Search: {:.1}x", speedup);
    println!();
    println!("Benchmark completed!");
}
