use std::collections::HashMap;
use std::env;
use std::fs;

fn main() {
    let fname = env::args().nth(1).unwrap();
    let a = fs::read_to_string(fname)
        .unwrap()
        .lines()
        .fold(0, |acc, l| {
            println!("{} {}", get_number(l), l);
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
    let mut first = keys_and_values
        .min_by_key(|v| line.find(*v).unwrap_or(999))
        .unwrap()
        .to_owned();
    if replacements.contains_key(first) {
        first = replacements[first];
    }

    let keys_and_values = replacements.keys().chain(replacements.values());
    let mut last = keys_and_values
        .max_by_key(|v| line.rfind(*v).unwrap_or(0))
        .unwrap()
        .to_owned();
    if replacements.contains_key(last) {
        last = replacements[last];
    }

    let number = first.to_owned() + last;
    number.parse().unwrap()
}
