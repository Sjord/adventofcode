use std::{env, fs};

use ringbuffer::{AllocRingBuffer, RingBufferWrite, RingBuffer, RingBufferRead, RingBufferExt};

fn main() {
    let fname = env::args().nth(1).unwrap();
    let contents = fs::read_to_string(fname).unwrap();

    let mut buf = AllocRingBuffer::<char>::with_capacity(4);
    for (i, ch) in contents.chars().enumerate() {
        buf.push(ch);
        if is_unique(&buf) {
            println!("{}", i + 1);
            return;
        }
    }
}

fn is_unique(buf: &AllocRingBuffer::<char>) -> bool {
    if buf.len() < 4 {
        return false;
    }

    for (i, a) in buf.iter().enumerate() {
        for (j, b) in buf.iter().enumerate() {
            if (i != j && a == b) {
                return false;
            }
        }
    }
    true
}
