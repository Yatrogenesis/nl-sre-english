//! # Quantitative Tokenizer Module
//!
//! Pre-semantic scanner for detecting numbers and units in text.
//! This runs BEFORE the main NLP pipeline to extract quantitative data.
//!
//! ## Design Philosophy
//!
//! The quantitative lexer operates as a "first-pass" scanner that:
//! 1. Identifies numeric literals (integers, floats, scientific notation)
//! 2. Associates units with numbers
//! 3. Passes non-numeric tokens through unchanged
//!
//! This separation allows the main semantic engine to focus on
//! natural language while quantitative data is handled deterministically.
//!
//! ## Architecture
//!
//! ```text
//! Input: "Set plasma temperature to 15 keV and current to 2.5 MA"
//!        ↓
//! ┌─────────────────────────────────────────────────────────────┐
//! │ QuantitativeTokenizer                                       │
//! │  ┌─────────────────────────────────────────────────────┐   │
//! │  │ Pre-scan: Find all number+unit patterns             │   │
//! │  │ "15 keV" → QuantityToken { 15.0, "keV" }           │   │
//! │  │ "2.5 MA" → QuantityToken { 2.5, "MA" }             │   │
//! │  └─────────────────────────────────────────────────────┘   │
//! └─────────────────────────────────────────────────────────────┘
//!        ↓
//! Output: [Word("Set"), Word("plasma"), Word("temperature"),
//!          Word("to"), Quantity(15.0, "keV"), Word("and"),
//!          Word("current"), Word("to"), Quantity(2.5, "MA")]
//! ```
//!
//! ## Author
//! Francisco Molina-Burgos, Avermex Research Division
//!
//! ## Date
//! January 2026

use std::collections::HashSet;

/// Token kind produced by the quantitative tokenizer
#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    /// A numeric value with optional unit
    Number {
        value: f64,
        raw: String,
    },
    /// A unit string (when detected separately from number)
    Unit(String),
    /// A quantity (number + unit combined)
    Quantity {
        value: f64,
        unit: String,
        raw: String,
    },
    /// A regular word
    Word(String),
    /// Punctuation
    Punctuation(char),
    /// Whitespace (preserved for position tracking)
    Whitespace(String),
    /// Unknown/unrecognized token
    Unknown(String),
}

/// A token with position information
#[derive(Debug, Clone)]
pub struct QuantitativeToken {
    /// The kind of token
    pub kind: TokenKind,
    /// Start position in original string
    pub start: usize,
    /// End position in original string
    pub end: usize,
    /// Line number (1-indexed)
    pub line: usize,
    /// Column number (1-indexed)
    pub column: usize,
}

impl QuantitativeToken {
    pub fn new(kind: TokenKind, start: usize, end: usize) -> Self {
        Self {
            kind,
            start,
            end,
            line: 1,
            column: start + 1,
        }
    }

    /// Check if this is a quantity token
    pub fn is_quantity(&self) -> bool {
        matches!(self.kind, TokenKind::Quantity { .. })
    }

    /// Check if this is a number token
    pub fn is_number(&self) -> bool {
        matches!(self.kind, TokenKind::Number { .. } | TokenKind::Quantity { .. })
    }

    /// Get the numeric value if this is a number/quantity
    pub fn value(&self) -> Option<f64> {
        match &self.kind {
            TokenKind::Number { value, .. } => Some(*value),
            TokenKind::Quantity { value, .. } => Some(*value),
            _ => None,
        }
    }

    /// Get the unit if this is a quantity
    pub fn unit(&self) -> Option<&str> {
        match &self.kind {
            TokenKind::Quantity { unit, .. } => Some(unit.as_str()),
            TokenKind::Unit(u) => Some(u.as_str()),
            _ => None,
        }
    }

    /// Get the word content if this is a word
    pub fn word(&self) -> Option<&str> {
        match &self.kind {
            TokenKind::Word(w) => Some(w.as_str()),
            _ => None,
        }
    }
}

