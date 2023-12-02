use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day2)]
fn parse(input: &str) -> Vec<(u32, u32, u32, u32)> {
    input
        .lines()
        .map(|line| {
            let (left, right) = line.split_once(':').unwrap();

            let left = left.replace("Game ", "").parse().unwrap();

            let (red, green, blue) = right
                .split(';')
                .map(|split| {
                    split
                        .split(',')
                        .map(str::trim)
                        .map(|split| {
                            let (num, color) = split.split_once(' ').unwrap();

                            (color, num.parse::<u32>().unwrap())
                        })
                        .fold((0, 0, 0), |mut acc, (color, num)| {
                            match color {
                                "red" if acc.0 < num => acc.0 = num,
                                "green" if acc.1 < num => acc.1 = num,
                                "blue" if acc.2 < num => acc.2 = num,
                                _ => {}
                            }
                            acc
                        })
                })
                .reduce(|mut acc, (red, green, blue)| {
                    if acc.0 < red {
                        acc.0 = red;
                    }
                    if acc.1 < green {
                        acc.1 = green;
                    }
                    if acc.2 < blue {
                        acc.2 = blue;
                    }
                    acc
                })
                .unwrap();

            (left, red, green, blue)
        })
        .collect()
}

#[aoc(day2, part1)]
fn part1(input: &[(u32, u32, u32, u32)]) -> u32 {
    input
        .iter()
        .filter(|(_, red, green, blue)| *red <= 12 && *green <= 13 && *blue <= 14)
        .map(|(index, ..)| index)
        .sum()
}

#[aoc(day2, part2)]
fn part2(input: &[(u32, u32, u32, u32)]) -> u32 {
    input
        .iter()
        .map(|(_, red, green, blue)| red * green * blue)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn parse_example() {
        let input = indoc! {"
            Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
            Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
            Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
            Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
            Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
        "};

        assert_eq!(
            parse(input),
            vec![
                (1, 4, 2, 6),
                (2, 1, 3, 4),
                (3, 20, 13, 6),
                (4, 14, 3, 15),
                (5, 6, 3, 2)
            ]
        );
    }

    #[test]
    fn part1_example() {
        let input = indoc! {"
            Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
            Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
            Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
            Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
            Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
        "};

        assert_eq!(part1(&parse(input)), 8);
    }

    #[test]
    fn part2_example() {
        let input = indoc! {"
            Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
            Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
            Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
            Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
            Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
        "};

        assert_eq!(part2(&parse(input)), 2286);
    }
}
