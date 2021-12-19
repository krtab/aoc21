use aoc21::*;

fn main() -> DynResult<()> {
    let vs: Vec<u32> = read_input!(parse);
    let res1 = vs
        .iter()
        .clone()
        .zip(vs.iter().skip(1))
        .filter(|(prev, cur)| prev < cur)
        .count();
    println!("Answer 1:\n{}", res1);

    let sums = vs.windows(3).map(|s| s.iter().sum());
    let res2 = sums
        .clone()
        .zip(sums.skip(1))
        .filter(|(prev, cur): &(u32, u32)| prev < cur)
        .count();
    println!("Answer 2:\n{}", res2);
    Ok(())
}
