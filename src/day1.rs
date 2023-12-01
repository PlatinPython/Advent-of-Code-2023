use aoc_runner_derive::aoc;
use std::ops::AddAssign;

#[aoc(day1, part1)]
fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let mut result = "".to_string();

            if let Some(c) = line.chars().find(char::is_ascii_digit) {
                result.add_assign(&c.to_string());
            }

            if let Some(c) = line.chars().rev().find(char::is_ascii_digit) {
                result.add_assign(&c.to_string());
            }

            result.parse::<u32>().unwrap()
        })
        .sum()
}

#[aoc(day1, part2)]
fn part2(input: &str) -> u32 {
    const SUBSTRINGS: &[&str] = &[
        "1", "2", "3", "4", "5", "6", "7", "8", "9", "one", "two", "three", "four", "five", "six",
        "seven", "eight", "nine",
    ];

    fn substring_to_digit(s: &str) -> &str {
        match s {
            "one" => "1",
            "two" => "2",
            "three" => "3",
            "four" => "4",
            "five" => "5",
            "six" => "6",
            "seven" => "7",
            "eight" => "8",
            "nine" => "9",
            _ => s,
        }
    }

    input
        .lines()
        .map(|line| {
            let mut result = "".to_string();

            if let Some((_, first)) = SUBSTRINGS
                .iter()
                .filter_map(|&sub| line.find(sub).map(|i| (i, sub)))
                .min_by_key(|&(i, _)| i)
            {
                result.add_assign(substring_to_digit(first));
            }

            if let Some((_, last)) = SUBSTRINGS
                .iter()
                .filter_map(|&sub| line.rfind(sub).map(|i| (i, sub)))
                .max_by_key(|&(i, _)| i)
            {
                result.add_assign(substring_to_digit(last));
            }

            result.parse::<u32>().unwrap()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    #[test]
    fn part1_example() {
        let input = indoc! {"
            1abc2
            pqr3stu8vwx
            a1b2c3d4e5f
            treb7uchet
        "};

        assert_eq!(part1(input), 142);
    }

    #[test]
    fn part2_example() {
        let input = indoc! {"
            two1nine
            eightwothree
            abcone2threexyz
            xtwone3four
            4nineeightseven2
            zoneight234
            7pqrstsixteen
        "};

        assert_eq!(part2(input), 281);
    }
}
