use aoc_runner_derive::{aoc, aoc_generator};
use grid::Grid;
use itertools::Itertools;
use std::cmp::max;
use std::collections::VecDeque;
use std::fmt::Debug;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Tiles {
    NorthSouth,
    EastWest,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
    Ground,
    Start,
    Fake,
}

impl Tiles {
    fn new_facing(&self, facing: Facing) -> Facing {
        match self {
            Tiles::NorthEast if facing == Facing::South => Facing::East,
            Tiles::NorthEast if facing == Facing::West => Facing::North,
            Tiles::NorthWest if facing == Facing::South => Facing::West,
            Tiles::NorthWest if facing == Facing::East => Facing::North,
            Tiles::SouthWest if facing == Facing::North => Facing::West,
            Tiles::SouthWest if facing == Facing::East => Facing::South,
            Tiles::SouthEast if facing == Facing::North => Facing::East,
            Tiles::SouthEast if facing == Facing::West => Facing::South,
            _ => facing,
        }
    }
}

impl From<char> for Tiles {
    fn from(value: char) -> Self {
        match value {
            '|' => Tiles::NorthSouth,
            '-' => Tiles::EastWest,
            'L' => Tiles::NorthEast,
            'J' => Tiles::NorthWest,
            '7' => Tiles::SouthWest,
            'F' => Tiles::SouthEast,
            '.' => Tiles::Ground,
            'S' => Tiles::Start,
            _ => panic!("Unknown Tile"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Facing {
    North,
    East,
    South,
    West,
}

impl Facing {
    fn step(&self, location: (usize, usize)) -> (usize, usize) {
        match self {
            Facing::North => (location.0 - 1, location.1),
            Facing::East => (location.0, location.1 + 1),
            Facing::South => (location.0 + 1, location.1),
            Facing::West => (location.0, location.1 - 1),
        }
    }

    fn valid_tile(&self, tile: Tiles) -> bool {
        match self {
            Facing::North => {
                tile == Tiles::NorthSouth || tile == Tiles::SouthWest || tile == Tiles::SouthEast
            }
            Facing::East => {
                tile == Tiles::EastWest || tile == Tiles::NorthWest || tile == Tiles::SouthWest
            }
            Facing::South => {
                tile == Tiles::NorthSouth || tile == Tiles::NorthEast || tile == Tiles::NorthWest
            }
            Facing::West => {
                tile == Tiles::EastWest || tile == Tiles::NorthEast || tile == Tiles::SouthEast
            }
        }
    }
}

#[aoc_generator(day10)]
fn parse(input: &str) -> Grid<Tiles> {
    let mut size = 0;
    Grid::from_vec(
        input
            .lines()
            .flat_map(|line| {
                size = line.len();
                line.chars().map(Tiles::from)
            })
            .collect(),
        size,
    )
}

#[aoc(day10, part1)]
fn part1(input: &Grid<Tiles>) -> u32 {
    let (start, _) = input
        .indexed_iter()
        .find(|(_, tile)| **tile == Tiles::Start)
        .unwrap();
    let (left, right) = get_valid_directions(input, start);

    let mut left = (left, left.step(start), 1);
    let mut right = (right, right.step(start), 1);
    while left.1 != right.1 {
        left.0 = input[left.1].new_facing(left.0);
        left.1 = left.0.step(left.1);
        left.2 += 1;

        right.0 = input[right.1].new_facing(right.0);
        right.1 = right.0.step(right.1);
        right.2 += 1;
    }

    max(left.2, right.2)
}

#[aoc(day10, part2)]
fn part2(input: &Grid<Tiles>) -> u32 {
    let (start, _) = input
        .indexed_iter()
        .find(|(_, tile)| **tile == Tiles::Start)
        .unwrap();
    let mut facing = get_valid_directions(input, start).0;

    let mut main_loop = vec![start, facing.step(start)];
    while main_loop.first().unwrap() != main_loop.last().unwrap() {
        let pos = *main_loop.last().unwrap();
        facing = input[pos].new_facing(facing);
        main_loop.push(facing.step(pos));
    }
    main_loop.pop();

    let mut grid = Grid::init(input.rows() * 2 + 2, input.cols() * 2 + 2, Tiles::Fake);
    input
        .indexed_iter()
        .map(|((row, col), _)| (row * 2 + 1, col * 2 + 1))
        .for_each(|pos| grid[pos] = Tiles::Ground);
    main_loop
        .iter()
        .copied()
        .map(|(row, col)| ((row, col), (row * 2 + 1, col * 2 + 1)))
        .for_each(|(old, new)| {
            let tile = input[old];
            grid[new] = tile;
            match tile {
                Tiles::NorthSouth => {
                    grid[(new.0 - 1, new.1)] = Tiles::NorthSouth;
                    grid[(new.0 + 1, new.1)] = Tiles::NorthSouth;
                }
                Tiles::EastWest => {
                    grid[(new.0, new.1 - 1)] = Tiles::EastWest;
                    grid[(new.0, new.1 + 1)] = Tiles::EastWest;
                }
                Tiles::NorthEast => {
                    grid[(new.0 - 1, new.1)] = Tiles::NorthSouth;
                    grid[(new.0, new.1 + 1)] = Tiles::EastWest;
                }
                Tiles::NorthWest => {
                    grid[(new.0 - 1, new.1)] = Tiles::NorthSouth;
                    grid[(new.0, new.1 - 1)] = Tiles::EastWest;
                }
                Tiles::SouthWest => {
                    grid[(new.0 + 1, new.1)] = Tiles::NorthSouth;
                    grid[(new.0, new.1 - 1)] = Tiles::EastWest;
                }
                Tiles::SouthEast => {
                    grid[(new.0 + 1, new.1)] = Tiles::NorthSouth;
                    grid[(new.0, new.1 + 1)] = Tiles::EastWest;
                }
                Tiles::Start => {
                    if old.0 > 0 && main_loop.contains(&(old.0 - 1, old.1)) {
                        grid[(new.0 - 1, new.1)] = Tiles::NorthSouth;
                    }
                    if old.0 < input.rows() - 1 && main_loop.contains(&(old.0 + 1, old.1)) {
                        grid[(new.0 + 1, new.1)] = Tiles::NorthSouth;
                    }
                    if old.1 > 0 && main_loop.contains(&(old.0, old.1 - 1)) {
                        grid[(new.0, new.1 - 1)] = Tiles::EastWest;
                    }
                    if old.1 < input.cols() - 1 && main_loop.contains(&(old.0, old.1 + 1)) {
                        grid[(new.0, new.1 + 1)] = Tiles::EastWest;
                    }
                }
                _ => {}
            }
        });

    let mut queue = VecDeque::new();
    queue.push_back((0, 0));
    while !queue.is_empty() {
        let node = queue.pop_front().unwrap();
        if grid[node] == Tiles::Ground || grid[node] == Tiles::Fake {
            grid[node] = Tiles::Start;
            if node.0 > 0 {
                queue.push_back((node.0 - 1, node.1));
            }
            if node.0 < grid.rows() - 1 {
                queue.push_back((node.0 + 1, node.1));
            }
            if node.1 > 0 {
                queue.push_back((node.0, node.1 - 1));
            }
            if node.1 < grid.cols() - 1 {
                queue.push_back((node.0, node.1 + 1));
            }
        }
    }

    grid.iter().filter(|tile| **tile == Tiles::Ground).count() as u32
}

fn get_valid_directions(grid: &Grid<Tiles>, pos: (usize, usize)) -> (Facing, Facing) {
    [Facing::North, Facing::East, Facing::South, Facing::West]
        .iter()
        .copied()
        .filter(|facing| match facing {
            Facing::North if pos.0 == 0 => false,
            Facing::East if pos.1 == grid.size().1 => false,
            Facing::South if pos.0 == grid.size().0 => false,
            Facing::West if pos.1 == 0 => false,
            _ => true,
        })
        .filter(|facing| facing.valid_tile(grid[facing.step(pos)]))
        .collect_tuple()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use grid::grid;
    use indoc::indoc;

    #[test]
    fn parse_example() {
        use super::Tiles::*;

        let input = indoc! {"
            -L|F7
            7S-7|
            L|7||
            -L-J|
            L|-JF
        "};

        let result = grid![
            [EastWest, NorthEast, NorthSouth, SouthEast, SouthWest]
            [SouthWest, Start, EastWest, SouthWest, NorthSouth]
            [NorthEast, NorthSouth, SouthWest, NorthSouth, NorthSouth]
            [EastWest, NorthEast, EastWest, NorthWest, NorthSouth]
            [NorthEast, NorthSouth, EastWest, NorthWest, SouthEast]
        ];

        assert_eq!(parse(input), result);

        let input = indoc! {"
            7-F7-
            .FJ|7
            SJLL7
            |F--J
            LJ.LJ
        "};

        let result = grid![
            [SouthWest, EastWest, SouthEast, SouthWest, EastWest]
            [Ground, SouthEast, NorthWest, NorthSouth, SouthWest]
            [Start, NorthWest, NorthEast, NorthEast, SouthWest]
            [NorthSouth, SouthEast, EastWest, EastWest, NorthWest]
            [NorthEast, NorthWest, Ground, NorthEast, NorthWest]
        ];

        assert_eq!(parse(input), result);
    }

    #[test]
    fn part1_example() {
        let input = indoc! {"
            .....
            .S-7.
            .|.|.
            .L-J.
            .....
        "};

        assert_eq!(part1(&parse(input)), 4);

        let input = indoc! {"
            ..F7.
            .FJ|.
            SJ.L7
            |F--J
            LJ...
        "};

        assert_eq!(part1(&parse(input)), 8);
    }

    #[test]
    fn part2_example() {
        let input = indoc! {"
            ...........
            .S-------7.
            .|F-----7|.
            .||.....||.
            .||.....||.
            .|L-7.F-J|.
            .|..|.|..|.
            .L--J.L--J.
            ...........
        "};

        assert_eq!(part2(&parse(input)), 4);

        let input = indoc! {"
            .F----7F7F7F7F-7....
            .|F--7||||||||FJ....
            .||.FJ||||||||L7....
            FJL7L7LJLJ||LJ.L-7..
            L--J.L7...LJS7F-7L7.
            ....F-J..F7FJ|L7L7L7
            ....L7.F7||L7|.L7L7|
            .....|FJLJ|FJ|F7|.LJ
            ....FJL-7.||.||||...
            ....L---J.LJ.LJLJ...
        "};

        assert_eq!(part2(&parse(input)), 8);

        let input = indoc! {"
            FF7FSF7F7F7F7F7F---7
            L|LJ||||||||||||F--J
            FL-7LJLJ||||||LJL-77
            F--JF--7||LJLJ7F7FJ-
            L---JF-JLJ.||-FJLJJ7
            |F|F-JF---7F7-L7L|7|
            |FFJF7L7F-JF7|JL---7
            7-L-JL7||F7|L7F-7F7|
            L.L7LFJ|||||FJL7||LJ
            L7JLJL-JLJLJL--JLJ.L
        "};

        assert_eq!(part2(&parse(input)), 10);
    }
}
