extern crate regex;

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

fn load_dictionary(dictionary_path: &Path) -> (
        HashMap<(char, usize, usize), HashSet<String>>,
        HashMap<usize, HashSet<String>>) {
    let display = dictionary_path.display();

    let file = match File::open(dictionary_path) {
        Err(why) => panic!("couldn't open {}: {}",
                           display,
                           Error::description(&why)),
        Ok(file) => file,
    };

    let mut ch_position_length_map = HashMap::new();
    let mut length_map = HashMap::new();
    let reader = BufReader::new(file);

    for readline in reader.lines() {
        let line = match readline {
            Err(why) => panic!("couldn't read from {}: {}",
                                display,
                                Error::description(&why)),
            Ok(readline) => readline,
        };

        let word = line.trim();
        for (index, ch) in word.to_ascii_lowercase().chars().enumerate() {
            let key = (ch, index, word.len());

            ch_position_length_map.entry(key).or_insert(
                HashSet::new()).insert(word.to_string());
            length_map.entry(word.len()).or_insert(
                HashSet::new()).insert(word.to_string());
        }
    }
    (ch_position_length_map, length_map)
}

fn match_pattern(pattern : &str,
         ch_position_length_map : &HashMap<(char, usize, usize), HashSet<String>>,
         length_map : &HashMap<usize, HashSet<String>>) -> Vec<String> {
    let pattern_parser = Regex::new(
        r"(?P<number>\d+)|(?P<letter>[A-Za-z])").unwrap();

    let mut pattern_length: usize = 0;
    let mut ch_and_index = Vec::new();

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
        if length_map.contains_key(&pattern_length) {
            return length_map.get(
                &pattern_length).unwrap().iter().cloned().collect();
        } else {
            return Vec::new();
        }
    }

    let mut word_sets = Vec::new();
    for &(ch, index) in ch_and_index.iter() {
        let key = (ch, index as usize, pattern_length as usize);
        if ch_position_length_map.contains_key(&key) {
            word_sets.push(ch_position_length_map.get(&key).unwrap());
        } else {
            // There are no words of pattern_length that ch at index.
            return Vec::new();
        }
    }

    assert!(!word_sets.is_empty());
    if word_sets.len() == 1 {
        return word_sets.get(0).unwrap().iter().cloned().collect();
    }

    word_sets.sort_by(|a, b| a.len().cmp(&b.len()));
    let mut refined_word_set : HashSet<String> =
        word_sets.get(0).unwrap().intersection(
            word_sets.get(1).unwrap()).cloned().collect();

    for word_set in word_sets.iter().skip(2) {
        refined_word_set = refined_word_set.intersection(
            word_set).cloned().collect();
    }

    return refined_word_set.iter().cloned().collect();
}

fn print_matches(words : Vec<String>) {
    if words.is_empty() {
        println!("\t<No Results>");
    } else {
        let mut sorted_words : Vec<String> = words.into_iter().collect();
        sorted_words.sort();
        for word in sorted_words {
            println!("\t{}", word);
        }
    }
}

fn main() {
    let dictionary_path = Path::new("/usr/share/dict/words");
    let (ch_position_length_map, length_map) = load_dictionary(
        &dictionary_path);

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let matches = match_pattern(&line.unwrap(),
                                    &ch_position_length_map,
                                    &length_map);
        print_matches(matches);
    }
}