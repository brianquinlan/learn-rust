/// Reverse the items in the list from [0..index].
#[inline]
fn reverse_until_index<T>(v: &mut Vec<T>, index: usize) {
    let (mut first, _) = v.split_at_mut(index+1);
    first.reverse();
}

/// Return the index of the largest item in the Vec between [0..index].
#[inline]
fn index_of_max<T>(v: &Vec<T>, index: usize) -> usize where T: PartialOrd {
    let mut max = &v[0];
    let mut max_index = 0;
    for i in 0..(index+1) {
        if v[i] > *max {
            max = &v[i];
            max_index = i;
        }
    }
    max_index
}

/// Sort the Vec using only `reverse_until_index` to exchange elements.
/// See: http://en.wikipedia.org/wiki/Pancake_sorting
///
/// # Examples
///
/// ```
/// use pancake::pancake_sort;
///
/// let mut v = vec![5, 1, 3, 4, 8, 2, 1, 9, 7];
/// pancake_sort(&mut v);
/// assert_eq!(vec![1, 1, 2, 3, 4, 5, 7, 8, 9], v);
/// ```
#[inline]
pub fn pancake_sort<T>(v: &mut Vec<T>) where T: PartialOrd {
    if v.len() == 0 {
        return;
    }
    let mut index = v.len() - 1;

    while index > 0 {
        let max_index = index_of_max(&v, index);
        if max_index != index {
            reverse_until_index(&mut *v, max_index);
            reverse_until_index(&mut *v, index);
        }
        index -= 1;
    }
}

#[test]
fn empty() {
    let mut v: Vec<i32> = vec![];
    pancake_sort(&mut v);
    assert!(v.is_empty());
}

#[test]
fn one_element() {
    let mut v = vec![1];
    pancake_sort(&mut v);
    assert_eq!(vec![1], v);
}

#[test]
fn reversed() {
    let mut v = vec![5, 4, 3, 2, 1];
    pancake_sort(&mut v);
    assert_eq!(vec![1, 2, 3, 4, 5], v);
}

#[test]
fn many_out_of_order() {
    let mut v = vec![4, 18, 3, -5, 7, 4, 1, 8, 3, 4, 19, 11, 23, 24, 25, 6, 0];
    pancake_sort(&mut v);
    assert_eq!(
        vec![-5, 0, 1, 3, 3, 4, 4, 4, 6, 7, 8, 11, 18, 19, 23, 24, 25],
        v);
}

#[test]
fn string() {
    let mut v = vec!["Charlie", "Alpha", "Brava", "Echo", "Delta", "Foxtrot"];
    pancake_sort(&mut v);
    assert_eq!(
        vec!["Alpha", "Brava", "Charlie", "Delta", "Echo", "Foxtrot"],
        v);
}
