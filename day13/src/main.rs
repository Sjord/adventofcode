use std::{env, fs};
use serde::Deserialize;
use serde_json::Result;

fn main() {
    let fname = env::args().nth(1).unwrap();
    let contents = fs::read_to_string(fname).unwrap();
    let parts = contents.split("\n\n");
    let pairs = parts.map(|part| {
        let a: Vec<Value> = part.lines().map(|l| serde_json::from_str(l).unwrap()).collect();
        (a[0].clone(), a[1].clone())
    });
    let sum : usize = pairs.enumerate().filter(|(i, p)| p.0 < p.1).map(|(i, p)| i + 1).sum();
    dbg!(sum);
}

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
#[serde(untagged)]
enum Value {
    Number(i32),
    List(Vec<Value>)
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Value {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Value::Number(l), Value::Number(r)) => l.cmp(r),
            (Value::Number(l), Value::List(_)) => Value::List(vec!(Value::Number(*l))).cmp(other),
            (Value::List(_), Value::Number(r)) => self.cmp(&Value::List(vec!(Value::Number(*r)))),
            (Value::List(l), Value::List(r)) => {
                let min_length = std::cmp::min(l.len(), r.len());
                for i in 0..min_length {
                    let c = l[i].cmp(&r[i]);
                    if c != std::cmp::Ordering::Equal {
                        return c;
                    }
                }
                return l.len().cmp(&r.len());
            }
        }
    }
}
