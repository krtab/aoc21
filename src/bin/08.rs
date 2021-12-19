use ahash::{AHashMap, AHashSet};

use aoc21::*;

const ZERO: [u8; 7] = *b"\0abcefg";
const ONE: [u8; 7] = *b"\0\0\0\0\0cf";
const TWO: [u8; 7] = *b"\0\0acdeg";
const THREE: [u8; 7] = *b"\0\0acdfg";
const FOUR: [u8; 7] = *b"\0\0\0bcdf";
const FIVE: [u8; 7] = *b"\0\0abdfg";
const SIX: [u8; 7] = *b"\0abdefg";
const SEVEN: [u8; 7] = *b"\0\0\0\0acf";
const EIGHT: [u8; 7] = *b"abcdefg";
const NINE: [u8; 7] = *b"\0abcdfg";

const ALL_DIGITS: [[u8; 7]; 10] = [ZERO, ONE, TWO, THREE, FOUR, FIVE, SIX, SEVEN, EIGHT, NINE];

#[derive(Clone, Debug)]
struct Subst([u8; 7]);

impl Subst {
    fn index(c: u8) -> usize {
        assert!(b'a' <= c);
        assert!(c <= b'g');
        (c - b'a') as usize
    }
    
    fn get(&self, c: u8) -> u8 {
        self.0[Self::index(c)]
    }

    fn substitute_char(&self, c: u8) -> u8 {
        if c == 0 {
            return 0;
        }
        self.get(c)
    }

    fn substitue_word(&self, word: &[u8; 7]) -> [u8; 7] {
        let mut res = *word;
        for c in &mut res {
            *c = self.substitute_char(*c)
        }
        res
    }

    fn idx_first_undefined(&self) -> Option<usize> {
        self.0.iter().position(|&c| c == b'?')
    }

    fn in_image(&self, c: u8) -> bool {
        self.0.contains(&c)
    }
}

fn main() -> DynResult<()> {
    let input = read_input!();
    let mut prefixes = AHashSet::new();
    let mut stack = Vec::from(ALL_DIGITS);
    let mut lookup = AHashMap::new();
    lookup.extend([
        (ZERO, 0),
        (ONE, 1),
        (TWO, 2),
        (THREE, 3),
        (FOUR, 4),
        (FIVE, 5),
        (SIX, 6),
        (SEVEN, 7),
        (EIGHT, 8),
        (NINE, 9),
    ]);
    while let Some(s) = stack.pop() {
        let newly_seen = prefixes.insert(s);
        if !newly_seen {
            continue;
        }
        for idx in 0..7 {
            let v = s[idx];
            if (b'a'..=b'g').contains(&v) {
                let mut buf = s;
                buf[idx] = b'?';
                buf.sort_unstable();
                stack.push(buf)
            }
        }
    }
    let mut res1 = 0_u64;
    let mut res2 = 0_u64;
    for l in input.lines() {
        let (left, right) = l.split_once('|').unwrap();
        let mut ten_scrambled = Vec::with_capacity(10);
        for x in left.split_ascii_whitespace() {
            let mut buf = [0; 7];
            buf[..x.len()].copy_from_slice(x.as_bytes());
            buf.sort_unstable();
            ten_scrambled.push(buf)
        }
        let mut stack = vec![Subst(*b"???????")];
        let mut matching_subs = None;
        while let Some(sub) = stack.pop() {
            let sound = ten_scrambled.iter().all(|w| {
                let mut buf = sub.substitue_word(w);
                buf.sort_unstable();
                prefixes.contains(&buf)
            });
            if !sound {
                continue;
            }
            match sub.idx_first_undefined() {
                Some(i) => {
                    for &c in b"abcdefg" {
                        if !sub.in_image(c) {
                            let mut new_sub = sub.clone();
                            new_sub.0[i] = c;
                            stack.push(new_sub);
                        }
                    }
                }
                None => {
                    matching_subs = Some(sub);
                    break;
                }
            }
        }
        let sub = matching_subs.unwrap();
        let mut val = 0;
        for x in right.split_ascii_whitespace() {
            let mut buf = [0; 7];
            buf[..x.len()].copy_from_slice(x.as_bytes());
            buf = sub.substitue_word(&buf);
            buf.sort_unstable();
            if [ONE, FOUR, SEVEN, EIGHT].contains(&buf) {
                res1 += 1;
            }
            val *= 10;
            val += lookup.get(&buf).unwrap();
        }
        res2 += val;
    }
    println!("Answer 1:\n{}", res1);
    println!("Answer 2:\n{}", res2);
    Ok(())
}