/// Configuration for the quantitative tokenizer
#[derive(Debug, Clone)]
pub struct TokenizerConfig {
    /// Known units to recognize
    pub known_units: HashSet<String>,
    /// Whether to combine number+unit into Quantity
    pub combine_quantity: bool,
    /// Whether to preserve whitespace tokens
    pub preserve_whitespace: bool,
    /// Whether to allow scientific notation (1e-3, 2.5E+6)
    pub allow_scientific: bool,
    /// Whether to allow negative numbers
    pub allow_negative: bool,
    /// Maximum digits before decimal point
    pub max_integer_digits: usize,
    /// Maximum digits after decimal point
    pub max_decimal_digits: usize,
}

impl Default for TokenizerConfig {
    fn default() -> Self {
        Self {
            known_units: Self::default_units(),
            combine_quantity: true,
            preserve_whitespace: false,
            allow_scientific: true,
            allow_negative: true,
            max_integer_digits: 15,
            max_decimal_digits: 10,
        }
    }
}

impl TokenizerConfig {
    fn default_units() -> HashSet<String> {
        [
            // SI base units
            "m", "kg", "s", "A", "K", "mol", "cd",
            // SI derived units
            "Hz", "N", "Pa", "J", "W", "C", "V", "F", "Ω", "ohm",
            "S", "Wb", "T", "H", "lm", "lx", "Bq", "Gy", "Sv", "kat",
            // Common prefixed units
            "km", "cm", "mm", "μm", "um", "nm", "pm", "fm",
            "kHz", "MHz", "GHz", "THz",
            "kW", "MW", "GW", "TW",
            "kV", "MV", "GV",
            "mA", "μA", "uA", "nA", "pA",
            "kPa", "MPa", "GPa",
            "kJ", "MJ", "GJ",
            "eV", "keV", "MeV", "GeV", "TeV",
            // Time
            "ms", "μs", "us", "ns", "ps", "fs",
            "min", "hr", "h",
            // Temperature
            "°C", "°F", "degC", "degF",
            // Angles
            "rad", "deg", "°", "mrad", "μrad",
            // Mass
            "g", "mg", "μg", "ug", "ng",
            "lb", "oz",
            // Length (non-SI)
            "in", "ft", "yd", "mi",
            // Volume
            "L", "mL", "μL", "uL",
            "gal", "qt", "pt",
            // Common composite
            "m/s", "km/h", "mph",
            "kg/m³", "g/cm³", "g/mL",
            "N/m", "Pa·s",
            "W/m²", "MW/m²",
            "A/m²", "MA",
            // Dimensionless but domain-specific
            "n20", "%", "ppm", "ppb",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect()
    }

    /// Add custom units to the tokenizer
    pub fn with_units<I, S>(mut self, units: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        for unit in units {
            self.known_units.insert(unit.into());
        }
        self
    }

    /// Set whether to combine number+unit
    pub fn combine_quantities(mut self, combine: bool) -> Self {
        self.combine_quantity = combine;
        self
    }

    /// Set whether to preserve whitespace
    pub fn preserve_whitespace(mut self, preserve: bool) -> Self {
        self.preserve_whitespace = preserve;
        self
    }
}

/// Quantitative tokenizer - first-pass scanner for numbers and units
pub struct QuantitativeTokenizer {
    config: TokenizerConfig,
}

impl Default for QuantitativeTokenizer {
    fn default() -> Self {
        Self::new()
    }
}

impl QuantitativeTokenizer {
    /// Create a new tokenizer with default configuration
    pub fn new() -> Self {
        Self {
            config: TokenizerConfig::default(),
        }
    }

    /// Create a tokenizer with custom configuration
    pub fn with_config(config: TokenizerConfig) -> Self {
        Self { config }
    }

    /// Add units to the tokenizer
    pub fn add_units<I, S>(&mut self, units: I)
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        for unit in units {
            self.config.known_units.insert(unit.into());
        }
    }

