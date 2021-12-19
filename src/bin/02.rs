use aoc21::*;

fn main() -> DynResult<()> {
    let input = read_input!();
    let mut depth_1 = 0_u32;
    let mut hpos_1 = 0_u32;
    let mut aim_2 = 0;
    let mut hpos_2 = 0;
    let mut depth_2 = 0;
    for l in input.lines() {
        let (cmd, param) = l.split_once(" ").unwrap();
        let param: u32 = param.parse().unwrap();
        match cmd {
            "forward" => {
                hpos_1 += param;
                hpos_2 += param;
                depth_2 += aim_2 * param
            }
            "down" => {
                depth_1 += param;
                aim_2 += param
            }
            "up" => {
                depth_1 -= param;
                aim_2 -= param
            }
            _ => panic!("Unknown command"),
        }
    }
    println!(
        "Answer 1:\n{}\nAnswer 2:\n{}",
        depth_1 * hpos_1,
        depth_2 * hpos_2
    );
    Ok(())
}
