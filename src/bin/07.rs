advent_of_code::solution!(7);

use rayon::prelude::*;

fn parse_and_solve(input: &str, second_part: bool) -> Option<u64> {
    Some(
        input
            //parallelizing with rayon
            .par_lines()
            //.lines()
            .filter_map(|s| {
                let (first, second) = s.split_once(':').unwrap();
                let target: u64 = first.parse().expect("Cannot parse string");
                let values: Vec<u64> = second
                    .split_whitespace()
                    .map(|num_str| num_str.parse().unwrap())
                    .collect();
                if !second_part {
                    solve_first(target, &values, values[0], 1).then_some(target)
                } else {
                    solve_second(target, &values, values[0], 1).then_some(target)
                }
            })
            .sum(),
    )
}

fn solve_first(target: u64, values: &[u64], current_calc: u64, index: usize) -> bool {
    if index == values.len() {
        return target == current_calc;
    }

    let next_value = values[index];

    solve_first(target, values, current_calc + next_value, index + 1)
        || solve_first(target, values, current_calc * next_value, index + 1)
}

fn number_of_digits(number: u64) -> u32 {
    // log can't handle zero
    if number == 0 {
        0
    } else {
        (number as f64).log10() as u32 + 1
    }
}

fn concat(first: u64, second: u64) -> u64 {
    first * 10u64.pow(number_of_digits(second)) + second
}

fn solve_second(target: u64, values: &[u64], current_calc: u64, index: usize) -> bool {
    if index == values.len() {
        return target == current_calc;
    }

    let next_value = values[index];

    solve_second(target, values, current_calc + next_value, index + 1)
        || solve_second(target, values, current_calc * next_value, index + 1)
        || solve_second(target, values, concat(current_calc, next_value), index + 1)
}

pub fn part_one(input: &str) -> Option<u64> {
    parse_and_solve(input, false)
}

pub fn part_two(input: &str) -> Option<u64> {
    parse_and_solve(input, true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
    #[test]
    fn test_concat() {
        assert_eq!(concat(3, 12), 312);
    }
}
