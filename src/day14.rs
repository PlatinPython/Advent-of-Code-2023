use aoc_runner_derive::{aoc, aoc_generator};
use grid::Grid;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

#[aoc_generator(day14)]
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

#[aoc(day14, part1)]
fn part1(input: &Grid<char>) -> usize {
    let mut input = input.clone();
    roll_north(&mut input);
    beam_load(&input)
}

#[derive(Clone, PartialEq, Eq)]
struct HashableGrid(Grid<char>);

impl Hash for HashableGrid {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.flatten().hash(state)
    }
}

#[aoc(day14, part2)]
fn part2(input: &Grid<char>) -> usize {
    let mut input = HashableGrid(input.clone());
    let mut map = HashMap::new();
    let mut i = 0;
    while !map.contains_key(&input) {
        map.insert(input.clone(), i);
        cycle(&mut input.0);
        i += 1;
    }
    let cycle_start = (1000000000 - i) % (i - map[&input]) + map[&input];
    if let Some((grid, _)) = map.iter().find(|(_, i)| **i == cycle_start) {
        beam_load(&grid.0)
    } else {
        0
    }
}

fn beam_load(grid: &Grid<char>) -> usize {
    grid.iter_cols()
        .map(|col| {
            let mut total = 0;
            for (i, x) in col.enumerate().rev() {
                if *x == 'O' {
                    total += grid.rows() - i
                }
            }
            total
        })
        .sum()
}

fn roll_north(grid: &mut Grid<char>) {
    for x in 0..grid.cols() {
        let mut current = 0;
        for y in (0..grid.rows()).rev() {
            match grid[(y, x)] {
                'O' => {
                    current += 1;
                    grid[(y, x)] = '.';
                }
                '#' => {
                    for y in (y + 1)..(y + 1 + current) {
                        grid[(y, x)] = 'O';
                    }
                    current = 0;
                }
                _ => {}
            }
        }
        for y in 0..current {
            grid[(y, x)] = 'O';
        }
    }
}

fn cycle(grid: &mut Grid<char>) {
    roll_north(grid);
    grid.rotate_right();
    roll_north(grid);
    grid.rotate_right();
    roll_north(grid);
    grid.rotate_right();
    roll_north(grid);
    grid.rotate_right();
}

#[cfg(test)]
mod tests {
    use super::*;
    use grid::grid;
    use indoc::indoc;
    use once_cell::sync::Lazy;

    static EXAMPLE_GRID: Lazy<Grid<char>> = Lazy::new(|| {
        grid![
            ['O', '.', '.', '.', '.', '#', '.', '.', '.', '.']
            ['O', '.', 'O', 'O', '#', '.', '.', '.', '.', '#']
            ['.', '.', '.', '.', '.', '#', '#', '.', '.', '.']
            ['O', 'O', '.', '#', 'O', '.', '.', '.', '.', 'O']
            ['.', 'O', '.', '.', '.', '.', '.', 'O', '#', '.']
            ['O', '.', '#', '.', '.', 'O', '.', '#', '.', '#']
            ['.', '.', 'O', '.', '.', '#', 'O', '.', '.', 'O']
            ['.', '.', '.', '.', '.', '.', '.', 'O', '.', '.']
            ['#', '.', '.', '.', '.', '#', '#', '#', '.', '.']
            ['#', 'O', 'O', '.', '.', '#', '.', '.', '.', '.']
        ]
    });

