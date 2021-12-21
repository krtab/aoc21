use aoc21::*;

const DIAGS: [(isize, isize); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

fn main() -> DynResult<()> {
    let input = read_input!();
    let mut map = [[u8::MAX; 10]; 10];
    for (line, row) in input.lines().zip(&mut map) {
        for (cr, cell) in line.bytes().zip(row) {
            *cell = cr - b'0'
        }
    }
    let mut flashes_before_100 = 0_u64;
    let mut step = 0_usize;
    let mut all_flashed = None;
    while step < 100 || all_flashed.is_none() {
        let mut flashed = [[false; 10]; 10];
        let mut stack = Vec::new();
        for x in 0..10 {
            for y in 0..10 {
                stack.push((x, y));
            }
        }
        while let Some((x, y)) = stack.pop() {
            if flashed[x][y] {
                continue;
            }
            map[x][y] += 1;
            if map[x][y] > 9 {
                if step < 100 {
                    flashes_before_100 += 1;
                }
                flashed[x][y] = true;
                map[x][y] = 0;
                for (dx, dy) in DIAGS {
                    let newx = (x as isize) + dx;
                    let newy = (y as isize) + dy;
                    if (0..10).contains(&newx) && (0..10).contains(&newy) {
                        stack.push((newx as usize, newy as usize))
                    }
                }
            }
        }
        step += 1;
        if flashed.iter().flatten().all(|&x| x) {
            all_flashed = Some(step)
        }
    }
    print_answer(1, flashes_before_100);
    print_answer(2, all_flashed.unwrap());
    Ok(())
}
