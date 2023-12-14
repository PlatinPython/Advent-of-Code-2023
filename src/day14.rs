use aoc_runner_derive::{aoc, aoc_generator};
use grid::Grid;

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
    input
        .iter_cols()
        .map(|col| {
            let mut total = 0;
            let mut current = 0;
            for (i, x) in col.clone().enumerate().rev() {
                match x {
                    'O' => current += 1,
                    '#' => {
                        total +=
                            (((input.rows() - i) - current)..(input.rows() - i)).sum::<usize>();
                        current = 0;
                    }
                    _ => {}
                }
            }
            total += ((input.rows() + 1 - current)..input.rows() + 1).sum::<usize>();
            total
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

        let result = grid![
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
        ];

        assert_eq!(parse(input), result);
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
}
