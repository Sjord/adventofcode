use std::{env, fs, slice::Iter};

pub mod parser;

use parser::monkeys;

fn main() {
    let fname = env::args().nth(1).unwrap();
    let contents = fs::read_to_string(fname).unwrap();
    let mut monkeys = monkeys(&contents).unwrap();
    println!("{:?}", monkeys);

    let modulo : i64 = monkeys.iter().map(|m| m.test).product();

    for round in 0..10000 {
        println!("Round {}", round);
            for i in 0..monkeys.len() {
            println!("  Monkey {}", i);
            let monkey = &mut monkeys[i];
            let throws = monkey.inspect_and_throw();
            for throw in throws {
                println!("    Throws {} to {}", throw.item, throw.destination);
                monkeys[throw.destination as usize].items.push(throw.item % modulo);
            }
        }
    }
    
    println!("{:?}", monkeys);

    monkeys.sort_by_key(|m| -m.inspections);
    println!("Monkey business: {}", monkeys[0].inspections * monkeys[1].inspections);
}


#[derive(Debug)]
pub struct Monkey {
    id: i64,
    items: Vec<i64>,
    operation: Operation,
    test: i64,
    trueMonkey: i64,
    falseMonkey: i64,
    inspections: i64,
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

impl Operation {
    fn evaluate(&self, old: i64) -> i64 {
        match self.operator {
            Operator::Add => self.left.value(old) + self.right.value(old),
            Operator::Multiply => self.left.value(old) * self.right.value(old),
        }
    }
}

#[derive(Debug)]
enum Term {
    Old,
    Const(i64)
}

impl Term {
    fn value(&self, old: i64) -> i64 {
        match self {
            Term::Old => old,
            Term::Const(i) => *i
        }
    }
}

impl Monkey {
    fn inspect_and_throw(&mut self) -> Vec<ThrownItem> {
        let mut result = Vec::with_capacity(self.items.len());
        for item in self.items.iter() {
            let item = self.operation.evaluate(*item);
            // let item = item / 3;
            let item = item;
            let destination = if item % self.test == 0 {
                self.trueMonkey
            } else {
                self.falseMonkey
            };
            result.push(ThrownItem { destination, item });
            self.inspections += 1;
        };
        self.items.clear();
        result
    }
}

struct ThrownItem {
    destination: i64,
    item: i64
}
