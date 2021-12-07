pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let contents = std::fs::read_to_string("inputs/input7.txt")?;

    let parsed = parse_input(&contents);

    // highest crab pos
    let x = parsed.iter().max().unwrap() + 1;
    // convert crab positions to array where index is the crab location and value is the amount of crabs there
    let crabs = parsed.iter().fold(vec![0 as usize; x], |mut acc, curr| {
        acc[*curr] += 1;
        acc
    });

    let part1 = calc_part1(&crabs);
    println!("part1 {}", part1);
    assert_eq!(part1, 344605);

    let part2 = calc_part2(&crabs);
    println!("part2 {}", part2);
    assert_eq!(part2, 93699985);

    Ok(())
}

fn parse_input(contents: &str) -> Vec<usize> {
    contents
        .lines()
        .filter(|x| !x.is_empty())
        .next().unwrap()
        .split(",")
        .map(|x| {
            x.parse().unwrap()
        }).collect()
}

fn calc_part1(crabs: &[usize]) -> usize {
    crabs.iter().enumerate().map(|(i, _)| {
        crabs
            .iter()
            .enumerate()
            .fold(0, |acc, (j, crab2)| acc + crab2 * (i.max(j) - i.min(j)))
    }).min().unwrap()
}

fn calc_part2(crabs: &[usize]) -> usize {
    crabs.iter().enumerate().fold(usize::MAX, |acc, (i, _)| {
        let cost_pos_1 = crabs
            .iter()
            .enumerate()
            .fold(0, |acc, (j, crab2)| acc + part2_fuel_cost(i, j, crab2));
        if acc < cost_pos_1 { acc } else {
            cost_pos_1
        }
    })
}

// Calculate the fuel cost from pos i to j for x amount of crabs
fn part2_fuel_cost(i: usize, j: usize, amount_of_crabs: &usize) -> usize {
    let distance = i.max(j) - i.min(j) + 1;
    if distance == 0 { 0 } else {
        distance * (distance - 1) / 2 * amount_of_crabs
    }
}

#[test]
fn test() {
    let test = r#"16,1,2,0,4,2,7,1,2,14"#;

    let vec = parse_input(test);
    assert_eq!(part2_fuel_cost(16, 5, &1), 66);
    assert_eq!(calc_part1(&vec), 37);
    assert_eq!(calc_part2(&vec), 168);
}
