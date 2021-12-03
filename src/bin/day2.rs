pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let contents = std::fs::read_to_string("inputs/input2.txt")?;

    let input_pairs: Vec<(&str, u32)> = parse_input(&contents);

    let part1 = calc_part1(&input_pairs);
    println!("part1 {}", part1);
    assert_eq!(part1, 2147104);

    let part2 = calc_part2(&input_pairs);
    println!("part2 {}", part2);
    assert_eq!(part2, 2044620088);
    Ok(())
}

fn parse_input(contents: &str) -> Vec<(&str, u32)> {
    contents
        .lines()
        .filter(|x| !x.is_empty())
        .map(|x| {
            let mut res = x.split_whitespace();
            (res.next().unwrap(), res.next().unwrap().parse::<u32>().unwrap())
        }).collect()
}

fn calc_part1(inputs: &[(&str, u32)]) -> u32 {
    let res = inputs.iter().fold((0, 0), |mut acc, curr| {
        // acc 0 = forward pos
        // acc 1 = depth
        match curr.0 {
            "forward" => acc.0 += curr.1,
            "up" => acc.1 -= curr.1,
            "down" => acc.1 += curr.1,
            _ => unreachable!()
        }
        acc
    });
    res.0 * res.1
}

fn calc_part2(inputs: &[(&str, u32)]) -> u32 {
    let res = inputs.iter().fold((0, 0, 0), |mut acc, curr| {
        // acc 0 = forward pos
        // acc 1 = depth
        // acc 2 = aim
        match curr.0 {
            "forward" => {
                acc.0 += curr.1;
                acc.1 += acc.2 * curr.1
            }
            "up" => acc.2 -= curr.1,
            "down" => acc.2 += curr.1,
            _ => unreachable!()
        }
        acc
    });
    res.0 * res.1
}

#[test]
fn test() {
    let test = r#"forward 5
down 5
forward 8
up 3
down 8
forward 2
"#;

    let vec = parse_input(test);
    println!("input: {}", vec.iter().flat_map(|x| x.0.chars()).collect::<String>());
    assert_eq!(calc_part1(&vec), 150)
}


#[test]
fn test2() {
    let test = r#"forward 5
down 5
forward 8
up 3
down 8
forward 2
"#;

    let vec = parse_input(test);
    println!("input: {}", vec.iter().flat_map(|x| x.0.chars()).collect::<String>());
    assert_eq!(calc_part2(&vec), 900)
}
