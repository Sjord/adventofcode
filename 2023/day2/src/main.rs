use std::{env, fs};
use nom::{
    branch::alt, bytes::complete::{tag, take_while_m_n}, character::complete::multispace0, combinator::map_res, multi::{separated_list0, separated_list1}, sequence::tuple, IResult};
use nom::character::complete::i32 as cc_i32;

fn main() {
    let fname = env::args().nth(1).unwrap();
    let binding = fs::read_to_string(fname)
        .unwrap();
    let lines = binding
        .lines();

    let max = Set { red: 12, green: 13, blue: 14 };
    let mut total = 0;
    for line in lines {
        let (_, game) = Game::parse_line(line).unwrap();
        if game.is_possible(&max) {
            total += game.id;
        }
    }
    dbg!(total);
}

#[derive(Debug, PartialEq, PartialOrd)]
struct Set {
    red: i32,
    green: i32,
    blue: i32
}

impl Set {
    fn is_possible(&self, other: &Self) -> bool {
        self.red <= other.red && self.green <= other.green && self.blue <= other.blue
    }
}

#[derive(Debug)]
struct Game {
    id: i32,
    sets: Vec<Set>
}

impl Game {
    fn parse_line(input: &str)  -> IResult<&str, Game> {
        let (i, (_, id, _, colors)) = tuple((
            tag("Game "),
            cc_i32,
            tag(":"),
            separated_list0(tag(";"),
              separated_list1(tag(","),
                    tuple((multispace0, cc_i32, tag(" "), alt((tag("red"), tag("green"), tag("blue")))))
                )
            )
        ))(input)?;
        
        let mut sets = Vec::with_capacity(colors.len());
        for setinfo in colors {
            let mut set = Set { red: 0, green: 0, blue: 0 };
            for (_, count, _, color) in setinfo {
                match color {
                    "red" => set.red = count,
                    "green" => set.green = count,
                    "blue" => set.blue = count,
                    _ => panic!("unexpected color: {}", color)
                }
            }
            sets.push(set);
        }
        
        Ok((i, Game {
            id,
            sets
        }))
    }

    fn is_possible(&self, max: &Set) -> bool {
        self.sets.iter().all(|s| s.is_possible(max))
    }
}