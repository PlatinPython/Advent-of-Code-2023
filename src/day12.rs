use aoc_runner_derive::{aoc, aoc_generator};
use std::fmt::{Debug, Formatter, Write};
use itertools::Itertools;

#[derive(PartialEq, Eq, Clone, Copy)]
enum State {
    Operational,
    Damaged,
    Unknown,
}

impl Debug for State {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            State::Operational => f.write_char('.'),
            State::Damaged => f.write_char('#'),
            State::Unknown => f.write_char('?'),
        }
    }
}

impl From<char> for State {
    fn from(value: char) -> Self {
        match value {
            '.' => State::Operational,
            '#' => State::Damaged,
            '?' => State::Unknown,
            _ => panic!("Unknown state"),
        }
    }
}

#[aoc_generator(day12)]
fn parse(input: &str) -> Vec<(Vec<State>, Vec<u32>)> {
    input
        .lines()
        .map(|line| {
            let (left, right) = line.split_once(' ').unwrap();
            (
                left.chars().map(State::from).collect(),
                right
                    .split(',')
                    .map(str::parse)
                    .filter_map(Result::ok)
                    .collect(),
            )
        })
        .collect()
}

#[aoc(day12, part1)]
fn part1(input: &[(Vec<State>, Vec<u32>)]) -> usize {
    input.iter().map(|(states, groups)| {
        states.iter().map(|&state| match state {
            State::Unknown => vec![State::Operational, State::Damaged],
            _ => vec![state],
        }).multi_cartesian_product().filter(|states| {
            let mut lengths = vec![];
            let mut count = 0;

            for state in states {
                match state {
                    State::Damaged => count += 1,
                    _ => {
                        if count > 0 {
                            lengths.push(count);
                            count = 0;
                        }
                    }
                }
            }

            if count > 0 {
                lengths.push(count);
            }

            lengths.eq(groups)
        }).count()
    }).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn parse_example() {
        use State::*;

        let input = indoc! {"
            ???.### 1,1,3
            .??..??...?##. 1,1,3
            ?#?#?#?#?#?#?#? 1,3,1,6
            ????.#...#... 4,1,1
            ????.######..#####. 1,6,5
            ?###???????? 3,2,1
        "};

        let result = vec![
            (
                vec![
                    Unknown,
                    Unknown,
                    Unknown,
                    Operational,
                    Damaged,
                    Damaged,
                    Damaged,
                ],
                vec![1, 1, 3],
            ),
            (
                vec![
                    Operational,
                    Unknown,
                    Unknown,
                    Operational,
                    Operational,
                    Unknown,
                    Unknown,
                    Operational,
                    Operational,
                    Operational,
                    Unknown,
                    Damaged,
                    Damaged,
                    Operational,
                ],
                vec![1, 1, 3],
            ),
            (
                vec![
                    Unknown, Damaged, Unknown, Damaged, Unknown, Damaged, Unknown, Damaged,
                    Unknown, Damaged, Unknown, Damaged, Unknown, Damaged, Unknown,
                ],
                vec![1, 3, 1, 6],
            ),
            (
                vec![
                    Unknown,
                    Unknown,
                    Unknown,
                    Unknown,
                    Operational,
                    Damaged,
                    Operational,
                    Operational,
                    Operational,
                    Damaged,
                    Operational,
                    Operational,
                    Operational,
                ],
                vec![4, 1, 1],
            ),
            (
                vec![
                    Unknown,
                    Unknown,
                    Unknown,
                    Unknown,
                    Operational,
                    Damaged,
                    Damaged,
                    Damaged,
                    Damaged,
                    Damaged,
                    Damaged,
                    Operational,
                    Operational,
                    Damaged,
                    Damaged,
                    Damaged,
                    Damaged,
                    Damaged,
                    Operational,
                ],
                vec![1, 6, 5],
            ),
            (
                vec![
                    Unknown, Damaged, Damaged, Damaged, Unknown, Unknown, Unknown, Unknown,
                    Unknown, Unknown, Unknown, Unknown,
                ],
                vec![3, 2, 1],
            ),
        ];

        assert_eq!(parse(input), result);
    }

    #[test]
    fn part1_example() {
        let input = indoc! {"
            ???.### 1,1,3
            .??..??...?##. 1,1,3
            ?#?#?#?#?#?#?#? 1,3,1,6
            ????.#...#... 4,1,1
            ????.######..#####. 1,6,5
            ?###???????? 3,2,1
        "};

        assert_eq!(part1(&parse(input)), 21);
    }
}
