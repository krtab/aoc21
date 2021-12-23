use std::fmt::Write;

use aoc21::*;

struct CountingIter<T> {
    inner: T,
    reads: usize,
}

impl<T: Iterator> Iterator for CountingIter<T> {
    type Item = T::Item;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.inner.next();
        if next.is_some() {
            self.reads += 1;
        }
        next
    }
}

impl<T> CountingIter<T> {
    fn new(it: T) -> Self {
        Self {
            inner: it,
            reads: 0,
        }
    }

    /// Get a reference to the counting iter's reads.
    fn reads(&self) -> usize {
        self.reads
    }
}

struct BitIter {
    inner: [u8; 4],
    yielded: u8,
}

impl Iterator for BitIter {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.yielded == 4 {
            return None;
        }
        self.yielded += 1;
        let res = self.inner[0];
        self.inner.rotate_left(1);
        Some(res)
    }
}

impl BitIter {
    fn new(x: char) -> Self {
        let x = x.to_digit(16).unwrap() as u8;
        let inner = [(x >> 3) & 1, (x >> 2) & 1, (x >> 1) & 1, (x >> 0) & 1];
        Self { inner, yielded: 0 }
    }
}

fn parse_base2_number(bits: &mut impl Iterator<Item = u8>, n: usize) -> u64 {
    let (count, res) = bits
        .take(n)
        .fold((0, 0), |(count, acc), x| (count + 1, acc * 2 + (x as u64)));
    assert_eq!(count, n);
    res
}

fn parse_header(bits: &mut impl Iterator<Item = u8>) -> (u8, u8) {
    let version = parse_base2_number(bits, 3);
    let type_id = parse_base2_number(bits, 3);
    return (version as u8, type_id as u8);
}

fn parse_literal(bits: &mut impl Iterator<Item = u8>) -> u64 {
    let mut res = 0;
    loop {
        let last_group = bits.next().unwrap();
        let tmp = parse_base2_number(bits, 4);
        res = res * 16 + tmp;
        if last_group == 0 {
            break;
        }
    }
    res
}

trait Visitor {
    fn literal(&mut self, v: u64, version: u8);
    fn start_operator(&mut self, version: u8, type_id: u8);
    fn finish_operator(&mut self);

    fn parse_packet_inner<T>(&mut self, bits: &mut CountingIter<T>)
    where
        T: Iterator<Item = u8>,
    {
        let (version, type_id) = parse_header(bits);
        if type_id == 4 {
            let v = parse_literal(bits);
            self.literal(v, version);
        } else {
            self.start_operator(version, type_id);
            let length_type_id = bits.next().unwrap();
            // dbg!(length_type_id);
            if length_type_id == 0 {
                let total_length = parse_base2_number(bits, 15) as usize;
                // dbg!(total_length);
                let starting_reads = bits.reads();
                while bits.reads() - starting_reads < total_length {
                    self.parse_packet_inner(bits)
                }
            } else {
                let n_packets = parse_base2_number(bits, 11);
                // dbg!(n_packets);
                for _ in 0..n_packets {
                    self.parse_packet_inner(bits);
                }
            }
            self.finish_operator();
        }
    }

    fn parse_packet<T>(&mut self, bits: T)
    where
        T: Iterator<Item = u8>,
    {
        let mut cnt_it = CountingIter::new(bits);
        self.parse_packet_inner(&mut cnt_it)
    }
}

struct Visitor1 {
    acc: u64,
}

impl Visitor1 {
    fn new() -> Self {
        Self { acc: 0 }
    }
}

impl Visitor for Visitor1 {
    fn literal(&mut self, _v: u64, version: u8) {
        self.acc += version as u64;
    }

    fn start_operator(&mut self, version: u8, _type_id: u8) {
        self.acc += version as u64;
    }

    fn finish_operator(&mut self) {
        ()
    }
}

struct DebugVisitor<V> {
    level: u8,
    inner: V,
    last: Option<String>,
}

impl<V> Drop for DebugVisitor<V> {
    fn drop(&mut self) {
        self.print_last()
    }
}

impl<V> DebugVisitor<V> {
    fn new(vis: V) -> Self {
        Self {
            level: 0,
            inner: vis,
            last: None,
        }
    }

    fn inner(&self) -> &V {
        &self.inner
    }

    fn print_last(&mut self) {
        if let Some(last) = self.last.take() {
            eprintln!("{}", last)
        }
    }

    fn put_last(&mut self, s: String) {
        if let Some(last) = self.last.replace(s) {
            eprintln!("{}", last)
        }
    }
}

fn level_string(level: u8) -> String {
    let mut s = String::new();
    for _ in 0..level.saturating_sub(1) {
        s.write_str("│  ").unwrap();
    }
    if level > 0 {
        s.write_str("├─ ").unwrap();
    }
    s
}

impl<V: Visitor> Visitor for DebugVisitor<V> {
    fn literal(&mut self, v: u64, version: u8) {
        let mut s = level_string(self.level);
        s.write_fmt(format_args!("Literal: {} (version: {})", v, version))
            .unwrap();
        self.put_last(s);
        self.inner.literal(v, version)
    }

    fn start_operator(&mut self, version: u8, type_id: u8) {
        let mut s = level_string(self.level);
        s.write_fmt(format_args!(
            "Operator: version={}, id={}",
            version,
            type_id
        ))
        .unwrap();
        self.put_last(s);
        self.level += 1;
        self.inner.start_operator(version, type_id);
    }

    fn finish_operator(&mut self) {
        self.level -= 1;
        self.last = self.last.as_ref().map(|s| s.replace('├',"└"));
        self.inner.finish_operator();
    }
}

fn main() -> DynResult<()> {
    let input = read_input!();
    eprintln!("Parsing: {}", &input);
    let bits = input.chars().map(|c| BitIter::new(c)).flatten();
    let res1 = {
        let mut vis = DebugVisitor::new(Visitor1::new());
        vis.parse_packet(bits);
        vis.inner().acc
    };
    print_answer(1, res1);
    Ok(())
}
