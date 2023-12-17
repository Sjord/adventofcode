use std::fs;
use std::env;

fn main() {
    let fname = env::args().nth(1).unwrap();
    let contents = fs::read_to_string(fname).unwrap();
    let parts = contents.split("\n\n");
    let mut sums: Vec<_> = 
        parts.map(
            |p| p.lines().map(
                |l| l.parse::<i32>().unwrap()
            ).sum::<i32>()
        ).collect();
    sums.sort();
    sums.reverse();
    let maxsum = sums.iter().take(3).sum::<i32>();
    dbg!(maxsum);
}
