use std::collections::HashMap;

advent_of_code::solution!(11);

const MAX_ITERATION_FIRST: u8 = 25;
const MAX_ITERATION_SECOND: u8 = 75;

const MULTIPLIER: u64 = 2024;
const TEN: u64 = 10;

fn split_number_in_half(number: u64, number_of_digits: u64) -> (u64, u64) {
    let second_half = number % (TEN.pow(number_of_digits as u32 / 2));
    let first_half = number / (TEN.pow(number_of_digits as u32 / 2));
    (first_half, second_half)
}

fn eval_value(
    cache: &mut HashMap<(u64, u8), u64>,
    value: u64,
    iteration: u8,
    max_iteration: u8,
) -> u64 {
    let mut ret = 0;
    let mut new_value = value;
    if iteration == max_iteration {
        return 1;
    }

    //check cash
    let key = (value, iteration);
    if let Some(&value) = cache.get(&key) {
        return value;
    }

    if value == 0 {
        new_value = 1;
    } else {
        let number_of_digits = (new_value.checked_ilog10().unwrap_or(0) + 1) as u64;
        if number_of_digits % 2 > 0 {
            new_value *= MULTIPLIER;
        }
        //split in half
        else {
            let (firs_half, second_half) = split_number_in_half(new_value, number_of_digits);
            ret += eval_value(cache, second_half, iteration + 1, max_iteration);
            new_value = firs_half;
        }
    }
    ret += eval_value(cache, new_value, iteration + 1, max_iteration);
    cache.insert((value, iteration), ret);
    ret
}

fn solve_recursive_with_cache(input: &str, max_iteration: u8) -> Option<u64> {
    let starting_numbers: Vec<u64> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let cache = &mut HashMap::with_capacity(5_000);

    Some(
        starting_numbers
            .into_iter()
            .map(|v| eval_value(cache, v, 0, max_iteration))
            .sum(),
    )
}

pub fn part_one(input: &str) -> Option<u64> {
    solve_recursive_with_cache(input, MAX_ITERATION_FIRST)
}

pub fn part_two(input: &str) -> Option<u64> {
    solve_recursive_with_cache(input, MAX_ITERATION_SECOND)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_number() {
        assert_eq!(split_number_in_half(244000, 6), (244, 0));
    }
    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(65601038650482));
    }
}
