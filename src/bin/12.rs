use std::{collections::HashMap, hash::Hash};

use aoc21::*;
use arrayvec::ArrayVec;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Cave {
    Start,
    End,
    Small([u8; 2]),
    Big([u8; 2]),
}

impl Cave {
    fn from_str(s: &str) -> Self {
        match s {
            "start" => Cave::Start,
            "end" => Cave::End,
            s => {
                let mut buf = [0; 2];
                buf.copy_from_slice(s.as_bytes());
                if buf.iter().all(|c| c.is_ascii_lowercase()) {
                    Cave::Small(buf)
                } else {
                    Cave::Big(buf)
                }
            }
        }
    }
}

#[derive(Clone)]
struct Path {
    head: Cave,
    smalls_in_path: ArrayVec<[u8; 2], 6>,
    visited_small_twice: bool,
}

impl Path {
    fn new() -> Self {
        Self {
            head: Cave::Start,
            smalls_in_path: ArrayVec::new(),
            visited_small_twice: false,
        }
    }
}

fn main() -> DynResult<()> {
    let input = read_input!();
    let mut graph: HashMap<_, Vec<_>> = HashMap::new();
    for l in input.lines() {
        let (a, b) = l.split_once('-').unwrap();
        let a = Cave::from_str(a);
        let b = Cave::from_str(b);
        graph.entry(a).or_default().push(b);
        graph.entry(b).or_default().push(a);
    }
    dbg!(graph.keys().filter(|c| matches!(c, Cave::Small(_))).count());
    let mut res1 = 0_u64;
    let mut res2 = 0_u64;
    let mut stack = vec![Path::new()];
    while let Some(p) = stack.pop() {
        let head = &p.head;
        let neighbors = graph.get(head).unwrap();
        for n in neighbors {
            match n {
                Cave::Start => continue,
                Cave::Small(s) => {
                    if p.smalls_in_path.binary_search(s).is_ok() {
                        if p.visited_small_twice {
                            continue;
                        } else {
                            let mut new_path = p.clone();
                            new_path.head = *n;
                            new_path.visited_small_twice = true;
                            stack.push(new_path);
                        }
                    } else {
                        let mut new_path = p.clone();
                        new_path.head = *n;
                        new_path.smalls_in_path.push(*s);
                        new_path.smalls_in_path.sort_unstable();
                        stack.push(new_path);
                    }
                }
                Cave::Big(_) => {
                    let mut new_path = p.clone();
                    new_path.head = *n;
                    stack.push(new_path);
                }
                Cave::End => {
                    if !p.visited_small_twice {
                        res1 += 1;
                    }
                    res2 += 1;
                }
            }
        }
    }
    print_answer(1, res1);
    print_answer(2, res2);
    Ok(())
}
