use std::collections::HashSet;

use itertools::Itertools;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    let part_1 = part_1(&input);
    let part_2 = part_2(&input);

    println!("Part 1: {part_1}, Part 2: {part_2}");
}

fn part_1(input: &str) -> usize {
    input
        .lines()
        .map(find_item_in_both_compartments)
        .map(priority)
        .sum()
}

#[test]
fn part_1_works() {
    let input = r#"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"#;

    assert_eq!(part_1(input), 157);
}

fn find_item_in_both_compartments(line: &str) -> char {
    let (left, right) = line.split_at(line.len() / 2);

    let left_items = left.chars().collect::<HashSet<_>>();

    right
        .chars()
        .find(|item| left_items.contains(item))
        .expect("No common item found")
}

#[test]
fn find_item_in_both_compartments_works() {
    assert_eq!(
        find_item_in_both_compartments("vJrwpWtwJgWrhcsFMMfFFhFp"),
        'p'
    );
    assert_eq!(
        find_item_in_both_compartments("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL"),
        'L'
    );
    assert_eq!(find_item_in_both_compartments("PmmdzqPrVvPwwTWBwg"), 'P');
    assert_eq!(
        find_item_in_both_compartments("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn"),
        'v'
    );
    assert_eq!(find_item_in_both_compartments("ttgJtRGJQctTZtZT"), 't');
    assert_eq!(
        find_item_in_both_compartments("CrZsJsPPZsGzwwsLwLmpwMDw"),
        's'
    );
}

const LOWERCASE_SUBTRACT: u8 = 'a' as u8 - 1;
const UPPERCASE_SUBTRACT: u8 = 'A' as u8 - 27;

fn priority(c: char) -> usize {
    if c.is_ascii_lowercase() {
        (c as u8 - LOWERCASE_SUBTRACT) as usize
    } else {
        (c as u8 - UPPERCASE_SUBTRACT) as usize
    }
}

#[test]
fn priority_works() {
    assert_eq!(priority('a'), 1);
    assert_eq!(priority('p'), 16);
    assert_eq!(priority('L'), 38);
    assert_eq!(priority('P'), 42);
    assert_eq!(priority('v'), 22);
    assert_eq!(priority('t'), 20);
    assert_eq!(priority('s'), 19);
}

fn part_2(input: &str) -> usize {
    input.lines().tuples().map(find_badge).map(priority).sum()
}

#[test]
fn part_2_works() {
    let input = r#"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"#;

    assert_eq!(part_2(input), 70);
}

fn find_badge((a, b, c): (&str, &str, &str)) -> char {
    let mut a = a.chars().collect::<HashSet<_>>();
    a.retain(|item| b.contains(*item));
    a.retain(|item| c.contains(*item));

    a.into_iter().next().expect("No badge found")
}

#[test]
fn find_badge_works() {
    assert_eq!(
        find_badge((
            "vJrwpWtwJgWrhcsFMMfFFhFp",
            "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
            "PmmdzqPrVvPwwTWBwg"
        )),
        'r'
    );
}
