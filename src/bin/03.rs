use aoc21::*;

fn main() -> DynResult<()> {
    let input = read_input!();
    let bitwidth = input.find('\n').unwrap();
    let mask = (!0_u16) >> (16 - bitwidth);
    let vs: Vec<_> = input
        .lines()
        .map(|s| u16::from_str_radix(s, 2).unwrap())
        .collect();
    let mut oxy_candidates = vs.clone();
    let mut co2_candidates = vs.clone();
    let mut gamma = 0_u16;
    for pos in (0..bitwidth).into_iter().rev() {
        let count_0 = vs.iter().filter(|&v| ((v >> pos) & 0x1) == 0).count();
        gamma |= if 2 * count_0 > vs.len() { 0 } else { 1 } << pos;
        if oxy_candidates.len() > 1 {
            let count_0_oxy = oxy_candidates
                .iter()
                .filter(|&v| ((v >> pos) & 0x1) == 0)
                .count();
            let most_common_oxy = if count_0_oxy * 2 > oxy_candidates.len() {
                0
            } else {
                1
            };
            oxy_candidates = oxy_candidates
                .into_iter()
                .filter(|&v| ((v >> pos) & 0x1) == most_common_oxy)
                .collect();
        }
        if co2_candidates.len() > 1 {
            let count_0_co2 = co2_candidates
                .iter()
                .filter(|&v| ((v >> pos) & 0x1) == 0)
                .count();
            let most_common_co2 = if count_0_co2 * 2 > co2_candidates.len() {
                0
            } else {
                1
            };
            co2_candidates = co2_candidates
                .into_iter()
                .filter(|&v| ((v >> pos) & 0x1) != most_common_co2)
                .collect();
        }
    }
    let epsilon = (!gamma) & mask;
    assert_eq!(oxy_candidates.len(), 1);
    assert_eq!(co2_candidates.len(), 1);
    let oxy_rate = oxy_candidates[0];
    let co2_rate = co2_candidates[0];
    println!("Answer 1: {}", (epsilon as u32) * (gamma as u32));
    println!("Answer 2: {}", (oxy_rate as u32) * (co2_rate as u32));
    Ok(())
}
