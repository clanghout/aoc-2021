use std::ops::Deref;

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let contents = std::fs::read_to_string("inputs/input3.txt")?;

    let mut inputs = parse_input(&contents);
    let gamma = calc_gamma_rate(&inputs);
    let part1 = calc_part1(&gamma);
    println!("part1 {}", part1);
    assert_eq!(part1, 3885894);

    let part2 = calc_part2(&mut inputs);
    println!("part2 {}", part2);
    // assert_eq!(part2, 2044620088);
    Ok(())
}

fn parse_input(contents: &str) -> Vec<Vec<char>> {
    contents
        .lines()
        .filter(|x| !x.is_empty())
        .map(|x| {
            x.chars().collect()
        }).collect()
}

fn calc_gamma_rate(inputs: &[Vec<char>]) -> Vec<char> {
    let size = inputs.len();
    sum_by_column(inputs)
        .into_iter()
        .map(|x| if x as f32 / size as f32 > 0.5 { 1 } else { 0 })
        .map(|x| char::from_digit(x, 2).unwrap())
        .collect()
}

fn sum_by_column(inputs: &[Vec<char>]) -> Vec<u32> {
    let len = inputs[0].len();
    inputs.iter()
        .fold(vec![0; len],
              |acc, curr| {
                  let x1: Vec<u32> = curr.iter().map(|x| x.to_digit(2).unwrap()).collect();
                  acc.iter().zip(x1).map(|(a, b)| a + b).collect()
              })
}

fn sum_column(inputs: &Vec<&Vec<char>>, i: usize) -> u32 {
    inputs.iter()
        .filter_map(|x| x[i].to_digit(2))
        .sum()
}

fn calc_part1(gamma: &[char]) -> isize {
    let gammastr: String = gamma.iter().collect();
    let epsilonstr: String = gamma.into_iter().map(|x| if x.to_digit(2).unwrap() == 1 { '0' } else { '1' }).collect();

    isize::from_str_radix(gammastr.as_str(), 2).unwrap() *
        isize::from_str_radix(epsilonstr.as_str(), 2).unwrap()
}

fn calc_part2(inputs: &mut [Vec<char>]) -> isize {
    let o2: String = String::from_iter(calc_oxygen_rating(inputs));
    let co2: String = String::from_iter(calc_co2_rating(inputs));

    isize::from_str_radix(o2.as_str(), 2).unwrap() *
        isize::from_str_radix(co2.as_str(), 2).unwrap()
}

fn calc_oxygen_rating(inputs: &[Vec<char>]) -> &Vec<char> {
    let mut inputs = Vec::from_iter(inputs);
    for i in 0..inputs[0].len() {
        if inputs.len() > 1 {
            let sum = sum_column(&inputs, i) as f32;
            let len = inputs[0].len() as f32;
            if sum / len >= 0.5 {
                inputs = part2_filter(&inputs, i,'1');
            } else {
                inputs = part2_filter(&inputs, i,'0');
            }
        };
    };
    println!("Len of oxygen result = {}", inputs.len());
    inputs.first().unwrap()
}

fn calc_co2_rating(inputs: &[Vec<char>]) -> &Vec<char> {
    let mut inputs = Vec::from_iter(inputs);
    for i in 0..inputs[0].len() {
        if inputs.len() > 1 {
            let sum = sum_column(&inputs, i) as f32;
            let len = inputs[0].len() as f32;
            if sum / len < 0.5 {
                inputs = part2_filter(&inputs, i, '1');
            } else {
                inputs = part2_filter(&inputs, i, '0');
            }
        };
    };
    println!("Len of co2 result = {}", inputs.len());
    inputs.first().unwrap()
}

fn part2_filter<'a>(inputs: &'a [&Vec<char>], column: usize, compare_char: char) -> Vec<&'a Vec<char>> {
   inputs.iter().filter(|x| x[column] == compare_char).map(Deref::deref).collect()
}

#[test]
fn test() {
    let test = r#"00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010
"#;

    let vec = parse_input(test);
    assert_eq!(calc_gamma_rate(&vec), vec!['1', '0', '1', '1', '0'])
}


#[test]
fn test2() {
    let test = r#"00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010
"#;
    let mut vec = parse_input(test);
    assert_eq!(calc_part2(&mut vec), 230)
}
