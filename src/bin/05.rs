use std::collections::HashSet;

advent_of_code::solution!(5);

fn solve_without_indexes(input: &str, part_two: bool) -> Option<u32> {
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
                let mut v: Vec<u32> = s
                    .split(',')
                    .map(|num_string| num_string.parse().unwrap())
                    .collect();

                let mut indexes_to_fix: Vec<(u32, u32)> = Vec::new();
                for i in 0..v.len() - 1 {
                    for j in i + 1..v.len() {
                        if hash_set.contains(&(v[j], v[i])) {
                            if !part_two {
                                return None;
                            } else {
                                indexes_to_fix.push((v[i], v[j]));
                            }
                        }
                    }
                }

                if part_two {
                    if indexes_to_fix.is_empty() {
                        return None;
                    } else {
                        for (first, second) in indexes_to_fix.into_iter() {
                            //try to move second before first
                            let second_index = v.iter().position(|x| *x == second).unwrap();
                            let first_index = v.iter().position(|x| *x == first).unwrap();

                            if second_index > first_index {
                                v.remove(second_index);
                                v.insert(first_index, second);
                            }
                        }
                    }
                }

                let index_in_the_middle = v.len() / 2;
                Some(v[index_in_the_middle])
            })
            .sum::<u32>(),
    )
}

pub fn part_one(input: &str) -> Option<u32> {
    solve_without_indexes(input, false)
}

pub fn part_two(input: &str) -> Option<u32> {
    solve_without_indexes(input, true)
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

/* part two doesn't work and it's not faster
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
                //println!("{s}");
                let mut v: Vec<u32> = s
                    .split(',')
                    .map(|num_string| num_string.parse().unwrap())
                    .collect();

                let mut indexes_to_fix: Vec<(usize, usize)> = Vec::new();
                for i in 0..v.len() - 1 {
                    for j in i + 1..v.len() {
                        if hash_set.contains(&(v[j], v[i])) {
                            //println!("Problem with : {} {} ({}, {})", v[i], v[j], i, j);
                            if !part_two {
                                return None;
                            } else {
                                indexes_to_fix.push((i, j));
                            }
                        }
                    }
                }

                //println!("{:?}", indexes_to_fix);

                if part_two {
                    if indexes_to_fix.is_empty() {
                        return None;
                    } else {
                        for k in 0..indexes_to_fix.len() {
                            let (i, j) = indexes_to_fix[k];

                            if i < j {
                                //println!("i: {} j: {}", i, j);
                                //if i >= v.len() {
                                //    println!("{:?}", v);
                                //}
                                let x = v.remove(j);
                                //println!("removed {}", x);
                                //if i > v.len() {
                                //    println!("i: {} j: {}", i, j);
                                //    println!("removed {}", x);
                                //    println!("{:?}", v);
                                //    println!("k :{}", k);
                                //}
                                v.insert(i, x);
                                //println!("{:?}", v);

                                //shift indexes
                                for l in k + 1..indexes_to_fix.len() {
                                    //if l == 29 {
                                    //    println!("i: {} j: {}", i, j);
                                    //    println!("{:?}", indexes_to_fix[l]);
                                    //}
                                    //all j become i
                                    //all i must be incremented
                                    if indexes_to_fix[l].0 == j {
                                        indexes_to_fix[l].0 = i
                                    } else if indexes_to_fix[l].0 == i {
                                        indexes_to_fix[l].0 += 1
                                    }

                                    if indexes_to_fix[l].1 == j {
                                        indexes_to_fix[l].1 = i
                                    } else if indexes_to_fix[l].1 == i {
                                        indexes_to_fix[l].1 += 1
                                    }
                                    //println!("{:?}", indexes_to_fix[l]);
                                }
                            }
                            //println!("{:?}", indexes_to_fix);
                        }
                    }
                }

                let index_in_the_middle = v.len() / 2;
                //println!("Returning: {}", v[index_in_the_middle]);
                Some(v[index_in_the_middle])
            })
            .sum::<u32>(),
    )
}
    */
