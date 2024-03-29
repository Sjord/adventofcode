use std::{collections::HashMap, env, fs};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self as cc, alpha1},
    IResult, multi::separated_list1, sequence::tuple,
};
use petgraph::prelude::UnGraphMap;
use petgraph::{
    algo::dijkstra,
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
    let interesting_valves = graph.nodes().filter(|n| n.flow_rate > 0).collect();
    let splits = possible_splits(interesting_valves);
    let optimal = splits.into_iter().map(|(me_valves, el_valves)| {
        let mut me_search = WalkState {
            position: start,
            minutes_left: 26,
            released_pressure: 0,
            pending_valves: me_valves,
            distance_map: &distance_map,
        };
        let mut el_search = WalkState {
            position: start,
            minutes_left: 26,
            released_pressure: 0,
            pending_valves: el_valves,
            distance_map: &distance_map,
        };
        me_search.search() + el_search.search()
    }).max();
    dbg!(optimal);
}

fn possible_splits(items: Vec<Valve>) -> Vec<(Vec<Valve>, Vec<Valve>)> {
    let count = (1 << items.len()) / 2;
    (0..count).map(|i| {
        let mut a = Vec::new();
        let mut b = Vec::new();
        for (j, item) in items.iter().enumerate() {
            if (i & (1 << j)) != 0 {
                a.push(*item);
            } else {
                b.push(*item);
            }
        }
        (a, b)
    }).collect()
}

#[derive(Clone)]
struct WalkState<'a> {
    position: Valve<'a>,
    minutes_left: i32,
    released_pressure: i32,
    pending_valves: Vec<Valve<'a>>,
    distance_map: &'a HashMap<(Valve<'a>, Valve<'a>), i32>,
}

impl<'a> WalkState<'a> {
    fn search(&mut self) -> i32 {
        if self.minutes_left <= 0 {
            return self.released_pressure;
        }

        let candidates = self.pending_valves.iter().filter(|n| {
            *n != &self.position
                && n.flow_rate > 0
                && 1 + self.distance(&self.position, n) < self.minutes_left
        });
        candidates
            .map(|c| {
                let mut state = self.clone();
                state.travel_and_turn(*c);
                state.search()
            })
            .max()
            .or(Some(self.released_pressure))
            .unwrap()
    }

    fn distance(&self, from: &Valve<'a>, dest: &Valve<'a>) -> i32 {
        self.distance_map[&(*from, *dest)]
    }

    fn travel_and_turn(&mut self, destination: Valve<'a>) {
        let distance = self.distance(&self.position, &destination);
        self.minutes_left -= distance + 1;
        self.released_pressure += self.minutes_left * destination.flow_rate;
        self.position = destination;
        self.pending_valves.retain(|v| v != &destination);
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
