use std::{collections::HashMap, cmp::{max, Ordering}, fmt::Display, env, fs};

use nom::{multi::separated_list0, character::complete as cc, bytes::complete::tag, sequence::{preceded, tuple}, IResult};

fn main() {
    let fname = env::args().nth(1).unwrap();
    let contents = fs::read_to_string(fname).unwrap();
    let lines = lines(&contents);
    
    let mut grid = SparseGrid::new();
    for l in lines {
        grid.draw_solid(&l);
    }
    
    let mut sand_count = 0;
    'all_sand: loop {
        let mut sand_pos = Coord { x: 500, y: 0 };
        'one_grain: loop {
            let below = sand_pos.directly_under();
            if grid.get(&below) == Fill::None {
                sand_pos = below;
            } else {
                let left_below = below.directly_left();
                if grid.get(&left_below) == Fill::None {
                    sand_pos = left_below;
                } else {
                    let right_below = below.directly_right();
                    if grid.get(&right_below) == Fill::None {
                        sand_pos = right_below;
                    } else {
                        grid.put_sand(sand_pos);
                        break 'one_grain;
                    }
                }
            }
            if sand_pos.y > 1000 {
                // in abyss
                break 'all_sand;
            }
        }
        sand_count += 1;
    }

    grid.print();
    println!("{}", sand_count);
}

fn lines(input: &str) -> Vec<Line> {
    let (i, lines) = separated_list0(tag("\n"), cont_line)(input).unwrap();
    lines.into_iter().flatten().collect()
}

fn cont_line(input: &str) -> IResult<&str, Vec<Line>> {
    let (i, coords) = separated_list0(tag(" -> "), coord)(input)?;
    let lines = coords.windows(2).map(|w| Line { from: w[0].clone(), to: w[1].clone() }).collect();
    Ok((i, lines))
}

fn coord(input: &str) -> IResult<&str, Coord> {
    let (i, (x, _, y)) = tuple((cc::i32, tag(","), cc::i32))(input)?;
    Ok((i, Coord{ x, y }))
}

#[derive(Clone, Eq, PartialEq, Hash)]
struct Coord {
    x: i32,
    y: i32
}

impl Coord {
    fn directly_under(&self) -> Coord {
        Coord { x: self.x, y: self.y + 1}
    }

    fn directly_left(&self)  -> Coord {
        Coord { x: self.x - 1, y: self.y}
    }

    fn directly_right(&self)  -> Coord {
        Coord { x: self.x + 1, y: self.y}
    }
}

struct Line {
    from: Coord,
    to: Coord
}

impl PartialOrd for Coord {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let dx = self.x.cmp(&other.x);
        let dy = self.y.cmp(&other.y);
        match (dx, dy) {
            (Ordering::Less, Ordering::Less) => Some(Ordering::Less),
            (Ordering::Equal, Ordering::Equal) => Some(Ordering::Equal),
            (Ordering::Greater, Ordering::Greater) => Some(Ordering::Greater),
            _ => None
        }
    }
}

struct SparseGrid {
    foo: HashMap<Coord, Fill>,
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum Fill {
    None,
    Solid,
    Sand,
}

impl Display for Fill {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Fill::None => ".",
            Fill::Solid => "#",
            Fill::Sand => "o"
        })
    }
}

impl SparseGrid {
    fn new() -> SparseGrid {
        SparseGrid { foo: HashMap::new() }
    }

    fn draw_solid(&mut self, line: &Line) {
        let dx = line.to.x - line.from.x;
        let dy = line.to.y - line.from.y;

        let steps = max(dx.abs(), dy.abs());
        let mut x = line.from.x;
        let mut y = line.from.y;
        for i in 0..=steps {
            self.foo.insert(Coord{x, y}, Fill::Solid);
            x += dx.signum();
            y += dy.signum();
        }
    }

    fn put_sand(&mut self, coord: Coord) {
        self.foo.insert(coord, Fill::Sand);
    }

    fn get(&self, coord: &Coord) -> Fill {
        let f = self.foo.get(coord);
        match f {
            None => Fill::None,
            Some(f) => *f
        }
    }

    fn print(&self) {
        let min_x = self.foo.iter().map(|(c, _)| c.x).min().unwrap();
        let max_x = self.foo.iter().map(|(c, _)| c.x).max().unwrap();
        let min_y = self.foo.iter().map(|(c, _)| c.y).min().unwrap();
        let max_y = self.foo.iter().map(|(c, _)| c.y).max().unwrap();
        for y in min_y..=max_y {
            print!("{: >3} ", y);
            for x in min_x..=max_x {
                print!("{}", self.get(&Coord{ x, y }));
            }
            println!();
        }
    }
}
