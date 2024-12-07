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
                //if solve(target, &values, values[0], 1, false) {
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
        assert_eq!(concat_values(3, 12), 312);
    }
}
