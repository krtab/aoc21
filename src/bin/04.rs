use aoc21::*;

#[allow(clippy::unusual_byte_groupings)]
const CHECK_MASKS: [u32; 10] = [
    0b00000_00000_00000_00000_11111,
    0b00000_00000_00000_11111_00000,
    0b00000_00000_11111_00000_00000,
    0b00000_11111_00000_00000_00000,
    0b11111_00000_00000_00000_00000,
    0b00001_00001_00001_00001_00001,
    0b00010_00010_00010_00010_00010,
    0b00100_00100_00100_00100_00100,
    0b01000_01000_01000_01000_01000,
    0b10000_10000_10000_10000_10000,
];

fn main() -> DynResult<()> {
    let input = read_input!();
    let mut ls = input.lines();
    let draws: Vec<u32> = ls
        .next()
        .unwrap()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();
    let mut boards = Vec::new();
    while let Some(_) = ls.next() {
        let b: Vec<u32> = ls
            .by_ref()
            .take(5)
            .map(|l| l.split_ascii_whitespace())
            .flatten()
            .map(|s| s.parse().unwrap())
            .collect();
        boards.push(b);
    }
    let mut masks = vec![0_u32; boards.len()];
    let mut first_win = true;
    let mut last_win_final_score = None;
    for draw in draws {
        for (m, b) in masks.iter_mut().zip(&boards) {
            if *m == !0 {
                continue;
            }
            if let Some(pos) = b.iter().position(|&x| x == draw) {
                *m |= 1 << pos;
                for check_m in CHECK_MASKS {
                    if (*m & check_m).count_ones() == 5 {
                        let final_score: u64 = b
                            .iter()
                            .copied()
                            .enumerate()
                            .filter_map(|(i, x)| (((*m >> i) & 1) == 0).then(|| x as u64))
                            .sum::<u64>()
                            * (draw as u64);
                        if first_win {
                            println!("Answer 1:\n{}", final_score);
                            first_win = false;
                        }
                        *m = !0;
                        last_win_final_score = Some(final_score);
                        break;
                    }
                }
            }
        }
    }
    println!("Answer 2:\n{}", last_win_final_score.unwrap());

    Ok(())
}
