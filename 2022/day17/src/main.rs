use std::env;
use std::fs;

fn main() {
    let fname = env::args().nth(1).unwrap();
    let contents = fs::read_to_string(fname).unwrap();
    let jets = contents.chars().map(|c| {
        match c {
            '<' => Direction::Left,
            '>' => Direction::Right,
            _ => panic!("{}", c)
        }
    });

    let shapes = vec!([
        vec!([ // __
            0, 0, 0, 0,
            0, 0, 0, 0,
            0, 0, 0, 0,
            1, 1, 1, 1
        ]),
        vec!([ // +
            0, 0, 0, 0,
            0, 1, 0, 0,
            1, 1, 1, 0,
            0, 1, 0, 0
        ]),
        vec!([ // ⅃
            0, 0, 0, 0,
            0, 0, 1, 0,
            0, 0, 1, 0,
            1, 1, 1, 0
        ]),
        vec!([ // |
            1, 0, 0, 0,
            1, 0, 0, 0,
            1, 0, 0, 0,
            1, 0, 0, 0
        ]),
        vec!([ // 𐌎
            0, 0, 0, 0,
            0, 0, 0, 0,
            1, 1, 0, 0,
            1, 1, 0, 0
        ]),
    ]);

    let stack = Stack::new();
    for i in 0..2022 {
        let shape = shapes.get(i % shapes.len());
    }
}

#[derive(Debug)]
enum Direction {
    Left,
    Right
}

struct Stack {
    rows: Vec<Vec<bool>>
}

impl Stack {
    fn new() -> Stack {
        Stack {
            rows: Vec::new()
        }
    }

    fn top_nonempty_row(&self) -> usize {
        for (i, row) in self.rows.iter().enumerate() {
            if row.iter().all(|i| !i) {
                return i - 1;
            }
        }
        self.rows.len()
    }
}
