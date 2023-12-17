use std::fs;
use std::env;

fn main() {
    let fname = env::args().nth(1).unwrap();
    let a = fs::read_to_string(fname).unwrap().lines().fold(0, |acc, l| {
        let digits : Vec<char> = l.chars().filter(|c| c.is_digit(10)).collect();
        let mut number: String = String::new();
        number.push(*digits.first().unwrap());
        number.push(*digits.last().unwrap());
        acc + number.parse::<i64>().unwrap()
    });
    dbg!(a);
}
