use std::ops::RangeInclusive;

use aoc21::*;

enum DoubleDirectionRange {
    Forward(RangeInclusive<usize>),
    Backward(RangeInclusive<usize>),
}

impl DoubleDirectionRange {
    fn new(a: usize, b: usize) -> Self {
        if a <= b {
            Self::Forward(a..=b)
        } else {
            Self::Backward(b..=a)
        }
    }
}

impl Iterator for DoubleDirectionRange {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            DoubleDirectionRange::Forward(r) => r.next(),
            DoubleDirectionRange::Backward(r) => r.next_back(),
        }
    }
}

fn main() -> DynResult<()> {
    let input = read_input!();
    let mut board1 = [[0_u8; 1000]; 1000];
    let mut board2 = [[0_u8; 1000]; 1000];
    let mut res1 = 0_u32;
    let mut res2 = 0_u32;
    for l in input.lines() {
        let (a, b) = l.split_once(" -> ").unwrap();
        let (ax, ay) = a.split_once(',').unwrap();
        let ax: usize = ax.parse().unwrap();
        let ay: usize = ay.parse().unwrap();
        let (bx, by) = b.split_once(',').unwrap();
        let bx: usize = bx.parse().unwrap();
        let by: usize = by.parse().unwrap();
        if ax == bx {
            for y in DoubleDirectionRange::new(ay, by) {
                let cell = &mut board1[ax][y];
                *cell += 1;
                if *cell == 2 {
                    res1 += 1;
                }
                let cell = &mut board2[ax][y];
                *cell += 1;
                if *cell == 2 {
                    res2 += 1;
                }
            }
        } else if ay == by {
            for x in DoubleDirectionRange::new(ax, bx) {
                let cell = &mut board1[x][ay];
                *cell += 1;
                if *cell == 2 {
                    res1 += 1;
                }
                let cell = &mut board2[x][ay];
                *cell += 1;
                if *cell == 2 {
                    res2 += 1;
                }
            }
        } else {
            for (x, y) in DoubleDirectionRange::new(ax, bx).zip(DoubleDirectionRange::new(ay, by)) {
                let cell = &mut board2[x][y];
                *cell += 1;
                if *cell == 2 {
                    res2 += 1;
                }
            }
        }
    }
    println!("Answer 1:\n{}", res1);
    println!("Answer 2:\n{}", res2);
    Ok(())
}
