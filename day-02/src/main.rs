fn main() {
    let input = std::fs::read_to_string("input/day_2.txt").unwrap();

    let part_1 = part_1(&input);
    let part_2 = part_2(&input);

    println!("Part 1: {part_1}, Part 2: {part_2}");
}

fn part_1(input: &str) -> usize {
    input
        .lines()
        .map(parse_shapes)
        .map(|(left, right)| right.score_against(left).score() + right.score_by_shape())
        .sum()
}

fn part_2(input: &str) -> usize {
    input
        .lines()
        .map(parse_shape_and_outcome)
        .map(|(shape, outcome)| {
            let needed_shape = shape.needed_for(outcome);
            outcome.score() + needed_shape.score_by_shape()
        })
        .sum()
}

#[derive(Debug, Clone, Copy)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug, Clone, Copy)]
enum Outcome {
    Loss,
    Draw,
    Win,
}

impl Outcome {
    fn score(self) -> usize {
        use Outcome::*;

        match self {
            Loss => 0,
            Draw => 3,
            Win => 6,
        }
    }
}

impl Shape {
    fn score_by_shape(self) -> usize {
        use Shape::*;

        match self {
            Rock => 1,
            Paper => 2,
            Scissors => 3,
        }
    }

    fn score_against(self, other: Self) -> Outcome {
        use Outcome::*;
        use Shape::*;

        match (self, other) {
            (Rock, Rock) => Draw,
            (Rock, Paper) => Loss,
            (Rock, Scissors) => Win,
            (Paper, Rock) => Win,
            (Paper, Paper) => Draw,
            (Paper, Scissors) => Loss,
            (Scissors, Rock) => Loss,
            (Scissors, Paper) => Win,
            (Scissors, Scissors) => Draw,
        }
    }

    fn needed_for(self, outcome: Outcome) -> Shape {
        use Outcome::*;
        use Shape::*;

        match (self, outcome) {
            (Rock, Loss) => Scissors,
            (Rock, Draw) => Rock,
            (Rock, Win) => Paper,
            (Paper, Loss) => Rock,
            (Paper, Draw) => Paper,
            (Paper, Win) => Scissors,
            (Scissors, Loss) => Paper,
            (Scissors, Draw) => Scissors,
            (Scissors, Win) => Rock,
        }
    }
}

fn parse_shapes(line: &str) -> (Shape, Shape) {
    use Shape::*;

    let (left, right) = line.split_once(" ").unwrap();

    let left = match left {
        "A" => Rock,
        "B" => Paper,
        "C" => Scissors,
        _ => unreachable!("invalid input"),
    };

    let right = match right {
        "X" => Rock,
        "Y" => Paper,
        "Z" => Scissors,
        _ => unreachable!("invalid input"),
    };

    (left, right)
}

fn parse_shape_and_outcome(line: &str) -> (Shape, Outcome) {
    use Outcome::*;
    use Shape::*;

    let (shape, outcome) = line.split_once(" ").unwrap();

    let shape = match shape {
        "A" => Rock,
        "B" => Paper,
        "C" => Scissors,
        _ => unreachable!("invalid input"),
    };

    let outcome = match outcome {
        "X" => Loss,
        "Y" => Draw,
        "Z" => Win,
        _ => unreachable!("invalid input"),
    };

    (shape, outcome)
}

#[test]
fn part_1_works() {
    assert_eq!(
        part_1(
            r#"A Y
B X
C Z"#
        ),
        15
    );
}

#[test]
fn part_2_works() {
    assert_eq!(
        part_2(
            r#"A Y
B X
C Z"#
        ),
        12
    );
}
