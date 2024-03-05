use std::collections::HashMap;
use std::iter::empty;
use std::{env, fs};
use nom::bytes::complete::tag;
use nom::multi::separated_list1;
use nom::sequence::tuple;
use nom::character::complete::{alpha1, i64 as cc_i64, newline, space1};
use nom::combinator::all_consuming;
use nom::error::Error;
use nom::Finish;

fn main() {
    let fname = env::args().nth(1).unwrap();
    let contents = fs::read_to_string(fname).unwrap();
    let almanac = from_str(&contents).unwrap();
    let min = almanac.lowest_location();
    dbg!(min);
}

#[derive(Debug)]
struct Almanac {
    seeds: Vec<i64>,
    sections: Vec<Section>
}

impl Almanac {
    fn get_section_from(&self, from: &str) -> &Section {
        self.sections.iter().find(|s| s.from == from).unwrap()
    }

    fn find_location(&self, seed: i64) -> i64 {
        let mut item = "seed";
        let mut number = seed;
        loop {
            let section = self.get_section_from(item);
            number = section.map_number(number);
            item = &section.to;
            if item == "location" {
                return number;
            }
        }
    }

    fn find_seed(&self, location: i64) -> i64 {
        let map : HashMap<_, _> = self.sections.iter().map(|s| (&s.to, s)).collect();

        let mut item = "location";
        let mut number = location;
        loop {
            let section = map[&item.to_owned()];
            number = section.map_number_reverse(number);
            item = &section.from;
            if item == "seed" {
                return number;
            }
        }
    }

    fn seeds(&self) -> Box<dyn Iterator<Item=i64>> {
        let mut result : Box<dyn Iterator<Item=i64>> = Box::new(empty());
        for slice in self.seeds.chunks(2).into_iter() {
            let start = slice[0];
            let len = slice[1];
            let range = start..(start + len);
            result = Box::new(result.chain(range));
        }
        result
    }

    fn has_seed(&self, seed: i64) -> bool {
        for slice in self.seeds.chunks(2).into_iter() {
            let start = slice[0];
            let len = slice[1];
            if seed >= start && seed <= (start + len) {
                return true;
            }
        }
        false
    }

    fn locations(&self) -> Vec<i64> {
        self.seeds().map(|s| self.find_location(s)).collect()
    }

    fn lowest_location(&self) -> Option<i64> {
        let possible_seeds = self.seeds();
        for location in 0..20000000 {
            let seed = self.find_seed(location);
            if self.has_seed(seed) {
                return Some(location);
            }
        }
        None
    }
}

#[derive(Debug)]
struct Section {
    from: String,
    to: String,
    maps: Vec<Map>
}

impl Section {
    fn map_number(&self, num: i64) -> i64 {
        for map in self.maps.iter() {
            if let Some(dst) = map.map_number(num) {
                return dst;
            }
        }
        num
    }

    fn map_number_reverse(&self, num: i64) -> i64 {
        for map in self.maps.iter() {
            if let Some(dst) = map.map_number_reverse(num) {
                return dst;
            }
        }
        num
    }
}

#[derive(Debug)]
struct Map {
    dest: i64,
    src: i64,
    len: i64
}

impl Map {
    fn map_number(&self, num: i64) -> Option<i64> {
        if num >= self.src && num <= self.src + self.len {
            return Some(num + self.dest - self.src);
        }
        None
    }

    fn map_number_reverse(&self, num: i64) -> Option<i64> {
        if num >= self.dest && num <= self.dest + self.len {
            return Some(num + self.src - self.dest);
        }
        None
    }
}

fn from_str(s: &str) -> Result<Almanac, Error<&str>> {
    let header = tuple((
        alpha1, tag("-to-"), alpha1, tag(" map:")
    ));
    let map = tuple((
        cc_i64, space1, cc_i64, space1, cc_i64
    ));
    let mut parser = all_consuming(tuple((
        tag("seeds:"), space1, separated_list1(space1, cc_i64),
        tag("\n\n"),
        separated_list1(tag("\n\n"), tuple((
            header, newline, separated_list1(newline, map)
        )))
    )));
    let (_, parsed) = parser(s).finish()?;
    let (_, _, seeds, _, sections) = parsed;
    let sections = sections.iter().map(|s| Section {
        from: s.0.0.to_owned(),
        to: s.0.2.to_owned(),
        maps: s.2.iter().map(|m| Map {
            dest: m.0,
            src: m.2,
            len: m.4
        }).collect()
    }).collect();
    Ok(Almanac {
        seeds,
        sections
    })
}