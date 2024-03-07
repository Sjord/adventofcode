use std::{cmp::Ordering, env, fs, ops::Index, str::FromStr};

fn main() {
    let fname = env::args().nth(1).unwrap();
    let contents = fs::read_to_string(fname).unwrap();
    let mut hands : Vec<_> = contents.lines().map(|l| Hand::from_str(l).unwrap()).collect();
    hands.sort();
    let mut result = 0;
    for (i, hand) in hands.iter().enumerate() {
        let rank = i + 1;
        println!("{} * {}", hand.bid, rank);
        result += rank * hand.bid as usize;
    }
    dbg!(result);
}

#[derive(Debug, PartialEq, Eq)]
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

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let self_ht = self.handtype();
        let other_ht = other.handtype();
        self_ht.cmp(&other_ht)
            .then(self.cards[0].cmp(&other.cards[0]))
            .then(self.cards[1].cmp(&other.cards[1]))
            .then(self.cards[2].cmp(&other.cards[2]))
            .then(self.cards[3].cmp(&other.cards[3]))
            .then(self.cards[4].cmp(&other.cards[4]))
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl Hand {
    fn handtype(&self) -> HandType {
        let groups = self.group_cards();
        if groups.contains(&5) {
            return HandType::FiveOfAKind;
        }
        if groups.contains(&4) {
            return HandType::FourOfAKind;
        }
        if groups.contains(&3) {
            if groups.contains(&2) {
                return HandType::FullHouse;
            } else {
                return HandType::ThreeOfAKind;
            }
        }
        if groups.contains(&2) {
            if groups.iter().filter(|g| **g == 2).count() == 2 {
                return HandType::TwoPair;
            } else {
                return HandType::OnePair;
            }
        }
        HandType::HighCard
    }

    fn group_cards(&self) -> Vec<usize> {
        let mut result = Vec::new();
        let possible_cards = b"23456789TJQKA";
        for search in possible_cards {
            let found = self.cards.iter().filter(|c| c.card == *search).count();
            if found != 0 {
                result.push(found)
            }
        }
        result.sort();
        result
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