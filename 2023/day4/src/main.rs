use std::collections::{HashSet, VecDeque};
use std::{env, fs};

use nom::bytes::complete::tag;
use nom::character::complete::{i32 as cc_i32, space1};
use nom::combinator::all_consuming;
use nom::error::Error;
use nom::multi::separated_list1;
use nom::sequence::tuple;
use nom::Finish;

fn main() {
    let fname = env::args().nth(1).unwrap();
    let contents = fs::read_to_string(fname).unwrap();
    let cards = parse_cards(&contents).unwrap();
    let mut stack : VecDeque<&Card> = VecDeque::from_iter(cards.iter());

    let mut seen_cards = 0;
    while let Some(card) = stack.pop_front() {
        let win_count = card.winning_numbers().len();
        // println!("card {} win_count {}", card.id, win_count);
        for i in 0..win_count {
            let idx : usize = card.id as usize + i;
            // println!("adding card {}", cards.get(idx).unwrap().id);
            stack.push_back(cards.get(idx).unwrap())
        }
        seen_cards += 1;

    }
    dbg!(seen_cards);
}

#[derive(Debug)]
struct Card {
    id: i32,
    have: Vec<i32>,
    winning: Vec<i32>,
}

impl Card {
    fn winning_numbers(&self) -> Vec<i32> {
        let have: HashSet<i32, _> = HashSet::<i32>::from_iter(self.have.clone().into_iter());
        let winning: HashSet<i32, _> = HashSet::<i32>::from_iter(self.winning.clone().into_iter());
        have.intersection(&winning).copied().collect()
    }
}

fn parse_cards(input: &str) -> Result<Vec<Card>, Error<&str>> {
    let mut parser = all_consuming(separated_list1(
        tag("\n"),
        tuple((
            tuple((tag("Card"), space1)),
            cc_i32,
            tuple((tag(":"), space1)),
            separated_list1(space1, cc_i32),
            tuple((space1, tag("|"), space1)),
            separated_list1(space1, cc_i32),
        )),
    ));
    let (_, cards) = parser(input).finish()?;
    Ok(cards
        .into_iter()
        .map(|(_, id, _, have, _, winning)| Card { id, have, winning })
        .collect())
}
