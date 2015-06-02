extern crate getopts;
extern crate regex;
extern crate time;

use getopts::Options;
use std::env;
use std::ascii::AsciiExt;
use std::error::Error;
use std::fs::File;
use std::io;
use std::io::BufReader;
use std::io::prelude::*;
use std::path::Path;
use std::collections::HashMap;
use std::collections::HashSet;
use regex::Regex;
use time::{Duration, PreciseTime};

/// Load a dictionary containing one word per line (e.g. from /usr/share/dict/).
fn load_dictionary(dictionary_path: &Path) -> Vec<String> {
    let display = dictionary_path.display();

    let file = match File::open(dictionary_path) {
        Err(why) => panic!("couldn't open {}: {}",
                           display,
                           Error::description(&why)),
        Ok(file) => file,
    };
    let reader = BufReader::new(file);
    let mut words = Vec::new();

    for readline in reader.lines() {
        let line = match readline {
            Err(why) => panic!("couldn't read from {}: {}",
                                display,
                                Error::description(&why)),
            Ok(readline) => readline,
        };

        words.push(line.trim().to_string());
    }
    return words;
}

/// Build the data structures needed for the matching algorithm given a list of
/// words.
///
/// The first hashmap maps (character, index, word length) to a list of words.
/// For example:
///   ('c', 1, 3) => ["can, "cat", "con", etc.]
///   ('t', 3, 3) => ["art, "cat", "mat", etc.]
///
/// The second hashmap maps word length to a list of words of that length.
/// For example:
///   1 => ["a", "I", etc.]
///   3 => ["art", "can", con", "mat", etc.]
fn build_maps(words: &Vec<String>) -> (
        HashMap<(char, usize, usize), HashSet<&str>>,
        HashMap<usize, Vec<&str>>) {
    let mut ch_position_length_map :
        HashMap<(char, usize, usize), HashSet<&str>> = HashMap::new();
    let mut length_map : HashMap<usize, Vec<&str>> = HashMap::new();

    for word in words.iter() {
        for (index, ch) in word.to_ascii_lowercase().chars().enumerate() {
            let key = (ch, index, word.len());

            ch_position_length_map.entry(key).or_insert_with(
                || HashSet::new()).insert(word);
        }
        length_map.entry(word.len()).or_insert_with(
            || Vec::new()).push(word);
    }
    (ch_position_length_map, length_map)
}

/// Match a pattern against a dictionary and return the subset of words that
/// match. The pattern must consist of ASCII letters or digits. Letters will be
/// matched exactly (ignoring case) while digits will be composed into numbers
/// (e.g. "18" will be treated as eighteen) and that many characters will be
/// skipped. So "i18n" would be equivalent to the regex pattern "^i.{18}n$".
///
/// The dictionary is presented as two HashMaps (see `build_maps`). The first
/// maps (character, index, word length) to a list of words. The second maps
/// word length to a list of words.
fn match_pattern<'a>(pattern : &str,
         ch_position_length_map : &HashMap<(char, usize, usize), HashSet<&'a str>>,
         length_map : &HashMap<usize, Vec<&'a str>>) -> Vec<&'a str> {
    let pattern_parser = Regex::new(
        r"(?P<number>\d+)|(?P<letter>[A-Za-z])").unwrap();

    let mut pattern_length: usize = 0;
    let mut ch_and_index = Vec::new();  // Known characters and their index.

    for number_or_letter in pattern_parser.captures_iter(
            &pattern.to_ascii_lowercase()) {
        if number_or_letter.name("number").is_some() {
            let number_as_string = number_or_letter.name("number").unwrap();
            let number: usize = number_as_string.parse().unwrap();
            pattern_length += number;
        } else {
            let match_word = number_or_letter.name("letter").unwrap();
            let ch = match_word.char_indices().next().unwrap().1;
            ch_and_index.push((ch, pattern_length));
            pattern_length += 1;
        }
    }

    if ch_and_index.is_empty() {
        // No characters were given so return all the words of the specified
        // length. This handles the case where the pattern is purely digits.
        return match length_map.get(&pattern_length) {
            // The division was valid
            Some(word_list) => word_list.clone(),
            // The division was invalid
            None => Vec::new()
        }
    }

    // Use "ch_and_index" to find all of the word sets that apply to the
    // pattern.
    let mut word_sets : Vec<&HashSet<&str>> = Vec::new();
    for &(ch, index) in ch_and_index.iter() {
        let key = (ch, index as usize, pattern_length as usize);
        match ch_position_length_map.get(&key) {
            // The division was valid
            Some(word_set) => word_sets.push(word_set),
            // There are no words of pattern_length with ch at index.
            None => return Vec::new()
        }
    }

    assert!(!word_sets.is_empty());  // Should have exited already in this case.
    if word_sets.len() == 1 {
        // Handles the case of a single letter specified e.g. "c2".
        return word_sets[0].iter().cloned().collect();
    }

    // Intersect the word sets from smallest set to largest set to minimize
    // intersection time.
    word_sets.sort_by(|a, b| a.len().cmp(&b.len()));
    let mut refined_word_set : HashSet<&str> =
        word_sets[0].intersection(
            word_sets[1]).cloned().collect();

    for word_set in word_sets.iter().skip(2) {
        refined_word_set = refined_word_set.intersection(
            word_set).cloned().collect();
    }

    return refined_word_set.iter().cloned().collect();
}

