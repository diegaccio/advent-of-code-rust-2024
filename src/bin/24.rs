use std::collections::VecDeque;

use advent_of_code::utils::hash::FastMap;

advent_of_code::solution!(24);

pub fn part_one(input: &str) -> Option<u64> {
    let (initial_values, gates) = input.split_once("\n\n")?;

    //parse line of initial value in the format x00: 1 to an hasmap with x00 as akey and 1 as value
    let mut values: FastMap<String, u32> = initial_values
        .lines()
        .map(|line| {
            let (key, value) = line.split_once(": ").unwrap();
            let key = key.replace(" ", "");
            let value = value.parse::<u32>().unwrap();
            (key, value)
        })
        .collect();

    //println!("{:?}", values);

    //parse line of gates in the format ntg XOR fgs -> mjb to a vect of tuples like this ("ntg", "XOR", "fgs", "mjb")
    let mut gates: VecDeque<(&str, &str, &str, &str)> = gates
        .lines()
        .map(|line| {
            let mut iter = line.split_whitespace();
            let first = iter.next().unwrap();
            let op = iter.next().unwrap();
            let second = iter.next().unwrap();
            let _ = iter.next();
            let result = iter.next().unwrap();

            (first, op, second, result)
        })
        .collect();

    //println!("{:?}", gates);

    while let Some((input1, gate, input2, output)) = gates.pop_front() {
        if values.contains_key(input1) && values.contains_key(input2) {
            let val1 = *values.get(input1).unwrap_or(&0);
            let val2 = *values.get(input2).unwrap_or(&0);
            let result = match gate {
                "AND" => val1 & val2,
                "OR" => val1 | val2,
                "XOR" => val1 ^ val2,
                _ => panic!("Unknown gate type: {}", gate),
            };
            values.insert(output.to_string(), result);
        } else {
            gates.push_back((input1, gate, input2, output));
        }
    }

    // Collect outputs starting with `z` and convert to a decimal number
    let mut output_bits: Vec<(usize, u64)> = values
        .iter()
        .filter(|(name, _)| name.starts_with('z'))
        .filter_map(|(name, &value)| {
            name[1..]
                .parse::<usize>()
                .ok()
                .map(|idx| (idx, value as u64))
        })
        .collect();

    output_bits.sort_by_key(|&(idx, _)| idx);

    let result = output_bits
        .iter()
        .rev()
        .fold(0, |acc, (_, bit)| (acc << 1) | bit);

    //println!("Output decimal number: {}", result);

    Some(result)
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2024));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
