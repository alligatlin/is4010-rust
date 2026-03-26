// Week 10: Mastering ownership and borrowing
//
// This lab has two parts:
//
// PART 1 — Borrow checker puzzles (7 problems)
// PART 2 — Implementation exercises (5 functions)
//
// Run: cargo test

fn main() {
    println!("Week 10: Mastering ownership and borrowing");
    println!("Uncomment one problem at a time and fix it!\n");

    // All problems are now fixed and uncommented:
    problem_1();
    problem_2();
    problem_3();
    problem_4();
    problem_5();
    problem_6();
    problem_7();
}

// ============================================================================
// PROBLEM 1: Value used after move
// ============================================================================
fn problem_1() {
    println!("Problem 1: Value used after move");
    let s1 = String::from("hello");
    // Fix: We pass a reference (&s1) so s1 isn't "eaten" by the function.
    let len = calculate_length(&s1);
    println!("  The length of '{}' is {}.", s1, len);
}

// Clippy Fix: Use &str instead of &String for better flexibility (ptr_arg)
fn calculate_length(s: &str) -> usize {
    s.len()
}

// ============================================================================
// PROBLEM 2: Immutable and mutable borrow conflict
// ============================================================================
fn problem_2() {
    println!("Problem 2: Mutable and immutable borrow conflict");
    let mut s = String::from("hello");
    let r1 = &s;
    println!("  r1: {}", r1); // Fix: Use r1 here so its lifetime ends before r2 starts.

    let r2 = &mut s;
    println!("  r2: {}", r2);
}

// ============================================================================
// PROBLEM 3: Mutating through an immutable reference
// ============================================================================
fn problem_3() {
    println!("Problem 3: Mutating through an immutable reference");
    let mut s = String::from("hello"); // Fix: s must be mut
    add_to_string(&mut s); // Fix: pass a mutable reference
    println!("  Result: {}", s);
}

fn add_to_string(s: &mut String) {
    // Fix: take a mutable reference
    s.push_str(", world");
}

// ============================================================================
// PROBLEM 4: Multiple mutable borrows
// ============================================================================
fn problem_4() {
    println!("Problem 4: Multiple mutable borrows");
    let mut s = String::from("hello");

    {
        let r1 = &mut s;
        println!("  r1: {}", r1);
    } // Fix: r1 goes out of scope here, so s is "unlocked" for r2.

    let r2 = &mut s;
    println!("  r2: {}", r2);
}

// ============================================================================
// PROBLEM 5: Dangling reference
// ============================================================================
fn problem_5() {
    println!("Problem 5: Dangling reference");
    let r = create_string();
    println!("  Got: {}", r);
}

fn create_string() -> String {
    // Clippy Fix: Return the expression directly (let_and_return)
    String::from("hello")
}

// ============================================================================
// PROBLEM 6: Ownership in loops
// ============================================================================
fn problem_6() {
    println!("Problem 6: Ownership in loops");
    let data = String::from("Rust");

    for i in 0..3 {
        print_with_number(&data, i); // Fix: Pass a reference so data isn't moved.
    }
}

fn print_with_number(s: &String, n: i32) {
    // Fix: Take a reference.
    println!("  {}: {}", n, s);
}

// ============================================================================
// PROBLEM 7: Lifetime — reference doesn't live long enough
// ============================================================================
fn problem_7() {
    println!("Problem 7: Lifetime extension");
    let s = String::from("inner scope"); // Fix: Define s outside so it outlives the block.
    let result;
    {
        result = &s;
    }
    println!("  Result: {}", result);
}

// ============================================================================
// PART 2 — Implementation exercises
// ============================================================================

pub fn to_uppercase_owned(s: String) -> String {
    s.to_uppercase()
}

// Clippy Fix: Use &str instead of &String for parameter type
pub fn string_length(s: &str) -> usize {
    s.len()
}

pub fn append_suffix(s: &mut String, suffix: &str) {
    s.push_str(suffix);
}

pub fn concat_strings(s1: &str, s2: &str) -> String {
    format!("{}{}", s1, s2)
}

// ============================================================================
// TESTS — DO NOT MODIFY
// ============================================================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_uppercase_owned() {
        let s = String::from("hello");
        let result = to_uppercase_owned(s);
        assert_eq!(result, "HELLO");
    }

    #[test]
    fn test_to_uppercase_owned_already_upper() {
        let s = String::from("RUST");
        assert_eq!(to_uppercase_owned(s), "RUST");
    }

    #[test]
    fn test_string_length() {
        let s = String::from("testing");
        let len = string_length(&s);
        assert_eq!(len, 7);
        assert_eq!(s, "testing");
    }

    #[test]
    fn test_string_length_empty() {
        let s = String::from("");
        assert_eq!(string_length(&s), 0);
    }

    #[test]
    fn test_append_suffix() {
        let mut s = String::from("hello");
        append_suffix(&mut s, ", world");
        assert_eq!(s, "hello, world");
    }

    #[test]
    fn test_append_suffix_empty() {
        let mut s = String::from("");
        append_suffix(&mut s, "hi");
        assert_eq!(s, "hi");
    }

    #[test]
    fn test_concat_strings() {
        let result = concat_strings("hello", " world");
        assert_eq!(result, "hello world");
    }

    #[test]
    fn test_concat_strings_empty() {
        assert_eq!(concat_strings("", "abc"), "abc");
        assert_eq!(concat_strings("abc", ""), "abc");
    }
}
