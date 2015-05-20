extern crate getopts;
extern crate regex;
extern crate time;

use getopts::Options;
use std::env;
use std::mem;
use std::ascii::AsciiExt;
use std::error::Error;
use std::fs::File;
use std::io;
use std::io::BufReader;
use std::io::prelude::*;
use std::path::Path;
use std::str::Chars;
use std::collections::HashMap;
use regex::Regex;
use time::{Duration, PreciseTime};

struct Node {
    words : Vec<String>,
    children: HashMap<char, Node>
}

enum Token {
    Character(char),
    Any
}

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

fn insert_internal(word : &str, remaining: &mut Chars, node: &mut Node) {
    match remaining.next() {
        Some(ch) => {
            if let Some(next_node) = node.children.get_mut(&ch) {
                insert_internal(word, remaining, next_node);
                return;
            }
            let mut next_node = Node {words: Vec::new(), children: HashMap::new()};
            insert_internal(word, remaining, &mut next_node);
            node.children.insert(ch, next_node);
        }
        None => node.words.push(word.to_string())
    }
}

fn insert(word : &str, node: &mut Node) {
    insert_internal(word, &mut word.to_ascii_lowercase().chars(), node);
}

fn build_length_to_trie_map(words: &Vec<String>) -> HashMap<usize, Node> {
    let mut length_to_trie = HashMap::new();

    for word in words.iter() {
        let mut trie = length_to_trie.entry(word.len()).or_insert_with(
            || Node {words: Vec::new(), children: HashMap::new()});
        insert(word, &mut trie);
    }
    length_to_trie
}

fn match_pattern<'a>(pattern : &str,
         build_length_to_trie_map : &'a HashMap<usize, Node>) -> Vec<&'a str> {

    let pattern_parser = Regex::new(
        r"(?P<number>\d+)|(?P<letter>[A-Za-z])").unwrap();

    let mut tokens = Vec::new();
    let mut pattern_length: usize = 0;

    for number_or_letter in pattern_parser.captures_iter(
            &pattern.to_ascii_lowercase()) {
        if number_or_letter.name("number").is_some() {
            let number_as_string = number_or_letter.name("number").unwrap();
            let number: usize = number_as_string.parse().unwrap();
            for _ in 0..number {
                tokens.push(Token::Any);
            }
            pattern_length += number;
        } else {
            let match_word = number_or_letter.name("letter").unwrap();
            let ch = match_word.char_indices().next().unwrap().1;
            tokens.push(Token::Character(ch));
            pattern_length += 1;
        }
    }

    let trie = match build_length_to_trie_map.get(&pattern_length) {
        Some(node) => node,
        None => return Vec::new()
    };

    let mut nodes : Vec<&Node> = vec![trie];
    let mut next_nodes : Vec<&Node> = Vec::new();

    for token in tokens.iter() {
        match *token {
            Token::Any => {
                while let Some(node) = nodes.pop() {
                    for next_node in node.children.values() {
                        next_nodes.push(next_node);
                    }
                }
            }
            Token::Character(ch) => {
                while let Some(node) = nodes.pop() {
                    if let Some(next_node) = node.children.get(&ch) {
                        next_nodes.push(next_node);
                    }
                }
            }
        }
        if next_nodes.len() == 0 {
            return Vec::new();
        }
        assert_eq!(nodes.len(), 0);
        mem::swap(&mut nodes, &mut next_nodes);
    }

    let mut words : Vec<&str> = Vec::new();
    for node in nodes.iter() {
        for word in node.words.iter() {
            words.push(&word);
        }
    }
    words
}

#[test]
fn match_number_only() {
    let words = vec![
            "cat".to_string(),
            "intercrystallization".to_string(),
            "parallelogrammatical".to_string(),
            "pseudoanthropological".to_string()];
    let trie = build_length_to_trie_map(&words);
    let mut matches = match_pattern("20", &trie);
    matches.sort();
    assert_eq!(matches, ["intercrystallization", "parallelogrammatical"]);
}

#[test]
fn match_number_only_no_match() {
    let words = vec![
            "cat".to_string(),
            "parallelogrammatical".to_string(),
            "pseudoanthropological".to_string()];
    let trie = build_length_to_trie_map(&words);
    let matches = match_pattern("2", &trie);
    assert!(matches.is_empty());
}

#[test]
fn match_letters_only() {
    let words = vec![
            "cat".to_string(),
            "intercrystallization".to_string(),
            "parallelogrammatical".to_string(),
            "pseudoanthropological".to_string()];
    let trie = build_length_to_trie_map(&words);
    let mut matches = match_pattern("parallelogrammatical", &trie);
    matches.sort();
    assert_eq!(matches, ["parallelogrammatical"]);
}

#[test]
fn match_letters_only_no_match() {
    let words = vec![
            "cat".to_string(),
            "intercrystallization".to_string(),
            "pseudoanthropological".to_string()];
    let trie = build_length_to_trie_map(&words);
    let matches = match_pattern("caterpillar", &trie);
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
    let trie = build_length_to_trie_map(&words);
    let mut matches = match_pattern("i18n", &trie);
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
    let trie = build_length_to_trie_map(&words);
    let mut matches = match_pattern("i1t16n", &trie);
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
    let trie = build_length_to_trie_map(&words);
    let mut matches = match_pattern("2t2n14", &trie);
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
    let trie = build_length_to_trie_map(&words);
    let mut matches = match_pattern("c1t", &trie);
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
    let length_to_trie = build_length_to_trie_map(&words);

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let start = PreciseTime::now();
        let word = line.unwrap();
        for _ in 0..num_runs {
            match_pattern(&word, &length_to_trie);
        }
        let matches = match_pattern(&word, &length_to_trie);
        print_matches(matches, num_runs, &start.to(PreciseTime::now()));
    }
}