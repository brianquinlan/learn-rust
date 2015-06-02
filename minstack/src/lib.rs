//! A stack type with heap-allocated contents.
//!
//! MinStacks have amortized `O(1)` push, pop and min (return the minimum value
//! in the stack).

use std::cmp::PartialEq;
use std::cmp::PartialOrd;

pub struct MinStack<T> {
    stack: Vec<T>,
    // Maintain a parallel stack of minimum values.
    // Whenever a value is pushed to `stack`, also push it to `min_stack` if
    // the value is <= the value at the top of `min_stack`.
    // Whenever a value is popped from `stack`, also pop `min_stack` if the 
    // value == the value at the top of `min_stack`.
    min_stack: Vec<T>
}

impl<T : Copy + PartialEq + PartialOrd> MinStack<T> {
    /// Constructs a new, empty `MinStack<T>`.
    ///
    /// # Examples
    ///
    /// ```
    /// use minstack::MinStack;
    ///
    /// let mut stack: MinStack<i32> = MinStack::new();
    /// ```
    #[inline]
    pub fn new() -> MinStack<T> {
        MinStack {
            stack: Vec::new(),
            min_stack: Vec::new()
        }
    }

    /// Appends an element to end of the stack.
    ///
    /// # Panics
    ///
    /// Panics if the number of elements in the vector overflows a `usize`.
    ///
    /// # Examples
    ///
    /// ```
    /// use minstack::MinStack;
    ///
    /// let mut stack = MinStack::new();
    /// stack.push(3);
    /// assert_eq!(Some(3), stack.pop());
    /// ```
    #[inline]
    pub fn push(&mut self, value: T) {
        self.stack.push(value);
        match self.min_stack.last() {
            Some(min) if *min < value => { return; },
            _ => { }
        }
        self.min_stack.push(value);
    }

    #[inline]
    fn should_pop_min(&self, popped_value: &Option<T>) -> bool {
        match *popped_value {
            Some(v) => match self.min_stack.last() {
                Some(min) => *min == v,
                None => false
            },
            None => false
        }
    }

    /// Removes the last element from the stack and returns it, or `None` if it
    /// is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use minstack::MinStack;
    ///
    /// let mut stack = MinStack::new();
    /// stack.push(3);
    /// assert_eq!(Some(3), stack.pop());
    /// assert_eq!(None, stack.pop());
    /// ```
    #[inline]
    pub fn pop(&mut self) -> Option<T> {
        let value = self.stack.pop();

        if self.should_pop_min(&value) {
            self.min_stack.pop();
        }
        value
    }

    /// Returns the smallest element in the stack, or `None` if it is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use minstack::MinStack;
    ///
    /// let mut stack = MinStack::new();
    /// assert_eq!(None, stack.min());
    /// stack.push(3);
    /// stack.push(1);
    /// stack.push(83);
    /// assert_eq!(Some(&1), stack.min());
    /// ```
    #[inline]
    #[inline]
    pub fn min(&self) -> Option<&T> {
        return self.min_stack.last();
    }
}

#[test]
fn empty_stack() {
    let mut stack : MinStack<i32> = MinStack::new();
    assert_eq!(None, stack.min());
    assert_eq!(None, stack.pop());
}

#[test]
fn ascending_push() {
    let mut stack : MinStack<i32> = MinStack::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    assert_eq!(Some(&1), stack.min());
    assert_eq!(Some(3), stack.pop());
    assert_eq!(Some(&1), stack.min());
    assert_eq!(Some(2), stack.pop());
    assert_eq!(Some(&1), stack.min());
    assert_eq!(Some(1), stack.pop());
    assert_eq!(None, stack.min());
    assert_eq!(None, stack.pop());
}

#[test]
fn decending_push() {
    let mut stack : MinStack<i32> = MinStack::new();
    stack.push(3);
    stack.push(2);
    stack.push(1);
    assert_eq!(Some(&1), stack.min());
    assert_eq!(Some(1), stack.pop());
    assert_eq!(Some(&2), stack.min());
    assert_eq!(Some(2), stack.pop());
    assert_eq!(Some(&3), stack.min());
    assert_eq!(Some(3), stack.pop());
    assert_eq!(None, stack.min());
    assert_eq!(None, stack.pop());
}

#[test]
fn duplicate_push() {
    let mut stack : MinStack<i32> = MinStack::new();
    stack.push(1);
    stack.push(2);
    stack.push(1);
    assert_eq!(Some(&1), stack.min());
    assert_eq!(Some(1), stack.pop());
    assert_eq!(Some(&1), stack.min());
    assert_eq!(Some(2), stack.pop());
    assert_eq!(Some(&1), stack.min());
    assert_eq!(Some(1), stack.pop());
    assert_eq!(None, stack.min());
    assert_eq!(None, stack.pop());
}
