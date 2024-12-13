use std::str::FromStr;

advent_of_code::solution!(13);

struct Point {
    x: i128,
    y: i128,
}

struct Machine {
    a: Point,
    b: Point,
    prize: Point,
}

#[derive(Debug, PartialEq, Eq)]
struct ParseMachineError;

impl FromStr for Machine {
    type Err = ParseMachineError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        Ok(Machine {
            a: parse_button(lines.next().unwrap().strip_prefix("Button A: ").unwrap()),
            b: parse_button(lines.next().unwrap().strip_prefix("Button B: ").unwrap()),
            prize: parse_prize(lines.next().unwrap().strip_prefix("Prize: ").unwrap()),
        })
    }
}

fn parse_button(s: &str) -> Point {
    let (x, y) = s.split_once(", ").unwrap();
    let x = x.strip_prefix("X+").unwrap().parse().unwrap();
    let y = y.strip_prefix("Y+").unwrap().parse().unwrap();
    Point { x, y }
}

fn parse_input(input: &str) -> Vec<Machine> {
    input
        .trim()
        .split("\n\n")
        .map(|s| Machine::from_str(s).expect("Cannot Parse Machine"))
        .collect()
}

fn parse_prize(s: &str) -> Point {
    let (x, y) = s.split_once(", ").unwrap();
    let x = x.strip_prefix("X=").unwrap().parse().unwrap();
    let y = y.strip_prefix("Y=").unwrap().parse().unwrap();
    Point { x, y }
}

fn parse_and_sove(input: &str, price_inc: i128) -> i128 {
    let games = parse_input(input);

    let mut tokens = 0;
    for Machine { a, b, mut prize } in games {
        prize.x += price_inc;
        prize.y += price_inc;
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
