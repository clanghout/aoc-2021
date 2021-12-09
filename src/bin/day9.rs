use std::collections::HashSet;

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let contents = std::fs::read_to_string("inputs/input9.txt")?;

    let parsed = parse_input(&contents);

    let part1 = calc_part1(&parsed);
    println!("part1 {}", part1);
    assert_eq!(part1, 524);

    let part2 = calc_part2(&parsed);
    println!("part2 {}", part2);
    assert_eq!(part2, 1235430);


    Ok(())
}

fn parse_input(contents: &str) -> Vec<Vec<u32>> {
    contents
        .trim()
        .lines()
        .map(|x| {
            x.chars().map(|c| c.to_digit(10).unwrap()).collect()
        }).collect()
}

fn as_neighbourhood(input: &[Vec<u32>]) -> Vec<Vec<(u32, Vec<u32>)>> {
    input.iter().enumerate().map(|(i, x)| {
        x.iter().enumerate().map(|(j, y)| {
            let mut neigbours: Vec<u32> = vec![];
            if i == 0 {
                if j == 0 {
                    neigbours.push(input[i][j + 1]);
                    neigbours.push(input[i + 1][j]);
                } else if j == input[0].len() - 1 {
                    neigbours.push(input[i][j - 1]);
                    neigbours.push(input[i + 1][j]);
                } else {
                    neigbours.push(input[i][j + 1]);
                    neigbours.push(input[i][j - 1]);
                    neigbours.push(input[i + 1][j]);
                }
            } else if i == input.len() - 1 {
                if j == 0 {
                    neigbours.push(input[i][j + 1]);
                    neigbours.push(input[i - 1][j]);
                } else if j == input[0].len() - 1 {
                    neigbours.push(input[i][j - 1]);
                    neigbours.push(input[i - 1][j]);
                } else {
                    neigbours.push(input[i][j + 1]);
                    neigbours.push(input[i][j - 1]);
                    neigbours.push(input[i - 1][j]);
                }
            } else {
                if j == 0 {
                    neigbours.push(input[i][j + 1]);
                    neigbours.push(input[i + 1][j]);
                    neigbours.push(input[i - 1][j]);
                } else if j == input[0].len() - 1 {
                    neigbours.push(input[i][j - 1]);
                    neigbours.push(input[i + 1][j]);
                    neigbours.push(input[i - 1][j]);
                } else {
                    neigbours.push(input[i][j + 1]);
                    neigbours.push(input[i][j - 1]);
                    neigbours.push(input[i + 1][j]);
                    neigbours.push(input[i - 1][j]);
                }
            }
            (*y, neigbours)
        }).collect()
    }).collect()
}

fn is_low_point(center: &(u32, Vec<u32>)) -> bool {
    return center.1.iter().fold(true, |acc, &curr| {
        acc && curr > center.0 // if curr is smaller, we do not have a low point; so our fold collapses to false
    });
}

#[derive(Debug, Eq, Hash, PartialEq, Copy, Clone)]
struct Coord(usize, usize);


fn basin_size(field: &[Vec<(u32, Vec<u32>)>], low_point_val: &(u32, Vec<u32>), low_point: Coord) -> u32 {
    let mut basin: HashSet<Coord> = HashSet::new();
    basin.insert(low_point);
    add_neighbours_to_basin(field, low_point_val, low_point, &mut basin, (field.len(), field[0].len()));
    basin.len() as u32
}

fn add_neighbours_to_basin(field: &[Vec<(u32, Vec<u32>)>], point_val: &(u32, Vec<u32>), point: Coord, basin: &mut HashSet<Coord>, field_size: (usize, usize)) {
    // we manually go over all neighbours
    let neighbors: Vec<Coord>;
    if point.0 == 0 {
        if point.1 == 0 {
            neighbors = vec![Coord(point.0 + 1, point.1), Coord(point.0, point.1 + 1)];
        } else if point.1 == field_size.1 - 1 {
            neighbors = vec![Coord(point.0 + 1, point.1), Coord(point.0, point.1 - 1)];
        } else {
            neighbors = vec![Coord(point.0 + 1, point.1), Coord(point.0, point.1 + 1), Coord(point.0, point.1 - 1)];
        }
    } else if point.0 == field_size.0 - 1 {
        if point.1 == 0 {
            neighbors = vec![Coord(point.0 - 1, point.1), Coord(point.0, point.1 + 1)];
        } else if point.1 == field_size.1 - 1 {
            neighbors = vec![Coord(point.0 - 1, point.1), Coord(point.0, point.1 - 1)];
        } else {
            neighbors = vec![Coord(point.0 - 1, point.1), Coord(point.0, point.1 + 1), Coord(point.0, point.1 - 1)];
        }
    } else {
        if point.1 == 0 { // on left row
            neighbors = vec![Coord(point.0 + 1, point.1), Coord(point.0, point.1 + 1), Coord(point.0 - 1, point.1)];
        } else if point.1 == field_size.1 - 1 { // on right row
            neighbors = vec![Coord(point.0 + 1, point.1), Coord(point.0, point.1 - 1), Coord(point.0 - 1, point.1)];
        } else {// somewhere in the middle
            neighbors = vec![Coord(point.0 + 1, point.1), Coord(point.0 - 1, point.1), Coord(point.0, point.1 + 1), Coord(point.0, point.1 - 1)];
        }
    }
    // check all neighbours if they are part of this basin
    neighbors.iter().for_each(|x| {
        let neighbour = &field[x.0][x.1];
        // check if this neighbor is not part of the basin already
        // if the val is not 9
        // and if this is one higher than the current point val WRONG -> check if it is any higher than the previous instead
        if !basin.contains(x) && neighbour.0 != 9 && neighbour.0 > point_val.0 {
            basin.insert(Coord(x.0, x.1));
            add_neighbours_to_basin(field, neighbour, Coord(x.0, x.1), basin, field_size);
        }
    })
}

fn calc_part1(inputs: &[Vec<u32>]) -> u32 {
    let neighbour_map = as_neighbourhood(inputs);
    neighbour_map.iter().fold(0, |acc, curr| {
        curr.iter().fold(0, |acc, curr| {
            if is_low_point(curr) {
                acc + curr.0 + 1
            } else {
                acc
            }
        }) + acc
    })
}

fn calc_part2(inputs: &[Vec<u32>]) -> u32 {
    let neighbour_map = as_neighbourhood(inputs);
    let mut basin_sizes = neighbour_map.iter().enumerate().fold(vec![], |mut acc, curr| {
        acc.append(&mut curr.1.iter().enumerate()
            .fold(vec![], |mut iacc, (j, icurr)| { // fold to basin size
                if is_low_point(icurr) { // should we check for existing in other basins? NO
                    // we are at the start of a basin
                    iacc.push(basin_size(&neighbour_map, icurr, Coord(curr.0, j)))
                }
                iacc
            }));
        acc
    });
    basin_sizes.sort();
    // take the largest 3 and multiply them together
    basin_sizes[basin_sizes.len() - 3..].iter().product()
}

#[test]
fn test() {
    let test = r#"2199943210
3987894921
9856789892
8767896789
9899965678"#;

    let vec = parse_input(test);
    // dbg!(&vec);
    assert_eq!(calc_part1(&vec), 15);
    assert_eq!(calc_part2(&vec), 1134);
}
