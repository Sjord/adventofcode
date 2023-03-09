use std::{env, fs, slice::Iter};
use nom::{
    IResult,
    bytes::complete::{tag, take_while_m_n, take_until, take_while},
    character::complete as cc,
    combinator::map,
    sequence::tuple, multi::separated_list0, character::{is_digit, complete::space0}, branch::alt};


fn main() {
    let fname = env::args().nth(1).unwrap();
    let contents = fs::read_to_string(fname).unwrap();
    println!("{:?}", monkeys(&contents));
}

fn monkeys(input: &str) -> IResult<&str, Vec<Monkey>> {
    separated_list0(tag("\n\n"), monkey)(input)
}

fn monkey(input: &str) -> IResult<&str, Monkey> {
    let (i, (_, id, _)) = tuple((tag("Monkey "), cc::i32, tag(":\n")))(input)?;
    let (i, (_, items, _)) = tuple((tag("  Starting items: "), separated_list0(tag(", "), cc::i32), tag("\n")))(i)?;
    let (i, (_, operation, _)) = tuple((tag("  Operation: new = "), operation, tag("\n")))(i)?;
    let (i, (_, test, _)) = tuple((tag("  Test: divisible by "), cc::i32, tag("\n")))(i)?;
    let (i, (_, trueMonkey, _)) = tuple((tag("    If true: throw to monkey "), cc::i32, tag("\n")))(i)?;
    let (i, (_, falseMonkey)) = tuple((tag("    If false: throw to monkey "), cc::i32))(i)?;

    Ok((i, Monkey { 
        id: id,
        items,
        operation,
        test,
        trueMonkey,
        falseMonkey
    }))
}

fn operation(input: &str) -> IResult<&str, Operation> {
    let (i, (left, _, operator, _, right)) = tuple((term, space0, operator, space0, term))(input)?;
    Ok((i, Operation { left, operator, right }))
}

fn term(input: &str) -> IResult<&str, Term> {
    alt((
        map(tag("old"), |_| Term::Old),
        map(cc::i32, |i| Term::Const(i))
    ))(input)
}

fn operator(input: &str) -> IResult<&str, Operator> {
    alt((
        map(tag("+"), |_| Operator::Add),
        map(tag("*"), |_| Operator::Multiply)
    ))(input)
}

#[derive(Debug)]
struct Monkey {
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
