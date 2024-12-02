advent_of_code::solution!(2);

fn parse_input(input: &str) -> Vec<Vec<u32>> {
    let vec: Vec<Vec<u32>> = input
        .lines()
        .map(|line| {
            let v = line
                .split_whitespace()
                .map(|num_str| num_str.parse().unwrap())
                .collect();
            v
        })
        .collect();

    vec
}

fn safe(levels: &[u32]) -> bool {
    let mut up = true;
    let mut down = true;
    let mut range = true;

    for w in levels.windows(2) {
        up &= w[0] < w[1];
        down &= w[0] > w[1];
        range &= (1..=3).contains(&w[0].abs_diff(w[1]));
    }

    //^ XOR
    (up ^ down) && range
}

fn solve(outer_v: Vec<Vec<u32>>, dampener: bool) -> usize {
    outer_v
        .iter()
        .filter_map(|row| {
            if safe(row) {
                Some(true)
            } else if dampener {
                let mut row_copy = row.clone();
                //remove one at a time and test again
                for i in 0..row_copy.len() {
                    let level = row_copy.remove(i);

                    if safe(&row_copy) {
                        return Some(true);
                    }

                    row_copy.insert(i, level);
                }
                None
            } else {
                None
            }
        })
        .count()
}

pub fn part_one(input: &str) -> Option<usize> {
    Some(solve(parse_input(input), false))
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(solve(parse_input(input), true))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
