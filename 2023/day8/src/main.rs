use std::{env, fs};

use nom::{bytes::complete::tag, character::complete::{alphanumeric1, newline, one_of}, multi::{many_till, separated_list1}, sequence::tuple, IResult};

use  num::integer::lcm;
use num::Integer;

fn main() {
    let fname = env::args().nth(1).unwrap();
    let contents = fs::read_to_string(fname).unwrap();
    let map = parse(&contents);
    dbg!(map.walk());
}

#[derive(Debug)]
struct Map {
    directions: Vec<Direction>,
    nodes: Vec<Node>
}

#[derive(Debug)]
struct Node {
    name: String,
    left: String,
    right: String
}

impl Node {
    fn is_start(&self) -> bool {
        self.name.ends_with('A')
    }

    fn is_end(&self) -> bool {
        self.name.ends_with('Z')
    }

    fn get_leg(&self, direction: &Direction) -> &str {
        match direction {
            Direction::Left => &self.left,
            Direction::Right => &self.right
        }
    }
}

#[derive(Debug)]
enum Direction {
    Left,
    Right
}

impl Map {
    fn walk(&self) -> i64 {
        let starts = self.nodes.iter().filter(|n| n.is_start());
        let steps = starts.map(|s| self.walk_from_node(s));
        steps.reduce(|a, b| a.lcm(&b)).unwrap()
    }

    fn walk_from_node(&self, node: &Node) -> i64{
        let mut step_ctr = 0;
        let mut dir_pointer = 0;
        let mut current = node;
        while !current.is_end() {
            let direction = &self.directions[dir_pointer];
            let next = current.get_leg(direction);
            current = self.find_node(next);
            dir_pointer = (dir_pointer + 1) % self.directions.len();
            step_ctr += 1;
        }
        step_ctr
    }

    fn find_node(&self, name: &str) -> &Node {
        self.nodes.iter().find(|n| n.name == name).expect(name)
    }
}

fn parse_direction(input: &str) -> IResult<&str, Direction> {
    let (remainder, ch) = one_of("LR")(input)?;
    let dir = match ch {
        'L' => Direction::Left,
        'R' => Direction::Right,
        _ => unreachable!(),
    };
    Ok((remainder, dir))
}

fn parse_node(input: &str) -> IResult<&str, Node> {
    let mut parser = tuple((alphanumeric1, tag(" = ("), alphanumeric1, tag(", "), alphanumeric1, tag(")")));
    let (remainder, parsed) = parser(input)?;
    let (name, _, left, _, right, _) = parsed;
    Ok((remainder, Node {
        name: name.to_owned(),
        left: left.to_owned(),
        right: right.to_owned()
    }))
}

fn parse(input: &str) -> Map {
    let mut parser = tuple((
        many_till(parse_direction, newline),
        newline,
        separated_list1(newline, parse_node)
    ));
    let (remainder, parsed) = parser(input).unwrap();
    let ((directions, _), _, nodes) = parsed;
    Map {
        directions,
        nodes
    }
}