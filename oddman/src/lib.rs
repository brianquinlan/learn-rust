/// The input slice consists of integers, all but one of which must be
/// duplicated exactly twice. Returns the non-duplicate integer.
///
/// # Examples
///
/// ```
/// use oddman::odd_man_out;
///
/// assert_eq!(8, odd_man_out(&[1, 8, 5, 6, 5, 1, 6]));
/// ```
#[inline]
pub fn odd_man_out(l: &[i32]) -> i32 {
    let mut mask : i32 = 0;
    for x in l {
        // Take advantage of the fact that xor-ing an integer with a value
        // twice results in the original integer. So every value except for
        // the unpaired integer should have cancelled itself out. Then take
        // advantage of the property that x^0 == x.
        mask ^= *x;
    }
    return mask;
}

#[test]
fn positive_only() {
    assert_eq!(5, odd_man_out(&[1, 2, 3, 4, 5, 1, 2, 3, 4]));
}

#[test]
fn negative_numbers() {
    assert_eq!(-5, odd_man_out(&[-1, 2, 3, 4, -5, -1, 2, 3, 4]));
}

#[test]
fn missing_zero() {
    assert_eq!(0, odd_man_out(&[1, 2, 3, 0, 1, 2, 3]));
}
