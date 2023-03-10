use std::{env, fs, slice::Iter};

pub mod parser;

use parser::monkeys;

fn main() {
    let fname = env::args().nth(1).unwrap();
    let contents = fs::read_to_string(fname).unwrap();
    let monkeys = monkeys(&contents).unwrap();
    println!("{:?}", monkeys);
}


#[derive(Debug)]
pub struct Monkey {
    id: i32,
    items: Vec<i32>,
    operation: Operation,
    test: i32,
    trueMonkey: i32,
    falseMonkey: i32
}

#[derive(Debug)]
struct Operation {
    left: Term,
    right: Term,
    operator: Operator
}

#[derive(Debug)]
enum Operator {
    Add,
    Multiply
}

#[derive(Debug)]
enum Term {
    Old,
    Const(i32)
}
