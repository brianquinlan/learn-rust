/// Returns true if the find string can be found in the search string.
/// # Examples
///
/// ```
/// use substring::has_substring;
///
/// assert!(has_substring("abcdef", "def"));
/// assert!(!has_substring("abcdef", "xyz"));
/// ```
pub fn has_substring(search: &str, find: &str) -> bool {
    // The non-brute-force algorithms would be too difficult to implement in an
    // interview without reference so accept that this will be O(nm).
    if search.is_empty() {
        return find.is_empty();
    }

    let search_bytes = search.as_bytes();
    let find_bytes = find.as_bytes();
    let max_search_index = search_bytes.len() - find_bytes.len();

    for i in 0..(max_search_index + 1) {
        let mut found = true;
        for j in 0..find_bytes.len() {
            if search_bytes[i + j] != find_bytes[j] {
                found = false;
                break;
            }
        }
        if found {
            return true;
        }
    }
    false
}

#[test]
fn substring_at_end() {
    assert!(has_substring("abcdef", "def"));
}

#[test]
fn substring_at_beginning() {
    assert!(has_substring("abcdef", "abc"));
}

#[test]
fn substring_in_middle() {
    assert!(has_substring("abcdef", "cde"));
}

#[test]
fn substring_empty() {
    assert!(has_substring("abcdef", ""));
}

#[test]
fn search_string_empty() {
    assert!(!has_substring("", "abc"));
}

#[test]
fn search_string_and_substring_empty() {
    assert!(has_substring("", ""));
}

#[test]
fn search_string_has_repeating_pattern() {
    assert!(has_substring("aaaaaaaaaaaaaba", "aaab"));
}

#[test]
fn substring_not_found() {
    assert!(!has_substring("aaaaaaaaaaaaaba", "aaabb"));
}

#[test]
fn non_ascii() {
    assert!(has_substring(
        "Οἱ δὲ Φοίνιϰες οὗτοι οἱ σὺν Κάδμῳ ἀπιϰόμενοι..",
        "Φοίνιϰες"));    
}