    #[test]
    fn parse_example() {
        let input = indoc! {"
            O....#....
            O.OO#....#
            .....##...
            OO.#O....O
            .O.....O#.
            O.#..O.#.#
            ..O..#O..O
            .......O..
            #....###..
            #OO..#....
        "};

        assert_eq!(parse(input), *EXAMPLE_GRID);
    }

    #[test]
    fn part1_example() {
        let input = indoc! {"
            O....#....
            O.OO#....#
            .....##...
            OO.#O....O
            .O.....O#.
            O.#..O.#.#
            ..O..#O..O
            .......O..
            #....###..
            #OO..#....
        "};

        assert_eq!(part1(&parse(input)), 136);
    }

    #[test]
    fn part2_example() {
        let input = indoc! {"
            O....#....
            O.OO#....#
            .....##...
            OO.#O....O
            .O.....O#.
            O.#..O.#.#
            ..O..#O..O
            .......O..
            #....###..
            #OO..#....
        "};

        assert_eq!(part2(&parse(input)), 64);
    }

    #[test]
    fn beam_load_example() {
        let grid = indoc! {"
            .....#....
            ....#...O#
            .....##...
            ...#......
            .....OOO#.
            .O#...O#.#
            ....O#...O
            ......OOOO
            #....###.O
            #.OOO#..OO
        "};

        assert_eq!(beam_load(&parse(grid)), 64);
    }

    #[test]
    fn roll_north_example() {
        let mut grid = EXAMPLE_GRID.clone();

        let result = grid![
            ['O', 'O', 'O', 'O', '.', '#', '.', 'O', '.', '.']
            ['O', 'O', '.', '.', '#', '.', '.', '.', '.', '#']
            ['O', 'O', '.', '.', 'O', '#', '#', '.', '.', 'O']
            ['O', '.', '.', '#', '.', 'O', 'O', '.', '.', '.']
            ['.', '.', '.', '.', '.', '.', '.', '.', '#', '.']
            ['.', '.', '#', '.', '.', '.', '.', '#', '.', '#']
            ['.', '.', 'O', '.', '.', '#', '.', 'O', '.', 'O']
            ['.', '.', 'O', '.', '.', '.', '.', '.', '.', '.']
            ['#', '.', '.', '.', '.', '#', '#', '#', '.', '.']
            ['#', '.', '.', '.', '.', '#', '.', '.', '.', '.']
        ];

        roll_north(&mut grid);
        assert_eq!(grid, result);
    }

    #[test]
    fn cycle_example() {
        let mut grid = EXAMPLE_GRID.clone();

        let grid_1_cycle = grid![
            ['.', '.', '.', '.', '.', '#', '.', '.', '.', '.']
            ['.', '.', '.', '.', '#', '.', '.', '.', 'O', '#']
            ['.', '.', '.', 'O', 'O', '#', '#', '.', '.', '.']
            ['.', 'O', 'O', '#', '.', '.', '.', '.', '.', '.']
            ['.', '.', '.', '.', '.', 'O', 'O', 'O', '#', '.']
            ['.', 'O', '#', '.', '.', '.', 'O', '#', '.', '#']
            ['.', '.', '.', '.', 'O', '#', '.', '.', '.', '.']
            ['.', '.', '.', '.', '.', '.', 'O', 'O', 'O', 'O']
            ['#', '.', '.', '.', 'O', '#', '#', '#', '.', '.']
            ['#', '.', '.', 'O', 'O', '#', '.', '.', '.', '.']
        ];

        let grid_2_cycle = grid![
            ['.', '.', '.', '.', '.', '#', '.', '.', '.', '.']
            ['.', '.', '.', '.', '#', '.', '.', '.', 'O', '#']
            ['.', '.', '.', '.', '.', '#', '#', '.', '.', '.']
            ['.', '.', 'O', '#', '.', '.', '.', '.', '.', '.']
            ['.', '.', '.', '.', '.', 'O', 'O', 'O', '#', '.']
            ['.', 'O', '#', '.', '.', '.', 'O', '#', '.', '#']
            ['.', '.', '.', '.', 'O', '#', '.', '.', '.', 'O']
            ['.', '.', '.', '.', '.', '.', '.', 'O', 'O', 'O']
            ['#', '.', '.', 'O', 'O', '#', '#', '#', '.', '.']
            ['#', '.', 'O', 'O', 'O', '#', '.', '.', '.', 'O']
        ];

        let grid_3_cycle = grid![
            ['.', '.', '.', '.', '.', '#', '.', '.', '.', '.']
            ['.', '.', '.', '.', '#', '.', '.', '.', 'O', '#']
            ['.', '.', '.', '.', '.', '#', '#', '.', '.', '.']
            ['.', '.', 'O', '#', '.', '.', '.', '.', '.', '.']
            ['.', '.', '.', '.', '.', 'O', 'O', 'O', '#', '.']
            ['.', 'O', '#', '.', '.', '.', 'O', '#', '.', '#']
            ['.', '.', '.', '.', 'O', '#', '.', '.', '.', 'O']
            ['.', '.', '.', '.', '.', '.', '.', 'O', 'O', 'O']
            ['#', '.', '.', '.', 'O', '#', '#', '#', '.', 'O']
            ['#', '.', 'O', 'O', 'O', '#', '.', '.', '.', 'O']
        ];

        cycle(&mut grid);
        assert_eq!(grid, grid_1_cycle);
        cycle(&mut grid);
        assert_eq!(grid, grid_2_cycle);
        cycle(&mut grid);
        assert_eq!(grid, grid_3_cycle);
    }
}
