use aoc21::*;

enum Foo {
    Score(u64),
    ExpectedClose(char),
}

fn analyze_char(c: char) -> Foo {
    match c {
        ')' => Foo::Score(3),
        ']' => Foo::Score(57),
        '}' => Foo::Score(1197),
        '>' => Foo::Score(25137),
        '(' => Foo::ExpectedClose(')'),
        '[' => Foo::ExpectedClose(']'),
        '{' => Foo::ExpectedClose('}'),
        '<' => Foo::ExpectedClose('>'),
        _ => unimplemented!(),
    }
}

fn main() -> DynResult<()> {
    let input = read_input!();
    let mut res1 = 0_u64;
    let mut complete_scores = Vec::new();
    'line_loop: for l in input.lines() {
        let mut stack = Vec::new();
        for c in l.chars() {
            match analyze_char(c) {
                Foo::ExpectedClose(c) => stack.push(c),
                Foo::Score(s) => {
                    let expected = stack.pop();
                    if expected != Some(c) {
                        res1 += s;
                        continue 'line_loop;
                    }
                }
            }
        }
        let mut line_score = 0_u64;
        while let Some(c) = stack.pop() {
            line_score *= 5;
            line_score += match c {
                ')' => 1,
                ']' => 2,
                '}' => 3,
                '>' => 4,
                _ => unimplemented!()
            }
        }
        complete_scores.push(line_score);
    }
    print_answer(1, res1);
    complete_scores.sort_unstable();
    print_answer(2, complete_scores[complete_scores.len()/2]);
    Ok(())
}
