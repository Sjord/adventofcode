use std::{env, fs, vec};

fn main() {
    let fname = env::args().nth(1).unwrap();
    let contents = fs::read(fname).unwrap();
    let pipes = contents.iter().map(|c| Pipe { char: *c }).collect();
    let width = 1 + contents.iter().position(|c| *c == b'\n').unwrap();
    let grid = Grid { pipes, width };
    let steps = grid.walk();
    dbg!((steps + 1) / 2);
}

#[derive(Debug)]
struct Grid {
    pipes: Vec<Pipe>,
    width: usize
}

impl Grid {
    fn find_start(&self) -> Coordinate {
        let pos = self.pipes.iter().position(|p| p.is_start()).unwrap();
        let x = pos % self.width;
        let y = pos / self.width;
        Coordinate { x, y }
    }

    fn walk(&self) -> usize {
        let mut step_count = 0;
        let mut current = self.find_start();
        let mut out_direction = self.start_direction(&current);

        loop {
            current = current.step(out_direction);
            let pipe = self.get(&current);

            if self.get(&current).is_start() {
                // Loop complete
                break;
            }

            let in_direction: Direction = out_direction.opposite();
            out_direction = pipe.traverse(in_direction);
            step_count += 1;
        }
        step_count
    }

    fn start_direction(&self, start: &Coordinate) -> Direction {
        for out_direction in Direction::all().into_iter() {
            let coord = start.step(out_direction);
            let pipe = self.get(&coord);
            let in_direction = out_direction.opposite();
            if let Some((from, to)) = pipe.directions() {
                if from == in_direction || to == in_direction {
                    return out_direction;
                }
            }
        }
        panic!("Could not determine where to go from the start");
    }

    fn get(&self, coord: &Coordinate) -> &Pipe {
        let offset = coord.y * self.width + coord.x;
        &self.pipes[offset]
    }
}

#[derive(Debug)]
struct Pipe {
    char: u8
}

impl Pipe {
    fn is_start(&self) -> bool {
        return self.char == b'S';
    }

    fn directions(&self) -> Option<(Direction, Direction)> {
        let pipes = vec![
            (b'|', Direction::North, Direction::South),
            (b'-', Direction::East, Direction::West),
            (b'L', Direction::North, Direction::East),
            (b'J', Direction::North, Direction::West),
            (b'7', Direction::South, Direction::West),
            (b'F', Direction::South, Direction::East),
        ];
        for (char, from, to) in pipes {
            if char == self.char {
                return Some((from, to));
            }
        }
        None
    }

    fn traverse(&self, incoming: Direction) -> Direction {
        if let Some((from, to)) = self.directions() {
            if from == incoming {
                return to;
            } else if to == incoming {
                return from;
            } else {
                panic!("Pipe {:?} expected incoming from {:?} or {:?} but actually from {:?}", &self, from, to, incoming);
            }
        }
        panic!("Unexpected pipe: {}", self.char);
    }
}

#[derive(Debug)]
struct Coordinate {
    x: usize,
    y: usize
}

impl Coordinate {
    fn step(&self, direction: Direction) -> Coordinate {
        match direction {
            Direction::North => Coordinate { x: self.x, y: self.y - 1 },
            Direction::East => Coordinate { x: self.x + 1, y: self.y },
            Direction::South => Coordinate { x: self.x, y: self.y + 1},
            Direction::West => Coordinate { x: self.x - 1, y: self.y },
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West
}

impl Direction {
    fn opposite(&self) -> Direction {
        match &self {
            Direction::North => Direction::South,
            Direction::East => Direction::West,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
        }
    }

    fn all() -> Vec<Direction> {
        vec![Direction::North, Direction::East, Direction::South, Direction::West]
    }
}