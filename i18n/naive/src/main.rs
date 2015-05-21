extern crate getopts;
extern crate regex;
extern crate time;

use getopts::Options;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io;
use std::io::BufReader;
use std::io::prelude::*;
use std::path::Path;
use regex::Regex;
use time::{Duration, PreciseTime};

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

fn match_pattern<'a>(pattern : &str, words : &'a Vec<String>)
        -> Vec<&'a str> {
    let pattern_parser = Regex::new(
        r"(?P<number>\d+)|(?P<letter>[A-Za-z])").unwrap();

    let mut regex_string = "(?i)^".to_string();
    for number_or_letter in pattern_parser.captures_iter(&pattern) {
        if number_or_letter.name("number").is_some() {
            let number_as_string = number_or_letter.name("number").unwrap();
            regex_string.push_str(".{");
            regex_string.push_str(number_as_string);
            regex_string.push('}');
        } else {
            let match_word = number_or_letter.name("letter").unwrap();
            let ch = match_word.char_indices().next().unwrap().1;
            regex_string.push(ch);
        }
    }

    regex_string.push('$');
    let pattern_matcher = Regex::new(&regex_string).unwrap();
    let mut matching_words : Vec<&str> = Vec::new();

    for word in words {
        if pattern_matcher.is_match(word) {
           matching_words.push(word) 
        }
    }
    matching_words
}

#[test]
fn match_number_only() {
    let words = vec![
            "cat".to_string(),
            "intercrystallization".to_string(),
            "parallelogrammatical".to_string(),
            "pseudoanthropological".to_string()];
    let mut matches = match_pattern("20", &words);
    matches.sort();
    assert_eq!(matches, ["intercrystallization", "parallelogrammatical"]);
}

#[test]
fn match_number_only_no_match() {
    let words = vec![
            "cat".to_string(),
            "parallelogrammatical".to_string(),
            "pseudoanthropological".to_string()];
    let matches = match_pattern("2", &words);
    assert!(matches.is_empty());
}

#[test]
fn match_letters_only() {
    let words = vec![
            "cat".to_string(),
            "intercrystallization".to_string(),
            "parallelogrammatical".to_string(),
            "pseudoanthropological".to_string()];
    let mut matches = match_pattern("parallelogrammatical", &words);
    matches.sort();
    assert_eq!(matches, ["parallelogrammatical"]);
}

#[test]
fn match_letters_only_no_match() {
    let words = vec![
            "cat".to_string(),
            "intercrystallization".to_string(),
            "pseudoanthropological".to_string()];
    let matches = match_pattern("caterpillar", &words);
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
    let mut matches = match_pattern("i18n", &words);
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
    let mut matches = match_pattern("i1t16n", &words);
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
    let mut matches = match_pattern("2t2n14", &words);
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
    let mut matches = match_pattern("c1t", &words);
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

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let start = PreciseTime::now();
        let word = line.unwrap();
        for _ in 0..num_runs {
            match_pattern(&word, &words);
        }
        let matches = match_pattern(&word, &words);
        print_matches(matches, num_runs, &start.to(PreciseTime::now()));
    }
}
