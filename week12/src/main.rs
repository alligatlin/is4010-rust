// Week 12: Generics and traits
use std::fmt;

fn main() {
    println!("Week 12: Generics and traits");

    // Try it out once you have implementations working:
    let mut s: Stack<i32> = Stack::new();
    s.push(1);
    s.push(2);
    s.push(3);
    println!("Stack display: {}", s); // requires Display impl

    // Basic peek/pop check
    if let Some(top) = s.peek() {
        println!("Top item is: {}", top);
    }
}

// ============================================================================
// STACK<T> — implementation
// ============================================================================

/// A generic last-in, first-out (LIFO) stack.
#[allow(dead_code)]
pub struct Stack<T> {
    data: Vec<T>,
}

#[allow(clippy::new_without_default)]
impl<T> Stack<T> {
    /// Creates a new, empty stack.
    pub fn new() -> Self {
        Stack { data: Vec::new() }
    }

    /// Pushes `item` onto the top of the stack.
    pub fn push(&mut self, item: T) {
        self.data.push(item);
    }

    /// Removes and returns the top item, or `None` if the stack is empty.
    pub fn pop(&mut self) -> Option<T> {
        self.data.pop()
    }

    /// Returns a reference to the top item without removing it.
    pub fn peek(&self) -> Option<&T> {
        self.data.last()
    }

    /// Returns `true` if the stack contains no items.
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Returns the number of items in the stack.
    pub fn len(&self) -> usize {
        self.data.len()
    }
}

// ============================================================================
// DISPLAY — format the stack as "[bottom, ..., top]"
// ============================================================================
impl<T: fmt::Debug> fmt::Display for Stack<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // We can leverage the Debug implementation of Vec, but the
        // instructions ask specifically for "[1, 2, 3]" format.
        write!(f, "{:?}", self.data)
    }
}

// ============================================================================
// TESTS — DO NOT MODIFY
// ============================================================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_stack_is_empty() {
        let s: Stack<i32> = Stack::new();
        assert!(s.is_empty());
        assert_eq!(s.len(), 0);
    }

    #[test]
    fn test_push_increases_len() {
        let mut s = Stack::new();
        s.push(1);
        assert_eq!(s.len(), 1);
        s.push(2);
        assert_eq!(s.len(), 2);
    }

    #[test]
    fn test_pop_returns_lifo_order() {
        let mut s = Stack::new();
        s.push(1);
        s.push(2);
        s.push(3);
        assert_eq!(s.pop(), Some(3));
        assert_eq!(s.pop(), Some(2));
        assert_eq!(s.pop(), Some(1));
        assert_eq!(s.pop(), None);
    }

    #[test]
    fn test_pop_empty_stack() {
        let mut s: Stack<i32> = Stack::new();
        assert_eq!(s.pop(), None);
    }

    #[test]
    fn test_peek_does_not_remove() {
        let mut s = Stack::new();
        s.push(42);
        assert_eq!(s.peek(), Some(&42));
        assert_eq!(s.len(), 1);
    }

    #[test]
    fn test_peek_empty_stack() {
        let s: Stack<i32> = Stack::new();
        assert_eq!(s.peek(), None);
    }

    #[test]
    fn test_is_empty_after_pop() {
        let mut s = Stack::new();
        s.push(1);
        s.pop();
        assert!(s.is_empty());
    }

    #[test]
    fn test_stack_of_strings() {
        let mut s = Stack::new();
        s.push(String::from("hello"));
        s.push(String::from("world"));
        assert_eq!(s.pop(), Some(String::from("world")));
    }

    #[test]
    fn test_stack_of_floats() {
        let mut s = Stack::new();
        s.push(1.1_f64);
        s.push(2.2_f64);
        assert_eq!(s.len(), 2);
    }

    #[test]
    fn test_display_empty() {
        let s: Stack<i32> = Stack::new();
        assert_eq!(format!("{}", s), "[]");
    }

    #[test]
    fn test_display_with_items() {
        let mut s = Stack::new();
        s.push(1);
        s.push(2);
        s.push(3);
        assert_eq!(format!("{}", s), "[1, 2, 3]");
    }
}