    /// Check if a string is a known unit
    pub fn is_unit(&self, s: &str) -> bool {
        self.config.known_units.contains(s)
    }

    /// Tokenize input text
    pub fn tokenize(&self, input: &str) -> Vec<QuantitativeToken> {
        let mut tokens = Vec::new();
        let chars: Vec<char> = input.chars().collect();
        let mut pos = 0;

        while pos < chars.len() {
            let start = pos;

            // Skip and optionally collect whitespace
            if chars[pos].is_whitespace() {
                let ws_start = pos;
                while pos < chars.len() && chars[pos].is_whitespace() {
                    pos += 1;
                }
                if self.config.preserve_whitespace {
                    let ws: String = chars[ws_start..pos].iter().collect();
                    tokens.push(QuantitativeToken::new(
                        TokenKind::Whitespace(ws),
                        ws_start,
                        pos,
                    ));
                }
                continue;
            }

            // Try to parse a number (possibly with unit)
            if let Some((token, new_pos)) = self.try_parse_number(&chars, pos) {
                tokens.push(token);
                pos = new_pos;
                continue;
            }

            // Try to parse a word
            if chars[pos].is_alphabetic() || chars[pos] == '_' {
                let word_start = pos;
                while pos < chars.len() && (chars[pos].is_alphanumeric() || chars[pos] == '_' || chars[pos] == '-') {
                    pos += 1;
                }
                let word: String = chars[word_start..pos].iter().collect();

                // Check if it's a standalone unit
                if self.is_unit(&word) {
                    tokens.push(QuantitativeToken::new(
                        TokenKind::Unit(word),
                        word_start,
                        pos,
                    ));
                } else {
                    tokens.push(QuantitativeToken::new(
                        TokenKind::Word(word),
                        word_start,
                        pos,
                    ));
                }
                continue;
            }

            // Punctuation
            if chars[pos].is_ascii_punctuation() && chars[pos] != '_' && chars[pos] != '-' {
                tokens.push(QuantitativeToken::new(
                    TokenKind::Punctuation(chars[pos]),
                    start,
                    pos + 1,
                ));
                pos += 1;
                continue;
            }

            // Unknown character
            tokens.push(QuantitativeToken::new(
                TokenKind::Unknown(chars[pos].to_string()),
                start,
                pos + 1,
            ));
            pos += 1;
        }

        // Post-process: combine number + unit if configured
        if self.config.combine_quantity {
            self.combine_numbers_and_units(&mut tokens);
        }

        tokens
    }

    /// Try to parse a number starting at position
    fn try_parse_number(&self, chars: &[char], start: usize) -> Option<(QuantitativeToken, usize)> {
        let mut pos = start;

        // Handle optional negative sign
        let has_negative = self.config.allow_negative && pos < chars.len() && chars[pos] == '-';
        if has_negative {
            pos += 1;
        }

        // Must start with digit or decimal point
        if pos >= chars.len() || (!chars[pos].is_ascii_digit() && chars[pos] != '.') {
            return None;
        }

        // Special case: lone decimal point is not a number
        if chars[pos] == '.'
            && (pos + 1 >= chars.len() || !chars[pos + 1].is_ascii_digit())
        {
            return None;
        }

        let _num_start = pos;

        // Integer part
        let mut integer_digits = 0;
        while pos < chars.len() && chars[pos].is_ascii_digit() {
            integer_digits += 1;
            pos += 1;
        }

        if integer_digits > self.config.max_integer_digits {
            return None;
        }

        // Decimal part
        let mut decimal_digits = 0;
        if pos < chars.len() && chars[pos] == '.' {
            pos += 1;
            while pos < chars.len() && chars[pos].is_ascii_digit() {
                decimal_digits += 1;
                pos += 1;
            }
        }

        if decimal_digits > self.config.max_decimal_digits {
            return None;
        }

        // Must have at least one digit
        if integer_digits == 0 && decimal_digits == 0 {
            return None;
        }

        // Scientific notation
        if self.config.allow_scientific && pos < chars.len() && (chars[pos] == 'e' || chars[pos] == 'E') {
            let exp_start = pos;
            pos += 1;

            // Optional sign in exponent
            if pos < chars.len() && (chars[pos] == '+' || chars[pos] == '-') {
                pos += 1;
            }

            // Must have at least one digit in exponent
            if pos >= chars.len() || !chars[pos].is_ascii_digit() {
                pos = exp_start; // Backtrack if not a valid exponent
            } else {
                while pos < chars.len() && chars[pos].is_ascii_digit() {
                    pos += 1;
                }
            }
        }

        // Build the raw number string
        let raw: String = chars[start..pos].iter().collect();

        // Parse the value
        let value: f64 = raw.parse().ok()?;

        Some((
            QuantitativeToken::new(
                TokenKind::Number { value, raw },
                start,
                pos,
            ),
            pos,
        ))
    }

