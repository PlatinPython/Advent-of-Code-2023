use aoc_runner_derive::{aoc, aoc_generator};
use grid::Grid;
use itertools::Itertools;
use std::iter::StepBy;
use std::slice::Iter;

#[aoc_generator(day13)]
fn parse(input: &str) -> Vec<Grid<char>> {
    input
        .split("\n\n")
        .map(|grid| {
            let mut size = 0;
            Grid::from_vec(
                grid.lines()
                    .flat_map(|line| {
                        size = line.len();
                        line.chars()
                    })
                    .collect(),
                size,
            )
        })
        .collect()
}

#[aoc(day13, part1)]
fn part1(input: &[Grid<char>]) -> usize {
    input.iter().map(|grid| summarize(grid, None)).sum()
}

#[aoc(day13, part2)]
fn part2(input: &[Grid<char>]) -> usize {
    input
        .iter()
        .map(|grid| {
            let mut grid = grid.clone();
            let old = summarize(&grid, None);
            let mut new;
            let mut x = 0;
            let mut y = 0;
            loop {
                let old_char = match grid[(y, x)] {
                    '.' => {
                        grid[(y, x)] = '#';
                        '.'
                    }
                    '#' => {
                        grid[(y, x)] = '.';
                        '#'
                    }
                    _ => panic!("Unknown char"),
                };
                new = summarize(&grid, Some(old));
                if new != 0 {
                    break;
                }
                grid[(y, x)] = old_char;
                x = (x + 1) % grid.cols();
                if x == 0 {
                    y += 1;
                    if y == grid.rows() {
                        break;
                    }
                }
            }
            new
        })
        .sum()
}

fn summarize(grid: &Grid<char>, old: Option<usize>) -> usize {
    for i in 1..grid.cols() {
        if grid.iter_rows().all(|row| is_mirror(row, i)) {
            if let Some(old) = old {
                if old == i {
                    continue;
                }
            }
            return i;
        }
    }
    for i in 1..grid.rows() {
        if grid.iter_cols().all(|col| is_mirror(col, i)) {
            if let Some(old) = old {
                if old == i * 100 {
                    continue;
                }
            }
            return i * 100;
        }
    }
    0
}

fn is_mirror(line: StepBy<Iter<char>>, i: usize) -> bool {
    let line = line.collect_vec();
    let (left, right) = line.split_at(i);
    left.iter().rev().zip(right.iter()).all(|(a, b)| a == b)
}

#[cfg(test)]
mod tests {
    use super::*;
    use grid::grid;
    use indoc::indoc;

    #[test]
    fn parse_example() {
        let input = indoc! {"
            #.##..##.
            ..#.##.#.
            ##......#
            ##......#
            ..#.##.#.
            ..##..##.
            #.#.##.#.

            #...##..#
            #....#..#
            ..##..###
            #####.##.
            #####.##.
            ..##..###
            #....#..#
        "};

        let result = vec![
            grid![
                ['#', '.', '#', '#', '.', '.', '#', '#', '.']
                ['.', '.', '#', '.', '#', '#', '.', '#', '.']
                ['#', '#', '.', '.', '.', '.', '.', '.', '#']
                ['#', '#', '.', '.', '.', '.', '.', '.', '#']
                ['.', '.', '#', '.', '#', '#', '.', '#', '.']
                ['.', '.', '#', '#', '.', '.', '#', '#', '.']
                ['#', '.', '#', '.', '#', '#', '.', '#', '.']
            ],
            grid![
                ['#', '.', '.', '.', '#', '#', '.', '.', '#']
                ['#', '.', '.', '.', '.', '#', '.', '.', '#']
                ['.', '.', '#', '#', '.', '.', '#', '#', '#']
                ['#', '#', '#', '#', '#', '.', '#', '#', '.']
                ['#', '#', '#', '#', '#', '.', '#', '#', '.']
                ['.', '.', '#', '#', '.', '.', '#', '#', '#']
                ['#', '.', '.', '.', '.', '#', '.', '.', '#']
            ],
        ];

        assert_eq!(parse(input), result);
    }

    #[test]
    fn part1_example() {
        let input = indoc! {"
            #.##..##.
            ..#.##.#.
            ##......#
            ##......#
            ..#.##.#.
            ..##..##.
            #.#.##.#.

            #...##..#
            #....#..#
            ..##..###
            #####.##.
            #####.##.
            ..##..###
            #....#..#
        "};

        assert_eq!(part1(&parse(input)), 405);
    }

    #[test]
    fn part2_example() {
        let input = indoc! {"
            #.##..##.
            ..#.##.#.
            ##......#
            ##......#
            ..#.##.#.
            ..##..##.
            #.#.##.#.

            #...##..#
            #....#..#
            ..##..###
            #####.##.
            #####.##.
            ..##..###
            #....#..#
        "};

        assert_eq!(part2(&parse(input)), 400);
    }
}
