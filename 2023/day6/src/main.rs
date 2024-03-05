use std::{env, fs};

fn main() {
    let fname = env::args().nth(1).unwrap();
    let contents = fs::read_to_string(fname).unwrap();
    let mut lines = contents.lines().map(|l| l.split_ascii_whitespace());
    let time : i64 = lines.next().unwrap().skip(1).fold(String::new(), |a, b| a + b).parse().unwrap();
    let distance : i64 = lines.next().unwrap().skip(1).fold(String::new(), |a, b| a + b).parse().unwrap();

    let limits = calc_winning_limits(time, distance);
    let options = 1 + limits.1 - limits.0;
    dbg!(options);
}

fn calc_winning_limits(time: i64, distance: i64) -> (i64, i64) {
    let i = ((time.pow(2) - 4 * distance) as f64).sqrt();
    let i = i - 0.1; // we want to be faster, not equal to
    ((0.5 * (time as f64 - i)).ceil() as i64, (0.5 * (time  as f64 + i)).floor() as i64)
}