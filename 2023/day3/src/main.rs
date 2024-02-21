use std::{cmp::max, env, fs, str};

fn main() {
    let fname = env::args().nth(1).unwrap();
    let s = fs::read_to_string(fname).unwrap();
    let grid = CharGrid::from_string(&s);
    let answer : i32 = grid.get_part_numbers().iter().sum();
    dbg!(answer);
}

#[derive(Debug)]
struct CharGrid {
    chars: Vec<GridValue>,
    line_len: usize,
    line_count: usize,
}

#[derive(Debug, Clone, Copy)]
enum GridValue {
    Empty,
    Symbol(u8),
    Number(u8, i32),
}

impl CharGrid {
    fn from_string(s: &str) -> CharGrid {
        let mut line_len = 0;
        let mut line_count = 1;
        let mut chars = Vec::with_capacity(s.len());
        let mut seen_number_count = 0;

        for ch in s.as_bytes() {
            if seen_number_count > 0 && !ch.is_ascii_digit() {
                // The last `seen_number_count` entries are numbers, convert them to int
                let from = chars.len() - seen_number_count;
                let to = chars.len();
                let x = &chars.as_slice()[from..]
                    .iter()
                    .map(|v| {
                        if let GridValue::Number(ch, _) = v {
                            *ch
                        } else {
                            panic!("Unexpected GridValue")
                        }
                    });
                let z : Vec<u8> = x.clone().collect();
                let num = str::from_utf8(&z).unwrap();
                let num : i32 = num.parse().unwrap();
                for i in from..to {
                    if let GridValue::Number(ch, _) = chars[i] {
                        chars[i] = GridValue::Number(ch, num)
                    } else {
                        panic!("Unexpected GridValue")
                    }
                }
                seen_number_count = 0;
            }

            if *ch == b'\n' {
                line_len = 0;
                line_count += 1;
            } else {
                if *ch == b'.' {
                    chars.push(GridValue::Empty);
                } else if ch.is_ascii_digit() {
                    let num = GridValue::Number(*ch, 0);
                    chars.push(num);
                    seen_number_count += 1;
                } else {
                    chars.push(GridValue::Symbol(*ch));
                }
                line_len += 1;
            }
        }
        CharGrid { line_len, chars, line_count }
    }

    fn get(&self, x: usize, y: usize) -> GridValue {
        let i = y * self.line_len + x;
        if i >= self.chars.len() {
            GridValue::Empty
        } else {
            self.chars[i]
        }
    }

    fn get_part_numbers(&self) -> Vec<i32> {
        let mut result = Vec::new();
        let mut last_number = 0;
        for (i, c) in self.chars.iter().enumerate() {
            if let GridValue::Number(_, num) = c {
                if *num != last_number {
                    let x = i % self.line_len;
                    let y = i / self.line_len;
                    let num_len = (1 + num.ilog10()) as usize;
                    
                    let left = if x == 0 { 0 } else { x - 1 };
                    let right = x + num_len;
                    let top = if y == 0 { 0 } else { y - 1 };
                    let bottom = y + 1;

                    let mut surrounded = false;
                    for xa in left..=right {
                        for ya in top..=bottom {
                            let val = self.get(xa, ya);
                            if let GridValue::Symbol(_) = val {
                                surrounded = true;
                            }
                        }
                    }

                    if surrounded {
                        result.push(*num);
                    }

                    last_number = *num;
                }
            }
        }
        result
    }
}
