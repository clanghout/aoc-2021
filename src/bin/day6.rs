pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let contents = std::fs::read_to_string("inputs/input6.txt")?;

    let parsed = parse_input(&contents);

    let part1 = calc_part1(&parsed);
    println!("part1 {}", part1);
    assert_eq!(part1, 365862);

    let part2 = calc_part2(&parsed);
    println!("part2 {}", part2);
    assert_eq!(part2, 1653250886439);


    Ok(())
}

fn parse_input(contents: &str) -> Vec<u8> {
    contents
        .lines()
        .find(|x| !x.is_empty())
        .unwrap()
        .split(',')
        .map(|x| {
            x.parse().unwrap()
        }).collect()
}

// returns amount of new fishes
fn fish_cycle(fishes: &mut Vec<u8>) -> u32 {
    fishes.iter_mut().map(|x| {
        if *x == 0 {
            *x = 6;
            1
        } else {
            *x -= 1;
            0
        }
    }).sum()
}

fn calc_part1(inputs: &[u8]) -> usize {
    let mut fishes = Vec::from(inputs);
    (0..80).for_each(|_| {
        let new_fish = fish_cycle(&mut fishes);
        (0..new_fish).for_each(|_| fishes.push(8));
    });
    fishes.len()
}

fn fish_cycle2(fishes: &mut Vec<u64>) {
    let new_fishes = fishes.remove(0);
    fishes.push(new_fishes);
    fishes[6] += new_fishes;
}

fn fish_cycle2_alternative(fishes: &mut [u64], index: usize) {
    // add the fishes of day 0 to the new day6 fishes.
    // the day 8 fishes are automatically the day 0 fishes of the previous day
    fishes[(index + 7) % fishes.len()] += fishes[index % fishes.len()];
}

fn calc_part2(inputs: &[u8]) -> u64 {
    let mut fish_map = inputs.iter().fold([0; 9], |mut acc, curr| {
        acc[*curr as usize] += 1;
        acc
    });
    (0..256).for_each(|i| fish_cycle2_alternative(&mut fish_map, i));
    fish_map.iter().sum()
}

#[test]
fn test() {
    let test = r#"3,4,3,1,2"#;

    let vec = parse_input(test);
    assert_eq!(calc_part1(&vec), 5934);
    assert_eq!(calc_part2(&vec), 26984457539)
}

