pub fn main() {
    let contents = include_bytes!("../../inputs/input11.txt");

    let parsed = parse_input(contents);

    let part1 = calc_part1(&parsed);
    println!("part1 {}", part1);
    assert_eq!(part1, 1634);

    let part2 = calc_part2(&parsed);
    println!("part2 {}", part2);
    assert_eq!(part2, 210);
}

fn parse_input(contents: &[u8]) -> Vec<Vec<u8>> {
    contents.split(|&x| x == b'\n')
        .map(|x| x.to_vec())
        .collect()
}

// cannot use slice due to mutablity
fn calc_part1(inputs: &Vec<Vec<u8>>) -> usize {
    let mut squid_map = inputs.clone();
    (0..100).map(|_cycle| {
        // add 1 to all values in inputs
        new_cycle(&mut squid_map);
        // while we have squids that have a value over 9
        let mut check_again = true;
        while check_again {
            check_again = false;
            // loop over all squares
            for x in 0..squid_map.len() {
                for y in 0..squid_map[0].len() {
                    if squid_map[x][y] > b'9' {
                        squid_map[x][y] = b'0';
                        // find all neighbours
                        for (ix, iy) in [(-1isize, -1isize), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)] {
                            // if neighbour is not 0 or out of bounds
                            let x = x as isize + ix;
                            let y = y as isize + iy;
                            if x >= 0 && x < squid_map.len() as isize &&
                                y >= 0 && y < squid_map[0].len() as isize &&
                                squid_map[x as usize][y as usize] != b'0' {
                                squid_map[x as usize][y as usize] += 1;
                                // make sure to run again for any squids that are now over 9
                                check_again = check_again || squid_map[x as usize][y as usize] > b'9';
                            }
                        };
                    }
                }
            }
        };
        // count all the glowsquids with value 0. That is the amount of glowsquids that triggered this round.
        squid_map.iter().map(|line| line.iter().filter(|&&x| x == b'0').count()).sum()
    }).sum()
}

fn new_cycle(squid_map: &mut [Vec<u8>]) {
    for x in 0..squid_map.len() {
        for y in 0..squid_map[0].len() {
            squid_map[x][y] += 1
        }
    }
}

fn calc_part2(inputs: &Vec<Vec<u8>>) -> usize {
    let mut squid_map = inputs.clone();
    let mut completed = false;
    let mut cycle = 0;
    while !completed {
        // add 1 to all values in inputs
        new_cycle(&mut squid_map);
        // while we have squids that have a value over 9
        let mut check_again = true;
        while check_again {
            check_again = false;
            // loop over all squares
            for x in 0..squid_map.len() {
                for y in 0..squid_map[0].len() {
                    if squid_map[x][y] > b'9' {
                        squid_map[x][y] = b'0';
                        // find all neighbours
                        for (ix, iy) in [(-1isize, -1isize), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)] {
                            // if neighbour is not 0 or out of bounds
                            let x = x as isize + ix;
                            let y = y as isize + iy;
                            if x >= 0 && x < squid_map.len() as isize &&
                                y >= 0 && y < squid_map[0].len() as isize &&
                                squid_map[x as usize][y as usize] != b'0' {
                                squid_map[x as usize][y as usize] += 1;
                                check_again = true;
                            }
                        };
                    }
                }
            }
        };

        // count all the glowsquids with value 0. That is the amount of glowsquids that triggered this round.
        let flashes_count: usize = squid_map.iter().map(|line| line.iter().filter(|&&x| x == b'0').count()).sum();
        // check if all squares are 0
        if flashes_count == squid_map.len() * squid_map[0].len() {
            completed = true;
        }
        cycle += 1;
    }
    cycle
}

#[test]
fn test() {
    let test = r#"5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526"#.as_bytes();

    let vec = parse_input(test);
    assert_eq!(calc_part1(&vec), 1656);
    assert_eq!(calc_part2(&vec), 195);
}
