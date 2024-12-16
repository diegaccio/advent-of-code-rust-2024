use std::collections::VecDeque;

use advent_of_code::utils::grid::Grid;
use advent_of_code::utils::hash::{FastMap, FastMapBuilder, FastSet, FastSetBuilder};
use advent_of_code::utils::point::*;

advent_of_code::solution!(16);

struct Tile {
    point: Point,
    direction: Point,
    score: u32,
    previous_points: Vec<Point>,
}

fn parse_and_solve(input: &str, second_part: bool) -> u32 {
    let mut map = Grid::parse(input);
    let mut bfs_queue = VecDeque::new();
    let starting_point = map.find('S').unwrap();
    let ending_point = map.find('E').unwrap();
    //remove the S to allow other paths through that point
    map[starting_point] = '.';

    let mut visited: FastMap<(Point, Point), u32> = FastMap::with_capacity(20_000);

    let mut covered_tiles: FastSet<Point> = FastSet::with_capacity(1_000);

    let mut min_score = u32::MAX;

    let mut v = Vec::with_capacity(1_000);
    v.push(starting_point);

    bfs_queue.push_back(Tile {
        point: starting_point,
        direction: RIGHT,
        score: 0,
        previous_points: vec![starting_point],
    });

    while let Some(tile) = bfs_queue.pop_front() {
        if let Some(score) = visited.get(&(tile.point, tile.direction)) {
            if *score < tile.score || (!second_part && *score <= tile.score) {
                continue;
            }
        }

        //seen[tile.point][tile.direction] = tile.score;

        visited.insert((tile.point, tile.direction), tile.score);

        if map[tile.point + tile.direction] == 'E'
            || map[tile.point + tile.direction.clockwise()] == 'E'
            || map[tile.point + tile.direction.counter_clockwise()] == 'E'
        {
            let mut new_score = tile.score + 1001;
            if map[tile.point + tile.direction] == 'E' {
                new_score = tile.score + 1;
            }

            if second_part {
                if new_score < min_score {
                    covered_tiles.clear();
                }

                if new_score <= min_score {
                    covered_tiles.insert(ending_point);
                    covered_tiles.insert(tile.point);
                    for point in tile.previous_points {
                        covered_tiles.insert(point);
                    }
                }
            }

            min_score = min_score.min(new_score);
        } else {
            let mut v;
            if second_part {
                v = tile.previous_points;
                v.push(tile.point);
            } else {
                v = vec![];
            }
            if map[tile.point + tile.direction] == '.' {
                bfs_queue.push_back(Tile {
                    point: tile.point + tile.direction,
                    direction: tile.direction,
                    score: tile.score + 1,
                    previous_points: v.clone(),
                });
            }
            if map[tile.point + tile.direction.clockwise()] == '.' {
                bfs_queue.push_back(Tile {
                    point: tile.point + tile.direction.clockwise(),
                    direction: tile.direction.clockwise(),
                    score: tile.score + 1001,
                    previous_points: v.clone(),
                });
            }
            if map[tile.point + tile.direction.counter_clockwise()] == '.' {
                bfs_queue.push_back(Tile {
                    point: tile.point + tile.direction.counter_clockwise(),
                    direction: tile.direction.counter_clockwise(),
                    score: tile.score + 1001,
                    previous_points: v,
                });
            }
        }
    }

    if !second_part {
        min_score
    } else {
        covered_tiles.len() as u32
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(parse_and_solve(input, false))
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(parse_and_solve(input, true))
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
        assert_eq!(result, Some(45));

        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(64));
    }
}
