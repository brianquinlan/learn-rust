/// Return any two numbers found in a `Vec` that can be added up to a target, or
// `None` if no such numbers exist.
///
/// # Examples
///
/// ```
/// use targetsum::target_sum;
///
/// assert_eq!(Some((3, 7)), target_sum(&vec![1, 2, 3, 5, 7, 11, 13], 10));
/// assert_eq!(None, target_sum(&vec![1, 2, 3, 5, 7, 11, 13], 11));
/// ```
pub fn target_sum(search: &[i32], target: i32) -> Option<(i32, i32)> {
    if search.len() < 2 {
        return None;
    }
    // For some reason hash tables are not allowed so sort the vector and
    // search from the ends.
    let mut sorted_search = search.to_vec();
    sorted_search.sort();
    let mut lowest_index = 0;
    let mut highest_index = sorted_search.len() - 1;

    while lowest_index < highest_index {
        let low = sorted_search[lowest_index];
        let high = sorted_search[highest_index];

        if low + high == target {
            return Some((low, high));
        } else if low + high > target {
            highest_index -= 1;
        } else {
            lowest_index += 1;
        }
    }
    None
}

#[test]
fn search_vector_empty() {
    assert_eq!(None, target_sum(&vec![], 11));
}

#[test]
fn search_vector_size_one() {
    assert_eq!(None, target_sum(&vec![5], 9));
    assert_eq!(None, target_sum(&vec![5], 5));
    assert_eq!(None, target_sum(&vec![5], 10));
}

#[test]
fn target_at_edges() {
    assert_eq!(Some((1, 10)), target_sum(&vec![1, 2, 5, 10], 11));
}

#[test]
fn target_same_number() {
    assert_eq!(Some((2, 2)), target_sum(&vec![1, 2, 2, 5, 10], 4));
}

#[test]
fn target_same_number_appears_once() {
    assert_eq!(None, target_sum(&vec![1, 2, 5, 10], 2));
}

#[test]
fn target_double_number() {
    assert_eq!(None, target_sum(&vec![1, 2, 5, 10], 4));
}

#[test]
fn target_not_found() {
    assert_eq!(None, target_sum(&vec![1, 2, 3, 5, 7, 13, 17, 19, 23], 11));
}

#[test]
fn negative_target() {
    assert_eq!(
        Some((-23, 23)),
        target_sum(&vec![1, 2, 3, 5, 7, 13, 17, 19, 23, 29, 31, 37, -23], 0));
}

