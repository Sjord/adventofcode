use std::{env, fs};
use nom::bytes::complete::tag;
use nom::multi::separated_list1;
use nom::sequence::tuple;
use nom::character::complete::{alpha1, i32 as cc_i32, newline, space1};
use nom::combinator::all_consuming;
use nom::error::Error;
use nom::Finish;

fn main() {
    let fname = env::args().nth(1).unwrap();
    let contents = fs::read_to_string(fname).unwrap();
    let almanac = from_str(&contents).unwrap();
    dbg!(almanac);
}

#[derive(Debug)]
struct Almanac {
    seeds: Vec<i32>,
    sections: Vec<Section>
}

#[derive(Debug)]
struct Section {
    from: String,
    to: String,
    maps: Vec<Map>
}

#[derive(Debug)]
struct Map {
    dest: i32,
    src: i32,
    len: i32
}

fn from_str(s: &str) -> Result<Almanac, Error<&str>> {
    let header = tuple((
        alpha1, tag("-to-"), alpha1, tag(" map:")
    ));
    let map = tuple((
        cc_i32, space1, cc_i32, space1, cc_i32
    ));
    let mut parser = all_consuming(tuple((
        tag("seeds:"), space1, separated_list1(space1, cc_i32),
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