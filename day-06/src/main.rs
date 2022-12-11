use std::collections::{HashSet, VecDeque};

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    println!("Part 1: {}, Part 2: {}", part_1(&input), part_2(&input));
}

fn part_1(input: &str) -> usize {
    let mut buffer = VecDeque::with_capacity(4);
    let mut chars = input.chars();
    let mut index = 0_usize;

    // setup
    for _ in 0..4 {
        buffer.push_back(chars.next().unwrap());
        index += 1;
    }

    if contains_all_unique_chars(&buffer) {
        return index;
    }

    while let Some(next) = chars.next() {
        buffer.pop_front();
        buffer.push_back(next);
        index += 1;

        if contains_all_unique_chars(&buffer) {
            return index;
        }
    }
    
    panic!("No start-of-packet marker found")
}

#[test]
fn part_1_works() {
    assert_eq!(part_1("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 7);
    assert_eq!(part_1("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
    assert_eq!(part_1("nppdvjthqldpwncqszvftbrmjlhg"), 6);
    assert_eq!(part_1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
    assert_eq!(part_1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
}

fn part_2(input: &str) -> usize {
    let mut buffer = VecDeque::with_capacity(14);
    let mut chars = input.chars();
    let mut index = 0_usize;

    // setup
    for _ in 0..14 {
        buffer.push_back(chars.next().unwrap());
        index += 1;
    }

    if contains_all_unique_chars(&buffer) {
        return index;
    }

    while let Some(next) = chars.next() {
        buffer.pop_front();
        buffer.push_back(next);
        index += 1;

        if contains_all_unique_chars(&buffer) {
            return index;
        }
    }
    
    panic!("No start-of-message marker found")
}

#[test]
fn part_2_works() {
    assert_eq!(part_2("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 19);
    assert_eq!(part_2("bvwbjplbgvbhsrlpgdmjqwftvncz"), 23);
    assert_eq!(part_2("nppdvjthqldpwncqszvftbrmjlhg"), 23);
    assert_eq!(part_2("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 29);
    assert_eq!(part_2("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 26);
}

fn contains_all_unique_chars(buffer: &VecDeque<char>) -> bool {
    buffer.iter().collect::<HashSet<_>>().len() == buffer.len()
}
