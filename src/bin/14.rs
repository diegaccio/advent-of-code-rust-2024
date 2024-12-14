use std::str::FromStr;

use advent_of_code::utils::{grid::Grid, point::Point};

advent_of_code::solution!(14);

const ITERATIONS: i32 = 100;

const MAP_WIDTH: i32 = 101;
const MAP_HEIGTH: i32 = 103;

#[derive(Debug)]
struct Robot {
    starting_point: Point,
    velocity: Point,
}

#[derive(Debug)]
struct ParseRobotError;

//p=0,4 v=3,-3

impl FromStr for Robot {
    type Err = ParseRobotError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split_whitespace();
        Ok(Robot {
            starting_point: Point::from_str(iter.next().unwrap().strip_prefix("p=").unwrap())
                .map_err(|_| ParseRobotError)?,
            velocity: Point::from_str(iter.next().unwrap().strip_prefix("v=").unwrap())
                .map_err(|_| ParseRobotError)?,
        })
    }
}

fn parse(input: &str) -> Vec<Robot> {
    input
        .lines()
        .map(|s| Robot::from_str(s).expect("Cannot Parse Robot"))
        .collect()
}

fn parse_and_solve(input: &str) -> u32 {
    let robots = parse(input);

    let mut q1 = 0;
    let mut q2 = 0;
    let mut q3 = 0;
    let mut q4 = 0;

    for robot in robots.into_iter() {
        let x = (robot.starting_point.x + ITERATIONS * robot.velocity.x).rem_euclid(MAP_WIDTH);
        let y = (robot.starting_point.y + ITERATIONS * robot.velocity.y).rem_euclid(MAP_HEIGTH);

        if x < MAP_WIDTH / 2 {
            if y < MAP_HEIGTH / 2 {
                q1 += 1;
            }
            if y > MAP_HEIGTH / 2 {
                q2 += 1;
            }
        }
        if x > MAP_WIDTH / 2 {
            if y < MAP_HEIGTH / 2 {
                q3 += 1;
            }
            if y > MAP_HEIGTH / 2 {
                q4 += 1;
            }
        }
    }

    q1 * q2 * q3 * q4
}

fn parse_and_solve_second(input: &str) -> u32 {
    let robots = parse(input);

    let mut iteration: u32 = 0;

    for i in 1..10_000 {
        let mut q1: u32 = 0;
        let mut q2: u32 = 0;
        let mut q3: u32 = 0;
        let mut q4: u32 = 0;
        let mut grid: Grid<char> = Grid::new(MAP_WIDTH, MAP_HEIGTH, ' ');
        for robot in robots.iter() {
            let x = (robot.starting_point.x + i * robot.velocity.x).rem_euclid(MAP_WIDTH);
            let y = (robot.starting_point.y + i * robot.velocity.y).rem_euclid(MAP_HEIGTH);

            grid[Point::new(x, y)] = 'X';

            if x < MAP_WIDTH / 2 {
                if y < MAP_HEIGTH / 2 {
                    q1 += 1;
                }
                if y > MAP_HEIGTH / 2 {
                    q2 += 1;
                }
            }
            if x > MAP_WIDTH / 2 {
                if y < MAP_HEIGTH / 2 {
                    q3 += 1;
                }
                if y > MAP_HEIGTH / 2 {
                    q4 += 1;
                }
            }
        }

        //6532

        //small optimization to check the string only if 1/3rd of the robots
        //are condensed in 1 quadrant
        let robots_threshold = robots.len() as u32 / 3;

        if (q1 > robots_threshold
            || q2 > robots_threshold
            || q3 > robots_threshold
            || q4 > robots_threshold)
            && grid.line_contains_string("XXXXXXXXXXXX")
        {
            iteration = i as u32;

            //remove comment if you want to see the christmas tree
            //println!("{}\n{}\n", i, grid);
            break;
        }
    }

    iteration
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(parse_and_solve(input))
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(parse_and_solve_second(input))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(230900224));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6532));
    }
}
