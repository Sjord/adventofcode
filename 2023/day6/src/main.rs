use std::{env, fs};

fn main() {
    let fname = env::args().nth(1).unwrap();
    let contents = fs::read_to_string(fname).unwrap();
    let mut lines = contents.lines().map(|l| l.split_ascii_whitespace());
    let times : Vec<i64> = lines.next().unwrap().skip(1).map(|t| t.parse().unwrap()).collect();
    let distances : Vec<i64> = lines.next().unwrap().skip(1).map(|t| t.parse().unwrap()).collect();

    let mut result = 1;
    for i in 0..times.len() {
        let time = times[i];
        let distance = distances[i];
        let limits = calc_winning_limits(time, distance);
        let options = 1 + limits.1 - limits.0;
        result *= options;
    }
    dbg!(result);
}

fn calc_winning_limits(time: i64, distance: i64) -> (i64, i64) {
    let i = ((time.pow(2) - 4 * distance) as f64).sqrt();
    let i = i - 0.1; // we want to be faster, not equal to
    ((0.5 * (time as f64 - i)).ceil() as i64, (0.5 * (time  as f64 + i)).floor() as i64)
}