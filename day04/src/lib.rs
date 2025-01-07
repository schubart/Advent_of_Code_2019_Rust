use std::iter::once;

use itertools::Itertools;

pub fn part1(min: usize, max: usize) -> usize {
    (min..=max)
        .map(|candidate| candidate.to_string().bytes().collect::<Vec<_>>())
        .filter(|digits| digits.iter().tuple_windows().all(|(d1, d2)| d1 <= d2))
        .filter(|digits| digits.iter().tuple_windows().any(|(d1, d2)| d1 == d2))
        .count()
}

pub fn part2(min: usize, max: usize) -> usize {
    (min..=max)
        .map(|candidate| candidate.to_string().bytes().map(Some).collect::<Vec<_>>())
        .filter(|digits| digits.iter().tuple_windows().all(|(d1, d2)| d1 <= d2))
        .filter(|digits| {
            let padded = once(None).chain(digits.iter().copied()).chain(once(None));
            padded.tuple_windows().any(|(d1, d2, d3, d4)| d1 != d2 && d2 == d3 && d3 != d4)
        })
        .count()
}

#[test]
fn test_part1() {
    assert_eq!(594, part1(347312, 805915));
}

#[test]
fn test_part2() {
    assert_eq!(364, part2(347312, 805915));
}
