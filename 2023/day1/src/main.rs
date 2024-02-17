use std::collections::HashMap;
use std::fs;
use std::env;
use regex::Regex;
use regex::Match;

fn main() {
    let fname = env::args().nth(1).unwrap();
    let a = fs::read_to_string(fname).unwrap().lines().fold(0, |acc, l| {
        acc + get_number(l)
    });
    dbg!(a);
}

fn get_number(line: &str) -> i64 {
    let replacements = HashMap::from([
        ("one", "1"), 
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ]);
    let keys_and_values = replacements.keys().chain(replacements.values());
    let re_str = keys_and_values.copied().collect::<Vec<_>>().join("|");
    let re = Regex::new(&re_str).unwrap();
    let matches : Vec<Match> = re.find_iter(line).collect();
    
    let mut first = matches.first().unwrap().as_str();
    if replacements.contains_key(first) {
        first = replacements[first];
    }

    let mut last = matches.last().unwrap().as_str();
    if replacements.contains_key(last) {
        last = replacements[last];
    }

    (first.to_owned() + last).parse().unwrap()
}