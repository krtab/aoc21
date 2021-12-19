use aoc21::*;

fn main() -> DynResult<()> {
    let input = read_input!();
    let mut positions: Vec<u16> = input.split(',').map(|s| s.parse().unwrap()).collect();
    positions.sort_unstable();
    let pos1 = positions[positions.len() / 2];
    let cost1: i32 = positions
        .iter()
        .map(|&p| ((p as i32) - (pos1 as i32)).abs())
        .sum();
    println!("Answer 1:\n{}", cost1);
    let pos2_approx =
        ((positions.iter().map(|&x| x as f64).sum::<f64>()) / (positions.len() as f64)).round();
    let mut cost2 = i32::MAX;
    for pos2_candidate in [pos2_approx - 1., pos2_approx, pos2_approx + 1.] {
        let cost: i32 = positions
            .iter()
            .map(|&p| {
                let delta = ((p as i32) - (pos2_candidate as i32)).abs();
                delta * (delta + 1) / 2
            })
            .sum();
        if cost < cost2 {
            cost2 = cost;
        }
    }
    println!("Answer 2:\n{}", cost2);
    Ok(())
}
