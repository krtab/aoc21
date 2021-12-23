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
    inner: u8,
    yielded: u8,
}

impl Iterator for BitIter {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.yielded == 4 {
            return None;
        }
        self.yielded += 1;
        let res = self.inner & 1;
        self.inner >>= 1;
        Some(res)
    }
}

impl BitIter {
    fn new(x: char) -> Self {
        let x = x.to_digit(16).unwrap() as u8;
        Self { inner: x.reverse_bits() >> 4, yielded: 0 }
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
    (version as u8, type_id as u8)
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
    type Return;
    fn literal(&mut self, v: u64, version: u8);
    fn start_operator(&mut self, version: u8, type_id: u8);
    fn end_operator(&mut self);
    fn finish(self) -> Self::Return;

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
            self.end_operator();
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
    type Return = u64;

    fn literal(&mut self, _v: u64, version: u8) {
        self.acc += version as u64;
    }

    fn start_operator(&mut self, version: u8, _type_id: u8) {
        self.acc += version as u64;
    }

    fn end_operator(&mut self) {}

    fn finish(self) -> Self::Return {
        self.acc
    }
}

#[derive(PartialEq, Eq, Debug)]
#[repr(u8)]
enum Operation {
    #[allow(dead_code)]
    Sum = 0,
    #[allow(dead_code)]
    Product = 1,
    #[allow(dead_code)]
    Minimum = 2,
    #[allow(dead_code)]
    Maximum = 3,
    #[allow(dead_code)]
    GreaterThan = 5,
    #[allow(dead_code)]
    LessThan = 6,
    #[allow(dead_code)]
    Equal = 7,
    Root,
}

impl Operation {
    fn new(op: u8) -> Self {
        assert!(op <= 7);
        assert_ne!(op, 4);
        unsafe { std::mem::transmute(op) }
    }
}

struct Visitor2 {
    stack: Vec<(Operation, Vec<u64>)>,
}

impl Visitor2 {
    fn new() -> Self {
        Self {
            stack: vec![(Operation::Root, vec![])],
        }
    }
}

impl Visitor for Visitor2 {
    type Return = u64;

    fn literal(&mut self, v: u64, _version: u8) {
        self.stack.last_mut().unwrap().1.push(v)
    }

    fn start_operator(&mut self, _version: u8, type_id: u8) {
        self.stack.push((Operation::new(type_id), vec![]))
    }

    fn end_operator(&mut self) {
        let (op, v) = self.stack.pop().unwrap();

        let res = match op {
            Operation::Sum => v.into_iter().sum(),
            Operation::Product => v.into_iter().product(),
            Operation::Minimum => v.into_iter().min().unwrap(),
            Operation::Maximum => v.into_iter().max().unwrap(),
            Operation::GreaterThan => {
                assert!(v.len() == 2);
                if v[0] > v[1] {
                    1
                } else {
                    0
                }
            }
            Operation::LessThan => {
                assert!(v.len() == 2);
                if v[0] < v[1] {
                    1
                } else {
                    0
                }
            }
            Operation::Equal => {
                assert!(v.len() == 2);
                if v[0] == v[1] {
                    1
                } else {
                    0
                }
            }
            Operation::Root => unreachable!(),
        };
        self.stack.last_mut().unwrap().1.push(res);
    }

    fn finish(mut self) -> Self::Return {
        assert!(self.stack.len() == 1);
        let (op, v) = self.stack.pop().unwrap();
        assert_eq!(op, Operation::Root);
        assert!(v.len() == 1);
        v[0]
    }
}

struct DebugVisitor<V> {
    level: u8,
    inner: V,
    last: Option<String>,
}

impl<V> DebugVisitor<V> {
    fn new(vis: V) -> Self {
        Self {
            level: 0,
            inner: vis,
            last: None,
        }
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
    type Return = V::Return;

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
            version, type_id
        ))
        .unwrap();
        self.put_last(s);
        self.level += 1;
        self.inner.start_operator(version, type_id);
    }

    fn end_operator(&mut self) {
        self.level -= 1;
        self.last = self.last.as_ref().map(|s| s.replace('├', "└"));
        self.inner.end_operator();
    }

    fn finish(mut self) -> Self::Return {
        self.print_last();
        self.inner.finish()
    }
}

impl<V1: Visitor, V2: Visitor> Visitor for (V1, V2) {
    type Return = (V1::Return, V2::Return);

    fn literal(&mut self, v: u64, version: u8) {
        self.0.literal(v, version);
        self.1.literal(v, version);
    }

    fn start_operator(&mut self, version: u8, type_id: u8) {
        self.0.start_operator(version, type_id);
        self.1.start_operator(version, type_id);
    }

    fn end_operator(&mut self) {
        self.0.end_operator();
        self.1.end_operator();
    }

    fn finish(self) -> Self::Return {
        (self.0.finish(), self.1.finish())
    }
}

fn main() -> DynResult<()> {
    let input = read_input!();
    eprintln!("Parsing: {}", &input);
    let bits = input.chars().map(BitIter::new).flatten();

    let mut vis = DebugVisitor::new((Visitor1::new(), Visitor2::new()));
    vis.parse_packet(bits);
    let (res1, res2) = vis.finish();

    print_answer(1, res1);
    print_answer(2, res2);
    Ok(())
}