#[test]
fn match_number_only() {
    let words = vec![
            "cat".to_string(),
            "intercrystallization".to_string(),
            "parallelogrammatical".to_string(),
            "pseudoanthropological".to_string()];
    let (ch_position_length_map, length_map) = build_maps(&words);
    let mut matches = match_pattern("20",
                                    &ch_position_length_map,
                                    &length_map);
    matches.sort();
    assert_eq!(matches, ["intercrystallization", "parallelogrammatical"]);
}

#[test]
fn match_number_only_no_match() {
    let words = vec![
            "cat".to_string(),
            "parallelogrammatical".to_string(),
            "pseudoanthropological".to_string()];
    let (ch_position_length_map, length_map) = build_maps(&words);
    let matches = match_pattern("2",
                                &ch_position_length_map,
                                &length_map);
    assert!(matches.is_empty());
}

#[test]
fn match_letters_only() {
    let words = vec![
            "cat".to_string(),
            "intercrystallization".to_string(),
            "parallelogrammatical".to_string(),
            "pseudoanthropological".to_string()];
    let (ch_position_length_map, length_map) = build_maps(&words);
    let mut matches = match_pattern("parallelogrammatical",
                                    &ch_position_length_map,
                                    &length_map);
    matches.sort();
    assert_eq!(matches, ["parallelogrammatical"]);
}

#[test]
fn match_letters_only_no_match() {
    let words = vec![
            "cat".to_string(),
            "intercrystallization".to_string(),
            "pseudoanthropological".to_string()];
    let (ch_position_length_map, length_map) = build_maps(&words);
    let matches = match_pattern("caterpillar",
                                &ch_position_length_map,
                                &length_map);
    assert!(matches.is_empty());
}

#[test]
fn match_letter_number_letter() {
    let words = vec![
            "i18n".to_string(),
            "in".to_string(),
            "intercrystallization".to_string(),
            "internationalization".to_string(),
            "internationalizationy".to_string()];
    let (ch_position_length_map, length_map) = build_maps(&words);
    let mut matches = match_pattern("i18n",
                                    &ch_position_length_map,
                                    &length_map);
    matches.sort();
    assert_eq!(matches, ["intercrystallization", "internationalization"]);
}

#[test]
fn match_letter_number_letter_number_letter() {
    let words = vec![
            "institutionalization".to_string(),
            "intercrystallization".to_string(),
            "internationalization".to_string(),
            "internationalizationy".to_string()];
    let (ch_position_length_map, length_map) = build_maps(&words);
    let mut matches = match_pattern("i1t16n",
                                    &ch_position_length_map,
                                    &length_map);
    matches.sort();
    assert_eq!(matches, ["intercrystallization", "internationalization"]);
}

#[test]
fn match_number_letter_number_letter_number() {
    let words = vec![
            "antianthropomorphism".to_string(),
            "institutionalization".to_string(),
            "intercrystallization".to_string(),
            "internationalization".to_string(),
            "internationalizationy".to_string()];
    let (ch_position_length_map, length_map) = build_maps(&words);
    let mut matches = match_pattern("2t2n14",
                                    &ch_position_length_map,
                                    &length_map);
    matches.sort();
    assert_eq!(matches, ["antianthropomorphism", "internationalization"]);
}

#[test]
fn match_ignores_case() {
    let words = vec![
            "Cat".to_string(),
            "cat".to_string(),
            "cot".to_string(),
            "dog".to_string()];
    let (ch_position_length_map, length_map) = build_maps(&words);
    let mut matches = match_pattern("c1t",
                                    &ch_position_length_map,
                                    &length_map);
    matches.sort();
    assert_eq!(matches, ["Cat", "cat", "cot"]);
}

fn print_matches(words : Vec<&str>, num_runs : u32, duration : &Duration) {
    if words.is_empty() {
        println!("\t<No Results>");
    } else {
        let len = words.len();
        let mut sorted_words : Vec<&str> = words.into_iter().collect();
        sorted_words.sort();
        for word in sorted_words {
            println!("\t{}", word);
        }

        match num_runs {
            1 => println!("\t => {} results in {}μs",
                          len,
                          duration.num_microseconds().unwrap()),
            _ => println!("\t => {} results in {}μs ({} runs)",
                          len,
                          duration.num_microseconds().unwrap(),
                          num_runs)
        }
    }
}

#[allow(dead_code)]
fn main() {
    let args: Vec<String> = env::args().collect();
    let mut opts = Options::new();
    opts.optopt(
        "n",
        "num_runs",
        "the number of times to run the match per line",
        "COUNT");
    let matches = opts.parse(&args[1..]).unwrap();
    let num_runs : u32 = match matches.opt_str("n") {
        Some(n) => n.parse().unwrap(),
        None => 1
    };

    let dictionary_path = Path::new("/usr/share/dict/words");
    let words = load_dictionary(&dictionary_path);
    let (ch_position_length_map, length_map) = build_maps(&words);

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let start = PreciseTime::now();
        let word = line.unwrap();
        for _ in 0..num_runs {
            match_pattern(&word, &ch_position_length_map, &length_map);
        }
        let matches = match_pattern(&word,
                                    &ch_position_length_map,
                                    &length_map);
        print_matches(matches, num_runs, &start.to(PreciseTime::now()));
    }
}