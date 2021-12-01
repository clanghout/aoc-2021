use std::fs::File;
use std::io::Read;

fn main() {
    let mut file = File::open("inputs/input1.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents);

    let input_numbers: Vec<u16> = contents
        .lines()
        .filter(|x| !x.is_empty())
        .map(|x| x.parse::<u16>().unwrap())
        .collect();

    let part1 = calc_part1(input_numbers.clone());

    println!("result: {}", part1);

    let part2 = calc_part2(input_numbers);

    println!("result: {}", part2);


    let test: Vec<u16> = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
    assert_eq!(calc_part1(test), 7)
}

fn calc_part1(input: Vec<u16>) -> u16 {
    let mut sum: u16 = 0;
    input
        .into_iter()
        .reduce(|a, b| {
            if a < b {
                sum = sum + 1;
            };
            b
        });
    sum
}

fn calc_part2(input: Vec<u16>) -> u16 {
    let mut sum: u16 = 0;
    let windows = input[..]
        .windows(3)
        .map(|x| x.into_iter().sum())
        .reduce(|a: u16, b: u16| {
            if a < b {
                sum = sum + 1;
            };
            b
        });
    sum
}


