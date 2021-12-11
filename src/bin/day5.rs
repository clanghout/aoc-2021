use std::collections::HashMap;

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let contents = std::fs::read_to_string("inputs/input5.txt")?;

    let parsed = parse_input(&contents);

    let part1 = calc_part1(&parsed);
    println!("part1 {}", part1);
    assert_eq!(part1, 7380);
    //
    let part2 = calc_part2(&parsed);
    println!("part2 {}", part2);
    assert_eq!(part2, 21373);

    Ok(())
}

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Coord(u32, u32);

fn parse_input(contents: &str) -> Vec<Vec<Coord>> {
    contents
        .lines()
        .filter(|x| !x.is_empty())
        .map(|x|
            x.split(" -> ").map(|y| {
                let mut split = y.split(',');
                Coord(split.next().unwrap().parse().unwrap(), split.next().unwrap().parse().unwrap())
            }).collect()
        ).collect()
}

fn interpolate_line(c1: &Coord, c2: &Coord) -> Vec<Coord> {
    if is_y_line(c1, c2) {
        (c1.1.min(c2.1)..=c1.1.max(c2.1)) // walk over y vals
            .map(|y| Coord(c1.0, y))// keep x the same
            .collect()
    } else if is_x_line(c1, c2) {
        (c1.0.min(c2.0)..=c1.0.max(c2.0)) // walk over x vals
            .map(|x| Coord(x, c1.1))// keep y val as one of the 2
            .collect()
        // must be diag line since we filtered first
    } else if is_left_up(c1, c2) {
        (c1.1.min(c2.1)..=c1.1.max(c2.1)) // walk over y vals
            .enumerate()
            .map(|(i, y)| Coord(c1.0.min(c2.0) + i as u32, y))
            // we start left top and add one to both x and y to move right and down
            .collect()
    } else { // right corner is top corner;
        (c1.1.min(c2.1)..=c1.1.max(c2.1))
            .enumerate()
            .map(|(i, y)| Coord(c1.0.max(c2.0) - i as u32, y))
            // we start right top; add one to y to move a step down; subtract from max x to step left
            .collect()
    }
}

fn is_y_line(c1: &Coord, c2: &Coord) -> bool {
    c1.0 == c2.0
}

fn is_x_line(c1: &Coord, c2: &Coord) -> bool {
    c1.1 == c2.1
}

// check if one of the points is left up of the other
fn is_left_up(c1: &Coord, c2: &Coord) -> bool {
    c1.1 < c2.1 && c1.0 < c2.0 ||
        c1.1 > c2.1 && c1.0 > c2.0
}

fn calc_part1(inputs: &[Vec<Coord>]) -> usize {
    inputs
        .iter()
        .filter(|x| is_straight_line(x))
        .flat_map(|coord| interpolate_line(&coord[0], &coord[1]))
        .fold(HashMap::new(), |mut map: HashMap<Coord, u8>, coord| {
            *map.entry(coord).or_default() += 1;
            map
        })
        .values()
        .filter(|x| **x > 1)
        .count()
}

fn is_straight_line(line: &[Coord]) -> bool {
    let c1 = &line[0];
    let c2 = &line[1];
    is_x_line(c1, c2) || is_y_line(c1, c2)
}

fn is_diag_line(line: &[Coord]) -> bool {
    let c1 = &line[0];
    let c2 = &line[1];
    let min_x = c1.0.min(c2.0);
    let max_x = c1.0.max(c2.0);
    let min_y = c1.1.min(c2.1);
    let max_y = c1.1.max(c2.1);
    max_x - min_x == max_y - min_y // check if the diff on x axis is same as y axis to check if they are on a diagonal
}

fn calc_part2(inputs: &[Vec<Coord>]) -> usize {
    inputs
        .iter()
        .filter(|x| is_straight_line(x) || is_diag_line(x))
        .flat_map(|coord| interpolate_line(&coord[0], &coord[1]))
        .fold(HashMap::new(), |mut map: HashMap<Coord, u8>, coord| {
            *map.entry(coord).or_default() += 1;
            map
        })
        .values()
        .filter(|x| **x > 1)
        .count()
}

#[test]
fn test() {
    let test = r#"0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2"#;

    let vec = parse_input(test);
    // dbg!(vec);
    assert_eq!(calc_part1(&vec), 5);
    assert_eq!(calc_part2(&vec), 12)
}

// #[test]
// fn test2() {
//     let test = r#"
// "#;
//
//     let vec = parse_input(test);
//     assert_eq!(calc_part2(&vec), 00)
// }
