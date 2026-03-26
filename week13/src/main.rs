// Week 13: Idiomatic Rust
use std::fmt;

fn main() {
    println!("Week 13: Idiomatic Rust");
}

// ============================================================================
// PART 1: Iterators and closures
// ============================================================================

pub fn analyze_text(text: &str) -> (usize, f64, String) {
    let words: Vec<&str> = text.split_whitespace().collect();

    if words.is_empty() {
        return (0, 0.0, String::new());
    }

    let word_count = words.len();

    // Sum up lengths of all words
    let total_chars: usize = words.iter().map(|w| w.len()).sum();
    let average_word_length = total_chars as f64 / word_count as f64;

    // Find the longest word. If multiple have the same length, max_by_key returns the last,
    // but the test accepts any of the longest.
    let longest_word = words
        .iter()
        .max_by_key(|w| w.len())
        .unwrap_or(&"")
        .to_string();

    (word_count, average_word_length, longest_word)
}

pub fn process_numbers(numbers: &[i32]) -> i32 {
    numbers
        .iter()
        .filter(|&&n| n % 2 == 0) // Keep only evens
        .map(|n| n * n) // Square them
        .sum() // Add them up
}

pub fn make_counter() -> impl FnMut() -> i32 {
    let mut count = 0;
    move || {
        count += 1;
        count
    }
}

// ============================================================================
// PART 2: Error handling with Result
// ============================================================================

pub fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err(String::from("division by zero"))
    } else {
        Ok(a / b)
    }
}

#[derive(Debug, PartialEq)]
pub enum ParseError {
    NotANumber,
    NotPositive,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::NotANumber => {
                write!(f, "The input string could not be parsed as an integer.")
            }
            ParseError::NotPositive => write!(f, "The parsed number is zero or negative."),
        }
    }
}

pub fn parse_positive_number(input: &str) -> Result<i32, ParseError> {
    // Attempt to parse to i32, map the error to our custom enum if it fails
    let num = input.parse::<i32>().map_err(|_| ParseError::NotANumber)?;

    if num > 0 {
        Ok(num)
    } else {
        Err(ParseError::NotPositive)
    }
}

// ============================================================================
// TESTS — DO NOT MODIFY
// ============================================================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_analyze_text_basic() {
        let (count, avg, longest) = analyze_text("hello world rust");
        assert_eq!(count, 3);
        assert!((avg - 14.0 / 3.0).abs() < 1e-9);
        assert_eq!(longest.len(), 5); // accept "hello" or "world"
    }

    #[test]
    fn test_analyze_text_empty() {
        let (count, avg, longest) = analyze_text("");
        assert_eq!(count, 0);
        assert_eq!(avg, 0.0);
        assert_eq!(longest, "");
    }

    #[test]
    fn test_analyze_text_single_word() {
        let (count, avg, longest) = analyze_text("Rust");
        assert_eq!(count, 1);
        assert_eq!(avg, 4.0);
        assert_eq!(longest, "Rust");
    }

    #[test]
    fn test_process_numbers_mixed() {
        assert_eq!(process_numbers(&[1, 2, 3, 4]), 20);
    }

    #[test]
    fn test_process_numbers_all_odd() {
        assert_eq!(process_numbers(&[1, 3, 5]), 0);
    }

    #[test]
    fn test_process_numbers_empty() {
        assert_eq!(process_numbers(&[]), 0);
    }

    #[test]
    fn test_process_numbers_negative_evens() {
        assert_eq!(process_numbers(&[-2, -1, 4]), 20);
    }

    #[test]
    fn test_make_counter_increments() {
        let mut counter = make_counter();
        assert_eq!(counter(), 1);
        assert_eq!(counter(), 2);
        assert_eq!(counter(), 3);
    }

    #[test]
    fn test_make_counter_independent() {
        let mut c1 = make_counter();
        let mut c2 = make_counter();
        assert_eq!(c1(), 1);
        assert_eq!(c1(), 2);
        assert_eq!(c2(), 1);
    }

    #[test]
    fn test_divide_ok() {
        assert_eq!(divide(10.0, 2.0), Ok(5.0));
    }

    #[test]
    fn test_divide_by_zero() {
        assert!(divide(5.0, 0.0).is_err());
    }

    #[test]
    fn test_divide_negative() {
        assert_eq!(divide(-9.0, 3.0), Ok(-3.0));
    }

    #[test]
    fn test_parse_positive_number_ok() {
        assert_eq!(parse_positive_number("42"), Ok(42));
        assert_eq!(parse_positive_number("1"), Ok(1));
    }

    #[test]
    fn test_parse_positive_number_not_a_number() {
        assert_eq!(parse_positive_number("abc"), Err(ParseError::NotANumber));
        assert_eq!(parse_positive_number(""), Err(ParseError::NotANumber));
    }

    #[test]
    fn test_parse_positive_number_not_positive() {
        assert_eq!(parse_positive_number("0"), Err(ParseError::NotPositive));
        assert_eq!(parse_positive_number("-5"), Err(ParseError::NotPositive));
    }

    #[test]
    fn test_parse_error_display() {
        let msg = format!("{}", ParseError::NotANumber);
        assert!(!msg.is_empty());
        let msg2 = format!("{}", ParseError::NotPositive);
        assert!(!msg2.is_empty());
    }
}
