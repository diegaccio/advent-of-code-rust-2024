//! # Bridge Repair
//!
//! Great optimization taken from maneatingape
//!
//! The equations can be validated using a recursive solution that checks all possible combinations
//! of operators. However the number of states to check grows exponentially as 2ⁿ in part one
//! and 3ⁿ in part two.
//!
//! As much faster approach works in reverse from the end of the equation to prune as many states
//! as possible by checking which operations are possible. For example:
//!
//! ```none
//!     Test Value: 123456
//!     Equation: 2 617 56
//!     Addition is possible as 123456 >= 56
//!     Multiplication is not possible as 56 is not a factor of 123456
//!     Concatenation is possible as 1234 || 56 = 123456
//! ```
//!
//! Following the concatenation branch and applying an inverse concentation
//! (the full solution would also follow the addition branch):
//!
//! ```none
//!     Test Value: 1234
//!     Equation: 2 617
//!     Addition is possible as 1234 >= 617
//!     Multiplication is possible as 2 * 617 = 1234
//!     Concatenation is not possible as 1234 does not end in 617
//! ```
//!
//! Following the multiplication branch:
//!
//! ```none
//!     Test Value: 2
//!     Equation: 2
//! ```
//!
//! The test value is equal to the last term which means that the equation is valid.
//!
//! Inverse concenation can be implemented without time consuming conversion to or from
//! strings by dividing the left term by the next power of ten greater than the right term.
//! For example:
//!
//! * 7 || 9 => 79 => 79 / 10 => 7
//! * 12 || 34 => 1234 => 1234 / 100 => 12
//! * 123 || 789 => 123789 => 123789 / 1000 => 123
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
                if backward_solve(values[0], &values, target, values.len() - 1, false) {
                    Some(target)
                } else if second_part {
                    backward_solve(values[0], &values, target, values.len() - 1, true)
                        .then_some(target)
                } else {
                    None
                }
            })
            .sum(),
    )
}

fn next_power_of_ten(n: u64) -> u64 {
    if n < 10 {
        10
    } else if n < 100 {
        100
    } else {
        1000
    }
}

fn backward_solve(
    target: u64,
    values: &[u64],
    current_calc: u64,
    index: usize,
    allow_concat: bool,
) -> bool {
    if index == 0 {
        return target == current_calc;
    }

    let next_value = values[index];

    (allow_concat
        && (current_calc % next_power_of_ten(next_value) == next_value)
        && backward_solve(
            target,
            values,
            current_calc / next_power_of_ten(next_value),
            index - 1,
            allow_concat,
        ))
        || (current_calc >= next_value)
            && backward_solve(
                target,
                values,
                current_calc - next_value,
                index - 1,
                allow_concat,
            )
        || (current_calc % next_value == 0)
            && backward_solve(
                target,
                values,
                current_calc / next_value,
                index - 1,
                allow_concat,
            )
}

fn _solve(target: u64, values: &[u64], current_calc: u64, index: usize, concat: bool) -> bool {
    if current_calc > target {
        return false;
    }

    if index == values.len() {
        return target == current_calc;
    }

    let next_value = values[index];

    (concat
        && _solve(
            target,
            values,
            _concat_values(current_calc, next_value),
            index + 1,
            concat,
        ))
        || _solve(target, values, current_calc + next_value, index + 1, concat)
        || _solve(target, values, current_calc * next_value, index + 1, concat)
}

fn _number_of_digits(number: u64) -> u32 {
    // log can't handle zero
    if number == 0 {
        0
    } else {
        (number as f64).log10() as u32 + 1
    }
}

fn _concat_values(first: u64, second: u64) -> u64 {
    first * 10u64.pow(_number_of_digits(second)) + second
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
        assert_eq!(_concat_values(3, 12), 312);
    }
}
