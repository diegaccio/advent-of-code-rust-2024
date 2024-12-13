use std::str::FromStr;

advent_of_code::solution!(13);

struct Point {
    x: i128,
    y: i128,
}

struct ClawMachine {
    a: Point,
    b: Point,
    prize: Point,
}
#[derive(Debug)]
struct ParseMachineError;

//Button A: X+77, Y+52
//Button B: X+14, Y+32
//Prize: X=5233, Y=14652

impl FromStr for ClawMachine {
    type Err = ParseMachineError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        Ok(ClawMachine {
            a: Point::from_str(
                lines
                    .next()
                    .unwrap()
                    .strip_prefix("Button A: X+")
                    .unwrap()
                    .replace("Y+", "")
                    .as_str(),
            )?,
            b: Point::from_str(
                lines
                    .next()
                    .unwrap()
                    .strip_prefix("Button B: X+")
                    .unwrap()
                    .replace("Y+", "")
                    .as_str(),
            )?,
            prize: Point::from_str(
                lines
                    .next()
                    .unwrap()
                    .strip_prefix("Prize: X=")
                    .unwrap()
                    .replace("Y=", "")
                    .as_str(),
            )?,
        })
    }
}

//123, 345
impl FromStr for Point {
    type Err = ParseMachineError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s.split_once(", ").ok_or(ParseMachineError)?;
        let x = x.parse().map_err(|_| ParseMachineError)?;
        let y = y.parse().map_err(|_| ParseMachineError)?;
        Ok(Point { x, y })
    }
}

fn parse_input(input: &str) -> Vec<ClawMachine> {
    input
        .trim()
        .split("\n\n")
        .map(|s| ClawMachine::from_str(s).expect("Cannot Parse Machine"))
        .collect()
}

fn parse_and_sove(input: &str, prize_inc: i128) -> i128 {
    let games = parse_input(input);

    let mut tokens = 0;
    for ClawMachine { a, b, mut prize } in games {
        prize.x += prize_inc;
        prize.y += prize_inc;
        let num_a = prize.y * b.x - prize.x * b.y;
        let denom_a = a.y * b.x - a.x * b.y;
        if num_a % denom_a != 0 {
            continue;
        }

        let num_b = prize.y * a.x - prize.x * a.y;
        let denom_b = -denom_a;
        if num_b % denom_b != 0 {
            continue;
        }

        tokens += num_a / denom_a * 3 + num_b / denom_b;
    }

    tokens
}

pub fn part_one(input: &str) -> Option<i128> {
    Some(parse_and_sove(input, 0))
}

pub fn part_two(input: &str) -> Option<i128> {
    Some(parse_and_sove(input, 10_000_000_000_000))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(480));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(875_318_608_908));
    }
}
