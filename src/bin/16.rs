use std::collections::{HashMap, HashSet, VecDeque};

use advent_of_code::utils::grid::Grid;
use advent_of_code::utils::point::*;

advent_of_code::solution!(16);

struct Tile {
    point: Point,
    direction: Point,
    score: u32,
}

fn update_min_score(mut score: u32, new_score: u32) -> u32 {
    if score == 0 || new_score < score {
        score = new_score;
    }

    score
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut map = Grid::parse(input);
    let mut bfs_queue = VecDeque::new();
    let current_point = map.find('S').unwrap();
    //remove the S to allow other paths through that point
    map[current_point] = '.';

    let mut visited = HashMap::new();

    let mut min_score = 0;
    bfs_queue.push_back(Tile {
        point: current_point,
        direction: RIGHT,
        score: 0,
    });

    while let Some(tile) = bfs_queue.pop_front() {
        if let Some(score) = visited.get(&(tile.point, tile.direction)) {
            if *score <= tile.score {
                continue;
            }
        }

        visited.insert((tile.point, tile.direction), tile.score);

        if map[tile.point + tile.direction] == 'E' {
            min_score = update_min_score(min_score, tile.score + 1)
        } else if map[tile.point + tile.direction.clockwise()] == 'E'
            || map[tile.point + tile.direction.counter_clockwise()] == 'E'
        {
            min_score = update_min_score(min_score, tile.score + 1001);
        } else {
            if map[tile.point + tile.direction] == '.' {
                bfs_queue.push_back(Tile {
                    point: tile.point + tile.direction,
                    direction: tile.direction,
                    score: tile.score + 1,
                });
            }
            if map[tile.point + tile.direction.clockwise()] == '.' {
                bfs_queue.push_back(Tile {
                    point: tile.point + tile.direction.clockwise(),
                    direction: tile.direction.clockwise(),
                    score: tile.score + 1001,
                });
            }
            if map[tile.point + tile.direction.counter_clockwise()] == '.' {
                bfs_queue.push_back(Tile {
                    point: tile.point + tile.direction.counter_clockwise(),
                    direction: tile.direction.counter_clockwise(),
                    score: tile.score + 1001,
                });
            }
            if map[tile.point + tile.direction.clockwise().clockwise()] == '.' {
                bfs_queue.push_back(Tile {
                    point: tile.point + tile.direction.clockwise().clockwise(),
                    direction: tile.direction.clockwise().clockwise(),
                    score: tile.score + 2001,
                });
            }
        }
    }

    Some(min_score)
    //134488 too high
    //108508 too high
    //106516 too high
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
        assert_eq!(result, Some(7036));

        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(11048));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
