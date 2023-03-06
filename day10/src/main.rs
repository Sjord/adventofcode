use std::env;
use std::fs;

fn main() {
    let fname = env::args().nth(1).unwrap();
    let contents = fs::read_to_string(fname).unwrap();
    let instructions = parse_instructions(&contents);
    let sum = run(instructions);
    println!("{}", sum);
}

fn run(instructions: Vec<Instruction>) -> i32 {
    let mut cpu = Cpu::new();

    for instr in instructions {
        match instr {
            Instruction::Noop => {
                cpu.noop();
            },
            Instruction::Addx(arg) => {
                cpu.addx(arg);
            }
        }
    }
    cpu.signal_sum
}

fn parse_instructions(input: &str) -> Vec<Instruction> {
    input.lines().map(|l| {
        if l == "noop" {
            Instruction::Noop
        } else {
            let mut parts = l.split_ascii_whitespace();
            assert!(parts.next() == Some("addx"));
            let i = parts.next().unwrap().parse().unwrap();
            Instruction::Addx(i)
        }
    }).collect()
}

enum Instruction {
    Noop,
    Addx(i32)
}

struct Cpu {
    cycle: i32,
    x: i32,
    signal_sum: i32,
}

impl Cpu {
    fn new() -> Cpu {
        Cpu {
            cycle: 0,
            x: 1,
            signal_sum: 0
        }
    }

    fn signal_strength(&self) -> i32 {
        self.cycle * self.x
    }
    
    fn step_cycle(&mut self) {
        self.cycle += 1;
        if (self.cycle - 20) % 40 == 0 {
            self.signal_sum += self.signal_strength();
        }
    }

    fn noop(&mut self) {
        self.step_cycle();
    }

    fn addx(&mut self, arg: i32) {
        self.step_cycle();
        self.step_cycle();
        self.x += arg;
    }
}
