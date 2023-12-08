use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use std::collections::HashMap;

#[aoc_generator(day8)]
fn parse(input: &str) -> (Vec<char>, HashMap<String, (String, String)>) {
    let directions = input.lines().next().unwrap().chars().collect_vec();
    let map = input
        .lines()
        .skip(2)
        .map(|line| {
            let (key, value) = line.split_once(" = ").unwrap();
            let value = value
                .split_once(", ")
                .map(|(left, right)| {
                    (
                        left.split_at(1).1.to_string(),
                        right.split_at(3).0.to_string(),
                    )
                })
                .unwrap();
            (key.to_string(), value)
        })
        .collect();
    (directions, map)
}

#[aoc(day8, part1)]
fn part1(input: &(Vec<char>, HashMap<String, (String, String)>)) -> u64 {
    get_steps(&input.0, &input.1, &"AAA".to_string(), |location| {
        location == "ZZZ"
    })
}

#[aoc(day8, part2)]
fn part2(input: &(Vec<char>, HashMap<String, (String, String)>)) -> u64 {
    input
        .1
        .keys()
        .filter(|key| key.ends_with('A'))
        .map(|location| {
            get_steps(&input.0, &input.1, location, |location| {
                location.ends_with('Z')
            })
        })
        .fold(1, lcm)
}

fn get_steps(
    directions: &[char],
    map: &HashMap<String, (String, String)>,
    start: &String,
    is_end: impl Fn(&String) -> bool,
) -> u64 {
    let mut current_location = start;
    let mut steps = 0;
    for direction in directions.iter().cycle() {
        current_location = match direction {
            'L' => &map.get(current_location).unwrap().0,
            'R' => &map.get(current_location).unwrap().1,
            _ => panic!("Unknown direction"),
        };
        steps += 1;
        if is_end(current_location) {
            break;
        }
    }
    steps
}

fn lcm(a: u64, b: u64) -> u64 {
    let mut u = a;
    let mut v = b;
    if v == 0 {
        return u;
    }
    loop {
        u %= v;
        if u == 0 {
            return (a / v) * b;
        }
        v %= u;
        if v == 0 {
            return (a / u) * b;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn parse_example() {
        let input = indoc! {"
            RL

            AAA = (BBB, CCC)
            BBB = (DDD, EEE)
            CCC = (ZZZ, GGG)
            DDD = (DDD, DDD)
            EEE = (EEE, EEE)
            GGG = (GGG, GGG)
            ZZZ = (ZZZ, ZZZ)
        "};

        let result = (
            vec!['R', 'L'],
            HashMap::from([
                ("AAA".into(), ("BBB".into(), "CCC".into())),
                ("BBB".into(), ("DDD".into(), "EEE".into())),
                ("CCC".into(), ("ZZZ".into(), "GGG".into())),
                ("DDD".into(), ("DDD".into(), "DDD".into())),
                ("EEE".into(), ("EEE".into(), "EEE".into())),
                ("GGG".into(), ("GGG".into(), "GGG".into())),
                ("ZZZ".into(), ("ZZZ".into(), "ZZZ".into())),
            ]),
        );

        assert_eq!(parse(input), result);

        let input = indoc! {"
            LLR

            AAA = (BBB, BBB)
            BBB = (AAA, ZZZ)
            ZZZ = (ZZZ, ZZZ)
        "};

        let result = (
            vec!['L', 'L', 'R'],
            HashMap::from([
                ("AAA".into(), ("BBB".into(), "BBB".into())),
                ("BBB".into(), ("AAA".into(), "ZZZ".into())),
                ("ZZZ".into(), ("ZZZ".into(), "ZZZ".into())),
            ]),
        );

        assert_eq!(parse(input), result);

        let input = indoc! {"
            LR

            11A = (11B, XXX)
            11B = (XXX, 11Z)
            11Z = (11B, XXX)
            22A = (22B, XXX)
            22B = (22C, 22C)
            22C = (22Z, 22Z)
            22Z = (22B, 22B)
            XXX = (XXX, XXX)
        "};

        let result = (
            vec!['L', 'R'],
            HashMap::from([
                ("11A".into(), ("11B".into(), "XXX".into())),
                ("11B".into(), ("XXX".into(), "11Z".into())),
                ("11Z".into(), ("11B".into(), "XXX".into())),
                ("22A".into(), ("22B".into(), "XXX".into())),
                ("22B".into(), ("22C".into(), "22C".into())),
                ("22C".into(), ("22Z".into(), "22Z".into())),
                ("22Z".into(), ("22B".into(), "22B".into())),
                ("XXX".into(), ("XXX".into(), "XXX".into())),
            ]),
        );

        assert_eq!(parse(input), result);
    }

    #[test]
    fn part1_example() {
        let input = indoc! {"
            RL

            AAA = (BBB, CCC)
            BBB = (DDD, EEE)
            CCC = (ZZZ, GGG)
            DDD = (DDD, DDD)
            EEE = (EEE, EEE)
            GGG = (GGG, GGG)
            ZZZ = (ZZZ, ZZZ)
        "};

        assert_eq!(part1(&parse(input)), 2);

        let input = indoc! {"
            LLR

            AAA = (BBB, BBB)
            BBB = (AAA, ZZZ)
            ZZZ = (ZZZ, ZZZ)
        "};

        assert_eq!(part1(&parse(input)), 6);
    }

    #[test]
    fn part2_example() {
        let input = indoc! {"
            LR

            11A = (11B, XXX)
            11B = (XXX, 11Z)
            11Z = (11B, XXX)
            22A = (22B, XXX)
            22B = (22C, 22C)
            22C = (22Z, 22Z)
            22Z = (22B, 22B)
            XXX = (XXX, XXX)
        "};

        assert_eq!(part2(&parse(input)), 6);
    }
}
