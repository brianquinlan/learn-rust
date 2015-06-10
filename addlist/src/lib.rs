#[derive(Debug)]
pub struct Node {
    pub value: u8,
    pub next: Link,
}

pub type Link = Option<Box<Node>>;

impl PartialEq for Node {
    fn eq(&self, other: &Node) -> bool {
        self.value == other.value && match self.next {
            None => other.next.is_none(),
            Some(ref node) => match other.next {
                None => false,
                Some(ref n2) => node == n2,
            },
        }
    }
}

/// Increment an integer whose digits are represented as seperate nodes in
/// a linked list. Return whether the increment resulted in overflow.
///
/// # Examples
///
/// ```
/// use addlist::{Node, increment};
///
/// let mut node = Node{
///     value: 1,
///     next: Some(Box::new(Node{value: 9, next: None}))};
/// assert!(!increment(&mut node));
/// assert_eq!(
///     node,
///     Node{value: 2, next: Some(Box::new(Node{value: 0, next: None}))});
/// ```
#[inline]
pub fn increment(n: &mut Node) -> bool {
    if let Some(ref mut node) = n.next {
        if increment(&mut *node) {
            n.value += 1;
        }

    } else {
        n.value += 1;
    }

    if n.value == 10 {
        n.value = 0;
        return true;
    }
    false
}

#[test]
fn eq_empty() {
    assert!(Node{value: 1, next: None} == Node{value: 1, next: None});
    assert!(Node{value: 1, next: None} != Node{value: 2, next: None});
}

#[test]
fn eq_left_empty() {
    assert!(Node{value: 1, next: None} !=
            Node{value: 1, next: Some(Box::new(Node{value: 9, next: None}))});
}

#[test]
fn eq_right_empty() {
    assert!(Node{value: 1, next: Some(Box::new(Node{value: 9, next: None}))} !=
            Node{value: 1, next: None});
}

#[test]
fn eq_some_next() {
    assert!(Node{value: 1, next: Some(Box::new(Node{value: 1, next: None}))} ==
            Node{value: 1, next: Some(Box::new(Node{value: 1, next: None}))});
    assert!(Node{value: 1, next: Some(Box::new(Node{value: 1, next: None}))} !=
            Node{value: 1, next: Some(Box::new(Node{value: 2, next: None}))});
}

#[test]
fn increment_0() {
    let mut node = Node{value: 0, next: None};
    assert!(!increment(&mut node));
    assert_eq!(Node{value: 1, next: None}, node);
}

#[test]
fn increment_9() {
    let mut node = Node{value: 9, next: None};
    assert!(increment(&mut node));
    assert_eq!(Node{value: 0, next: None}, node);
}

#[test]
fn increment_099() {
    let mut node = Node{value: 0, next: Some(Box::new(
        Node{value: 9, next: Some(Box::new(
            Node{value: 9, next: None}))}))};
    assert!(!increment(&mut node));
    assert_eq!(
        Node{value: 1,
             next: Some(Box::new(
                Node{value: 0, next: Some(Box::new(
                    Node{value: 0, next: None}))}))}, node);

}

#[test]
fn increment_123() {
    let mut node = Node{value: 1, next: Some(Box::new(
        Node{value: 2, next: Some(Box::new(
            Node{value: 3, next: None}))}))};
    assert!(!increment(&mut node));
    assert_eq!(
        Node{value: 1,
             next: Some(Box::new(
                Node{value: 2, next: Some(Box::new(
                    Node{value: 4, next: None}))}))}, node);

}

#[test]
fn increment_999() {
    let mut node = Node{value: 9, next: Some(Box::new(
        Node{value: 9, next: Some(Box::new(
            Node{value: 9, next: None}))}))};
    assert!(increment(&mut node));
    assert_eq!(
        Node{value: 0,
             next: Some(Box::new(
                Node{value: 0, next: Some(Box::new(
                    Node{value: 0, next: None}))}))}, node);

}
