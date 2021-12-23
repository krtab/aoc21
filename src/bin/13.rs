use std::io::{stdout, Write};

use aoc21::*;

#[derive(Clone, Copy)]
enum Fold {
    X(u16),
    Y(u16),
}
use Fold::*;

fn symmetry(x: u16, axis: u16) -> u16 {
    if x > axis {
        let delta = x - axis;
        axis - delta
    } else {
        x
    }
}

impl Fold {
    fn fold(&self, (x, y): (u16, u16)) -> (u16, u16) {
        match *self {
            X(axis) => (symmetry(x, axis), y),
            Y(axis) => (x, symmetry(y, axis)),
        }
    }
}

// |--> x
// |
// V  y
fn main() -> DynResult<()> {
    let input = read_input!();
    let (coords, fold_cmds) = input.split_once("\n\n").unwrap();
    let mut folds = Vec::new();
    for l in fold_cmds.lines() {
        let (axis, val) = l
            .strip_prefix("fold along ")
            .unwrap()
            .split_once('=')
            .unwrap();
        let val = val.parse().unwrap();
        folds.push(match axis {
            "x" => X(val),
            "y" => Y(val),
            _ => unimplemented!(),
        })
    }
    let first_fold = &folds[0];
    let remainig_folds = &folds[1..];
    let mut grid1 = [[false; 2000]; 2000];
    let mut grid2 = [[false; 2000]; 2000];
    let mut max_x = 0;
    let mut max_y = 0;
    for l in coords.lines() {
        let (x, y) = l.split_once(',').unwrap();
        let x = x.parse().unwrap();
        let y = y.parse().unwrap();
        let (x, y) = first_fold.fold((x, y));
        grid1[y as usize][x as usize] = true;
        let (x, y) = remainig_folds.iter().fold((x, y), |c, f| f.fold(c));
        grid2[y as usize][x as usize] = true;
        max_x = std::cmp::max(x, max_x);
        max_y = std::cmp::max(y, max_y);
    }
    let res1 = grid1.iter().flatten().filter(|&&b| b).count();
    print_answer(1, res1);
    println!("Answer 2:");
    let stdout = stdout();
    let mut lock = stdout.lock();
    for r in &grid2[..=max_y as usize] {
        for &c in &r[..=max_x as usize] {
            lock.write_all(match c {
                true => "█".as_bytes(),
                false => "░".as_bytes(),
            })
            .unwrap();
        }
        lock.write_all(b"\n").unwrap();
    }
    Ok(())
}
