use std::{env, fs, io::{empty, Read}};

fn main() {
    let fname = env::args().nth(1).unwrap();
    let contents = fs::read(fname).unwrap();
    let universe = read_input(&contents);
    let universe = universe.expand();
    dbg!(universe.distances());
}

fn read_input(input: &Vec<u8>) -> Universe {
    let mut x = 0;
    let mut y = 0;
    let mut galaxies = Vec::new();
    for ch in input {
        match ch {
            b'#' => { galaxies.push(Coordinate { x, y }); x += 1; }
            b'.' => { x += 1;}
            b'\n' => { x = 0; y += 1 }
            _ => { panic!("Unexpected character {}", ch); }
        }
    }
    Universe { galaxies }
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
struct Coordinate {
    x: usize,
    y: usize
}

impl Coordinate {
    fn distance(&self, other: &Coordinate) -> usize {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}

#[derive(Debug)]
struct Universe {
    galaxies: Vec<Coordinate>,
}

impl Universe {
    fn expand(&self) -> Universe {
        let expand_ratio = 1000000;
        let width = 1 + self.galaxies.iter().map(|c| c.x).max().unwrap();
        let height = 1 + self.galaxies.iter().map(|c| c.y).max().unwrap();

        let empty_cols : Vec<_> = (0..width).filter(|x| !self.galaxies.iter().any(|c| c.x == *x)).collect();
        let empty_rows : Vec<_> = (0..height).filter(|y| !self.galaxies.iter().any(|c| c.y == *y)).collect();
        
        let galaxies = self.galaxies.iter().map(|c| {
            Coordinate {
                x: c.x + empty_cols.iter().filter(|x| **x < c.x).count() * (expand_ratio - 1),
                y: c.y + empty_rows.iter().filter(|y| **y < c.y).count() * (expand_ratio - 1),
            }
        }).collect();
        Universe { galaxies }
    }

    fn distances(&self) -> usize {
        self.galaxies.iter().map(|c1| {
            self.galaxies.iter().filter(|c2| *c2 > c1).map(|c2| {
                c1.distance(c2)
            }).sum::<usize>()
        }).sum()
    }
}