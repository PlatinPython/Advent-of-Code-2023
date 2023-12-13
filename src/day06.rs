use aoc_runner_derive::aoc;
use itertools::Itertools;

#[aoc(day6, part1)]
fn part1(input: &str) -> usize {
    let (time, distance) = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .skip(1)
                .map(str::parse)
                .map(Result::unwrap)
        })
        .collect_tuple()
        .unwrap();

    time.interleave(distance)
        .tuples()
        .map(|(time, distance)| (0u32..time).filter(|i| i * (time - i) > distance).count())
        .product()
}

#[aoc(day6, part2)]
fn part2(input: &str) -> u64 {
    let (time, distance): (f64, f64) = input
        .lines()
        .map(|line| line.split_whitespace().skip(1).join("").parse().unwrap())
        .collect_tuple()
        .unwrap();

    let half_time = time / 2.;
    let max = half_time + ((half_time * half_time) - distance).sqrt();
    (max - (time - max)) as u64
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
