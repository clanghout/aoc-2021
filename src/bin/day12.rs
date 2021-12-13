use std::collections::{HashMap};

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let contents = std::fs::read_to_string("inputs/input12.txt")?;

    let parsed = parse_input(&contents);

    let part1 = calc_part1(&parsed);
    println!("part1 {}", part1);
    assert_eq!(part1, 4749);

    let part2 = calc_part2(&parsed);
    println!("part2 {}", part2);
    assert_eq!(part2, 123054);


    Ok(())
}

fn parse_input(contents: &str) -> HashMap<&str, Vec<&str>> {
    contents
        .trim()
        .lines()
        .fold(HashMap::new(), |mut acc, x| {
            let (left, right) = x.split_once('-').unwrap();
            acc.entry(left).or_default().push(right);
            acc.entry(right).or_default().push(left);
            acc
        })
}

trait IsSmallCave {
    fn is_small_cave(&self) -> bool;
}

impl IsSmallCave for &str {
    fn is_small_cave(&self) -> bool {
        self.chars().all(|x| x.is_ascii_lowercase())
    }
}

fn depth_first_search(map: &HashMap<&str, Vec<&str>>, pos: &str, visited: &[&str]) -> u32 {
    if pos == "end" {
        return 1;
    }

    let mut new_visited = visited.to_owned();

    if pos.is_small_cave() {
        new_visited.push(pos);
    }

    map
        .get(pos)
        .unwrap()// get all next nodes
        .iter()
        .filter(|x| !visited.contains(x)) // filter the ones we can still go to
        .map(|x| depth_first_search(map, x, &new_visited)) // find which route till the end
        .sum() // returns 0 for empty list
}

fn calc_part1(inputs: &HashMap<&str, Vec<&str>>) -> u32 {
    // start at the start node and recursively visit all edges
    let vec1: Vec<&str> = vec![];
    depth_first_search(inputs, "start", &vec1)
}

fn dfs2(map: &HashMap<&str, Vec<&str>>, pos: &str, visited: &[&str], mut visited_again: bool) -> u32 {
    if pos == "end" {
        return 1;
    }
    if visited.contains(&pos) {
        if visited_again || pos == "start" {
            return 0;
        } else if pos.is_small_cave() {
            visited_again = true;
        }
    }


    let mut new_visited = visited.to_owned();

    if pos.is_small_cave() {
        new_visited.push(pos);
    }

    map
        .get(pos)
        .unwrap()// get all next nodes
        .iter() // filter the ones we can still go to
        .map(|x| dfs2(map, x, &new_visited, visited_again))// find which route till the end
        .sum() // returns 0 for empty list
}

fn calc_part2(inputs: &HashMap<&str, Vec<&str>>) -> u32 {
    dfs2(inputs, "start", &[], false)
}

#[test]
fn test() {
    let test = r#"start-A
start-b
A-c
A-b
b-d
A-end
b-end"#;

    let vec = parse_input(test);
    // assert_eq!(calc_part1(&vec), 10);
    assert_eq!(calc_part2(&vec), 36);
}

#[test]
fn test2() {
    let test = r#"dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc"#;

    let vec = parse_input(test);
    assert_eq!(calc_part1(&vec), 19);
    assert_eq!(calc_part2(&vec), 103);
}

#[test]
fn test3() {
    let test = r#"fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW"#;

    let vec = parse_input(test);
    assert_eq!(calc_part1(&vec), 226);
    assert_eq!(calc_part2(&vec), 3509);
}
