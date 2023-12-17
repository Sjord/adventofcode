use std::{env, fs};

use nom::{
    bytes::complete::tag,
    character::complete as cc,
    multi::separated_list0,
    sequence::{preceded, tuple},
    IResult,
};

fn main() {
    let fname = env::args().nth(1).unwrap();
    let contents = fs::read_to_string(fname).unwrap();
    let sensors = sensors(&contents);

    let min_x = sensors
        .iter()
        .map(|r| r.sensor.x - r.beacon_distance() as i32)
        .min()
        .unwrap();
    let max_x = sensors
        .iter()
        .map(|r| r.sensor.x + r.beacon_distance() as i32)
        .max()
        .unwrap();

    let limits = 4000000;

    for y in 0..=limits {
        let mut x = 0;
        while x <= limits {
            let coord = Coord {x, y};
            let mut can_be_beacon = true;
            
            for r in sensors.iter() {
                if r.beacon == coord {
                    x += 1;
                    can_be_beacon = false;
                    break;
                } else if r.sensor.distance(&coord) <= r.beacon_distance() {
                    x = 1 + r.sensor.x + r.beacon_distance() as i32 - coord.y.abs_diff(r.sensor.y) as i32;
                    can_be_beacon = false;
                    break;
                }
            }

            if (can_be_beacon) {
                println!("x: {}, y: {}, freq: {}", x, y, x as u64 * 4000000 + y as u64);
                return;
            }
        }
    }
}

fn cannot_be_beacon(records: &Vec<Record>, coord: Coord) -> bool {
    for r in records {
        if r.beacon == coord {
            return true;
        }
        if r.sensor.distance(&coord) <= r.beacon_distance() {
            return true;
        }
    }
    return false;
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
struct Coord {
    x: i32,
    y: i32,
}

impl Coord {
    fn distance(&self, other: &Coord) -> u32 {
        return self.x.abs_diff(other.x) + self.y.abs_diff(other.y);
    }
}

#[derive(Debug)]
struct Record {
    sensor: Coord,
    beacon: Coord,
}

impl Record {
    fn beacon_distance(&self) -> u32 {
        self.sensor.distance(&self.beacon)
    }
}

fn sensors(input: &str) -> Vec<Record> {
    let (i, records) = separated_list0(tag("\n"), sensor)(input).unwrap();
    records
}

fn sensor(input: &str) -> IResult<&str, Record> {
    let (i, (_, sx, _, sy, _, bx, _, by)) = tuple((
        tag("Sensor at x="),
        cc::i32,
        tag(", y="),
        cc::i32,
        tag(": closest beacon is at x="),
        cc::i32,
        tag(", y="),
        cc::i32,
    ))(input)?;
    Ok((
        i,
        Record {
            sensor: Coord { x: sx, y: sy },
            beacon: Coord { x: bx, y: by },
        },
    ))
}
