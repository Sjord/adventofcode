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
            tail_visited.insert(r.parts[9].clone());
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
    parts: [Coord; 10]
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
            parts: [Coord { x: 0, y: 0 }; 10] 
        }
    }

    fn move_head(&mut self, dir: &Direction) {
        match dir {
            Direction::Up => self.parts[0].y -= 1,
            Direction::Down => self.parts[0].y += 1,
            Direction::Left => self.parts[0].x -= 1,
            Direction::Right => self.parts[0].x += 1,
        }
        for i in 1..10 {
            self.move_part(i)
        }

    }
    
    fn move_part(&mut self, i: usize) {
        let head = self.parts[i - 1];
        let mut tail = self.parts[i];

        if head.distance(&tail) > 1 {
            let xdiff = head.x - tail.x;
            let ydiff = head.y - tail.y;

            if head.x == tail.x {
                tail.y += ydiff / 2;
            } else if head.y == tail.y {
                tail.x += xdiff / 2;
            } else if (i32::abs(xdiff) > 1) && (i32::abs(ydiff) > 1) {
                tail.y += ydiff / 2;
                tail.x += xdiff / 2;
            } else if (i32::abs(xdiff) > 1) {
                tail.x += xdiff / 2;
                tail.y += ydiff;
            } else if (i32::abs(ydiff) > 1) {
                tail.x += xdiff;
                tail.y += ydiff / 2;
            } else {
                panic!();
            }
            self.parts[i] = tail;
        }
    }
}

impl Coord {
    fn distance(&self, other: &Coord) -> i32 {
        max(i32::abs(other.x - self.x), i32::abs(other.y - self.y))
    }
}
