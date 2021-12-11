use std::collections::{HashMap, HashSet};

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let contents = std::fs::read_to_string("inputs/input8.txt")?;

    let parsed = parse_input(&contents);

    let part1 = calc_part1(&parsed);
    println!("part1 {}", part1);
    assert_eq!(part1, 392);

    let part2 = calc_part2(&parsed);
    println!("part2 {}", part2);
    assert_eq!(part2, 1004688);

    Ok(())
}

fn parse_input(contents: &str) -> Vec<(Vec<&str>, Vec<&str>)> {
    contents
        .lines()
        .filter(|x| !x.is_empty())
        .map(|x| {
            let mut map = x.split(" | ").map(|digit_line| digit_line.split(' ').collect());
            (map.next().unwrap(), map.next().unwrap())
        }).collect()
}

fn calc_part1(inputs: &[(Vec<&str>, Vec<&str>)]) -> usize {
    let allowed_lens = [2, 3, 4, 7];
    inputs.iter().fold(0, |acc, input| input.1.iter().filter(|x| allowed_lens.contains(&x.len())).count() + acc)
}

fn calc_part2(inputs: &[(Vec<&str>, Vec<&str>)]) -> usize {
    inputs.iter().fold(0, |acc, input| decode_number(input) + acc)
}

fn decode_number(input: &(Vec<&str>, Vec<&str>)) -> usize {
    let mut translations: HashMap<&str, usize> = HashMap::new();

    let mut both = input.0.iter().map(|x| {
        let mut chars = x.chars().collect::<Vec<char>>();
        chars.sort_unstable();
        chars.iter().collect::<String>()
    }).collect::<Vec<String>>();
    let mut x2 = input.1.iter().map(|x| {
        let mut chars = x.chars().collect::<Vec<char>>();
        chars.sort_unstable();
        chars.iter().collect::<String>()
    }).collect::<Vec<String>>();
    both.append(&mut x2);
    let one = both.iter().find(|x| x.len() == 2).unwrap();
    let seven = both.iter().find(|x| x.len() == 3).unwrap();
    let four = both.iter().find(|x| x.len() == 4).unwrap();
    let eight = both.iter().find(|x| x.len() == 7).unwrap();
    translations.insert(one, 1);
    translations.insert(seven, 7);
    translations.insert(four, 4);
    translations.insert(eight, 8);
    let m_lt: Vec<char> = four.chars().filter(|&x| !one.chars().any(|a| a == x)).collect();

    let chars_len5 = both.iter().filter(|x| x.len() == 5).map(|x| x.as_str()).collect::<HashSet<&str>>(); // 2, 5, 3
    let chars_len6 = both.iter().filter(|x| x.len() == 6).map(|x| x.as_str()).collect::<HashSet<&str>>(); // 0, 6, 9

    let five = chars_len5.iter().filter(|x| {
        x.chars().filter(|y| {
            !m_lt.contains(y)
        }).count() == 3
    }).collect::<Vec<&&str>>();

    if !five.is_empty() {
        translations.insert(*five[0], 5);
    }
    let three = chars_len5.iter().filter(|x| {
        x.chars().filter(|y| {
            !m_lt.contains(y)
        }).count() != 3 &&
        x.chars().filter(|&y| {
            !seven.chars().any(|x| x == y)
        }).count() == 2
    }).collect::<Vec<&&str>>();
    if !three.is_empty() {
        translations.insert(*three[0], 3);
    }
    let two = chars_len5.iter().filter(|x| {
        x.chars().filter(|&y| {
            !m_lt.contains(&y)
        }).count() != 3 &&
            x.chars().filter(|&y| {
                !seven.chars().any(|x| x == y)
            }).count() == 3
    }).collect::<Vec<&&str>>();
    if !two.is_empty() {
        translations.insert(*two[0], 2);
    }

    let zero = chars_len6.iter().filter(|x| {
        x.chars().filter(|y| {
            !m_lt.contains(y)
        }).count() == 5
    }).collect::<Vec<&&str>>();
    if !zero.is_empty() {
        translations.insert(*zero[0], 0);
    }
    let six = chars_len6.iter()
        .filter(|x| {
            x.chars().filter(|&y| {
                !one.chars().any(|x| x == y)
            }).count() == 5
        }).collect::<Vec<&&str>>();
    if !six.is_empty() {
        translations.insert(*six[0], 6);
    }
    let nine = chars_len6.iter()
        .filter(|x| {
            x.chars().filter(|y| {
                !m_lt.contains(y)
            }).count() != 5 &&
            x.chars().filter(|&y| {
                !one.chars().any(|x| x == y)
            }).count() == 4
        }).collect::<Vec<&&str>>();
    if !nine.is_empty() {
        translations.insert(*nine[0], 9);
    }

    let x1 = input.1.iter().map(|x| {
        let mut vec = x.chars().collect::<Vec<char>>();
        vec.sort_unstable();
        let x3 = vec.iter().collect::<String>();
        translations.get(x3.as_str()).unwrap().to_string()
    }).collect::<String>();
    x1.parse().unwrap()
}

#[test]
fn test() {
    let test = r#"be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce"#;

    let vec = parse_input(test);
    assert_eq!(calc_part1(&vec), 26);
    assert_eq!(calc_part2(&vec), 61229);
}
