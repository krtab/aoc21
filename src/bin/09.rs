use aoc21::*;

fn main() -> DynResult<()> {
    let input = read_input!();
    let mut map = [[9_u8; 101]; 101];
    let grid_size = input.lines().next().unwrap().len();
    dbg!(grid_size);
    for (line, row) in input.lines().zip(&mut map) {
        for (cr, cell) in line.bytes().zip(row) {
            *cell = cr - b'0'
        }
    }
    let mut res1 = 0_u64;
    let mut bassin_sizes = Vec::new();
    let mut visited = [[false; 101]; 101];
    let mut stack = Vec::new();
    for x in 0..grid_size {
        'yloop: for y in 0..grid_size {
            let v = map[x][y];
            if (x > 0 && map[x - 1][y] <= v)
                || (y > 0 && map[x][y - 1] <= v)
                || map[x + 1][y] <= v
                || map[x][y + 1] <= v
            {
                continue 'yloop;
            }
            res1 += (v + 1) as u64;
            let bassin_idx = bassin_sizes.len();
            bassin_sizes.push(0);
            stack.push(((x, y), bassin_idx));
        }
    }
    print_answer(1, res1);
    while let Some(((x, y), idx)) = stack.pop() {
        if std::mem::replace(&mut visited[x][y], true) {
            continue;
        }
        if map[x][y] == 9 {
            continue;
        }
        bassin_sizes[idx] += 1;
        if x > 0 {
            stack.push(((x - 1, y), idx));
        }
        if y > 0 {
            stack.push(((x, y - 1), idx));
        }
        stack.push(((x + 1, y), idx));
        stack.push(((x, y + 1), idx));
    }
    bassin_sizes.sort_unstable_by_key(|&x| std::cmp::Reverse(x));
    let res2: u64 = bassin_sizes[..3].iter().product();
    print_answer(2, res2);
    Ok(())
}
