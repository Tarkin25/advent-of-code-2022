use std::ops::RangeInclusive;

#[cfg(test)]
use indoc::indoc;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    println!("Part 1: {}, Part 2: {}", part_1(&input), part_2(&input));
}

fn part_1(input: &str) -> usize {
    input.lines()
    .map(parse_ranges)
    .filter(one_is_fully_contained)
    .count()
}

#[test]
fn part_1_works() {
    let input = indoc! {"
        2-4,6-8
        2-3,4-5
        5-7,7-9
        2-8,3-7
        6-6,4-6
        2-6,4-8"
    };

    assert_eq!(part_1(input), 2);
}

fn parse_ranges(input: &str) -> (RangeInclusive<usize>, RangeInclusive<usize>) {
    let (left, right) = input.split_once(',').expect("Expected ',' to separate ranges");

    (parse_range(left), parse_range(right))
}

#[test]
fn parse_ranges_works() {
    let (left, right) = parse_ranges("2-4,6-8");
    assert_eq!(left.start(), &2);
    assert_eq!(left.end(), &4);
    assert_eq!(right.start(), &6);
    assert_eq!(right.end(), &8);
}

fn parse_range(input: &str) -> RangeInclusive<usize> {
    let (lower, upper) = input.split_once('-').expect("Expected '-' to separate range bounds");
    
    lower.parse().unwrap()..=upper.parse().unwrap()
}

#[test]
fn parse_range_works() {
    let range = parse_range("2-4");
    assert_eq!(range.start(), &2);
    assert_eq!(range.end(), &4);
}

fn one_is_fully_contained((a, b): &(RangeInclusive<usize>, RangeInclusive<usize>)) -> bool {
    (a.start() <= b.start() && a.end() >= b.end()) ||
    (b.start() <= a.start() && b.end() >= a.end())
}

#[test]
fn one_is_fully_contained_works() {
    assert!(one_is_fully_contained(&(2..=8, 3..=7)));
    assert!(one_is_fully_contained(&(6..=6, 4..=6)));
}

fn part_2(input: &str) -> usize {
    input.lines()
    .map(parse_ranges)
    .filter(do_overlap)
    .count()
}

#[test]
fn part_2_works() {
    let input = indoc! {"
        2-4,6-8
        2-3,4-5
        5-7,7-9
        2-8,3-7
        6-6,4-6
        2-6,4-8"
    };

    assert_eq!(part_2(input), 4);
}

fn do_overlap((a, b): &(RangeInclusive<usize>, RangeInclusive<usize>)) -> bool {
    if a.start() <= b.end() {
        a.end() >= b.start()
    } else {
        b.end() >= a.start()
    }
}

#[test]
fn do_overlap_works() {
    assert!(!do_overlap(&(2..=4, 6..=8)));
    assert!(do_overlap(&(5..=7, 7..=9)));
}
