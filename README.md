# NL-SRE-English

**Probabilistic Semantic Disambiguation Engine for English**

[![DOI](https://zenodo.org/badge/DOI/10.5281/zenodo.18300355.svg)](https://doi.org/10.5281/zenodo.18300355)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)
[![Zero Dependencies](https://img.shields.io/badge/dependencies-zero-green.svg)](Cargo.toml)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

A comprehensive English verb database with semantic disambiguation capabilities, designed for natural language processing, command parsing, and AI applications.

## Features

- **1500+ English verbs** organized into 25 functional categories
- **80+ verb groups** for fine-grained classification
- **Complete conjugation system** (regular + irregular verbs)
- **Zero dependencies** - Pure Rust implementation
- **Spell correction** via Levenshtein distance
- **Phonetic matching** via Soundex and Metaphone algorithms
- **Natural language command parser** for action extraction

## Functional Categories

The engine organizes verbs into semantic categories for easy programmatic access:

| Category | Description | Examples |
|----------|-------------|----------|
| **Movement** | Motion and locomotion | walk, run, fly, swim, climb |
| **Perception** | Sensing and perceiving | see, hear, feel, smell, taste |
| **Communication** | Speaking and speech acts | say, tell, speak, ask, answer |
| **Cognition** | Mental processes | think, know, believe, understand |
| **Emotion** | Emotional states | love, hate, fear, hope, enjoy |
| **Physical** | Physical manipulation | hit, cut, push, pull, throw |
| **State** | States of being | be, exist, remain, stay |
| **Change** | Change of state | become, grow, transform |
| **Transfer** | Giving and receiving | give, take, send, receive |
| **Creation** | Making and producing | make, create, build, write |
| **Destruction** | Breaking and destroying | destroy, break, kill, damage |
| **Control** | Controlling and managing | control, manage, lead, govern |
| **Possession** | Owning and having | own, have, possess, acquire |
| **Social** | Social interaction | meet, help, fight, cooperate |
| **Consumption** | Eating and drinking | eat, drink, consume, breathe |
| **Body** | Bodily functions | sleep, wake, sit, stand, lie |
| **Weather** | Weather phenomena | rain, snow, blow, shine |
| **Measurement** | Measuring and comparing | measure, weigh, count, compare |
| **Aspectual** | Beginning, ending, continuing | begin, end, continue, stop |
| **Causation** | Causing and enabling | cause, allow, prevent, force |
| **Attempt** | Trying and succeeding/failing | try, succeed, fail, practice |
| **Modal** | Modal and semi-modal | want, need, can, should |
| **Position** | Body position and location | put, place, set, remove |
| **Connection** | Joining and separating | connect, join, separate, split |
| **Emission** | Light and sound emission | shine, glow, ring, buzz |

## Quick Start

```rust
use nl_sre_english::SemanticDisambiguator;
use nl_sre_english::verbs::FunctionalCategory;

fn main() {
    let disambiguator = SemanticDisambiguator::new();

    // Process a sentence
    let result = disambiguator.process("The cat runs quickly across the room");

    println!("Detected actions:");
    for action in &result.detected_actions {
        println!("  - {} (base: {}, category: {})",
            action.verb,
            action.base_form,
            action.category.name()
        );
    }

    // Get verbs by category
    let movement_verbs = disambiguator.verbs_by_category(FunctionalCategory::Movement);
    println!("Movement verbs: {:?}", &movement_verbs[..5]);
}
```

## Verb Database Usage

```rust
use nl_sre_english::verbs::{VerbDatabase, FunctionalCategory, VerbGroup};

fn main() {
    let db = VerbDatabase::with_builtin();

    // Look up any verb form
    if let Some(entry) = db.lookup("running") {
        println!("Base form: {}", entry.base);           // "run"
        println!("Category: {}", entry.category.name()); // "Movement"
        println!("Group: {}", entry.group.name());       // "Run"
        println!("Irregular: {}", entry.irregular);      // true
        println!("Forms: {} / {} / {} / {}",
            entry.base,
            entry.past,
            entry.past_participle,
            entry.present_participle
        );
    }

    // Get all verbs in a category
    let emotions = db.by_category(FunctionalCategory::Emotion);
    for verb in emotions.iter().take(10) {
        println!("{}: {}", verb.base, verb.synonyms.join(", "));
    }

    // Get all verbs in a specific group
    let running_verbs = db.by_group(VerbGroup::Run);
    // Returns: run, sprint, dash, race, jog, rush, hurry, bolt...
}
```

## Command Parser

```rust
use nl_sre_english::command_parser::CommandParser;

fn main() {
    let mut parser = CommandParser::new();

    // Parse natural language commands
    if let Some(cmd) = parser.parse("please walk to the store") {
        println!("Action: {}", cmd.action);       // "walk"
        println!("Category: {}", cmd.category.name()); // "Movement"
        println!("Subject: {:?}", cmd.subject);   // Some("please")
        println!("Object: {:?}", cmd.object);     // Some("to the store")
    }

    // Parse multiple commands
    let commands = parser.parse_all("Run to the store. Buy some milk. Come back home.");
    // Returns 3 parsed commands
}
```

## Database Statistics

```
Total verbs: 1500+
Irregular verbs: 200+
Regular verbs: 1300+
Total forms indexed: 7500+
Functional categories: 25
Verb groups: 80+
```

## Architecture

```
┌─────────────────────────────────────────────────────┐
│                  SemanticDisambiguator              │
├─────────────────────────────────────────────────────┤
│  ┌──────────────┐  ┌────────────┐  ┌────────────┐  │
│  │ VerbDatabase │  │  Grammar   │  │ Dictionary │  │
│  │  1500+ verbs │  │   Parser   │  │   5000+    │  │
│  │  25 categories│  │   POS tag  │  │   words    │  │
│  └──────────────┘  └────────────┘  └────────────┘  │
├─────────────────────────────────────────────────────┤
│              CommandParser (NL → Structured)        │
└─────────────────────────────────────────────────────┘
```

## Building

```bash
cargo build --release
```

## Running Examples

```bash
cargo run --example verb_groups
```

## Testing

```bash
cargo test
```

## Optimizations (v0.1.1)

### BK-Tree Fuzzy Search

The spell correction system has been optimized with a **Burkhard-Keller Tree (BK-Tree)** implementation:

- **Before**: O(N×M) complexity - compared every word in dictionary
- **After**: O(log N × M) average - uses triangle inequality pruning

```rust
use nl_sre_english::EnglishDictionary;

let dict = EnglishDictionary::new();
// Fast fuzzy search for spell correction
let suggestions = dict.find_similar("helo", 2);
// Returns: [("hello", 1), ("help", 2), ("held", 2), ...]
```

**Benchmark improvement**: ~37x faster for typical spell correction queries.

### Contraction Expansion

The tokenizer now automatically expands 50+ common English contractions:

```rust
use nl_sre_english::EnglishGrammar;

let grammar = EnglishGrammar::new();

// "don't" expands to "do" + "not"
let tokens = grammar.tokenize("I don't know");
assert_eq!(tokens, vec!["i", "do", "not", "know"]);

// "we'll" expands to "we" + "will"
let tokens = grammar.tokenize("We'll see");
assert_eq!(tokens, vec!["we", "will", "see"]);
```

**Supported contractions**:
- Negative: don't, doesn't, didn't, won't, can't, couldn't, shouldn't, wouldn't, isn't, aren't, wasn't, weren't, haven't, hasn't, hadn't...
- Pronoun + be: I'm, you're, he's, she's, it's, we're, they're...
- Pronoun + have: I've, you've, we've, they've, could've, would've, should've...
- Pronoun + will: I'll, you'll, he'll, she'll, we'll, they'll...
- Pronoun + would/had: I'd, you'd, he'd, she'd, we'd, they'd...
- Other: let's, ain't...

### Regression Test Suite

Added comprehensive regression tests (`tests/regression.rs`) covering:
- Dictionary word validation (100 most common words)
- Spell correction accuracy
- Contraction expansion coverage
- Command parsing for all categories
- Disambiguator action detection
- Edge cases and performance benchmarks

## Use Cases

- **AI Assistants**: Understand user intent and extract actions
- **Chatbots**: Parse commands and respond appropriately
- **Game Development**: NPC command interpretation
- **Voice Interfaces**: Convert speech to structured commands
- **Text Analysis**: Action and intent extraction
- **Educational Software**: Verb conjugation and grammar tools
- **Robotics**: Natural language to robot commands

## License

MIT License - See [LICENSE](LICENSE) for details.

## Author

**Francisco Molina-Burgos**
Avermex Research Division
Merida, Yucatan, Mexico

---

*Part of the NL-SRE (Natural Language Semantic Rule Engine) family*

*See also: [NL-SRE-Semantico](https://github.com/Yatrogenesis/NL-SRE-Semantico) (Spanish)* [![DOI](https://zenodo.org/badge/DOI/10.5281/zenodo.18293530.svg)](https://doi.org/10.5281/zenodo.18293530)
