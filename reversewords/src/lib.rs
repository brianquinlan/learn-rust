use std::ascii::AsciiExt;

const SPACE: u8 = 0x20;

/// Reverse the contents of the input vector between `from` and `to`
/// inclusively.
#[inline]
fn reverse_vec(v: &mut [u8], from: usize, to: usize) {
    let mut left = from;
    let mut right = to;
    while left < right {
        v.swap(left, right);
        left += 1;
        right -= 1;
    }
}

/// Reverse the words in the input string. Words are defined as any characters
/// other than a space. Only ASCII input is acceptable.
///
/// # Examples
///
/// ```
/// use reversewords::ascii_reverse_words;
///
/// let mut s = "Hello from Rust!".to_string();
/// ascii_reverse_words(&mut s);
/// assert_eq!("Rust! from Hello", s)
/// ```
#[inline]
pub fn ascii_reverse_words(s: &mut String) {
    let len = s.len();
    if len == 0 {
        return;
    }

    if !s.is_ascii() {
        // Reversing non-ASCII strings is hard because you have to make sure
        // not to spit graphemes.
        panic!("Unexpected non-ASCII string: \"{}\"", s);
    }

    unsafe {
        let ref mut bytes = s.as_mut_vec();  // Unsafe.
        // Reverse the entire string. So:
        // "Hello from Rust!" => "!tsuR morf olleH"
        reverse_vec(bytes, 0, len-1);

        // Find each "word" (non-space) in `bytes` and reverse it. So
        // "!tsuR" => "Rust1".
        let mut left = 0;
        while left < len {
            if bytes[left] == SPACE {
                left += 1;
            } else {
                let mut right = left;
                while right < len && bytes[right] != SPACE {
                    right += 1;
                }
                reverse_vec(bytes, left, right - 1);
                left = right;
            }
        }
    }
}

#[test]
fn ascii_hello_world() {
    let mut s = "Hello World".to_string();
    ascii_reverse_words(&mut s);
    assert_eq!("World Hello", s)
}

#[test]
fn ascii_empty() {
    let mut s = "".to_string();
    ascii_reverse_words(&mut s);
    assert_eq!("", s)
}

#[test]
fn ascii_single_space_only() {
    let mut s = " ".to_string();
    ascii_reverse_words(&mut s);
    assert_eq!(" ", s)
}

#[test]
fn ascii_multiple_spaces_only() {
    let mut s = "   ".to_string();
    ascii_reverse_words(&mut s);
    assert_eq!("   ", s)
}

#[test]
fn ascii_single_letter_only() {
    let mut s = "a".to_string();
    ascii_reverse_words(&mut s);
    assert_eq!("a", s)
}

#[test]
fn ascii_single_word_only() {
    let mut s = "Hello".to_string();
    ascii_reverse_words(&mut s);
    assert_eq!("Hello", s)
}

#[test]
fn ascii_space_and_word() {
    let mut s = " Hello".to_string();
    ascii_reverse_words(&mut s);
    assert_eq!("Hello ", s)
}

#[test]
fn ascii_space_word_space() {
    let mut s = " Hello  ".to_string();
    ascii_reverse_words(&mut s);
    assert_eq!("  Hello ", s)
}

#[test]
#[should_panic]
fn non_ascii() {
    let mut s = "Я люблю тебя.".to_string();
    ascii_reverse_words(&mut s);
}
