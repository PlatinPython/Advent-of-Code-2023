use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, PartialEq, Eq, Clone)]
struct Game {
    id: u32,
    winning: Vec<u32>,
    containing: Vec<u32>,
}

#[aoc_generator(day4)]
fn parse(input: &str) -> Vec<Game> {
    input
        .lines()
        .map(|line| {
            let (left, right) = line.split_once(':').unwrap();
            let id = left.replace("Card ", "").trim().parse().unwrap();
            let (winning, containing) = right.split_once('|').unwrap();
            Game {
                id,
                winning: winning
                    .trim()
                    .split(' ')
                    .filter_map(|s| s.parse().ok())
                    .collect(),
                containing: containing
                    .trim()
                    .split(' ')
                    .filter_map(|s| s.parse().ok())
                    .collect(),
            }
        })
        .collect()
}

#[aoc(day4, part1)]
fn part1(input: &[Game]) -> u32 {
    input
        .iter()
        .map(|game| {
            2u32.pow(
                game.containing
                    .iter()
                    .filter(|num| game.winning.contains(num))
                    .count() as u32,
            ) / 2
        })
        .sum()
}

#[aoc(day4, part2)]
fn part2(input: &[Game]) -> u32 {
    let mut games: Vec<(u32, &Game)> = input.iter().map(|game| (1, game)).collect();
    let mut i = 0;
    while i < games.len() {
        let (copies, game) = games[i];
        let new_cards = game
            .containing
            .iter()
            .filter(|num| game.winning.contains(num))
            .count();
        for j in 1..(new_cards + 1) {
            if i + j < games.len() {
                games[i + j].0 += copies;
            }
        }
        i += 1;
    }
    games.iter().map(|(copies, _)| *copies).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn parse_example() {
        let input = indoc! {"
            Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
            Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
            Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
            Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
            Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
            Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
        "};

        let result = vec![
            Game {
                id: 1,
                winning: vec![41, 48, 83, 86, 17],
                containing: vec![83, 86, 6, 31, 17, 9, 48, 53],
            },
            Game {
                id: 2,
                winning: vec![13, 32, 20, 16, 61],
                containing: vec![61, 30, 68, 82, 17, 32, 24, 19],
            },
            Game {
                id: 3,
                winning: vec![1, 21, 53, 59, 44],
                containing: vec![69, 82, 63, 72, 16, 21, 14, 1],
            },
            Game {
                id: 4,
                winning: vec![41, 92, 73, 84, 69],
                containing: vec![59, 84, 76, 51, 58, 5, 54, 83],
            },
            Game {
                id: 5,
                winning: vec![87, 83, 26, 28, 32],
                containing: vec![88, 30, 70, 12, 93, 22, 82, 36],
            },
            Game {
                id: 6,
                winning: vec![31, 18, 13, 56, 72],
                containing: vec![74, 77, 10, 23, 35, 67, 36, 11],
            },
        ];

        assert_eq!(parse(input), result);
    }

    #[test]
    fn part1_example() {
        let input = indoc! {"
            Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
            Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
            Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
            Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
            Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
            Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
        "};

        assert_eq!(part1(&parse(input)), 13);
    }

    #[test]
    fn part2_example() {
        let input = indoc! {"
            Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
            Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
            Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
            Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
            Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
            Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
        "};

        assert_eq!(part2(&parse(input)), 30);
    }
}
