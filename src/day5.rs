use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;
use std::ops::Range;

#[derive(Debug, PartialEq, Eq)]
struct Almanac {
    seeds: Vec<u64>,
    seed_to_soil: Map,
    soil_to_fertilizer: Map,
    fertilizer_to_water: Map,
    water_to_light: Map,
    light_to_temperature: Map,
    temperature_to_humidity: Map,
    humidity_to_location: Map,
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

pub fn map(x: u64, in_min: u64, in_max: u64, out_min: u64, out_max: u64) -> u64 {
    ((x - in_min) as f64 * (out_max - out_min) as f64 / (in_max - in_min) as f64) as u64 + out_min
}

#[aoc_generator(day5)]
fn parse(input: &str) -> Almanac {
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

    Almanac {
        seeds: input[0]
            .replace("seeds: ", "")
            .split_whitespace()
            .map(str::parse)
            .filter_map(Result::ok)
            .collect(),
        seed_to_soil: to_range_map(input[1]),
        soil_to_fertilizer: to_range_map(input[2]),
        fertilizer_to_water: to_range_map(input[3]),
        water_to_light: to_range_map(input[4]),
        light_to_temperature: to_range_map(input[5]),
        temperature_to_humidity: to_range_map(input[6]),
        humidity_to_location: to_range_map(input[7]),
    }
}

#[aoc(day5, part1)]
fn part1(almanac: &Almanac) -> u64 {
    almanac
        .seeds
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn parse_example() {
        let input = indoc! {"
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

        let result = Almanac {
            seeds: vec![79, 14, 55, 13],
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

        assert_eq!(parse(input), result);
    }

    #[test]
    fn part1_example() {
        let input = indoc! {"
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

        assert_eq!(part1(&parse(input)), 35);
    }
}
