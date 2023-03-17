use std::{collections::HashMap, env, fs};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self as cc, alpha1},
    multi::{separated_list0, separated_list1},
    sequence::{preceded, tuple},
    IResult,
};
use petgraph::prelude::UnGraphMap;
use petgraph::{
    algo::dijkstra,
    dot::{Config, Dot},
};

fn main() {
    let fname = env::args().nth(1).unwrap();
    let contents = fs::read_to_string(fname).unwrap();
    let records = parse(&contents);

    let mut graph = UnGraphMap::<Valve, ()>::new();
    for r in &records {
        graph.add_node(Valve {
            name: r.name,
            flow_rate: r.flow_rate,
        });
    }
    for r in records {
        for tunnel in r.tunnels {
            let from = graph.nodes().find(|n| n.name == r.name).unwrap();
            let to = graph.nodes().find(|n| n.name == tunnel).unwrap();
            graph.add_edge(from, to, ());
        }
    }

    let mut distance_map = HashMap::<(Valve, Valve), i32>::new();
    for start in graph.nodes() {
        let distances = dijkstra(&graph, start, None, |_| 1);
        for (dest, distance) in distances.into_iter() {
            distance_map.insert((start, dest), distance);
        }
    }

    let start = graph.nodes().find(|n| n.name == "AA").unwrap();
    let mut search = WalkState {
        graph,
        position: start,
        minutes_left: 30,
        released_pressure: 0,
        open_valves: Vec::new(),
        distance_map: &distance_map,
    };
    let optimal = search.search();
    dbg!(optimal);
}

#[derive(Clone)]
struct WalkState<'a> {
    graph: UnGraphMap<Valve<'a>, ()>,
    position: Valve<'a>,
    minutes_left: i32,
    released_pressure: i32,
    open_valves: Vec<Valve<'a>>,
    distance_map: &'a HashMap<(Valve<'a>, Valve<'a>), i32>,
}

impl<'a> WalkState<'a> {
    fn search(&mut self) -> i32 {
        if self.minutes_left <= 0 {
            return self.released_pressure;
        }

        let candidates = self
            .graph
            .nodes()
            .filter(|n| 
                *n != self.position
                && n.flow_rate > 0
                && !self.open_valves.contains(n)
                && self.distance(&self.position, n) < self.minutes_left);
        candidates
            .map(|c| {
                let mut state = self.clone();
                state.walk_to(c);
                state.turn_valve(c);
                state.search()
            })
            .max()
            .or_else(|| {
                while self.minutes_left > 0 {
                    self.minute_passed();
                }
                Some(self.released_pressure)
            })
            .unwrap()
    }

    fn distance(&self, from: &Valve<'a>, dest: &Valve<'a>) -> i32 {
        self.distance_map[&(*from, *dest)]
    }

    fn walk_to(&mut self, destination: Valve<'a>) {
        let distance = self.distance(&self.position, &destination);
        for step in 0..distance {
            self.minute_passed();
        }
        self.position = destination;
    }

    fn turn_valve(&mut self, valve: Valve<'a>) {
        self.minute_passed();
        self.open_valves.push(valve);
    }

    fn minute_passed(&mut self) {
        self.minutes_left -= 1;
        if self.minutes_left >= 0 {
            self.released_pressure += self.open_valves.iter().map(|v| v.flow_rate).sum::<i32>();
        }
    }
}

fn parse(input: &str) -> Vec<ValveRecord> {
    separated_list1(tag("\n"), parse_valve)(input).unwrap().1
}

fn parse_valve(input: &str) -> IResult<&str, ValveRecord> {
    let (i, (_, name, _, flow_rate, _, tunnels)) = tuple((
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
    Ok((
        i,
        ValveRecord {
            name,
            flow_rate,
            tunnels,
        },
    ))
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
    tunnels: Vec<&'a str>,
}

#[derive(Debug, Hash, Clone, Copy, Eq, PartialOrd)]
struct Valve<'a> {
    name: &'a str,
    flow_rate: i32,
}

impl<'a> Ord for Valve<'a> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.name.cmp(&other.name)
    }
}

impl<'a> PartialEq for Valve<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}
