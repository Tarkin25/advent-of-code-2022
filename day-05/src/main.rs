use std::collections::VecDeque;

use nom::{
    bytes::complete::tag,
    character::complete::{anychar, char, digit1, newline},
    combinator::{map_res, opt, recognize},
    multi::{count, many_till, separated_list0, separated_list1},
    sequence::{delimited, tuple},
    Finish, IResult,
};

#[cfg(test)]
use indoc::indoc;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    println!("Part 1: {}, Part 2: {}", part_1(&input), part_2(&input));
}

fn part_1(input: &str) -> String {
    let (_, (mut crates, instructions)) = parse_input(input).finish().unwrap();
    apply_instructions(&mut crates, &instructions);
    top_crates(&crates)
}

#[test]
fn part_1_works() {
    let input = indoc! {
        "    [D]
        [N] [C]    
        [Z] [M] [P]
         1   2   3 
        
        move 1 from 2 to 1
        move 3 from 1 to 3
        move 2 from 2 to 1
        move 1 from 1 to 2"
    };

    assert_eq!(part_1(input), "CMZ".to_owned());
}

fn part_2(input: &str) -> String {
    let (_, (mut crates, instructions)) = parse_input(input).finish().unwrap();
    apply_instructions_part_2(&mut crates, &instructions);
    top_crates(&crates)
}

#[test]
fn part_2_works() {
    let input = indoc! {
        "    [D]
        [N] [C]    
        [Z] [M] [P]
         1   2   3 
        
        move 1 from 2 to 1
        move 3 from 1 to 3
        move 2 from 2 to 1
        move 1 from 1 to 2"
    };

    assert_eq!(part_2(input), "MCD".to_owned());
}

fn apply_instructions(crates: &mut Vec<VecDeque<char>>, instructions: &[Instruction]) {
    for &Instruction { amount, from, to } in instructions {
        for _ in 0..amount {
            let c = crates[from - 1].pop_front().expect("No crate available");
            crates[to - 1].push_front(c);
        }
    }
}

fn apply_instructions_part_2(crates: &mut Vec<VecDeque<char>>, instructions: &[Instruction]) {
    for &Instruction { amount, from, to } in instructions {
        let mut moved_crates = VecDeque::with_capacity(amount);

        for _ in 0..amount {
            let c = crates[from - 1].pop_front().expect("No crate available");
            moved_crates.push_front(c);
        }

        while let Some(moved_crate) = moved_crates.pop_front() {
            crates[to - 1].push_front(moved_crate);
        }
    }
}

fn top_crates(crates: &[VecDeque<char>]) -> String {
    crates
        .into_iter()
        .map(|stack| *stack.front().expect("No crate in stack"))
        .collect()
}

fn parse_input(input: &str) -> IResult<&str, (Vec<VecDeque<char>>, Vec<Instruction>)> {
    let (input, crates) = parse_crates(input)?;
    let (input, _) = many_till(anychar, count(newline, 2))(input)?;
    let (input, instructions) = parse_instructions(input)?;

    Ok((input, (crates, instructions)))
}

#[test]
fn parse_input_works() {
    let input = indoc! {
        "    [D]
        [N] [C]    
        [Z] [M] [P]
         1   2   3 
        
        move 1 from 2 to 1
        move 3 from 1 to 3
        move 2 from 2 to 1
        move 1 from 1 to 2"
    };

    let (_, (_crates, instructions)) = parse_input(input).finish().unwrap();

    assert_eq!(
        instructions,
        vec![
            Instruction {
                amount: 1,
                from: 2,
                to: 1
            },
            Instruction {
                amount: 3,
                from: 1,
                to: 3
            },
            Instruction {
                amount: 2,
                from: 2,
                to: 1
            },
            Instruction {
                amount: 1,
                from: 1,
                to: 2
            },
        ]
    );
}

fn parse_crates(input: &str) -> IResult<&str, Vec<VecDeque<char>>> {
    let (input, lines) = separated_list1(newline, parse_line)(input)?;

    let mut crates = vec![VecDeque::new(); lines[0].len()];

    for line in lines {
        for (index, c) in line.into_iter().enumerate() {
            if let Some(c) = c {
                crates[index].push_back(c);
            }
        }
    }

    Ok((input, crates))
}

#[test]
fn parse_crates_works() {
    let input = indoc! {
        "    [D]
        [N] [C]    
        [Z] [M] [P]
         1   2   3"
    };

    let expected = vec![vec!['N', 'Z'], vec!['D', 'C', 'M'], vec!['P']];

    let (input, crates) = parse_crates(input).unwrap();

    assert_eq!(crates, expected);
    assert_eq!(input, "\n 1   2   3");
}

fn parse_line(input: &str) -> IResult<&str, Vec<Option<char>>> {
    let (input, crates) = separated_list1(tag(" "), try_parse_crate)(input)?;

    Ok((input, crates))
}

fn try_parse_crate(input: &str) -> IResult<&str, Option<char>> {
    let (input, c) = opt(parse_crate)(input)?;

    if c.is_some() {
        Ok((input, c))
    } else {
        let (input, _) = tag("   ")(input)?;
        Ok((input, None))
    }
}

fn parse_crate(input: &str) -> IResult<&str, char> {
    let (input, c) = delimited(char('['), anychar, char(']'))(input)?;

    Ok((input, c))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Instruction {
    amount: usize,
    from: usize,
    to: usize,
}

fn parse_instructions(input: &str) -> IResult<&str, Vec<Instruction>> {
    let (input, instructions) = separated_list0(newline, parse_instruction)(input)?;
    Ok((input, instructions))
}

fn parse_instruction(input: &str) -> IResult<&str, Instruction> {
    let (input, (_, amount, _, from, _, to)) = tuple((
        tag("move "),
        parse_usize,
        tag(" from "),
        parse_usize,
        tag(" to "),
        parse_usize,
    ))(input)?;

    Ok((input, Instruction { amount, from, to }))
}

fn parse_usize(input: &str) -> IResult<&str, usize> {
    map_res(recognize(digit1), str::parse)(input)
}
