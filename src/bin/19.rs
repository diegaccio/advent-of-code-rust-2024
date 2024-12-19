use advent_of_code::utils::hash::{FastMap, FastMapBuilder, FastSet};

advent_of_code::solution!(19);

fn parse_and_solve(input: &str, find_first: bool) -> Option<u64> {
    let (towels, designs) = parse(input);
    let mut cache = FastMap::new();

    Some(
        designs
            .iter()
            .map(|&t| count_allowed(t, &towels, &mut cache, find_first))
            .sum(),
    )
}

fn parse(input: &str) -> (FastSet<&str>, Vec<&str>) {
    let (first, second) = input.split_once("\n\n").unwrap();

    (first.split(", ").collect(), second.lines().collect())
}

fn count_allowed(
    design: &str,
    towels: &FastSet<&str>,
    cache: &mut FastMap<String, u64>,
    find_first: bool,
) -> u64 {
    if let Some(&result) = cache.get(design) {
        return result;
    }

    let mut combs = 0;
    for &p in towels {
        if design == p {
            combs += 1;
            if find_first {
                break;
            }
        }
        if design.starts_with(p) {
            let new_towel = design.replacen(p, "", 1);
            combs += count_allowed(&new_towel, towels, cache, find_first);
            if find_first && combs > 0 {
                combs = 1;
                break;
            }
        }
    }

    cache.insert(design.to_string(), combs);
    combs
}

pub fn part_one(input: &str) -> Option<u64> {
    parse_and_solve(input, true)
}

pub fn part_two(input: &str) -> Option<u64> {
    parse_and_solve(input, false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));

        /*let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(6));*/
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(16));
    }
}
