use std::{env, fs};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self as cc, alpha1},
    multi::{separated_list0, separated_list1},
    sequence::{preceded, tuple},
    IResult,
};

fn main() {
    let fname = env::args().nth(1).unwrap();
    let contents = fs::read_to_string(fname).unwrap();
    let records = parse(&contents);
    dbg!(records);
}

fn parse(input: &str) -> Vec<ValveRecord> {
    separated_list1(tag("\n"), parse_valve)(input).unwrap().1
}

fn parse_valve(input: &str) -> IResult<&str, ValveRecord> {
    let (i, (_, name, _, flow_rate, _, tunnels)) =
    tuple((
        tag("Valve "),
        valve_name,
        tag(" has flow rate="),
        cc::i32,
        alt((
            tag("; tunnels lead to valves "),
            tag("; tunnel leads to valve "),
        )),
        valve_names,
    ))(input)?;
    Ok((i, ValveRecord { name, flow_rate, tunnels }))
}

fn valve_names(input: &str) -> IResult<&str, Vec<&str>> {
    separated_list1(tag(", "), valve_name)(input)
}

fn valve_name(input: &str) -> IResult<&str, &str> {
    alpha1(input)
}

#[derive(Debug)]
struct ValveRecord<'a> {
    name: &'a str,
    flow_rate: i32,
    tunnels: Vec<&'a str>
}
