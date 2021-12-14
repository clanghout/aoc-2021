use std::collections::HashMap;

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let contents = std::fs::read_to_string("inputs/input14.txt")?;

    let (template, insertions) = parse_input(&contents);

    let template_chars: Vec<char> = template.chars().collect();
    let part1 = calc_part1(template, &insertions, 10, template_chars.first().unwrap(), template_chars.last().unwrap());
    println!("part1 {}", part1);
    assert_eq!(part1, 2112);

    let part2 = calc_part1(template, &insertions, 40, template_chars.first().unwrap(), template_chars.last().unwrap());
    println!("part2 {}", part2);
    assert_eq!(part2, 3_243_771_149_914);

    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Insertion {
    present: (char, char),
    fill: char,
}

fn parse_input(contents: &str) -> (&str, Vec<Insertion>) {
    let (template, rules) = contents.split_once("\n\n").unwrap();
    let rules = rules.trim()
        .lines()
        .map(|x| {
            let (present, fill) = x.split_once(" -> ").unwrap();
            let present = present.chars().collect::<Vec<_>>();
            let present = (present[0], present[1]);
            Insertion { present, fill: fill.chars().next().unwrap() }
        }).collect();
    (template, rules)
}

fn calc_part1(template: &str, insertions: &[Insertion], steps: usize, first: &char, last: &char) -> usize {
    let mut polymer: HashMap<(char,char),usize> = HashMap::new();
    template.chars()
        .collect::<Vec<char>>()
        .as_slice()
        .windows(2)
        .for_each(|x| {
            let (a, b) = (x[0], x[1]);
            *polymer.entry((a, b)).or_default() += 1;
        });
    (0..steps).for_each(|_| {
        let mut new_polymer: HashMap<(char,char),usize> = HashMap::new();
        // fill new polymer
        insertions.iter().for_each(|x| {
            let (present, fill) = (x.present, x.fill);
            let &count = polymer.get(&(present.0, present.1)).unwrap_or(&0);
            *polymer.entry((present.0, present.1)).or_default() = 0;
            if count > 0 {
                *new_polymer.entry((present.0, fill)).or_default() += &count;
                *new_polymer.entry((fill, present.1)).or_default() += &count;
            }
        });
        // finish by adding all remaining elements (is this necesary?)
        polymer.iter().for_each(|(k, &v)| {
            if v > 0 {
                new_polymer.insert(*k, v);
            }
        });
        polymer = new_polymer;
    });
    let element_counts = polymer.iter().fold(HashMap::new(), |mut acc: HashMap<char, usize>, (k, v)| {
        *acc.entry(k.0).or_default() += v;
        *acc.entry(k.1).or_default() += v;
        acc
    }).iter().map(|(&k, v)| {
        if k == *first || k == *last {
            return v/2+1
        }
        v/2
    }).collect::<Vec<_>>();
    element_counts.iter().max().unwrap() - element_counts.iter().min().unwrap()
}

#[test]
fn test() {
    let test = r#"NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C"#;

    let (template, steps) = parse_input(test);
    let template_chars: Vec<char> = template.chars().collect();
    assert_eq!(calc_part1(template, &steps, 1, template_chars.first().unwrap(), template_chars.last().unwrap()), 1);
    assert_eq!(calc_part1(template, &steps, 10, template_chars.first().unwrap(), template_chars.last().unwrap()), 1588);
    assert_eq!(calc_part1(template, &steps, 40, template_chars.first().unwrap(), template_chars.last().unwrap()), 2188189693529);
}
