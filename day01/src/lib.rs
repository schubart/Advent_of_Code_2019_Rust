pub fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|line| line.parse::<usize>().unwrap())
        .map(|mass| (mass / 3) - 2)
        .sum()
}

pub fn part2(input: &str) -> usize {
    input
        .lines()
        .map(|line| line.parse::<usize>().unwrap())
        .map(|mut mass| {
            let mut fuel = 0;

            while mass > 0 {
                mass = (mass / 3).saturating_sub(2);
                fuel += mass;
            }

            fuel
        })
        .sum()
}

#[test]
fn test_part1() {
    assert_eq!(2 + 2 + 654 + 33583, part1(include_str!("example.txt")));
    assert_eq!(3345909, part1(include_str!("input.txt")));
}

#[test]
fn test_part2() {
    assert_eq!(2 + 2 + 966 + 50346, part2(include_str!("example.txt")));
    assert_eq!(5015983, part2(include_str!("input.txt")));
}
