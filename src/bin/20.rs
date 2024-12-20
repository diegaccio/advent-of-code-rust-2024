use std::collections::{BinaryHeap, VecDeque};

use advent_of_code::utils::grid::Grid;
use advent_of_code::utils::point::*;

advent_of_code::solution!(20);

const MAX_VALUE: u32 = u32::MAX;

fn parse_and_solve(input: &str, _second_part: bool) -> u32 {
    let map = Grid::parse(input);
    let mut bfs_queue = VecDeque::new();
    let starting_point = map.find('S').unwrap();
    let ending_point = map.find('E').unwrap();

    //remove borders
    let mut visited = Grid::<u32>::new(map.width - 2, map.height - 2, MAX_VALUE);
    visited[starting_point + UP_LEFT] = 0;

    bfs_queue.push_back(starting_point);

    while let Some(point) = bfs_queue.pop_front() {
        if point == ending_point {
            break;
        }

        for next in ORTHOGONAL.map(|o| point + o) {
            if map.contains(next)
                && map[next] != '#'
                && visited[next + UP_LEFT] > visited[point + UP_LEFT] + 1
            {
                visited[next + UP_LEFT] = visited[point + UP_LEFT] + 1;
                bfs_queue.push_back(next);
                //println!("Visiting: {:?}, Distance: {}", next, visited[next]);
            }
        }
    }

    //println!("{}", visited);

    //let mut max_heap = BinaryHeap::with_capacity(7_000);

    let mut counter: u32 = 0;

    for x in 0..visited.width {
        for y in 0..visited.height {
            let current_point = Point::new(x, y);
            if visited[current_point] == MAX_VALUE {
                for p in ORTHOGONAL.into_iter() {
                    let mut savings = 0;
                    if visited.contains(current_point + p)
                        && visited.contains(current_point + p.invert())
                        && visited[current_point + p] < MAX_VALUE
                        && visited[current_point + p.invert()] < visited[current_point + p]
                    {
                        savings =
                            visited[current_point + p] - visited[current_point + p.invert()] - 2;
                        /*println!(
                            "1 - next: {} invert: {} savings: {}",
                            visited[current_point + p],
                            visited[current_point + p.invert()],
                            savings
                        );*/
                    }

                    if visited.contains(current_point + p * 2)
                        && visited.contains(current_point + p.invert())
                        && visited[current_point + p * 2] < MAX_VALUE
                        && visited[current_point + p.invert()] < visited[current_point + p * 2]
                        && visited[current_point + p * 2] - visited[current_point + p.invert()] - 3
                            > savings
                    {
                        savings = visited[current_point + p * 2]
                            - visited[current_point + p.invert()]
                            - 3;

                        /*println!(
                            "2 - next: {} invert: {} savings: {}",
                            visited[current_point + p * 2],
                            visited[current_point + p.invert()],
                            savings
                        );*/
                    }

                    if savings >= 100 {
                        counter += 1;
                    }
                }
            }
        }
    }

    counter
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(parse_and_solve(input, false))
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
    //Some(parse_and_solve(input, true))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(0));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
