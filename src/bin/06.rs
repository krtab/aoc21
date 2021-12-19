use aoc21::*;

fn main() -> DynResult<()> {
    let input = read_input!();
    let mut fishes = [0_u64; 9];
    for a in input.split(',') {
        fishes[a.parse::<usize>().unwrap()] += 1;
    }
    for _ in 0..80 {
        fishes.rotate_left(1);
        fishes[6] += fishes[8];
    }
    println!("Answer 1:\n{}", fishes.iter().sum::<u64>());
    for _ in 0..(256 - 80) {
        fishes.rotate_left(1);
        fishes[6] += fishes[8];
    }
    println!("Answer 2:\n{}", fishes.iter().sum::<u64>());
    Ok(())
}