    /// Post-process to combine Number + Unit tokens into Quantity tokens
    fn combine_numbers_and_units(&self, tokens: &mut Vec<QuantitativeToken>) {
        let mut i = 0;
        while i < tokens.len() {
            // Check if current is Number and next is Unit (possibly with whitespace between)
            if let TokenKind::Number { value, raw } = &tokens[i].kind {
                let value = *value;
                let raw = raw.clone();
                let start = tokens[i].start;

                // Look ahead for unit (skip whitespace if present)
                let mut next_idx = i + 1;
                while next_idx < tokens.len() {
                    if let TokenKind::Whitespace(_) = &tokens[next_idx].kind {
                        next_idx += 1;
                    } else {
                        break;
                    }
                }

                if next_idx < tokens.len() {
                    let is_unit = match &tokens[next_idx].kind {
                        TokenKind::Unit(_) => true,
                        TokenKind::Word(w) => self.is_unit(w),
                        _ => false,
                    };

                    if is_unit {
                        let unit = match &tokens[next_idx].kind {
                            TokenKind::Unit(u) => u.clone(),
                            TokenKind::Word(w) => w.clone(),
                            _ => unreachable!(),
                        };
                        let end = tokens[next_idx].end;

                        // Create combined Quantity token
                        let combined_raw = format!("{} {}", raw, unit);
                        let combined = QuantitativeToken::new(
                            TokenKind::Quantity {
                                value,
                                unit,
                                raw: combined_raw,
                            },
                            start,
                            end,
                        );

                        // Replace tokens[i] with combined, remove intermediate tokens
                        tokens[i] = combined;
                        // Remove tokens from i+1 to next_idx (inclusive)
                        tokens.drain(i + 1..=next_idx);
                    }
                }
            }
            i += 1;
        }
    }

    /// Extract only quantity tokens from input
    pub fn extract_quantities(&self, input: &str) -> Vec<(f64, String)> {
        self.tokenize(input)
            .into_iter()
            .filter_map(|t| {
                match t.kind {
                    TokenKind::Quantity { value, unit, .. } => Some((value, unit)),
                    TokenKind::Number { value, .. } => Some((value, String::new())),
                    _ => None,
                }
            })
            .collect()
    }

    /// Get tokens that are words (for passing to semantic engine)
    pub fn extract_words(&self, input: &str) -> Vec<String> {
        self.tokenize(input)
            .into_iter()
            .filter_map(|t| {
                match t.kind {
                    TokenKind::Word(w) => Some(w),
                    _ => None,
                }
            })
            .collect()
    }
}

/// Result of quantitative analysis
#[derive(Debug, Clone)]
pub struct QuantitativeAnalysis {
    /// All tokens from the input
    pub tokens: Vec<QuantitativeToken>,
    /// Extracted quantities (value, unit pairs)
    pub quantities: Vec<(f64, String)>,
    /// Words to pass to semantic engine
    pub words: Vec<String>,
    /// Whether any emergency keywords were found
    pub has_emergency: bool,
    /// Detected emergency keywords
    pub emergency_keywords: Vec<String>,
}

