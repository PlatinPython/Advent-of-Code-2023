use aoc_runner_derive::aoc;
use itertools::Itertools;

#[aoc(day15, part1)]
fn part1(input: &str) -> u32 {
    input.split(',').map(hash).sum()
}

#[aoc(day15, part2)]
fn part2(input: &str) -> usize {
    const EMPTY_VEC: Vec<(&str, u32)> = Vec::new();
    let mut boxes: [Vec<(&str, u32)>; 256] = [EMPTY_VEC; 256];
    input.split(',').for_each(|x| {
        if let Some(label) = x.strip_suffix('-') {
            let vec = &mut boxes[hash(label) as usize];
            if let Some((pos, _)) = vec.iter().find_position(|(l, _)| *l == label) {
                vec.remove(pos);
            }
        } else if let Some((label, focal)) = x.split_once('=') {
            let focal = focal.parse().unwrap();
            let vec = &mut boxes[hash(label) as usize];
            if let Some(elem) = vec.iter_mut().find(|(l, _)| *l == label) {
                elem.1 = focal;
            } else {
                vec.push((label, focal));
            }
        }
    });
    boxes
        .iter()
        .enumerate()
        .flat_map(|(pos, vec)| {
            let box_num = pos + 1;
            vec.iter()
                .enumerate()
                .map(move |(pos, (_, focal))| box_num * (pos + 1) * *focal as usize)
        })
        .sum()
}

fn hash(value: &str) -> u32 {
    value
        .chars()
        .map(|c| c as u32)
        .fold(0, |acc, x| ((acc + x) * 17) % 256)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        assert_eq!(part1(input), 1320);
    }

    #[test]
    fn part2_example() {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        assert_eq!(part2(input), 145);
    }

    #[test]
    fn hash_example() {
        assert_eq!(hash("HASH"), 52);
        assert_eq!(hash("rn=1"), 30);
        assert_eq!(hash("cm-"), 253);
        assert_eq!(hash("qp=3"), 97);
        assert_eq!(hash("cm=2"), 47);
        assert_eq!(hash("qp-"), 14);
        assert_eq!(hash("pc=4"), 180);
        assert_eq!(hash("ot=9"), 9);
        assert_eq!(hash("ab=5"), 197);
        assert_eq!(hash("pc-"), 48);
        assert_eq!(hash("pc=6"), 214);
        assert_eq!(hash("ot=7"), 231);
    }
}
