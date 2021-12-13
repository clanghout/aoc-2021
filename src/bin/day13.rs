use std::collections::HashSet;

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let contents = std::fs::read_to_string("inputs/input13.txt")?;

    let (parsed_dots, parsed_fold_ops) = parse_input(&contents);

    let part1 = calc_part1(&parsed_dots, &parsed_fold_ops);
    println!("part1 {}", part1);
    assert!(part1 < 958);
    assert_eq!(part1, 795);

    // only printing today
    calc_part2(&parsed_dots, &parsed_fold_ops);

    Ok(())
}

#[derive(Debug, Eq, Hash, PartialEq, Copy, Clone)]
struct Dot(u32, u32);

#[derive(Debug)]
struct FoldOperation {
    direction: Direction,
    index: u32,
}

#[derive(Debug, PartialEq)]
enum Direction {
    Up,
    Left,
}

fn parse_input(contents: &str) -> (HashSet<Dot>, Vec<FoldOperation>) {
    let mut dots = HashSet::new();
    let mut fold_ops = vec![];
    let mut reading_dots = true;
    for line in contents.lines() {
        if line.is_empty() {
            reading_dots = false;
        } else if reading_dots {
            let dot = line.split(',').map(|x| x.parse::<u32>().unwrap()).collect::<Vec<u32>>();
            dots.insert(Dot(dot[0], dot[1]));
        } else {
            let fold_op: Vec<&str> = line.split('=').collect();
            let direction = match fold_op[0].contains('y') {
                true => Direction::Up,
                false => Direction::Left,
            };
            let index = fold_op[1].parse::<u32>().unwrap();
            fold_ops.push(FoldOperation { direction, index });
        }
    };
    (dots, fold_ops)
}

fn do_fold(dots: &mut HashSet<Dot>, fold_op: &FoldOperation) {
    let loopy = dots.clone();
    for dot in loopy {
        dots.remove(&dot);
        if fold_op.direction == Direction::Up {
            if dot.1 > fold_op.index {
                dots.insert(Dot(dot.0, fold_op.index - (dot.1 - fold_op.index)));
            } else {
                dots.insert(Dot(dot.0, dot.1));
            }
        } else if dot.0 > fold_op.index {
            dots.insert(Dot(fold_op.index - (dot.0 - fold_op.index), dot.1));
        } else {
            dots.insert(Dot(dot.0, dot.1));
        }
    };
}

fn calc_part1(dots: &HashSet<Dot>, fold_ops: &[FoldOperation]) -> usize {
    let mut dots = dots.clone();
    let operation = fold_ops.first().unwrap();
    do_fold(&mut dots, operation);
    dots.len()
}

fn calc_part2(dots: &HashSet<Dot>, fold_ops: &[FoldOperation]) {
    let mut mut_dots = dots.clone();
    for op in fold_ops.iter() {
        do_fold(&mut mut_dots, op);
    }

    let max_dot = mut_dots.iter().fold((0, 0), |acc, dot| {
        if dot.0 > acc.0 {
            (dot.0, acc.1)
        } else if dot.1 > acc.1 {
            (acc.0, dot.1)
        } else {
            acc
        }
    });
    for y in 0..=max_dot.1 {
        for x in 0..=max_dot.0 {
            if mut_dots.contains(&Dot(x, y)) {
                print!("⬜️");
            } else {
                print!("⬛️");
            }
        }
        println!();
    }
}


#[test]
fn test() {
    let test = r#"6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5"#;

    let (dots, fold_ops) = parse_input(test);
    assert_eq!(calc_part1(&dots, &fold_ops), 17);
    calc_part2(&dots, &fold_ops);
}


// #[test]
// fn test2() {
//     let test = r#"
// "#;
//
//     let vec = parse_input(test);
//     assert_eq!(calc_part2(&vec), 00)
// }
