use std::{env, fs, ops::RangeInclusive};


fn main() {
    let fname = env::args().nth(1).unwrap();
    let contents = fs::read_to_string(fname).unwrap();
    let pairs = contents.lines().map(|l| {
        let mut ranges = l.split(',').map(|r| {
            let nums : Vec<u8> = r.split('-').map(|n| n.parse().unwrap()).collect();
            RangeInclusive::new(nums[0], nums[1])
        });
        Pair { 0: ranges.next().unwrap(), 1: ranges.next().unwrap() }
    });
    let count = pairs.filter(|p| p.overlaps()).count();
    println!("{}", count);
}

struct Pair (RangeInclusive<u8>, RangeInclusive<u8>);

impl Pair {
    fn overlaps(&self) -> bool {
        self.0.overlaps(&self.1) || self.1.overlaps(&self.0)
    }
}

trait Overlaps {
    fn overlaps(&self, other: &Self) -> bool;
}


impl Overlaps for RangeInclusive<u8> {
    fn overlaps(&self, other: &RangeInclusive<u8>) -> bool {
        self.contains(other.start())
        || self.contains(other.end())
        || other.contains(self.start())
        || other.contains(self.end())
    }
}
