pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let contents = std::fs::read_to_string("inputs/input4.txt")?;

    let parsed = parse_input(&contents);

    let part1 = calc_part1(&parsed);
    println!("part1 {}", part1);
    assert_eq!(part1, 2745);

    let part2 = calc_part2(&parsed);
    println!("part2 {}", part2);
    assert_eq!(part2, 6594);

    Ok(())
}

#[derive(Debug)]
struct BingoCardLine {
    line: Vec<u32>,
}

#[derive(Debug)]
struct BingoCard {
    lines: Vec<BingoCardLine>,
}

fn has_bingo(card: &BingoCard, matches: &[u32]) -> bool {
    let has_line_bingo = card.lines.iter().any(|line| line_is_bingo(matches, line));
    let has_col_bingo = card.lines
        .iter()
        .fold(vec![0, 0, 0, 0, 0], |acc, line|
            {
                let line_matches: Vec<bool> = line.line.iter().map(|x| matches.contains(x)).collect::<Vec<bool>>();
                let res = acc.iter().zip(line_matches)
                    .map(|(a, b)| if b { a + 1 } else { *a }).collect();
                res
            })
        .iter()
        .any(|cur| *cur == 5);
    has_line_bingo || has_col_bingo
}

fn line_is_bingo(matches: &[u32], line: &BingoCardLine) -> bool {
    line.line.iter().all(|x| matches.contains(x))
}

// returns list of bingo numbers and list of bingo cards
fn parse_input(contents: &str) -> (Vec<u32>, Vec<BingoCard>) {
    let mut lines = contents.lines();
    let number_line: Vec<u32> = lines.next().unwrap().split(',').map(|x| x.parse::<u32>().unwrap()).collect();
    lines.next(); // skip first empty line
    let cards = lines
        .fold((0, vec!(BingoCard { lines: vec!() })),
              |mut acc, curr| if curr.is_empty() {
                  acc.1.push(BingoCard { lines: vec!() });
                  (acc.0 + 1, acc.1)
              } else if !curr.is_empty() {
                  // dbg!(curr);
                  let bingo_line: BingoCardLine = BingoCardLine {
                      line: curr
                          .split(' ')
                          .filter(|x| !x.is_empty())
                          .map(|x| x.parse::<u32>().unwrap())
                          .collect()
                  };
                  acc.1[acc.0].lines.push(bingo_line);
                  (acc.0, acc.1)
              } else {
                  acc
              },
        ).1;
    (number_line, cards)
}

fn get_card_score(card: &BingoCard, numbers: &[u32]) -> u32 {
    card.lines.iter().fold(0, |acc, curr| {
        let x1: u32 = curr.line.iter().filter(|x| !numbers.contains(x)).sum();
        acc + x1
    })
}

fn calc_part1(inputs: &(Vec<u32>, Vec<BingoCard>)) -> u32 {
    for i in 0..inputs.0.len() {
        let numbers = &inputs.0[0..i];
        let cards: Vec<&BingoCard> = inputs.1.iter().filter(|card| has_bingo(card, numbers)).collect();
        if !cards.is_empty() {
            return get_card_score(cards.first().unwrap(), numbers) * numbers[i - 1];
        }
    }
    0
}

fn calc_part2(inputs: &(Vec<u32>, Vec<BingoCard>)) -> u32 {
    for i in 0..inputs.0.len() {
        let numbers = &inputs.0[0..i];
        let cards: Vec<&BingoCard> = inputs.1.iter().filter(|card| !has_bingo(card, numbers)).collect();
        if cards.len() == 1 {
            let card1 = cards.first().unwrap();
            let mut j = i;
            while !has_bingo(card1, &inputs.0[0..j]) {
                j += 1;
            }
            return get_card_score(card1, &inputs.0[0..j]) * inputs.0[j - 1];
        }
    }
    0
}

#[test]
fn test() {
    let test = r#"7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7"#;

    let vec = parse_input(test);

    // dbg!(vec);
    assert_eq!(calc_part1(&vec), 4512)
}


#[test]
fn test2() {
    let test = r#"
"#;

    let vec = parse_input(test);
    // assert_eq!(calc_part2(&vec), 00)
}
