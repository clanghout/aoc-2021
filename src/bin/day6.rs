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
        .filter(|x| !x.is_empty())
        .next().unwrap()
        .split(",")
        .map(|x| {
            x.parse().unwrap()
        }).collect()
}

// returns amount of new fishes
fn fish_cycle(fishes: &mut Vec<u8>) -> u32 {
    fishes.into_iter().map(|x| {
        if *x == 0 as u8 {
            *x = 6 as u8;
            1
        } else {
            *x -= 1;
            0
        }
    }).sum()
}

fn calc_part1(inputs: &Vec<u8>) -> usize {
    let mut fishes = inputs.clone();
    (0..80).for_each(|_| {
        let new_fish = fish_cycle(&mut fishes);
        (0..new_fish).for_each(|_| fishes.push(8));
        // dbg!(&fishes);
    });
    fishes.iter().count()
}

fn fish_cycle2(fishes: &mut Vec<u64>) {
    let new_fishes = fishes.remove(0);
    fishes.push(new_fishes);
    fishes[6] += new_fishes;
}

fn calc_part2(inputs: &Vec<u8>) -> u64 {
    let fishes = inputs;
    let mut fish_map = fishes.iter().fold(vec![0 as u64, 0, 0, 0, 0, 0, 0, 0, 0], |mut acc, curr| {
        acc[*curr as usize] += 1;
        acc
    });
    (0..256).for_each(|_| fish_cycle2(&mut fish_map));

    fish_map.iter().sum()
}

#[test]
fn test() {
    let test = r#"3,4,3,1,2"#;

    let vec = parse_input(test);
    assert_eq!(calc_part1(&vec), 5934);
    assert_eq!(calc_part2(&vec), 26984457539)
}

