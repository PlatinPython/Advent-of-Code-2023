use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use std::collections::HashMap;
use std::ops::Range;

#[derive(Debug, PartialEq, Eq)]
struct Almanac {
    seeds: Seeds,
    seed_to_soil: Map,
    soil_to_fertilizer: Map,
    fertilizer_to_water: Map,
    water_to_light: Map,
    light_to_temperature: Map,
    temperature_to_humidity: Map,
    humidity_to_location: Map,
}

#[derive(Debug, PartialEq, Eq)]
enum Seeds {
    Single(Vec<u64>),
    Ranges(Vec<Range<u64>>),
}

type Map = HashMap<Range<u64>, Range<u64>>;

trait RangeGet {
    fn range_get(&self, value: u64) -> u64;
}

impl RangeGet for Map {
    fn range_get(&self, value: u64) -> u64 {
        self.keys()
            .find(|key| key.contains(&value))
            .and_then(|key| self.get_key_value(key))
            .map(|(src, dest)| map(value, src.start, src.end - 1, dest.start, dest.end - 1))
            .unwrap_or(value)
    }
}

fn map(x: u64, in_min: u64, in_max: u64, out_min: u64, out_max: u64) -> u64 {
    ((x - in_min) as f64 * (out_max - out_min) as f64 / (in_max - in_min) as f64) as u64 + out_min
}

fn map_ranges(range: Range<u64>, map: &Map) -> Vec<Range<u64>> {
    let mut vec = map.keys().collect_vec();
    vec.sort_by_key(|range| range.start);

    let mut result = Vec::new();
    let mut start = range.start;

    for r in vec {
        if r.start > start {
            if range.end <= r.start {
                continue;
            } else {
                result.push(start..r.start);
                start = r.start;
            }
        }
        if start > r.end {
            continue;
        }
        if r.end <= range.end {
            result.push(start..r.end);
            start = r.end;
        }
    }

    if range.end > start {
        result.push(start..range.end);
    }

    result
        .iter()
        .map(
            |range| match map.iter().find(|(from, _)| from.contains(&range.start)) {
                None => range.clone(),
                Some((from, to)) => {
                    if from == range {
                        to.clone()
                    } else if from.start < range.start && from.end == range.end {
                        (to.start + (range.start - from.start))..to.end
                    } else if from.start == range.start && from.end > range.end {
                        to.start..(to.end - (from.end - range.end))
                    } else {
                        (to.start + (range.start - from.start))..(to.end - (from.end - range.end))
                    }
                }
            },
        )
        .collect_vec()
}

fn combine_ranges(ranges: Vec<Range<u64>>) -> Vec<Range<u64>> {
    let mut ranges = ranges;
    ranges.sort_by_key(|r| r.start);

    let mut combined_ranges = Vec::new();
    let mut current_range = ranges[0].clone();

    for range in ranges.iter().skip(1) {
        if range.start <= current_range.end {
            current_range.end = current_range.end.max(range.end);
        } else {
            combined_ranges.push(current_range);
            current_range = range.clone();
        }
    }

    combined_ranges.push(current_range);
    combined_ranges
}

fn parse(input: &str, seeds_are_ranges: bool) -> Almanac {
    let input: Vec<&str> = input.split("\n\n").collect();

    fn to_range_map(input: &str) -> Map {
        input
            .lines()
            .skip(1)
            .map(|line| {
                let values = line
                    .split_whitespace()
                    .map(str::parse)
                    .filter_map(Result::ok)
                    .collect::<Vec<u64>>();
                (
                    values[1]..(values[1] + values[2]),
                    values[0]..(values[0] + values[2]),
                )
            })
            .collect()
    }

    let seeds = input[0]
        .replace("seeds: ", "")
        .split_whitespace()
        .map(str::parse)
        .filter_map(Result::ok)
        .collect::<Vec<u64>>();

    let seeds = if seeds_are_ranges {
        Seeds::Ranges(
            seeds
                .iter()
                .copied()
                .tuples()
                .map(|(start, length)| start..(start + length))
                .collect(),
        )
    } else {
        Seeds::Single(seeds)
    };

    Almanac {
        seeds,
        seed_to_soil: to_range_map(input[1]),
        soil_to_fertilizer: to_range_map(input[2]),
        fertilizer_to_water: to_range_map(input[3]),
        water_to_light: to_range_map(input[4]),
        light_to_temperature: to_range_map(input[5]),
        temperature_to_humidity: to_range_map(input[6]),
        humidity_to_location: to_range_map(input[7]),
    }
}

#[aoc_generator(day5, part1)]
fn parse_part1(input: &str) -> Almanac {
    parse(input, false)
}

#[aoc_generator(day5, part2)]
fn parse_part2(input: &str) -> Almanac {
    parse(input, true)
}

#[aoc(day5, part1)]
fn part1(almanac: &Almanac) -> u64 {
    if let Seeds::Single(seeds) = &almanac.seeds {
        seeds
            .iter()
            .copied()
            .map(|seed| almanac.seed_to_soil.range_get(seed))
            .map(|soil| almanac.soil_to_fertilizer.range_get(soil))
            .map(|fertilizer| almanac.fertilizer_to_water.range_get(fertilizer))
            .map(|water| almanac.water_to_light.range_get(water))
            .map(|light| almanac.light_to_temperature.range_get(light))
            .map(|temperature| almanac.temperature_to_humidity.range_get(temperature))
            .map(|humidity| almanac.humidity_to_location.range_get(humidity))
            .min()
            .unwrap()
    } else {
        unreachable!()
    }
}

