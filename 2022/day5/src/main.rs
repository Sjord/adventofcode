use std::{env, fs};
use regex::Regex;
use std::collections::VecDeque;


fn main() {
    let fname = env::args().nth(1).unwrap();
    let contents = fs::read_to_string(fname).unwrap();
    let mut parts = contents.split("\n\n");
    let mut stacks = read_stacks(parts.next().unwrap());
    let instructions = read_instructions(parts.next().unwrap());
    stacks.execute(instructions);
    println!("{}", stacks.top_crates());
}

fn read_stacks(input: &str) -> Stacks {
    let mut stacks = Stacks { 0: Vec::new() };
    for l in input.lines() {
        let num_stacks = (l.len() + 1) / 4;
        for x in 0..num_stacks {
            let ch = l.chars().nth(4 * x + 1).unwrap();
            if ch != ' ' && ch.is_alphabetic() {
                while stacks.0.len() <= x {
                    stacks.0.push(Stack { 0: VecDeque::new() });
                }
                stacks.0[x].0.push_back(ch);
            }
        }
    }
    stacks
}

fn read_instructions(input: &str) -> Vec<Instruction> {
    let re = Regex::new(r"^move (\d+) from (\d+) to (\d+)$").unwrap();
    input.lines().map(|l| {
        let m = re.captures(l).unwrap();
        Instruction {
            count: m[1].parse().unwrap(),
            from: m[2].parse().unwrap(),
            to: m[3].parse().unwrap()
        }
    }).collect()

}

#[derive(Debug)]
struct Stacks(Vec<Stack>);

impl Stacks {
    fn execute(&mut self, instructions: Vec<Instruction>) {
        for i in instructions {
            let mut popped : Vec<char> = (0..i.count).map(|j| self.0[i.from - 1].0.pop_front().unwrap()).collect();
            for elem in popped.iter().rev() {
                self.0[i.to - 1].0.push_front(*elem);
            }
        }
    }

    fn top_crates(&self) -> String {
        self.0.iter().map(|s| s.0[0]).collect()
    }
}

#[derive(Debug)]
struct Stack(VecDeque<char>);

struct Instruction {
    count: usize,
    from: usize,
    to: usize
}
