use std::collections::VecDeque;

use advent_of_code::utils::{
    grid::Grid,
    point::{Point, ORTHOGONAL},
};

advent_of_code::solution!(10);

fn parse_and_solve(input: &str, allow_already_visited: bool) -> Option<u32> {
    let map = Grid::u32_parse(input);

    let mut starting_points = Vec::new();

    //I have 248 nines 311 zeros in my inpunt so I'll start at nines
    for y in 0..map.height {
        for x in 0..map.width {
            let point = Point::new(x, y);
            if map[point] == 9 {
                starting_points.push(point);
            }
        }
    }

    //https://en.wikipedia.org/wiki/Breadth-first_search with a queue as suggested by maneeatingape implementation

    let mut bfs_queue = VecDeque::new();
    let mut visited = map.same_size_with(u32::MAX);
    let mut result = 0;

    for (index, &start) in starting_points.iter().enumerate() {
        bfs_queue.push_back(start);

        while let Some(point) = bfs_queue.pop_front() {
            for next in ORTHOGONAL.map(|o| point + o) {
                if map.contains(next)
                    && map[next] + 1 == map[point]
                    && (allow_already_visited || (visited[next] != index as u32))
                {
                    if !allow_already_visited {
                        visited[next] = index as u32;
                    }

                    if map[next] == 0 {
                        result += 1;
                    } else {
                        bfs_queue.push_back(next);
                    }
                }
            }
        }
    }

    Some(result)
}

pub fn part_one(input: &str) -> Option<u32> {
    parse_and_solve(input, false)
}

pub fn part_two(input: &str) -> Option<u32> {
    parse_and_solve(input, true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }
}
