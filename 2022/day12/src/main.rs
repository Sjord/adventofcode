use std::{env, fs};

use grid::Grid;
use petgraph::{prelude::{GraphMap, DiGraphMap}, algo::dijkstra, dot::Dot};

fn main() {
    let fname = env::args().nth(1).unwrap();
    let contents = fs::read_to_string(fname).unwrap();
    let mut grid = Grid::<u8>::new(0, 0);
    for l in contents.lines() {
        let b : Vec<u8> = l.as_bytes().into_iter().map(|b| b.clone()).collect();
        grid.push_row(b);
    }

    let start = search(&grid, b'S').unwrap();
    let end = search(&grid, b'E').unwrap();
    *grid.get_mut(start.0, start.1).unwrap() = b'a';
    *grid.get_mut(end.0, end.1).unwrap() = b'z';

    let graph = to_graph(&grid);
    let paths = dijkstra(&graph, end, None, |_| 1);
    let min_path = paths.iter().filter(|(c, _)| grid.get(c.0, c.1) == Some(&b'a')).min_by_key(|(_, p)| *p);
    dbg!(min_path);
}

fn search(grid: &Grid<u8>, needle: u8) -> Option<(usize, usize)> {
    for y in 0..grid.rows() {
        for x in 0..grid.cols() {
            if grid.get(y, x) == Some(&needle) {
                return Some((y, x));
            }
        }
    }
    return None;
}

fn to_graph(grid: &Grid<u8>) -> DiGraphMap<(usize, usize), i8> {
    let mut graph : DiGraphMap<(usize, usize), i8> = DiGraphMap::new();
    let deltas: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    for y in 0..grid.rows() {
        for x in 0..grid.cols() {
            let coord = (y, x);
            let height = grid.get(y, x).unwrap();
            for (dy, dx) in deltas {
                let dest_y = y.checked_add_signed(dy);
                let dest_x = x.checked_add_signed(dx);
                if let (Some(dest_y), Some(dest_x)) = (dest_y, dest_x) {
                    let dest = (dest_y, dest_x);
                    let dest_height = grid.get(dest_y, dest_x);
                    if let Some(dest_height) = dest_height {
                        let uphil : i8 = *height as i8 - *dest_height as i8;
                        if uphil <= 1 {
                            // println!("{:?} -> {:?} {}", coord, dest, uphil);
                            graph.add_edge(coord, dest, uphil);
                        }
                    }
                }
            }
        }
    }
    graph
}
