//! # Character Processing Module
//!
//! Character-level similarity and processing.

/// Calculate character similarity between two strings
pub fn char_similarity(a: &str, b: &str) -> f64 {
    if a == b { return 1.0; }
    if a.is_empty() || b.is_empty() { return 0.0; }

    let a_chars: Vec<char> = a.chars().collect();
    let b_chars: Vec<char> = b.chars().collect();

    let matches = a_chars.iter()
        .zip(b_chars.iter())
        .filter(|(ca, cb)| ca == cb)
        .count();

    let max_len = a_chars.len().max(b_chars.len());
    matches as f64 / max_len as f64
}

/// Normalize string for comparison
pub fn normalize(s: &str) -> String {
    s.to_lowercase()
        .chars()
        .filter(|c| c.is_alphanumeric())
        .collect()
}

/// Check if strings are similar (> threshold)
pub fn is_similar(a: &str, b: &str, threshold: f64) -> bool {
    char_similarity(&normalize(a), &normalize(b)) >= threshold
}

/// Soundex code for phonetic matching
pub fn soundex(word: &str) -> String {
    if word.is_empty() { return String::new(); }

    let word = word.to_uppercase();
    let chars: Vec<char> = word.chars().collect();

    let mut result = String::new();
    result.push(chars[0]);

    let code = |c: char| -> Option<char> {
        match c {
            'B' | 'F' | 'P' | 'V' => Some('1'),
            'C' | 'G' | 'J' | 'K' | 'Q' | 'S' | 'X' | 'Z' => Some('2'),
            'D' | 'T' => Some('3'),
            'L' => Some('4'),
            'M' | 'N' => Some('5'),
            'R' => Some('6'),
            _ => None,
        }
    };

    let mut last_code = code(chars[0]);
    for &c in &chars[1..] {
        let current = code(c);
        if current.is_some() && current != last_code {
            if let Some(digit) = current {
                result.push(digit);
            }
        }
        if current.is_some() {
            last_code = current;
        }
    }

    while result.len() < 4 {
        result.push('0');
    }

    result.chars().take(4).collect()
}

/// Metaphone code (simplified)
pub fn metaphone(word: &str) -> String {
    let word = word.to_uppercase();
    let mut result = String::new();

    let chars: Vec<char> = word.chars().collect();
    let mut i = 0;

    while i < chars.len() && result.len() < 6 {
        let c = chars[i];
        let next = chars.get(i + 1).copied();

        match c {
            'A' | 'E' | 'I' | 'O' | 'U' if i == 0 => result.push(c),
            'B' => result.push('B'),
            'C' => {
                if next == Some('H') {
                    result.push('X');
                    i += 1;
                } else if matches!(next, Some('I') | Some('E') | Some('Y')) {
                    result.push('S');
                } else {
                    result.push('K');
                }
            }
            'D' => result.push('T'),
            'F' => result.push('F'),
            'G' => result.push('K'),
            'H' => result.push('H'),
            'J' => result.push('J'),
            'K' => result.push('K'),
            'L' => result.push('L'),
            'M' => result.push('M'),
            'N' => result.push('N'),
            'P' => {
                if next == Some('H') {
                    result.push('F');
                    i += 1;
                } else {
                    result.push('P');
                }
            }
            'Q' => result.push('K'),
            'R' => result.push('R'),
            'S' => result.push('S'),
            'T' => {
                if next == Some('H') {
                    result.push('0'); // 'TH' sound
                    i += 1;
                } else {
                    result.push('T');
                }
            }
            'V' => result.push('F'),
            'W' => result.push('W'),
            'X' => { result.push('K'); result.push('S'); }
            'Y' => result.push('Y'),
            'Z' => result.push('S'),
            _ => {}
        }

        i += 1;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_char_similarity() {
        assert_eq!(char_similarity("hello", "hello"), 1.0);
        assert!(char_similarity("hello", "hallo") > 0.7);
        assert!(char_similarity("cat", "dog") < 0.5);
    }

    #[test]
    fn test_soundex() {
        assert_eq!(soundex("Robert"), "R163");
        assert_eq!(soundex("Rupert"), "R163");
    }

    #[test]
    fn test_normalize() {
        assert_eq!(normalize("Hello, World!"), "helloworld");
    }
}
