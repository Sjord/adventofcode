use std::str::FromStr;
use std::fs;
use std::env;

fn main() {
    let fname = env::args().nth(1).unwrap();
    let contents = fs::read_to_string(fname).unwrap();
    let rucksacks = contents.lines().map(|l| l.parse::<Rucksack>().unwrap());
    let sum : u32 = rucksacks.map(|r| r.double_item().unwrap().priority() as u32).sum();
    println!("{}", sum);
}

#[derive(Debug)]
struct Rucksack {
    compartment: [Compartment; 2]
}

impl Rucksack {
    fn double_item(&self) -> Option<&Item> {
        for l in &self.compartment[0].items {
            if self.compartment[1].items.contains(l) {
                return Some(l)
            }
        }
        None
    }
}

#[derive(Debug)]
struct Compartment {
    items: Vec<Item>
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct Item {
    c: u8
}

impl Item {
    fn priority(&self) -> u8 {
        match self.c {
            b'a' ..= b'z' => 1 + self.c - b'a',
            b'A' ..= b'Z' => 27 + self.c - b'A',
            _ => 0
        }
    }
}

impl From<u8> for Item {
    fn from(c: u8) -> Self {
        Item { c }
    }
}

impl FromStr for Rucksack {
    type Err  = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut items: Vec<Item> = Vec::with_capacity(s.len());
        for ch in s.bytes() {
            items.push(ch.into());
        }
        let (left, right) = items.split_at(items.len() / 2);
        Ok(Rucksack {
            compartment: [
                Compartment { items: left.to_vec() },
                Compartment { items: right.to_vec() },
            ]
        })
    }
}
