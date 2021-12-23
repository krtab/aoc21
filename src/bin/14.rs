use std::collections::HashMap;

use aoc21::*;

fn compute_res_from_pair_counts(pairs_count: &HashMap<(u8, u8), u64>, init: &str) -> u64 {
    let mut final_counts = HashMap::new();
    for (&(pl, pr), &c) in pairs_count {
        for elem in [pl, pr] {
            *final_counts.entry(elem).or_insert(0) += c
        }
    }
    for (elem, c) in &mut final_counts {
        if *elem == init.bytes().next().unwrap() || *elem == init.bytes().last().unwrap() {
            *c += 1;
        }
        *c /= 2;
    }
    let (_, count_max) = final_counts.iter().max_by_key(|(_, &c)| c).unwrap();
    let (_, count_min) = final_counts.iter().min_by_key(|(_, &c)| c).unwrap();
    count_max - count_min
}

fn main() -> DynResult<()> {
    let input = read_input!();
    let (init, rules_str) = input.split_once("\n\n").unwrap();
    let mut pairs_count = HashMap::new();
    for pair in init.as_bytes().windows(2) {
        let pair = (pair[0], pair[1]);
        *pairs_count.entry(pair).or_insert(0_u64) += 1;
    }
    let mut rules = HashMap::new();
    for l in rules_str.lines() {
        let l = l.as_bytes();
        let pair = (l[0], l[1]);
        let target = l[6];
        rules.insert(pair, target);
    }
    for step in 0..40 {
        let mut new_count = HashMap::with_capacity(pairs_count.capacity());
        for (p @ (pl, pr), c) in pairs_count {
            match rules.get(&p) {
                None => {
                    *new_count.entry(p).or_default() += c;
                }
                Some(&target) => {
                    *new_count.entry((pl, target)).or_default() += c;
                    *new_count.entry((target, pr)).or_default() += c;
                }
            }
        }
        pairs_count = new_count;
        if step == 9 {
            let res1 = compute_res_from_pair_counts(&pairs_count, init);
            print_answer(1, res1)
        }
    }
    let res2 = compute_res_from_pair_counts(&pairs_count, init);
    print_answer(2, res2);
    Ok(())
}
