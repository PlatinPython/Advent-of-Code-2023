use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day3)]
fn parse(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

#[aoc(day3, part1)]
pub fn part1(input: &Vec<Vec<char>>) -> u32 {
    let mut sum = 0;
    for y in 0..input.len() {
        let mut x = 0;
        while x < input[y].len() {
            let mut current_number = String::new();
            let mut is_included = false;
            while x < input[y].len() && input[y][x].is_ascii_digit() {
                current_number.push(input[y][x]);

                let checks = get_checks(y, x);

                for (y, x) in checks {
                    if y.is_negative() || x.is_negative() {
                        continue;
                    }
                    if let Some(Some(c)) = input.get(y as usize).map(|v| v.get(x as usize)) {
                        is_included |= !c.is_ascii_digit() && *c != '.';
                    }
                }

                x += 1;
            }
            if is_included {
                sum += current_number.parse::<u32>().unwrap();
            }

            x += 1;
        }
    }

    sum
}

#[aoc(day3, part2)]
fn part2(input: &Vec<Vec<char>>) -> u32 {
    let mut sum = 0;

    for y in 0..input.len() {
        for x in 0..input[y].len() {
            if input[y][x] == '*' {
                let checks = get_checks(y, x);

                let mut vec = vec![];

                for (y, x) in checks {
                    if y.is_negative() || x.is_negative() {
                        continue;
                    }
                    if let Some(Some(c)) = input.get(y as usize).map(|v| v.get(x as usize)) {
                        if c.is_ascii_digit() {
                            vec.push((y, x));
                        }
                    }
                }

                let mut i = 0;
                while i < vec.len() {
                    let (y, x) = vec[i];
                    if vec.contains(&(y, x - 1)) || vec.contains(&(y, x + 1)) {
                        vec.remove(i);
                    } else {
                        i += 1;
                    }
                }

                if vec.len() != 2 {
                    continue;
                }

                sum += vec
                    .iter()
                    .map(|(y, mut x)| {
                        let mut num = String::new();

                        while x > 0 && input[*y as usize][(x - 1) as usize].is_ascii_digit() {
                            x -= 1;
                        }
                        while x < input[*y as usize].len() as i32
                            && input[*y as usize][x as usize].is_ascii_digit()
                        {
                            num.push(input[*y as usize][x as usize]);
                            x += 1;
                        }

                        num.parse::<u32>().unwrap()
                    })
                    .product::<u32>();
            }
        }
    }

    sum
}

fn get_checks(y: usize, x: usize) -> [(i32, i32); 8] {
    [
        (y as i32 - 1, x as i32 - 1),
        (y as i32 - 1, x as i32),
        (y as i32 - 1, x as i32 + 1),
        (y as i32, x as i32 - 1),
        (y as i32, x as i32 + 1),
        (y as i32 + 1, x as i32 - 1),
        (y as i32 + 1, x as i32),
        (y as i32 + 1, x as i32 + 1),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn parse_example() {
        let input = indoc! {"
            467..114..
            ...*......
            ..35..633.
            ......#...
            617*......
            .....+.58.
            ..592.....
            ......755.
            ...$.*....
            .664.598..
        "};

        let result = vec![
            vec!['4', '6', '7', '.', '.', '1', '1', '4', '.', '.'],
            vec!['.', '.', '.', '*', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '3', '5', '.', '.', '6', '3', '3', '.'],
            vec!['.', '.', '.', '.', '.', '.', '#', '.', '.', '.'],
            vec!['6', '1', '7', '*', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '+', '.', '5', '8', '.'],
            vec!['.', '.', '5', '9', '2', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '7', '5', '5', '.'],
            vec!['.', '.', '.', '$', '.', '*', '.', '.', '.', '.'],
            vec!['.', '6', '6', '4', '.', '5', '9', '8', '.', '.'],
        ];

        assert_eq!(parse(input), result);
    }

    #[test]
    fn part1_example() {
        let input = indoc! {"
            467..114..
            ...*......
            ..35..633.
            ......#...
            617*......
            .....+.58.
            ..592.....
            ......755.
            ...$.*....
            .664.598..
        "};

        assert_eq!(part1(&parse(input)), 4361);
    }

    #[test]
    fn part2_example() {
        let input = indoc! {"
            467..114..
            ...*......
            ..35..633.
            ......#...
            617*......
            .....+.58.
            ..592.....
            ......755.
            ...$.*....
            .664.598..
        "};

        assert_eq!(part2(&parse(input)), 467835);
    }
}
