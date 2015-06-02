pub struct Node {
    pub left: Link,
    pub right: Link,
    pub value: u32
}

pub enum Link {
    Child(Box<Node>),
    Nil
}

fn is_valid_range(node: &Node, min: Option<u32>, max: Option<u32>) -> bool {
    if let Some(m) = min {
        if node.value <= m {
            return false;
        }
    }

    if let Some(m) = max {
        if node.value >= m {
            return false;
        }
    }

    if let Link::Child(ref left) = node.left {
        if !is_valid_range(&left, min, Some(node.value)) {
            return false;
        }
    }
    if let Link::Child(ref right) = node.right {
        if !is_valid_range(&right, Some(node.value), max) {
            return false;
        }
    }
    true
}

/// Return true if the given binary search tree is valid.
///
/// # Examples
///
/// ```
/// use validbtree::*;
///
/// let n3 = Node{left: Link::Nil, right: Link::Nil, value: 3};
/// let n7 = Node{left: Link::Nil, right: Link::Nil, value: 7};
///
/// let n5 = Node {left: Link::Child(Box::new(n7)),
///                right: Link::Child(Box::new(n3)),
///                value: 5};
/// assert!(!is_valid(&n5));
/// ```
pub fn is_valid(node: &Node) -> bool {
    is_valid_range(node, None, None)
}

#[test]
fn root_only() {
    assert!(is_valid(&Node { left: Link::Nil, right: Link::Nil, value: 25}));
}

#[test]
fn valid_tree() {
    //                   9
    //                  / \
    //                 5   23
    //                / \   \
    //               3   7   25
    let n3 = Node{left: Link::Nil, right: Link::Nil, value: 3};
    let n7 = Node{left: Link::Nil, right: Link::Nil, value: 7};
    let n25 = Node{left: Link::Nil, right: Link::Nil, value: 25};

    let n5 = Node{left: Link::Child(Box::new(n3)),
                  right: Link::Child(Box::new(n7)),
                  value: 5};
    let n23 = Node{left: Link::Nil, 
                   right: Link::Child(Box::new(n25)),
                   value: 23};

    let n9 = Node{left: Link::Child(Box::new(n5)),
                  right: Link::Child(Box::new(n23)),
                  value: 9};

    assert!(is_valid(&n9));
}

#[test]
fn leaf_in_left_subtree_greater_than_root() {
    //                   9
    //                  / \
    //                 5   23
    //                / \   \
    //               3  11   25
    let n3 = Node{left: Link::Nil, right: Link::Nil, value: 3};
    let n11 = Node{left: Link::Nil, right: Link::Nil, value: 11};
    let n25 = Node{left: Link::Nil, right: Link::Nil, value: 25};

    let n5 = Node{left: Link::Child(Box::new(n3)),
                  right: Link::Child(Box::new(n11)),
                  value: 5};
    let n23 = Node{left: Link::Nil, 
                   right: Link::Child(Box::new(n25)),
                   value: 23};

    let n9 = Node{left: Link::Child(Box::new(n5)),
                  right: Link::Child(Box::new(n23)),
                  value: 9};

    assert!(!is_valid(&n9));
}

#[test]
fn duplicate_in_left_sub_tree() {
    //                   9
    //                  /
    //                 9
    let leaf = Node{left: Link::Nil, right: Link::Nil, value: 9};
    let root = Node{left: Link::Child(Box::new(leaf)), 
                    right: Link::Nil,
                    value: 9};
        assert!(!is_valid(&root));
}

#[test]
fn duplicate_in_right_sub_tree() {
    //               9 
    //                \
    //                 9
    let leaf = Node{left: Link::Nil, right: Link::Nil, value: 9};
    let root = Node{left: Link::Nil, 
                    right: Link::Child(Box::new(leaf)),
                    value: 9};
        assert!(!is_valid(&root));
}