fn main() {
    let input = std::fs::read_to_string("input/day_1.txt").unwrap();

    let part_1 = part_1(&input);
    let part_2 = part_2(&input);

    println!("Part 1: {part_1}, Part 2: {part_2}");
}

fn part_1(input: &str) -> usize {
    input.split("\n\n")
    .map(|elf| elf.lines().map(|item| item.parse::<usize>().unwrap()).sum())
    .max()
    .unwrap()
}

fn part_2(input: &str) -> usize {
    let mut elves = input.split("\n\n")
    .map(|elf| elf.lines().map(|line| line.parse::<usize>().unwrap()).sum())
    .collect::<Vec<usize>>();

    elves.sort();

    elves.into_iter().rev().take(3).sum()
}

#[test]
fn part_1_works() {
    let input = r#"1000
2000
3000

4000

5000
6000

7000
8000
9000

10000"#;

    assert_eq!(part_1(input), 24000);
}

#[test]
fn part_2_works() {
    let input = r#"1000
2000
3000

4000

5000
6000

7000
8000
9000

10000"#;

    assert_eq!(part_2(input), 45000);
}