use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[aoc_generator(day9)]
fn parse(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(str::parse)
                .map(Result::unwrap)
                .collect()
        })
        .collect()
}

#[aoc(day9, part1)]
fn part1(input: &[Vec<i32>]) -> i32 {
    input.iter().map(|seq| extrapolate(seq, false)).sum()
}

#[aoc(day9, part2)]
fn part2(input: &[Vec<i32>]) -> i32 {
    input.iter().map(|seq| extrapolate(seq, true)).sum()
}

fn extrapolate(sequence: &[i32], backwards: bool) -> i32 {
    let mut iter = vec![sequence.to_vec()];
    while !iter.last().unwrap().iter().all_equal() {
        iter.push(
            iter.last()
                .unwrap()
                .iter()
                .tuple_windows()
                .map(|(a, b)| b - a)
                .collect(),
        );
    }
    let mut current_value = 0;
    for a in iter.iter().rev() {
        if backwards {
            current_value = a.first().unwrap() - current_value;
        } else {
            current_value += a.last().unwrap();
        }
    }
    current_value
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn parse_example() {
        let input = indoc! {"
            0 3 6 9 12 15
            1 3 6 10 15 21
            10 13 16 21 30 45
        "};

        assert_eq!(
            parse(input),
            vec![
                vec![0, 3, 6, 9, 12, 15],
                vec![1, 3, 6, 10, 15, 21],
                vec![10, 13, 16, 21, 30, 45],
            ]
        )
    }

    #[test]
    fn part1_example() {
        let input = indoc! {"
            0 3 6 9 12 15
            1 3 6 10 15 21
            10 13 16 21 30 45
        "};

        assert_eq!(part1(&parse(input)), 114);
    }

    #[test]
    fn part2_example() {
        let input = indoc! {"
            10 13 16 21 30 45
        "};

        assert_eq!(part2(&parse(input)), 5);
    }
}
