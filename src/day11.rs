use aoc_runner_derive::{aoc, aoc_generator};
use grid::Grid;
use itertools::Itertools;
use std::cmp::{max, min};

#[aoc_generator(day11)]
fn parse(input: &str) -> Grid<char> {
    let mut size = 0;
    Grid::from_vec(
        input
            .lines()
            .flat_map(|line| {
                size = line.len();
                line.chars()
            })
            .collect(),
        size,
    )
}

#[aoc(day11, part1)]
fn part1(input: &Grid<char>) -> usize {
    find_expanded_distances(input, 2)
}

#[aoc(day11, part2)]
fn part2(input: &Grid<char>) -> usize {
    find_expanded_distances(input, 1000000)
}

fn find_expanded_distances(grid: &Grid<char>, expansion: usize) -> usize {
    let expanded_cols = grid
        .iter_cols()
        .enumerate()
        .filter(|(_, col)| !col.clone().contains(&'#'))
        .map(|(i, _)| i)
        .collect_vec();

    let expanded_rows = grid
        .iter_rows()
        .enumerate()
        .filter(|(_, row)| !row.clone().contains(&'#'))
        .map(|(i, _)| i)
        .collect_vec();

    grid.indexed_iter()
        .filter(|(_, c)| **c == '#')
        .map(|(pos, _)| pos)
        .collect_vec()
        .iter()
        .tuple_combinations()
        .map(|((y1, x1), (y2, x2))| {
            let (x1, x2) = (min(x1, x2), max(x1, x2));
            let (y1, y2) = (min(y1, y2), max(y1, y2));
            ((x2 + expanded_cols.iter().filter(|&i| i > x1 && i < x2).count() * (expansion - 1))
                - x1)
                + ((y2
                    + expanded_rows.iter().filter(|&i| i > y1 && i < y2).count() * (expansion - 1))
                    - y1)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use grid::grid;
    use indoc::indoc;

    #[test]
    fn parse_example() {
        let input = indoc! {"
            ...#......
            .......#..
            #.........
            ..........
            ......#...
            .#........
            .........#
            ..........
            .......#..
            #...#.....
        "};

        let result = grid![
            ['.', '.', '.', '#', '.', '.', '.', '.', '.', '.']
            ['.', '.', '.', '.', '.', '.', '.', '#', '.', '.']
            ['#', '.', '.', '.', '.', '.', '.', '.', '.', '.']
            ['.', '.', '.', '.', '.', '.', '.', '.', '.', '.']
            ['.', '.', '.', '.', '.', '.', '#', '.', '.', '.']
            ['.', '#', '.', '.', '.', '.', '.', '.', '.', '.']
            ['.', '.', '.', '.', '.', '.', '.', '.', '.', '#']
            ['.', '.', '.', '.', '.', '.', '.', '.', '.', '.']
            ['.', '.', '.', '.', '.', '.', '.', '#', '.', '.']
            ['#', '.', '.', '.', '#', '.', '.', '.', '.', '.']
        ];

        assert_eq!(parse(input), result);
    }

    #[test]
    fn find_expanded_distances_example() {
        let input = indoc! {"
            ...#......
            .......#..
            #.........
            ..........
            ......#...
            .#........
            .........#
            ..........
            .......#..
            #...#.....
        "};

        let parse = parse(input);

        assert_eq!(find_expanded_distances(&parse, 2), 374);
        assert_eq!(find_expanded_distances(&parse, 10), 1030);
        assert_eq!(find_expanded_distances(&parse, 100), 8410);
    }
}
