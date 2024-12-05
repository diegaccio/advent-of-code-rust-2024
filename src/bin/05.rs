use std::collections::HashSet;

advent_of_code::solution!(5);

fn solve(input: &str, part_two: bool) -> Option<u32> {
    let (first_part, second_part) = input.split_once("\n\n").unwrap();

    let hash_set: HashSet<(u32, u32)> = first_part
        .lines()
        .map(|s| {
            let (left_s, right_s) = s.split_once("|").unwrap();
            (left_s.parse().unwrap(), right_s.parse().unwrap())
        })
        .collect();

    Some(
        second_part
            .lines()
            .filter_map(|s| {
                println!("{s}");
                let mut v: Vec<u32> = s
                    .split(',')
                    .map(|num_string| num_string.parse().unwrap())
                    .collect();

                let mut indexes_to_fix: Vec<(usize, usize)> = Vec::new();
                for i in 0..v.len() - 1 {
                    for j in i + 1..v.len() {
                        if hash_set.contains(&(v[j], v[i])) {
                            println!("Problem with : {} {} ({}, {})", v[i], v[j], i, j);
                            if !part_two {
                                return None;
                            } else {
                                indexes_to_fix.push((i, j));
                            }
                        }
                    }
                }

                if part_two {
                    if indexes_to_fix.is_empty() {
                        return None;
                    } else {
                        for k in 0..indexes_to_fix.len() {
                            let (i, j) = indexes_to_fix[k];
                            println!("i: {} j: {}", i, j);
                            let x = v.remove(j);
                            println!("removed {}", x);
                            v.insert(i, x);
                            println!("{:?}", v);

                            //shift indexes
                            for l in k + 1..indexes_to_fix.len() {
                                //all j become i
                                if indexes_to_fix[l].0 == j {
                                    indexes_to_fix[l].0 = i
                                }

                                if indexes_to_fix[l].1 == j {
                                    indexes_to_fix[l].1 = i
                                }

                                //all i must be incremented
                                if indexes_to_fix[l].0 == i {
                                    indexes_to_fix[l].0 += 1
                                }

                                if indexes_to_fix[l].1 == i {
                                    indexes_to_fix[l].1 += 1
                                }
                            }
                        }
                    }
                }

                let index_in_the_middle = v.len() / 2;
                println!("Returning: {}", v[index_in_the_middle]);
                Some(v[index_in_the_middle])
            })
            .sum::<u32>(),
    )
}

pub fn part_one(input: &str) -> Option<u32> {
    solve(input, false)
}

pub fn part_two(input: &str) -> Option<u32> {
    solve(input, true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
