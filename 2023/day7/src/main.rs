use std::{env, fs, ops::Index, str::FromStr};

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
        let cards = parts.next().unwrap().bytes().map(|card| Card { card }).collect();
        let bid = parts.next().unwrap().parse()?;
        Ok(Hand {
            bid,
            cards
        })
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Card {
    card: u8
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let order = b"23456789TJQKA";
        let self_index = order.iter().position(|c| *c == self.card).unwrap();
        let other_index = order.iter().position(|c| *c == other.card).unwrap();
        self_index.cmp(&other_index)
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}