impl QuantitativeAnalysis {
    /// Get the number of quantities detected
    pub fn quantity_count(&self) -> usize {
        self.quantities.len()
    }

    /// Get quantity by index
    pub fn get_quantity(&self, index: usize) -> Option<(f64, &str)> {
        self.quantities.get(index).map(|(v, u)| (*v, u.as_str()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_tokenization() {
        let tokenizer = QuantitativeTokenizer::new();
        let tokens = tokenizer.tokenize("Set temperature to 15 keV");

        assert_eq!(tokens.len(), 4); // "Set", "temperature", "to", "15 keV"

        // Check that the number+unit was combined
        let quantity = tokens.iter().find(|t| t.is_quantity()).unwrap();
        assert_eq!(quantity.value(), Some(15.0));
        assert_eq!(quantity.unit(), Some("keV"));
    }

    #[test]
    fn test_scientific_notation() {
        let tokenizer = QuantitativeTokenizer::new();
        let tokens = tokenizer.tokenize("density is 1.5e20");

        let number = tokens.iter().find(|t| t.is_number()).unwrap();
        assert_eq!(number.value(), Some(1.5e20));
    }

    #[test]
    fn test_negative_numbers() {
        let tokenizer = QuantitativeTokenizer::new();
        let tokens = tokenizer.tokenize("offset -3.5 V");

        let quantity = tokens.iter().find(|t| t.is_quantity()).unwrap();
        assert_eq!(quantity.value(), Some(-3.5));
        assert_eq!(quantity.unit(), Some("V"));
    }

    #[test]
    fn test_multiple_quantities() {
        let tokenizer = QuantitativeTokenizer::new();
        let quantities = tokenizer.extract_quantities("Set T to 10 keV and I to 2.5 MA");

        assert_eq!(quantities.len(), 2);
        assert_eq!(quantities[0], (10.0, "keV".to_string()));
        assert_eq!(quantities[1], (2.5, "MA".to_string()));
    }

    #[test]
    fn test_custom_units() {
        let config = TokenizerConfig::default()
            .with_units(vec!["n20", "β_N", "q95"]);
        let tokenizer = QuantitativeTokenizer::with_config(config);

        assert!(tokenizer.is_unit("n20"));
        assert!(tokenizer.is_unit("keV")); // Default units still there
    }

    #[test]
    fn test_extract_words() {
        let tokenizer = QuantitativeTokenizer::new();
        let words = tokenizer.extract_words("Set plasma temperature to 15 keV");

        assert_eq!(words, vec!["Set", "plasma", "temperature", "to"]);
    }

    #[test]
    fn test_no_combine() {
        let config = TokenizerConfig::default().combine_quantities(false);
        let tokenizer = QuantitativeTokenizer::with_config(config);
        let tokens = tokenizer.tokenize("15 keV");

        // Should have separate Number and Unit tokens
        let number = tokens.iter().find(|t| matches!(t.kind, TokenKind::Number { .. }));
        let unit = tokens.iter().find(|t| matches!(t.kind, TokenKind::Unit(_)));

        assert!(number.is_some());
        assert!(unit.is_some());
    }

    #[test]
    fn test_preserve_whitespace() {
        let config = TokenizerConfig::default().preserve_whitespace(true);
        let tokenizer = QuantitativeTokenizer::with_config(config);
        let tokens = tokenizer.tokenize("a  b");

        // Should have: Word, Whitespace (2 spaces combined into Quantity with b? No, "b" is not a unit)
        // Actually: Word("a"), Whitespace("  "), Word("b")
        assert!(tokens.iter().any(|t| matches!(t.kind, TokenKind::Whitespace(_))));
    }
}
