use std::{env, fs};

fn main() {
    let fname = env::args().nth(1).unwrap();
    let contents = fs::read_to_string(fname).unwrap();
    let lines = contents.lines();
    let mut sum = 0;
    for l in lines {
        let sequence = l.split_ascii_whitespace().map(|s| s.parse().unwrap()).collect();
        let prediction = predict(sequence);
        dbg!(prediction);
        sum += prediction;
    }
    dbg!(sum);
}

fn predict(sequence: Vec<i64>) -> i64 {
    if sequence.iter().all(|n| *n == 0) {
        0
    } else {
        let differences = derive(&sequence);
        let prev = predict(differences);
        sequence.first().unwrap() - prev
    }
}

fn derive(sequence: &Vec<i64>) -> Vec<i64> {
    sequence.windows(2).map(|w| w[1] - w[0]).collect()
}
