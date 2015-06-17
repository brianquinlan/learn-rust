//! A stack type with heap-allocated contents.
//!
//! MinStacks have amortized `O(1)` push, pop and min (return the minimum value
//! in the stack).

pub struct MinStack<T> {
    stack: Vec<T>,
    // Maintain a parallel stack of locations of minimum values.
    // Whenever a value is pushed to `stack`, also push its location to `min_stack` if
    // the value is <= the value at the top of `min_stack`.
    // Whenever a value is popped from `stack`, also pop `min_stack` if the 
    // position == the value at the top of `min_stack`.
    min_stack: Vec<usize>
}

impl<T : Ord> MinStack<T> {
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
        let idx = self.stack.len();

        match self.min_stack.last() {
            Some(&min) if value <= self.stack[min] =>
                self.min_stack.push(idx),
            None =>
                self.min_stack.push(idx),
            _ => { }
        }

        self.stack.push(value);
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

        match self.min_stack.last() {
            Some(&min) if min == self.stack.len() => {
                self.min_stack.pop();
            }
            _ => { }
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
    pub fn min(&self) -> Option<&T> {
        self.min_stack.last().map(|&n| &self.stack[n])
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
