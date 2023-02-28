use std::{env, fs, slice::Iter};

fn main() {
    let fname = env::args().nth(1).unwrap();
    let contents = fs::read_to_string(fname).unwrap();

    let mut buf = RingBuffer::<char>::with_capacity(14);
    for (i, ch) in contents.chars().enumerate() {
        buf.push(ch);
        if is_unique(&buf) {
            println!("{}", i + 1);
            return;
        }
    }
}

fn is_unique(buf: &RingBuffer::<char>) -> bool {
    if buf.len() < buf.capacity {
        return false;
    }

    for (i, a) in buf.iter().enumerate() {
        for (j, b) in buf.iter().enumerate() {
            // println!("{} {} {} {}", i, j, a, b);
            if (i != j && a == b) {
                return false;
            }
        }
    }
    true
}

struct RingBuffer<T> {
    capacity: usize,
    buffer: Vec<T>,
    writer: usize,
}

impl<T> RingBuffer<T> {
    fn with_capacity(capacity: usize) -> RingBuffer<T> {
        Self {
            capacity,
            buffer: Vec::with_capacity(capacity),
            writer: 0
        }
    }

    fn iter(&self) -> Iter<T> {
        self.buffer.iter()
    }

    fn push(&mut self, elem: T) {
        if self.writer >= self.buffer.len() {
            self.buffer.push(elem);
        } else {
            self.buffer[self.writer] = elem;
        }
        self.writer = (self.writer + 1) % self.capacity;
    }

    fn len(&self) -> usize {
        self.buffer.len()
    }
}