#[aoc(day5, part2)]
fn part2(almanac: &Almanac) -> u64 {
    if let Seeds::Ranges(seeds) = &almanac.seeds {
        combine_ranges(seeds.clone())
            .iter()
            .map(|range| map_ranges(range.clone(), &almanac.seed_to_soil))
            .fold(Vec::<Range<u64>>::new(), |mut acc, mut vec| {
                acc.append(&mut vec);
                combine_ranges(acc)
            })
            .iter()
            .map(|range| map_ranges(range.clone(), &almanac.soil_to_fertilizer))
            .fold(Vec::<Range<u64>>::new(), |mut acc, mut vec| {
                acc.append(&mut vec);
                combine_ranges(acc)
            })
            .iter()
            .map(|range| map_ranges(range.clone(), &almanac.fertilizer_to_water))
            .fold(Vec::<Range<u64>>::new(), |mut acc, mut vec| {
                acc.append(&mut vec);
                combine_ranges(acc)
            })
            .iter()
            .map(|range| map_ranges(range.clone(), &almanac.water_to_light))
            .fold(Vec::<Range<u64>>::new(), |mut acc, mut vec| {
                acc.append(&mut vec);
                combine_ranges(acc)
            })
            .iter()
            .map(|range| map_ranges(range.clone(), &almanac.light_to_temperature))
            .fold(Vec::<Range<u64>>::new(), |mut acc, mut vec| {
                acc.append(&mut vec);
                combine_ranges(acc)
            })
            .iter()
            .map(|range| map_ranges(range.clone(), &almanac.temperature_to_humidity))
            .fold(Vec::<Range<u64>>::new(), |mut acc, mut vec| {
                acc.append(&mut vec);
                combine_ranges(acc)
            })
            .iter()
            .map(|range| map_ranges(range.clone(), &almanac.humidity_to_location))
            .fold(Vec::<Range<u64>>::new(), |mut acc, mut vec| {
                acc.append(&mut vec);
                combine_ranges(acc)
            })
            .iter()
            .flat_map(|range| range.clone())
            .min()
            .unwrap()
    } else {
        unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const INPUT: &str = indoc! {"
        seeds: 79 14 55 13

        seed-to-soil map:
        50 98 2
        52 50 48

        soil-to-fertilizer map:
        0 15 37
        37 52 2
        39 0 15

        fertilizer-to-water map:
        49 53 8
        0 11 42
        42 0 7
        57 7 4

        water-to-light map:
        88 18 7
        18 25 70

        light-to-temperature map:
        45 77 23
        81 45 19
        68 64 13

        temperature-to-humidity map:
        0 69 1
        1 0 69

        humidity-to-location map:
        60 56 37
        56 93 4
    "};

    #[test]
    fn parse_part1_example() {
        let result = Almanac {
            seeds: Seeds::Single(vec![79, 14, 55, 13]),
            seed_to_soil: HashMap::from([(98..100, 50..52), (50..98, 52..100)]),
            soil_to_fertilizer: HashMap::from([(15..52, 0..37), (52..54, 37..39), (0..15, 39..54)]),
            fertilizer_to_water: HashMap::from([
                (53..61, 49..57),
                (11..53, 0..42),
                (0..7, 42..49),
                (7..11, 57..61),
            ]),
            water_to_light: HashMap::from([(18..25, 88..95), (25..95, 18..88)]),
            light_to_temperature: HashMap::from([
                (77..100, 45..68),
                (45..64, 81..100),
                (64..77, 68..81),
            ]),
            temperature_to_humidity: HashMap::from([(69..70, 0..1), (0..69, 1..70)]),
            humidity_to_location: HashMap::from([(56..93, 60..97), (93..97, 56..60)]),
        };

        assert_eq!(parse_part1(INPUT), result);
    }

    #[test]
    fn parse_part2_example() {
        let result = Almanac {
            seeds: Seeds::Ranges(vec![79..93, 55..68]),
            seed_to_soil: HashMap::from([(98..100, 50..52), (50..98, 52..100)]),
            soil_to_fertilizer: HashMap::from([(15..52, 0..37), (52..54, 37..39), (0..15, 39..54)]),
            fertilizer_to_water: HashMap::from([
                (53..61, 49..57),
                (11..53, 0..42),
                (0..7, 42..49),
                (7..11, 57..61),
            ]),
            water_to_light: HashMap::from([(18..25, 88..95), (25..95, 18..88)]),
            light_to_temperature: HashMap::from([
                (77..100, 45..68),
                (45..64, 81..100),
                (64..77, 68..81),
            ]),
            temperature_to_humidity: HashMap::from([(69..70, 0..1), (0..69, 1..70)]),
            humidity_to_location: HashMap::from([(56..93, 60..97), (93..97, 56..60)]),
        };

        assert_eq!(parse_part2(INPUT), result);
    }

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_part1(INPUT)), 35);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse_part2(INPUT)), 46);
    }

    #[test]
    fn test_map_ranges() {
        let map = Map::from([(2..5, 7..10), (7..10, 2..5)]);
        assert_eq!(map_ranges(0..2, &map), vec![0..2]);
        assert_eq!(map_ranges(0..5, &map), vec![0..2, 7..10]);
        assert_eq!(map_ranges(3..8, &map), vec![8..10, 5..7, 2..3]);
        assert_eq!(map_ranges(7..17, &map), vec![2..5, 10..17]);
        assert_eq!(map_ranges(11..17, &map), vec![11..17]);
    }

    #[test]
    fn test_combine_ranges() {
        assert_eq!(combine_ranges(vec![0..10, 15..20]), vec![0..10, 15..20]);
        assert_eq!(combine_ranges(vec![0..15, 5..10]), vec![0..15]);
        assert_eq!(combine_ranges(vec![0..10, 10..20]), vec![0..20]);
        assert_eq!(combine_ranges(vec![0..10, 5..15]), vec![0..15]);
    }
}
