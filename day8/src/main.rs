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
            if is_visible(&grid, x, y) {
                visibleCount += 1;
            }
        }
    }
    println!("visible trees: {}", visibleCount);
}

fn is_visible(grid: &Grid<u8>, x: usize, y: usize) -> bool {
    let height = *grid.get(y, x).unwrap();

    let mut leftVisible = true;
    for left in 0..x {
        if *grid.get(y, left).unwrap() >= height {
            leftVisible = false;
        }
    }
    println!("x {} y {} leftvis {}", x, y, leftVisible);

    let mut rightVisible = true;
    for right in (x + 1)..grid.cols() {
        if *grid.get(y, right).unwrap() >= height {
            rightVisible = false;
        }
    }

    let mut topVisible = true;
    for top in 0..y {
        if *grid.get(top, x).unwrap() >= height {
            topVisible = false;
        }
    }

    let mut bottomVisible = true;
    for bottom in (y + 1)..grid.rows() {
        if *grid.get(bottom, x).unwrap() >= height {
            bottomVisible = false;
        }
    }

    leftVisible || rightVisible || topVisible || bottomVisible
}
