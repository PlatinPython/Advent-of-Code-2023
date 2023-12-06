use aoc_runner_derive::aoc;
use itertools::Itertools;

#[aoc(day6, part1)]
fn part1(input: &str) -> usize {
    let (time, distance) = input.lines().collect_tuple().unwrap();

    let time = time
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse::<u32>())
        .map(Result::unwrap);
    let distance = distance
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse::<u32>())
        .map(Result::unwrap);
    let races: Vec<(u32, u32)> = time.interleave(distance).tuples().collect_vec();

    races
        .iter()
        .map(|(time, distance)| (0u32..*time).filter(|i| i * (time - i) > *distance).count())
        .product()
}

#[aoc(day6, part2)]
fn part2(input: &str) -> usize {
    let (time, distance) = input
        .lines()
        .map(|line| line.split_whitespace().skip(1).join("").parse().unwrap())
        .collect_tuple()
        .unwrap();

    (0u64..time).filter(|i| i * (time - i) > distance).count()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn part1_example() {
        let input = indoc! {"
            Time:      7  15   30
            Distance:  9  40  200
        "};

        assert_eq!(part1(input), 288);
    }

    #[test]
    fn part2_example() {
        let input = indoc! {"
            Time:      7  15   30
            Distance:  9  40  200
        "};

        assert_eq!(part2(input), 71503);
    }
}
