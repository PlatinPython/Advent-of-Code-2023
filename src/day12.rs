use aoc_runner_derive::{aoc, aoc_generator};
use memoize::memoize;

#[aoc_generator(day12)]
fn parse(input: &str) -> Vec<(Vec<char>, Vec<u32>)> {
    input
        .lines()
        .map(|line| {
            let (left, right) = line.split_once(' ').unwrap();
            (
                left.chars().collect(),
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
fn part1(input: &[(Vec<char>, Vec<u32>)]) -> usize {
    input
        .iter()
        .map(|(states, groups)| solve(states.clone(), groups.clone(), 0, 0))
        .sum()
}

#[aoc(day12, part2)]
fn part2(input: &[(Vec<char>, Vec<u32>)]) -> usize {
    input
        .iter()
        .map(|(states, groups)| {
            let states = {
                let mut states = states.clone();
                states.push('?');
                let mut states = states.repeat(5);
                states.pop();
                states
            };
            solve(states, groups.repeat(5), 0, 0)
        })
        .sum()
}

// Taken from: https://github.com/alcatrazEscapee/AdventOfCode/blob/main/2023/src/day12.cor#L12-L104
#[memoize]
fn solve(states: Vec<char>, groups: Vec<u32>, states_index: usize, groups_index: usize) -> usize {
    let mut states_index = states_index;
    let mut groups_index = groups_index;

    if states_index >= states.len() {
        return (groups_index == groups.len()) as usize;
    }

    while states[states_index] == '.' {
        states_index += 1;
        if states_index >= states.len() {
            return (groups_index == groups.len()) as usize;
        }
    }

    let mut n = 0;
    if states[states_index] == '?' {
        n += solve(
            states.clone(),
            groups.clone(),
            states_index + 1,
            groups_index,
        );
    }

    if groups_index >= groups.len() {
        return n;
    }

    let p = groups[groups_index] as usize - 1;
    if states_index + p >= states.len() {
        return n;
    }

    for _ in 0..p {
        states_index += 1;

        if states[states_index] == '.' {
            return n;
        }
    }

    states_index += 1;
    groups_index += 1;

    if states_index >= states.len() {
        n += (groups_index == groups.len()) as usize;
        return n;
    }

    if states[states_index] == '#' {
        return n;
    }

    n += solve(states, groups, states_index + 1, groups_index);

    n
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn parse_example() {
        let input = indoc! {"
            ???.### 1,1,3
            .??..??...?##. 1,1,3
            ?#?#?#?#?#?#?#? 1,3,1,6
            ????.#...#... 4,1,1
            ????.######..#####. 1,6,5
            ?###???????? 3,2,1
        "};

        let result = vec![
            (vec!['?', '?', '?', '.', '#', '#', '#'], vec![1, 1, 3]),
            (
                vec![
                    '.', '?', '?', '.', '.', '?', '?', '.', '.', '.', '?', '#', '#', '.',
                ],
                vec![1, 1, 3],
            ),
            (
                vec![
                    '?', '#', '?', '#', '?', '#', '?', '#', '?', '#', '?', '#', '?', '#', '?',
                ],
                vec![1, 3, 1, 6],
            ),
            (
                vec![
                    '?', '?', '?', '?', '.', '#', '.', '.', '.', '#', '.', '.', '.',
                ],
                vec![4, 1, 1],
            ),
            (
                vec![
                    '?', '?', '?', '?', '.', '#', '#', '#', '#', '#', '#', '.', '.', '#', '#', '#',
                    '#', '#', '.',
                ],
                vec![1, 6, 5],
            ),
            (
                vec!['?', '#', '#', '#', '?', '?', '?', '?', '?', '?', '?', '?'],
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

    #[test]
    fn part2_example() {
        let input = indoc! {"
            ???.### 1,1,3
            .??..??...?##. 1,1,3
            ?#?#?#?#?#?#?#? 1,3,1,6
            ????.#...#... 4,1,1
            ????.######..#####. 1,6,5
            ?###???????? 3,2,1
        "};

        assert_eq!(part2(&parse(input)), 525152);
    }
}
