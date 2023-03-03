use std::{env, fs};

use grid::Grid;

fn main() {
    let fname = env::args().nth(1).unwrap();
    let contents = fs::read_to_string(fname).unwrap();
    let mut grid = Grid::<u8>::new(0, 0);
    for l in contents.lines() {
        let b : Vec<u8> = l.as_bytes().into_iter().map(|b| b.clone()).collect();
        grid.push_row(b);
    }
    let mut visibleCount = 0;
    for y in 0..grid.rows() {
        for x in 0..grid.cols() {
            let score = scenic_score(&grid, x, y);
            println!("visible trees x{} y{} score{}", x, y, score);
            if score > visibleCount {
                visibleCount = score;
            }
        }
    }
    println!("max score: {}", visibleCount);
}

fn scenic_score(grid: &Grid<u8>, x: usize, y: usize) -> u32 {
    let height = *grid.get(y, x).unwrap();

    let mut leftVisible = 0;
    for left in (0..x).rev() {
        leftVisible += 1;
        if *grid.get(y, left).unwrap() >= height {
            break;
        }
    }

    let mut rightVisible = 0;
    for right in (x + 1)..grid.cols() {
        rightVisible += 1;
        if *grid.get(y, right).unwrap() >= height {
            break;
        }
    }

    let mut topVisible = 0;
    for top in (0..y).rev() {
        topVisible += 1;
        if *grid.get(top, x).unwrap() >= height {
            break;
        }
    }

    let mut bottomVisible = 0;
    for bottom in (y + 1)..grid.rows() {
        bottomVisible += 1;
        if *grid.get(bottom, x).unwrap() >= height {
            break;
        }
    }

    leftVisible * rightVisible * topVisible * bottomVisible
}
