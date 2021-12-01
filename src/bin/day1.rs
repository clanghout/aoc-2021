pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let contents = std::fs::read_to_string("inputs/input1.txt")?;

    let input_numbers: Vec<u16> = contents
        .lines()
        .filter_map(|x| x.parse::<u16>().ok())
        .collect();

    let part1 = calc_part1(&input_numbers);

    println!("result: {}", part1);

    let part2 = calc_part2(&input_numbers);

    println!("result: {}", part2);

    Ok(())
}

fn calc_part1(input: &[u16]) -> usize {
    input.windows(2)
        .filter(|x| x[0] < x[1])
        .count()
}

fn calc_part2(input: &[u16]) -> usize {
    let windows: Vec<u16> = input
        .windows(3)
        .map(|x| x.iter().sum())
        .collect();
    calc_part1(&windows)
}


#[test]
fn test() {
    let test: Vec<u16> = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
    assert_eq!(calc_part1(&test), 7)
}
