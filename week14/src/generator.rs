#![allow(dead_code, unused_imports)]
use rand::Rng;

pub fn generate_random(length: usize, use_symbols: bool) -> String {
    if length == 0 {
        panic!("Length cannot be 0");
    }

    let mut charset =
        String::from("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789");
    if use_symbols {
        charset.push_str("!@#$%^&*");
    }

    let mut rng = rand::thread_rng();
    (0..length)
        .map(|_| {
            let idx = rng.gen_range(0..charset.len());
            charset.chars().nth(idx).unwrap()
        })
        .collect()
}

pub fn generate_passphrase(word_count: usize, separator: char) -> String {
    let mut rng = rand::thread_rng();
    let mut words = Vec::new();

    for _ in 0..word_count {
        let idx = rng.gen_range(0..WORD_LIST.len());
        words.push(WORD_LIST[idx]);
    }

    words.join(&separator.to_string())
}

pub fn generate_pin(length: usize) -> String {
    if length == 0 {
        panic!("Length cannot be 0");
    }
    let mut rng = rand::thread_rng();
    (0..length)
        .map(|_| rng.gen_range(0..10).to_string())
        .collect()
}

pub const WORD_LIST: &[&str] = &[
    "apple", "river", "cloud", "stone", "flame", "ocean", "tiger", "maple", "storm", "frost",
    "eagle", "cedar", "brook", "ember", "coral", "prism", "solar", "lunar", "amber", "blaze",
    "cliff", "delta", "fable", "grove", "haven", "ivory", "jewel", "knoll", "lemon", "meadow",
    "north", "olive", "pearl", "quill", "ridge", "spark", "thorn", "umbra", "valor", "willow",
    "xenon", "yarrow", "zenith", "acorn", "birch", "crane", "drift", "elder", "flint", "glade",
    "hyena", "inlet", "junco", "kestrel",
];

// ... (Tests remain same as original)
// ============================================================================
// TESTS — DO NOT MODIFY
// ============================================================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_random_correct_length() {
        let pwd = generate_random(12, false);
        assert_eq!(pwd.len(), 12);
    }

    #[test]
    fn test_random_no_symbols_only_alphanumeric() {
        let pwd = generate_random(100, false);
        assert!(pwd.chars().all(|c| c.is_ascii_alphanumeric()));
    }

    #[test]
    fn test_random_with_symbols_contains_valid_chars() {
        let valid: std::collections::HashSet<char> =
            "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!@#$%^&*"
                .chars()
                .collect();
        let pwd = generate_random(100, true);
        assert!(pwd.chars().all(|c| valid.contains(&c)));
    }

    #[test]
    fn test_passphrase_word_count() {
        let phrase = generate_passphrase(4, '-');
        assert_eq!(phrase.split('-').count(), 4);
    }

    #[test]
    fn test_passphrase_separator() {
        let phrase = generate_passphrase(3, '_');
        assert!(phrase.contains('_'));
    }

    #[test]
    fn test_pin_only_digits() {
        let pin = generate_pin(20);
        assert!(pin.chars().all(|c| c.is_ascii_digit()));
    }
}
