use std::{env, fs, str::FromStr};

fn main() {
    let fname = env::args().nth(1).unwrap();
    let contents = fs::read_to_string(fname).unwrap();
    let cards : Vec<_> = contents.lines().map(|l| Hand::from_str(l).unwrap()).collect();
    dbg!(cards);
}

#[derive(Debug)]
struct Hand {
    bid: i32,
    cards: Vec<Card>
}

impl FromStr for Hand {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_ascii_whitespace();
        let cards = parts.next().unwrap().chars().map(|card| Card { card }).collect();
        let bid = parts.next().unwrap().parse()?;
        Ok(Hand {
            bid,
            cards
        })
    }
}

#[derive(Debug)]
struct Card {
    card: char
}