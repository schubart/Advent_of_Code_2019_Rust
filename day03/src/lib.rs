use std::collections::HashMap;

type Dimension = i16;
type Position = (Dimension, Dimension);
type Distance = u32;
type Path = HashMap<Position, Distance>;

pub fn part1(input: &str) -> Dimension {
    let (path1, path2) = parse(input);

    path1.keys()
        .filter(|pos| path2.contains_key(pos))
        .map(|(x, y)| x.abs() + y.abs())
        .min()
        .unwrap()
}

pub fn part2(input: &str) -> Distance {
    let (path1, path2) = parse(input);

    path1.iter()
        .filter_map(|(pos, dist1)| path2.get(pos).map(|dist2| (dist1, dist2)))
        .map(|(dist1, dist2)| dist1 + dist2)
        .min()
        .unwrap()
}

fn path(line: &str) -> Path {
    let mut result = HashMap::new();

    let (mut x, mut y) = (0, 0);
    let mut distance = 0;
    for instruction in line.split(',') {
        let (dir, steps) = instruction.split_at(1);
        let steps = steps.parse().unwrap();

        for _ in 0..steps {
            (x, y) = match dir {
                "L" => (x - 1, y),
                "R" => (x + 1, y),
                "U" => (x, y - 1),
                "D" => (x, y + 1),
                _ => panic!(),
            };

            distance += 1;

            // Remember only the first (shortest) distance.
            result.entry((x, y)).or_insert(distance);
        }
    }

    result
}

fn parse(input: &str) -> (Path, Path) {
    let mut lines = input.lines();
    let path1 = path(lines.next().unwrap());
    let path2 = path(lines.next().unwrap());

    (path1, path2)
}

#[test]
fn test_part1() {
    assert_eq!(159, part1(include_str!("example1.txt")));
    assert_eq!(135, part1(include_str!("example2.txt")));
    assert_eq!(529, part1(include_str!("input.txt")));
}

#[test]
fn test_part2() {
    assert_eq!(610, part2(include_str!("example1.txt")));
    assert_eq!(410, part2(include_str!("example2.txt")));
    assert_eq!(20386, part2(include_str!("input.txt")));
}
