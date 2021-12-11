pub fn main() {
    let contents = include_bytes!("../../inputs/input10.txt");

    let parsed = parse_input(contents);

    let part1 = calc_part1(&parsed);
    println!("part1 {}", part1);
    assert_eq!(part1, 193275);

    let part2 = calc_part2(&parsed);
    println!("part2 {}", part2);
    assert_eq!(part2, 2429644557);
}

// parse input into a vector of vectors
fn parse_input(contents: &[u8]) -> Vec<Vec<u8>> {
    let mut parsed: Vec<Vec<u8>> = Vec::new();
    let mut current_line: Vec<u8> = Vec::new();

    for c in contents {
        match c {
            b'\n' => {
                parsed.push(current_line);
                current_line = Vec::new();
            }
            _ => {
                current_line.push(*c);
            }
        }
    }
    parsed.push(current_line);

    parsed
}

fn calc_part1(inputs: &[Vec<u8>]) -> usize {
    inputs.iter().map(|line| {
        let mut stack = Vec::new();
        line.iter().filter_map(|c| {
            // push opening char to stack
            match c {
                b'(' | b'[' | b'{' | b'<' => {
                    stack.push(*c);
                    None
                }
                b')' => {
                    let i = stack.pop().unwrap();
                    if i != b'(' {
                        Some(3)
                    } else {
                        None
                    }
                }
                b']' => {
                    if stack.pop().unwrap() != b'[' {
                        Some(57)
                    } else {
                        None
                    }
                }
                b'}' => {
                    if stack.pop().unwrap() != b'{' {
                        Some(1197)
                    } else {
                        None
                    }
                }
                b'>' => {
                    if stack.pop().unwrap() != b'<' {
                        Some(25137)
                    } else {
                        None
                    }
                }
                _ => unreachable!()
            }
        }).next().unwrap_or(0usize)
    }).sum()
}

fn calc_part2(inputs: &[Vec<u8>]) -> usize {
    let mut line_totals = inputs.iter().enumerate().map(|(i, line)| {
        let mut stack = Vec::new();
        let is_valid_line = line.iter().all(|c| {
            // push opening char to stack
            match c {
                b'(' | b'[' | b'{' | b'<' => {
                    stack.push(*c);
                    true
                }
                b')' => {
                    let i = stack.pop().unwrap();
                    if i != b'(' {
                        false
                    } else {
                        true
                    }
                }
                b']' => {
                    if stack.pop().unwrap() != b'[' {
                        false
                    } else {
                        true
                    }
                }
                b'}' => {
                    if stack.pop().unwrap() != b'{' {
                        false
                    } else {
                        true
                    }
                }
                b'>' => {
                    if stack.pop().unwrap() != b'<' {
                        false
                    } else {
                        true
                    }
                }
                _ => unreachable!()
            }
        });
        if is_valid_line {
            stack.iter().rev().map(|c| {
                match c {
                    b'(' => 1,
                    b'[' => 2,
                    b'{' => 3,
                    b'<' => 4,
                    _ => unreachable!()
                }
            }).fold(0, |acc, i| {
                acc * 5 + i
            })
        } else { 0 }
    }).filter(
        |&i| i > 0
    ).collect::<Vec<usize>>();
    // sort by value
    line_totals.sort_unstable();
    // find middle value
    let middle = line_totals.len() / 2;
    line_totals[middle]
}

#[test]
fn test() {
    let test = r#"[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]"#.as_bytes();

    let vec = parse_input(test);
    // dbg!(&vec);
    assert_eq!(calc_part1(&vec), 26397);
    assert_eq!(calc_part2(&vec), 288957)
}
