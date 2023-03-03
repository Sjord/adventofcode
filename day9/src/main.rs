use std::cmp::max;
use std::collections::HashSet;
use std::{env, fs};

fn main() {
    let fname = env::args().nth(1).unwrap();
    let contents = fs::read_to_string(fname).unwrap();
    
    let mut r = Rope::new();
    let mut tail_visited = HashSet::new();
    
    for l in contents.lines() {
        let mut parts = l.split_ascii_whitespace();
        let dir = match parts.next().unwrap() {
            "U" => Direction::Up,
            "D" => Direction::Down,
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => panic!()
        };
        let count : i32 = parts.next().unwrap().parse().unwrap();

        for i in 0..count {
            r.move_head(&dir);
            tail_visited.insert(r.tail.clone());
        }
        println!("{:?}", r);
    }
    println!("tail visited positions: {}", tail_visited.len());
}

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
struct Coord {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Rope {
    head: Coord,
    tail: Coord
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

impl Rope {
    fn new() -> Self {
        Rope {
            head: Coord { x: 0, y: 0 },
            tail: Coord { x: 0, y: 0 },
        }
    }

    fn move_head(&mut self, dir: &Direction) {
        match dir {
            Direction::Up => self.head.y -= 1,
            Direction::Down => self.head.y += 1,
            Direction::Left => self.head.x -= 1,
            Direction::Right => self.head.x += 1,
        }
        if self.head.distance(&self.tail) > 1 {
            let xdiff = self.head.x - self.tail.x;
            let ydiff = self.head.y - self.tail.y;

            if self.head.x == self.tail.x {
                self.tail.y += ydiff / 2;
            } else if self.head.y == self.tail.y {
                self.tail.x += xdiff / 2;
            } else if (i32::abs(xdiff) > 1) {
                self.tail.x += xdiff / 2;
                self.tail.y += ydiff;
            } else if (i32::abs(ydiff) > 1) {
                self.tail.x += xdiff;
                self.tail.y += ydiff / 2;
            } else {
                panic!();
            }
        }
    }
}

impl Coord {
    fn distance(&self, other: &Coord) -> i32 {
        max(i32::abs(other.x - self.x), i32::abs(other.y - self.y))
    }
}
/*
  H
T
  */
