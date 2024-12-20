use std::collections::VecDeque;
use std::sync::atomic::{AtomicU32, Ordering};

use advent_of_code::utils::grid::Grid;
use advent_of_code::utils::point::*;
use advent_of_code::utils::threads::spawn_batches;

advent_of_code::solution!(20);

const MAX_VALUE: u32 = u32::MAX;

pub struct Input {
    grid: Grid<char>,
    forward: Grid<i32>,
    reverse: Grid<i32>,
    full: i32,
}

fn bfs(grid: &Grid<char>, start: Point) -> Grid<i32> {
    let mut todo = VecDeque::new();
    let mut seen = grid.same_size_with(i32::MAX);

    todo.push_back((start, 0));
    seen[start] = 0;

    while let Some((position, cost)) = todo.pop_front() {
        let cost = cost + 1;

        for next in ORTHOGONAL.map(|o| position + o) {
            if grid[next] != '#' && cost < seen[next] {
                todo.push_back((next, cost));
                seen[next] = cost;
            }
        }
    }

    seen
}

pub fn parse(input: &str) -> Input {
    let grid = Grid::parse(input);
    let start = grid.find('S').unwrap();
    let end = grid.find('E').unwrap();

    let forward = bfs(&grid, start);
    let reverse = bfs(&grid, end);
    let full = forward[end];

    Input {
        grid,
        forward,
        reverse,
        full,
    }
}

fn parse_and_solve_part_1(input: &str) -> u32 {
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
    Some(parse_and_solve_part_1(input))
}

fn worker(input: &Input, total: &AtomicU32, batch: Vec<Point>) {
    let Input {
        grid,
        forward,
        reverse,
        full,
    } = input;
    let mut cheats = 0;

    for first in batch {
        for y in -20..21_i32 {
            for x in (y.abs() - 20)..(21 - y.abs()) {
                let second = first + Point::new(x, y);

                if grid.contains(second) && grid[second] != '#' {
                    let manhattan = x.abs() + y.abs();
                    let cost = forward[first] + reverse[second] + manhattan;

                    if *full - cost >= 100 {
                        cheats += 1;
                    }
                }
            }
        }
    }

    total.fetch_add(cheats, Ordering::Relaxed);
}

pub fn part_two(input: &str) -> Option<u32> {
    let input = parse(input);
    let Input { ref grid, .. } = input;
    let mut items = Vec::with_capacity(10_000);

    for y in 1..grid.height - 1 {
        for x in 1..grid.width - 1 {
            let point = Point::new(x, y);
            if grid[point] != '#' {
                items.push(point);
            }
        }
    }

    let total = AtomicU32::new(0);
    spawn_batches(items, |batch| worker(&input, &total, batch));
    Some(total.into_inner())
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
